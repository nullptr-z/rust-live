package graph

import (
	"fmt"
	"go/ast"
	"go/token"
	"strings"

	"golang.org/x/tools/go/callgraph"
	"golang.org/x/tools/go/packages"
	"golang.org/x/tools/go/ssa"
)

// Builder builds the code graph from SSA and call graph
type Builder struct {
	fset        *token.FileSet
	pkgs        []*packages.Package
	projectPkgs map[string]bool   // project package paths (to filter out dependencies)
	nodeMap     map[string]int64  // maps function name to node ID
	insertFn    func(*Node) (int64, error)
	edgeFn      func(*Edge) error
}

// NewBuilder creates a new graph builder
func NewBuilder(
	fset *token.FileSet,
	pkgs []*packages.Package,
	insertFn func(*Node) (int64, error),
	edgeFn func(*Edge) error,
) *Builder {
	// Extract project package paths
	projectPkgs := make(map[string]bool)
	for _, pkg := range pkgs {
		if pkg.PkgPath != "" {
			projectPkgs[pkg.PkgPath] = true
		}
	}

	return &Builder{
		fset:        fset,
		pkgs:        pkgs,
		projectPkgs: projectPkgs,
		nodeMap:     make(map[string]int64),
		insertFn:    insertFn,
		edgeFn:      edgeFn,
	}
}

// isProjectFunction checks if a function belongs to the project (not a dependency)
func (b *Builder) isProjectFunction(fn *ssa.Function) bool {
	if fn.Pkg == nil {
		return false
	}
	pkgPath := fn.Pkg.Pkg.Path()
	return b.projectPkgs[pkgPath]
}

// Build processes the call graph and stores nodes/edges
func (b *Builder) Build(cg *callgraph.Graph) error {
	// First pass: create function nodes (only for project functions)
	for fn, node := range cg.Nodes {
		if fn == nil || node == nil {
			continue
		}

		// Skip synthetic functions (init, etc.) unless they have position info
		if fn.Synthetic != "" && fn.Pos() == token.NoPos {
			continue
		}

		// Skip functions not in the project (dependencies, stdlib)
		if !b.isProjectFunction(fn) {
			continue
		}

		nodeID, err := b.createFunctionNode(fn)
		if err != nil {
			return fmt.Errorf("failed to create node for %s: %w", fn.String(), err)
		}
		b.nodeMap[fn.String()] = nodeID
	}

	// Second pass: create call edges
	for fn, node := range cg.Nodes {
		if fn == nil || node == nil {
			continue
		}

		fromID, ok := b.nodeMap[fn.String()]
		if !ok {
			continue
		}

		for _, edge := range node.Out {
			if edge.Callee == nil || edge.Callee.Func == nil {
				continue
			}

			toID, ok := b.nodeMap[edge.Callee.Func.String()]
			if !ok {
				continue
			}

			// Get call site info
			var callSiteFile string
			var callSiteLine int
			if edge.Site != nil && edge.Site.Pos() != token.NoPos {
				pos := b.fset.Position(edge.Site.Pos())
				callSiteFile = pos.Filename
				callSiteLine = pos.Line
			}

			err := b.edgeFn(&Edge{
				FromID:       fromID,
				ToID:         toID,
				Kind:         EdgeKindCalls,
				CallSiteFile: callSiteFile,
				CallSiteLine: callSiteLine,
			})
			if err != nil {
				return fmt.Errorf("failed to create edge: %w", err)
			}
		}
	}

	return nil
}

// createFunctionNode creates a node for a function
func (b *Builder) createFunctionNode(fn *ssa.Function) (int64, error) {
	pos := b.fset.Position(fn.Pos())

	// Build signature string
	sig := fn.Signature.String()

	// Get doc comment if available
	doc := b.getDocComment(fn)

	// Determine package path
	pkgPath := ""
	if fn.Pkg != nil {
		pkgPath = fn.Pkg.Pkg.Path()
	}

	// Build fully qualified name
	name := fn.String()

	node := &Node{
		Kind:      NodeKindFunc,
		Name:      name,
		Package:   pkgPath,
		File:      pos.Filename,
		Line:      pos.Line,
		Signature: sig,
		Doc:       doc,
	}

	return b.insertFn(node)
}

// getDocComment extracts the doc comment for a function
func (b *Builder) getDocComment(fn *ssa.Function) string {
	if fn.Syntax() == nil {
		return ""
	}

	// Try to get the AST node
	switch syntax := fn.Syntax().(type) {
	case *ast.FuncDecl:
		if syntax.Doc != nil {
			return strings.TrimSpace(syntax.Doc.Text())
		}
	case *ast.FuncLit:
		// Function literals don't have doc comments
		return ""
	}

	return ""
}

// BuildStats returns statistics about the built graph
type BuildStats struct {
	TotalNodes int
	TotalEdges int
}

// GetNodeCount returns the number of nodes created
func (b *Builder) GetNodeCount() int {
	return len(b.nodeMap)
}

