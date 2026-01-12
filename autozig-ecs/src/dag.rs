//! DAG (Directed Acyclic Graph) System - 有向无环图系统
//! 用于系统调度和依赖管理

use std::collections::{HashMap, HashSet};

// ============================================================================
// DAG Core Types - DAG核心类型
// ============================================================================

/// NodeId - 节点ID（可以是系统ID或其他节点标识符）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeId {
    System(usize),
    Set(usize),
    Anonymous(usize),
}

impl NodeId {
    pub fn as_system(&self) -> Option<usize> {
        match self {
            Self::System(id) => Some(*id),
            _ => None,
        }
    }
    
    pub fn as_set(&self) -> Option<usize> {
        match self {
            Self::Set(id) => Some(*id),
            _ => None,
        }
    }
}

/// Direction - 图的方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Incoming,
    Outgoing,
}

/// Dag - 有向无环图
pub struct Dag {
    nodes: HashSet<NodeId>,
    edges: HashMap<NodeId, HashSet<NodeId>>,
    reverse_edges: HashMap<NodeId, HashSet<NodeId>>,
}

impl Dag {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
        }
    }
    
    pub fn add_node(&mut self, node: NodeId) {
        self.nodes.insert(node);
    }
    
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) -> Result<(), DagCrossDependencyError> {
        // 检查是否会创建循环
        if self.would_create_cycle(from, to) {
            return Err(DagCrossDependencyError {
                from,
                to,
            });
        }
        
        self.edges.entry(from).or_insert_with(HashSet::new).insert(to);
        self.reverse_edges.entry(to).or_insert_with(HashSet::new).insert(from);
        Ok(())
    }
    
    fn would_create_cycle(&self, from: NodeId, to: NodeId) -> bool {
        // 简化实现：检查从to是否能到达from
        self.has_path(to, from)
    }
    
    fn has_path(&self, from: NodeId, to: NodeId) -> bool {
        if from == to {
            return true;
        }
        
        let mut visited = HashSet::new();
        let mut stack = vec![from];
        
        while let Some(node) = stack.pop() {
            if node == to {
                return true;
            }
            
            if visited.insert(node) {
                if let Some(neighbors) = self.edges.get(&node) {
                    stack.extend(neighbors.iter().copied());
                }
            }
        }
        
        false
    }
    
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, DiGraphToposortError> {
        let mut result = Vec::new();
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        
        // 计算入度
        for &node in &self.nodes {
            in_degree.insert(node, 0);
        }
        for edges in self.edges.values() {
            for &to in edges {
                *in_degree.entry(to).or_insert(0) += 1;
            }
        }
        
        // 收集入度为0的节点
        let mut queue: Vec<NodeId> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(&node, _)| node)
            .collect();
        
        while let Some(node) = queue.pop() {
            result.push(node);
            
            if let Some(neighbors) = self.edges.get(&node) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(&neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(neighbor);
                        }
                    }
                }
            }
        }
        
        if result.len() != self.nodes.len() {
            return Err(DiGraphToposortError::CycleDetected);
        }
        
        Ok(result)
    }
    
    pub fn neighbors(&self, node: NodeId, direction: Direction) -> Vec<NodeId> {
        match direction {
            Direction::Outgoing => {
                self.edges.get(&node)
                    .map(|set| set.iter().copied().collect())
                    .unwrap_or_default()
            }
            Direction::Incoming => {
                self.reverse_edges.get(&node)
                    .map(|set| set.iter().copied().collect())
                    .unwrap_or_default()
            }
        }
    }
}

impl Default for Dag {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DAG Error Types - DAG错误类型
// ============================================================================

/// DagCrossDependencyError - DAG交叉依赖错误
#[derive(Debug, Clone)]
pub struct DagCrossDependencyError {
    pub from: NodeId,
    pub to: NodeId,
}

impl std::fmt::Display for DagCrossDependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cross dependency detected between {:?} and {:?}", self.from, self.to)
    }
}

impl std::error::Error for DagCrossDependencyError {}

/// DagOverlappingGroupError - DAG重叠组错误
#[derive(Debug, Clone)]
pub struct DagOverlappingGroupError {
    pub group_a: String,
    pub group_b: String,
    pub overlapping_nodes: Vec<NodeId>,
}

impl std::fmt::Display for DagOverlappingGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Overlapping groups: {} and {}, overlapping nodes: {:?}", 
               self.group_a, self.group_b, self.overlapping_nodes)
    }
}

impl std::error::Error for DagOverlappingGroupError {}

/// DagRedundancyError - DAG冗余错误
#[derive(Debug, Clone)]
pub struct DagRedundancyError {
    pub redundant_edge: (NodeId, NodeId),
}

impl std::fmt::Display for DagRedundancyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Redundant edge: {:?} -> {:?}", self.redundant_edge.0, self.redundant_edge.1)
    }
}

impl std::error::Error for DagRedundancyError {}

/// DiGraphToposortError - 有向图拓扑排序错误
#[derive(Debug, Clone)]
pub enum DiGraphToposortError {
    CycleDetected,
    InvalidNode(NodeId),
}

impl std::fmt::Display for DiGraphToposortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CycleDetected => write!(f, "Cycle detected in graph"),
            Self::InvalidNode(node) => write!(f, "Invalid node: {:?}", node),
        }
    }
}

impl std::error::Error for DiGraphToposortError {}

// ============================================================================
// DAG Analysis and Utilities - DAG分析和工具
// ============================================================================

/// DagAnalysis - DAG分析结果
pub struct DagAnalysis {
    pub node_count: usize,
    pub edge_count: usize,
    pub has_cycles: bool,
    pub longest_path: Vec<NodeId>,
}

impl DagAnalysis {
    pub fn analyze(dag: &Dag) -> Self {
        let node_count = dag.nodes.len();
        let edge_count = dag.edges.values().map(|set| set.len()).sum();
        let has_cycles = dag.topological_sort().is_err();
        let longest_path = Vec::new(); // 简化实现
        
        Self {
            node_count,
            edge_count,
            has_cycles,
            longest_path,
        }
    }
}

/// DagGroups - DAG节点分组
pub struct DagGroups {
    groups: HashMap<String, HashSet<NodeId>>,
}

impl DagGroups {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }
    
    pub fn add_to_group(&mut self, group_name: String, node: NodeId) {
        self.groups.entry(group_name).or_insert_with(HashSet::new).insert(node);
    }
    
    pub fn get_group(&self, group_name: &str) -> Option<&HashSet<NodeId>> {
        self.groups.get(group_name)
    }
    
    pub fn check_overlaps(&self) -> Vec<DagOverlappingGroupError> {
        let mut errors = Vec::new();
        let group_names: Vec<_> = self.groups.keys().cloned().collect();
        
        for i in 0..group_names.len() {
            for j in (i + 1)..group_names.len() {
                let group_a = &group_names[i];
                let group_b = &group_names[j];
                
                if let (Some(nodes_a), Some(nodes_b)) = 
                    (self.groups.get(group_a), self.groups.get(group_b)) {
                    let overlapping: Vec<_> = nodes_a.intersection(nodes_b).copied().collect();
                    
                    if !overlapping.is_empty() {
                        errors.push(DagOverlappingGroupError {
                            group_a: group_a.clone(),
                            group_b: group_b.clone(),
                            overlapping_nodes: overlapping,
                        });
                    }
                }
            }
        }
        
        errors
    }
}

impl Default for DagGroups {
    fn default() -> Self {
        Self::new()
    }
}

/// Edges - 边集合
pub struct Edges {
    edges: Vec<(NodeId, NodeId)>,
}

impl Edges {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }
    
    pub fn add(&mut self, from: NodeId, to: NodeId) {
        self.edges.push((from, to));
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &(NodeId, NodeId)> {
        self.edges.iter()
    }
    
    pub fn len(&self) -> usize {
        self.edges.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

impl Default for Edges {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Compact Representations - 紧凑表示
// ============================================================================

/// CompactNodeIdAndDirection - 紧凑的节点ID和方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactNodeIdAndDirection {
    data: u64, // 高32位存储节点ID，低位存储方向
}

impl CompactNodeIdAndDirection {
    pub fn new(node_id: u32, direction: Direction) -> Self {
        let dir_bit = match direction {
            Direction::Incoming => 0u64,
            Direction::Outgoing => 1u64,
        };
        Self {
            data: ((node_id as u64) << 32) | dir_bit,
        }
    }
    
    pub fn node_id(&self) -> u32 {
        (self.data >> 32) as u32
    }
    
    pub fn direction(&self) -> Direction {
        if (self.data & 1) == 0 {
            Direction::Incoming
        } else {
            Direction::Outgoing
        }
    }
}

/// CompactNodeIdPair - 紧凑的节点ID对（用于表示边）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactNodeIdPair {
    data: u64, // 高32位存储from，低32位存储to
}

impl CompactNodeIdPair {
    pub fn new(from: u32, to: u32) -> Self {
        Self {
            data: ((from as u64) << 32) | (to as u64),
        }
    }
    
    pub fn from(&self) -> u32 {
        (self.data >> 32) as u32
    }
    
    pub fn to(&self) -> u32 {
        (self.data & 0xFFFFFFFF) as u32
    }
}

// ============================================================================
// Graph Traits - 图trait
// ============================================================================

/// GraphNodeId - 图节点ID trait
pub trait GraphNodeId: Copy + Eq + std::hash::Hash {
    fn as_usize(&self) -> usize;
}

impl GraphNodeId for NodeId {
    fn as_usize(&self) -> usize {
        match self {
            NodeId::System(id) => *id,
            NodeId::Set(id) => *id + 1_000_000,
            NodeId::Anonymous(id) => *id + 2_000_000,
        }
    }
}

/// Internable - 可内部化trait（用于字符串池等）
pub trait Internable: Clone + Eq + std::hash::Hash {
    fn intern(&self) -> usize;
}

impl Internable for String {
    fn intern(&self) -> usize {
        // 简化实现：使用哈希值
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }
}