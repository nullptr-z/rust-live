/**
 * Mermaid graph rendering functions
 */

// Initialize Mermaid with modern dark theme
function initMermaid() {
  mermaid.initialize({
    startOnLoad: false,
    theme: 'base',
    themeVariables: {
      // 背景和基础色
      background: '#0d1117',
      primaryColor: '#1f2937',
      primaryTextColor: '#f3f4f6',
      primaryBorderColor: '#374151',
      // 线条
      lineColor: '#6b7280',
      // 次要色
      secondaryColor: '#111827',
      tertiaryColor: '#1f2937',
      // 文字
      nodeTextColor: '#f9fafb',
      textColor: '#e5e7eb',
      // 字体
      fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
      fontSize: '14px',
      // 边框
      nodeBorder: '#4b5563',
      clusterBkg: 'transparent',
      clusterBorder: '#374151',
      // 箭头
      arrowheadColor: '#9ca3af',
    },
    flowchart: {
      useMaxWidth: false,
      htmlLabels: true,
      curve: 'monotoneY',
      nodeSpacing: 30,
      rankSpacing: 40,
      padding: 8,
      diagramPadding: 12,
    },
    securityLevel: 'loose',
  });
}

// Generate and render Mermaid flowchart with hierarchical structure
// skipAutoFit: 如果为 true，则不自动适配缩放（用于展开节点时保持位置）
async function renderMermaidGraph(data, skipAutoFit = false) {
  const container = document.getElementById('mermaid-graph');
  const node = data.node;
  const callers = data.callers || [];
  const callees = data.callees || [];

  // Build mermaid code
  let lines = [];
  lines.push(`flowchart ${currentDirection}`);

  // Modern style definitions with gradients and rounded corners
  lines.push(`  %% Modern styles`);
  lines.push(`  classDef current fill:#0ea5e9,stroke:#38bdf8,color:#fff,stroke-width:2px,rx:12,ry:12`);
  lines.push(`  classDef caller fill:#6366f1,stroke:#818cf8,color:#fff,stroke-width:1px,rx:8,ry:8`);
  lines.push(`  classDef callee fill:#10b981,stroke:#34d399,color:#fff,stroke-width:1px,rx:8,ry:8`);
  // 叶子节点样式 - 虚线边框表示可展开
  lines.push(`  classDef leafCaller fill:#4f46e5,stroke:#a5b4fc,color:#fff,stroke-width:2px,stroke-dasharray:4 2,rx:8,ry:8`);
  lines.push(`  classDef leafCallee fill:#059669,stroke:#6ee7b7,color:#fff,stroke-width:2px,stroke-dasharray:4 2,rx:8,ry:8`);

  // Current node (center) - use stadium shape for emphasis
  const currentId = nodeIdMap[node.id];
  const currentLabel = escapeLabel(node.label);
  lines.push(`  ${currentId}(["${currentLabel}"]):::current`);

  // Track all displayed nodes for count and edges
  let displayedNodes = new Set([node.id]);
  let edges = [];
  let leafCallers = new Set(); // 叶子调用者节点（可展开上游）
  let leafCallees = new Set(); // 叶子被调用节点（可展开下游）
  let nodeInfoMap = new Map(); // 存储节点详情用于悬浮提示
  nodeInfoMap.set(nodeIdMap[node.id], node); // 当前节点

  // Collect all caller nodes and edges recursively
  function collectCallers(nodes, targetId) {
    nodes.forEach(caller => {
      const callerId = nodeIdMap[caller.id];
      if (!callerId) return;
      displayedNodes.add(caller.id);
      edges.push({ from: callerId, to: targetId });
      nodeInfoMap.set(callerId, caller); // 存储节点信息

      // Recurse into children or mark as leaf
      if (caller.children && caller.children.length > 0) {
        collectCallers(caller.children, callerId);
      } else {
        leafCallers.add(caller.id); // 没有 children，是叶子
      }
    });
  }

  // Collect all callee nodes and edges recursively
  function collectCallees(nodes, sourceId) {
    nodes.forEach(callee => {
      const calleeId = nodeIdMap[callee.id];
      if (!calleeId) return;
      displayedNodes.add(callee.id);
      edges.push({ from: sourceId, to: calleeId });
      nodeInfoMap.set(calleeId, callee); // 存储节点信息

      // Recurse into children or mark as leaf
      if (callee.children && callee.children.length > 0) {
        collectCallees(callee.children, calleeId);
      } else {
        leafCallees.add(callee.id); // 没有 children，是叶子
      }
    });
  }

  // Collect callers (they point TO current node, but we traverse UP from current)
  collectCallers(callers, currentId);

  // Collect callees (current node points TO them)
  collectCallees(callees, currentId);

  // Add all caller nodes (without subgraph for cleaner look)
  if (callers.length > 0) {
    addNodesRecursive(callers, lines, 'caller');
  }

  // Add all callee nodes
  if (callees.length > 0) {
    addNodesRecursive(callees, lines, 'callee');
  }

  // Add all edges with modern arrow style
  lines.push(`  %% Connections`);
  lines.push(`  linkStyle default stroke:#6b7280,stroke-width:2px`);
  edges.forEach(edge => {
    lines.push(`  ${edge.from} --> ${edge.to}`);
  });

  // 构建点击处理器映射（渲染后绑定）
  // 单击：只用于叶子节点展开（带 ⊕ 的节点）
  // 双击：所有节点都可以双击下钻导航
  const clickHandlers = new Map();  // 单击处理器（仅叶子节点展开）
  const dblClickHandlers = new Map();  // 双击处理器（所有节点导航）

  displayedNodes.forEach(id => {
    if (id !== node.id && nodeIdMap[id]) {
      // 双击：所有非当前节点都可以双击导航
      dblClickHandlers.set(nodeIdMap[id], () => selectNode(id));

      // 单击：只有可展开的叶子节点（非终端节点）才响应单击
      if (!terminalNodes.has(id)) {
        if (leafCallers.has(id)) {
          clickHandlers.set(nodeIdMap[id], () => expandLeafNode(id, 'callers'));
        } else if (leafCallees.has(id)) {
          clickHandlers.set(nodeIdMap[id], () => expandLeafNode(id, 'callees'));
        }
      }
    }
  });

  // Helper: add nodes recursively with rounded rectangle shape
  function addNodesRecursive(nodes, lines, styleClass) {
    nodes.forEach(n => {
      const nId = nodeIdMap[n.id];
      if (!nId) return;
      const label = escapeLabel(n.label);

      // 叶子节点使用特殊样式（虚线边框 + 展开图标）
      // 但如果节点已确认无法展开（在 terminalNodes 中），则使用普通样式
      const isLeaf = !n.children || n.children.length === 0;
      const isTerminal = terminalNodes.has(n.id);
      let actualStyle = styleClass;
      let displayLabel = label;

      if (isLeaf && !isTerminal) {
        // 可展开的叶子节点：显示虚线边框和展开图标
        actualStyle = styleClass === 'caller' ? 'leafCaller' : 'leafCallee';
        displayLabel = `${label} ⊕`;
      }
      // 终端节点或已展开的节点：使用普通样式，无展开图标

      lines.push(`    ${nId}["${displayLabel}"]:::${actualStyle}`);
      if (n.children && n.children.length > 0) {
        addNodesRecursive(n.children, lines, styleClass);
      }
    });
  }

  currentMermaidCode = lines.join('\n');

  // Render
  try {
    const { svg } = await mermaid.render('mermaid-svg', currentMermaidCode);
    container.innerHTML = svg;

    const svgEl = container.querySelector('svg');
    if (svgEl) {
      // 移除宽度限制，让图形显示实际大小
      svgEl.style.maxWidth = 'none';
      svgEl.style.height = 'auto';

      // 手动绑定点击事件到节点
      const tooltipEl = document.getElementById('node-tooltip');

      // 点击其他地方隐藏 tooltip
      document.addEventListener('click', (e) => {
        if (!tooltipEl.contains(e.target)) {
          tooltipEl.style.display = 'none';
        }
      });

      svgEl.querySelectorAll('.node').forEach(nodeEl => {
        // 获取节点 ID（Mermaid 生成的节点 ID 格式为 flowchart-nXXX-YYY）
        const nodeId = nodeEl.id;
        // 从 ID 中提取我们的节点标识 (n123)
        const match = nodeId.match(/flowchart-(n\d+)-/);
        if (match) {
          const mermaidId = match[1];

          // 单击事件处理
          const info = nodeInfoMap.get(mermaidId);
          const expandHandler = clickHandlers.get(mermaidId); // 叶子节点展开处理器

          nodeEl.style.cursor = 'pointer';
          nodeEl.addEventListener('click', (e) => {
            e.preventDefault();
            e.stopPropagation();

            // 如果是可展开的叶子节点，检测点击位置
            if (expandHandler) {
              const rect = nodeEl.getBoundingClientRect();
              const clickX = e.clientX;
              // 点击节点右侧 30px 区域（⊕ 图标区域）则展开
              if (clickX > rect.right - 30) {
                expandHandler();
                return;
              }
            }

            // 显示 tooltip
            if (info) {
              showNodeTooltip(tooltipEl, nodeEl, info);
            }
          });

          // 绑定双击事件（所有节点导航下钻）
          const dblClickHandler = dblClickHandlers.get(mermaidId);
          if (dblClickHandler) {
            nodeEl.style.cursor = 'pointer';
            nodeEl.addEventListener('dblclick', (e) => {
              e.preventDefault();
              e.stopPropagation();
              dblClickHandler();
            });
          }
        }
      });

      // 自动适配缩放：计算最佳初始缩放比例（展开节点时跳过）
      if (!skipAutoFit) {
        setTimeout(() => {
          autoFitZoom();
        }, 50);
      }
    }

    document.getElementById('visible-count').textContent = displayedNodes.size;
  } catch (err) {
    console.error('Mermaid render error:', err);
    container.innerHTML = `<div style="color: #f85149; padding: 20px;">渲染失败: ${err.message}</div>`;
  }
}

// Show tooltip for a node
function showNodeTooltip(tooltipEl, nodeEl, info) {
  let html = `<div class="tooltip-title">${info.fullName || info.label}</div>`;
  if (info.file) {
    html += `<div class="tooltip-row"><span>📁</span>${shortPath(info.file)}:${info.line}</div>`;
  }
  if (info.signature) {
    html += `<div class="tooltip-row"><span>📝</span>${info.signature}</div>`;
  }
  if (info.doc) {
    const docText = info.doc.length > 150 ? info.doc.substring(0, 150) + '...' : info.doc;
    html += `<div class="tooltip-doc">💬 ${docText}</div>`;
  }
  tooltipEl.innerHTML = html;
  tooltipEl.style.display = 'block';
  // 位置在节点下方
  const rect = nodeEl.getBoundingClientRect();
  let x = rect.left;
  let y = rect.bottom + 8;
  // 防止超出屏幕
  const maxX = window.innerWidth - tooltipEl.offsetWidth - 10;
  const maxY = window.innerHeight - tooltipEl.offsetHeight - 10;
  if (y > maxY) y = rect.top - tooltipEl.offsetHeight - 8;
  tooltipEl.style.left = Math.min(x, maxX) + 'px';
  tooltipEl.style.top = Math.max(10, y) + 'px';
}

// 展开叶子节点 - 获取其下一层调用关系并合并到当前视图
// direction: 'callers' 表示该节点在 callers 树中，需要继续往上游展开（获取谁调用了它）
// direction: 'callees' 表示该节点在 callees 树中，需要继续往下游展开（获取它调用了谁）
async function expandLeafNode(nodeId, direction) {
  try {
    // 获取叶子节点的调用链数据（深度1）
    const chainData = await fetchCallChain(nodeId, 1);

    // 根据方向获取要展开的节点
    // callers 树中的叶子节点：继续获取它的 callers（谁调用了它）
    // callees 树中的叶子节点：继续获取它的 callees（它调用了谁）
    const newNodes = direction === 'callers' ? chainData.callers : chainData.callees;

    if (!newNodes || newNodes.length === 0) {
      // 没有更多节点可展开，标记为终端节点并重新渲染去掉展开样式
      console.log(`节点 ${nodeId} 没有更多的 ${direction} 可展开`);
      terminalNodes.add(nodeId);

      // 重新渲染以更新样式（保持位置）
      await renderMermaidGraph(currentNodeData, true);
      return;
    }

    // 递归查找并更新叶子节点
    function findAndExpand(nodes, targetId, newChildren) {
      for (let i = 0; i < nodes.length; i++) {
        if (nodes[i].id === targetId) {
          // 找到目标节点，添加 children
          nodes[i].children = newChildren;
          return true;
        }
        if (nodes[i].children && nodes[i].children.length > 0) {
          if (findAndExpand(nodes[i].children, targetId, newChildren)) {
            return true;
          }
        }
      }
      return false;
    }

    // 在对应的调用链中查找并展开
    if (direction === 'callers') {
      findAndExpand(currentNodeData.callers, nodeId, newNodes);
    } else {
      findAndExpand(currentNodeData.callees, nodeId, newNodes);
    }

    // 重新渲染（保持当前缩放和位置）
    await renderMermaidGraph(currentNodeData, true);

  } catch (err) {
    console.error('Failed to expand leaf node:', err);
  }
}
