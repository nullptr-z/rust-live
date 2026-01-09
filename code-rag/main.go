package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
	"github.com/zheng/crag/internal/analyzer"
	"github.com/zheng/crag/internal/export"
	"github.com/zheng/crag/internal/graph"
	"github.com/zheng/crag/internal/mcp"
	"github.com/zheng/crag/internal/storage"
	"github.com/zheng/crag/internal/watcher"
	"github.com/zheng/crag/internal/web"
)

var (
	dbPath      string
	projectPath string
)

func main() {
	rootCmd := &cobra.Command{
		Use:   "crag",
		Short: "Code RAG - Go 代码调用图分析工具",
		Long:  "crag 是一个 Go 代码静态分析工具，用于构建函数调用图并分析代码变更影响范围。",
	}

	rootCmd.PersistentFlags().StringVar(&dbPath, "db", ".crag.db", "数据库文件路径")
	rootCmd.PersistentFlags().StringVar(&projectPath, "project", ".", "项目路径")

	rootCmd.AddCommand(
		indexCmd(),
		exportCmd(),
		serveCmd(),
		mcpCmd(),
		watchCmd(),
	)

	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func indexCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:     "index",
		Aliases: []string{"analyze"},
		Short:   "索引项目代码，构建调用图",
		RunE: func(cmd *cobra.Command, args []string) error {
			// Handle positional argument
			targetPath := projectPath
			if len(args) > 0 {
				targetPath = args[0]
			}

			// Load packages
			pkgs, err := analyzer.LoadPackages(targetPath)
			if err != nil {
				return fmt.Errorf("加载包失败: %w", err)
			}

			// Filter packages with source
			pkgs = analyzer.FilterMainPackages(pkgs)
			if len(pkgs) == 0 {
				return fmt.Errorf("未找到有效的 Go 包")
			}

			// Build SSA
			prog, _ := analyzer.BuildSSA(pkgs)

			// Build call graph
			cg, err := analyzer.BuildCallGraph(prog)
			if err != nil {
				return fmt.Errorf("构建调用图失败: %w", err)
			}

			// Open database
			db, err := storage.Open(dbPath)
			if err != nil {
				return fmt.Errorf("打开数据库失败: %w", err)
			}
			defer db.Close()

			// Clear existing data
			if err := db.Clear(); err != nil {
				return fmt.Errorf("清空数据库失败: %w", err)
			}

			// Build and store graph
			builder := graph.NewBuilder(
				prog.Fset,
				pkgs,
				db.InsertNode,
				db.InsertEdge,
			)

			if err := builder.Build(cg); err != nil {
				return fmt.Errorf("构建图失败: %w", err)
			}

			nodeCount, edgeCount, _ := db.GetStats()
			fmt.Printf("索引完成: %d 个函数, %d 条调用关系\n", nodeCount, edgeCount)
			return nil
		},
	}

	// Support positional argument for project path
	cmd.Args = cobra.MaximumNArgs(1)
	cmd.Flags().StringVarP(&dbPath, "output", "o", ".crag.db", "输出数据库文件路径")

	return cmd
}

func exportCmd() *cobra.Command {
	var outputFile string
	var projectName string

	cmd := &cobra.Command{
		Use:   "export",
		Short: "导出调用图为 Markdown 文档",
		RunE: func(cmd *cobra.Command, args []string) error {
			db, err := storage.Open(dbPath)
			if err != nil {
				return fmt.Errorf("打开数据库失败: %w", err)
			}
			defer db.Close()

			// Create output file
			var out *os.File
			if outputFile == "" || outputFile == "-" {
				out = os.Stdout
			} else {
				out, err = os.Create(outputFile)
				if err != nil {
					return fmt.Errorf("创建输出文件失败: %w", err)
				}
				defer out.Close()
			}

			exporter := export.NewExporter(db)
			opts := export.DefaultExportOptions()
			if projectName != "" {
				opts.ProjectName = projectName
			}

			if err := exporter.Export(out, opts); err != nil {
				return fmt.Errorf("导出失败: %w", err)
			}

			if outputFile != "" && outputFile != "-" {
				fmt.Printf("导出完成: %s\n", outputFile)
			}
			return nil
		},
	}

	cmd.Flags().StringVarP(&dbPath, "database", "d", ".crag.db", "数据库文件路径")
	cmd.Flags().StringVarP(&outputFile, "output", "o", "", "输出文件路径 (默认输出到标准输出)")
	cmd.Flags().StringVarP(&projectName, "name", "n", "", "项目名称")

	return cmd
}

func serveCmd() *cobra.Command {
	var port int
	cmd := &cobra.Command{
		Use:   "serve",
		Short: "启动 Web 服务器",
		RunE: func(cmd *cobra.Command, args []string) error {
			db, err := storage.Open(dbPath)
			if err != nil {
				return fmt.Errorf("打开数据库失败: %w", err)
			}
			defer db.Close()

			server := web.NewServer(db, port)
			fmt.Printf("Web 服务器启动在 http://localhost:%d\n", port)
			return server.Run()
		},
	}
	cmd.Flags().IntVarP(&port, "port", "p", 9998, "服务端口")
	return cmd
}

func mcpCmd() *cobra.Command {
	return &cobra.Command{
		Use:   "mcp",
		Short: "启动 MCP 服务器 (用于 Claude Code 集成)",
		RunE: func(cmd *cobra.Command, args []string) error {
			db, err := storage.Open(dbPath)
			if err != nil {
				return fmt.Errorf("打开数据库失败: %w", err)
			}
			defer db.Close()

			server := mcp.NewServer(db)
			return server.Run()
		},
	}
}

func watchCmd() *cobra.Command {
	return &cobra.Command{
		Use:   "watch",
		Short: "监视文件变化并自动更新索引",
		RunE: func(cmd *cobra.Command, args []string) error {
			w, err := watcher.New(projectPath, dbPath)
			if err != nil {
				return fmt.Errorf("创建监视器失败: %w", err)
			}

			fmt.Printf("开始监视 %s 的文件变化...\n", projectPath)
			w.Start()

			// Block forever
			select {}
		},
	}
}
