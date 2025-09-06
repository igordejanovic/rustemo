use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    fmt::Debug,
    rc::Rc,
};

use petgraph::{graph::Edges, prelude::*};

use crate::{
    context::Context, input::Input, lexer::Token, lr::builder::LRBuilder, parser::State,
    position::SourceSpan, Position,
};

/// Graph Structured Stack
///
/// Nodes keep information about state while edges keep all alternative
/// sub-trees constructed by reduction across the edge.
pub struct GssGraph<'i, I: Input + ?Sized, S, P, TK: Copy>(
    #[allow(clippy::type_complexity)] Graph<GssHead<'i, I, S, TK>, Rc<Parent<'i, I, P, TK>>>,
);

impl<I, S, P, TK> Default for GssGraph<'_, I, S, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'i, I, S, P, TK> GssGraph<'i, I, S, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_head(&mut self, head: GssHead<'i, I, S, TK>) -> NodeIndex {
        self.0.add_node(head)
    }

    #[inline]
    pub fn head(&self, head: NodeIndex) -> &GssHead<'i, I, S, TK> {
        self.0.node_weight(head).expect("Invalid Gss head index!")
    }

    #[inline]
    pub fn head_mut(&mut self, head: NodeIndex) -> &mut GssHead<'i, I, S, TK> {
        self.0
            .node_weight_mut(head)
            .expect("Invalid Gss head index!")
    }

    #[inline]
    pub fn parent(&self, index: EdgeIndex) -> Rc<Parent<'i, I, P, TK>> {
        self.0.edge_weight(index).unwrap().clone()
    }

    #[inline]
    pub fn add_parent(
        &mut self,
        start: NodeIndex,
        end: NodeIndex,
        parent: Rc<Parent<'i, I, P, TK>>,
    ) -> EdgeIndex {
        self.0.add_edge(start, end, parent)
    }

    /// Registers a new solution for the given parent link between start and end
    /// nodes.
    ///
    /// If the link doesn't exist create it.
    ///
    /// Returns EdgeIndex of the created link or None if no link is created.
    pub fn add_solution(
        &mut self,
        start: NodeIndex,
        end: NodeIndex,
        solution: Rc<SPPFTree<'i, I, P, TK>>,
    ) -> Option<EdgeIndex> {
        if let Some(edge) = self.edge_between(start, end) {
            self.parent(edge).possibilities.borrow_mut().push(solution);
            None
        } else {
            Some(self.add_parent(start, end, Rc::new(Parent::new(end, start, vec![solution]))))
        }
    }

    #[inline]
    pub fn backedges(&self, head: NodeIndex) -> Edges<'_, Rc<Parent<'i, I, P, TK>>, Directed> {
        self.0.edges_directed(head, Direction::Outgoing)
    }

    #[inline]
    pub fn start(&self, edge: EdgeIndex) -> NodeIndex {
        self.0
            .edge_endpoints(edge)
            .expect("Invalid Gss edge index!")
            .0
    }

    #[inline]
    pub fn end(&self, edge: EdgeIndex) -> NodeIndex {
        self.0
            .edge_endpoints(edge)
            .expect("Invalid Gss edge index!")
            .1
    }

    #[inline]
    pub fn edge_between(&self, start: NodeIndex, end: NodeIndex) -> Option<EdgeIndex> {
        self.0.find_edge(start, end)
    }
}

/// A node/head in the Graph Structured Stack (GSS). Implements [`Context`] for
/// GLR parsing.
///
/// Each head is related to a LR parser state and a single token ahead. Lexical
/// ambiguity, where a head may be followed by multiple different tokens, is
/// handled by splitting the head and using the same GLR mechanics for syntax
/// ambiguity handling. Effectively, we have per-token sub-frontiers.
#[derive(Debug)]
pub struct GssHead<'i, I, S, TK>
where
    I: Input + ?Sized,
{
    /// LR state reached when this node is created. Since LR state is related to
    /// grammar symbol this carries also an information is what is the last
    /// grammar symbol the parser has seen when reaching the current position.
    state: S,

    /// A frontier this node belongs to
    pub frontier: usize,

    /// Current position in the input
    position: Position,

    /// The span of token/non-terminal during shift/reduce operation.
    span: SourceSpan,

    /// Layout before the first token ahead
    layout_ahead: Option<&'i I>,

    /// Token found ahead of this node. At first it is initialized to `None`.
    /// Finding more than one token at the current position will split the head.
    token_ahead: Option<Token<'i, I, TK>>,
}

impl<I, S, TK> Clone for GssHead<'_, I, S, TK>
where
    I: Input + ?Sized,
    S: State,
    TK: Copy,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state,
            frontier: self.frontier,
            position: self.position,
            span: self.span,
            layout_ahead: self.layout_ahead,
            token_ahead: self.token_ahead().cloned(),
        }
    }
}

impl<I: Input + ?Sized, S: Default, TK> Default for GssHead<'_, I, S, TK> {
    fn default() -> Self {
        Self {
            state: Default::default(),
            frontier: Default::default(),
            position: I::start_position(),
            span: SourceSpan {
                start: I::start_position(),
                end: I::start_position(),
            },
            layout_ahead: Default::default(),
            token_ahead: Default::default(),
        }
    }
}

impl<'i, I, S, TK> GssHead<'i, I, S, TK>
where
    I: Input + ?Sized,
    S: State,
    TK: Copy,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        state: S,
        frontier: usize,
        position: Position,
        span: SourceSpan,
        layout_ahead: Option<&'i I>,
        token_ahead: Option<Token<'i, I, TK>>,
    ) -> Self {
        Self {
            state,
            frontier,
            position,
            span,
            layout_ahead,
            token_ahead,
        }
    }
    pub fn with_tok_state(&self, token_ahead: Token<'i, I, TK>, state: S) -> Self {
        Self {
            state,
            token_ahead: Some(token_ahead),
            ..*self
        }
    }
    pub fn with_tok(&self, token_ahead: Token<'i, I, TK>) -> Self {
        Self {
            token_ahead: Some(token_ahead),
            ..*self
        }
    }
}

impl<'i, S, I, TK> Context<'i, I, S, TK> for GssHead<'i, I, S, TK>
where
    I: Input + ?Sized,
    S: State,
{
    #[inline]
    fn state(&self) -> S {
        self.state
    }

    #[inline]
    fn set_state(&mut self, state: S) {
        self.state = state
    }

    #[inline]
    fn position(&self) -> Position {
        self.position
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        self.position = position
    }

    #[inline]
    fn span(&self) -> SourceSpan {
        self.span
    }

    #[inline]
    fn set_span(&mut self, span: SourceSpan) {
        self.span = span
    }

    #[inline]
    fn token_ahead(&self) -> Option<&Token<'i, I, TK>> {
        self.token_ahead.as_ref()
    }

    #[inline]
    fn set_token_ahead(&mut self, token: Token<'i, I, TK>) {
        self.token_ahead = Some(token)
    }

    #[inline]
    fn layout_ahead(&self) -> Option<&'i I> {
        self.layout_ahead
    }

    #[inline]
    fn set_layout_ahead(&mut self, layout: Option<&'i I>) {
        self.layout_ahead = layout
    }
}

/// A node of the Shared Packed Parse Forest (SPPF) (sub)tree
#[derive(Debug, Default)]
pub enum SPPFTree<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    Term {
        token: Token<'i, I, TK>,
        data: TreeData<'i, I>,
    },
    NonTerm {
        prod: P,
        data: TreeData<'i, I>,

        /// Child nodes determined by the production.
        /// References to Parent backlinks to support ambiguity as
        /// the parent links keeps all solutions along that back-path.
        children: RefCell<VecDeque<Rc<Parent<'i, I, P, TK>>>>,
    },
    // Empty Tree
    #[default]
    Empty,
}

impl<'i, I, P, TK> SPPFTree<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    fn solutions(&self) -> usize {
        match self {
            SPPFTree::Term { .. } => 1,
            SPPFTree::NonTerm { children, .. } => {
                children.borrow().iter().map(|p| p.solutions()).product()
            }
            SPPFTree::Empty => 0,
        }
    }

    #[allow(clippy::mutable_key_type)]
    fn ambiguities(&self, visited: &mut HashSet<Rc<Parent<'i, I, P, TK>>>) -> usize {
        match self {
            SPPFTree::Term { .. } | SPPFTree::Empty => 0,
            SPPFTree::NonTerm { children, .. } => children
                .borrow()
                .iter()
                .map(|p| {
                    if visited.contains(p) {
                        0
                    } else {
                        visited.insert(Rc::clone(p));
                        p.ambiguities(visited)
                    }
                })
                .sum(),
        }
    }
}

/// Implementation of Context trait for usage in Tree::build.
impl<'i, I, S, P, TK> Context<'i, I, S, TK> for SPPFTree<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
    S: State,
{
    fn state(&self) -> S {
        panic!("state() called on SPPFTree")
    }

    fn set_state(&mut self, _state: S) {}

    fn position(&self) -> Position {
        panic!("position() called on SPPFTree")
    }

    fn set_position(&mut self, _position: Position) {}

    fn span(&self) -> SourceSpan {
        match self {
            SPPFTree::Term { data, .. } | SPPFTree::NonTerm { data, .. } => data.span,
            _ => panic!("Called span() on empty tree!"),
        }
    }

    fn set_span(&mut self, _span: SourceSpan) {}

    fn token_ahead(&self) -> Option<&Token<'i, I, TK>> {
        None
    }

    fn set_token_ahead(&mut self, _token: Token<'i, I, TK>) {}

    fn layout_ahead(&self) -> Option<&'i I> {
        match self {
            SPPFTree::Term { data, .. } | SPPFTree::NonTerm { data, .. } => data.layout,
            _ => panic!("Called layout_ahead() on empty tree!"),
        }
    }

    fn set_layout_ahead(&mut self, _layout: Option<&'i I>) {}
}

impl<I, P, TK> Clone for SPPFTree<'_, I, P, TK>
where
    I: Input + ?Sized,
    P: Clone,
    TK: Copy,
{
    fn clone(&self) -> Self {
        match self {
            Self::Term { token, data } => Self::Term {
                token: token.clone(),
                data: data.clone(),
            },
            Self::NonTerm {
                prod,
                data,
                children,
            } => Self::NonTerm {
                prod: prod.clone(),
                data: data.clone(),
                children: children.clone(),
            },
            Self::Empty => Self::Empty,
        }
    }
}

/// Additional data shared by both term/non-term tree nodes.
#[derive(Debug)]
pub struct TreeData<'i, I>
where
    I: Input + ?Sized,
{
    pub span: SourceSpan,
    pub layout: Option<&'i I>,
}

impl<I> Clone for TreeData<'_, I>
where
    I: Input + ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            span: self.span,
            layout: self.layout,
        }
    }
}

/// Parent backlink in the GSS structure. Keeps all possibilities/ambiguities
/// between the root_node and the head_node.
#[derive(Debug)]
pub struct Parent<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    pub root_node: NodeIndex,
    pub head_node: NodeIndex,

    /// This models ambiguity. `RefCell` is needed as we need an Interior
    /// Mutability pattern to add new possibilities as they are discovered while
    /// keeping the rest of the structure immutable.
    pub possibilities: RefCell<Vec<Rc<SPPFTree<'i, I, P, TK>>>>,
}

impl<I, P, TK> PartialEq for Parent<'_, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.root_node == other.root_node && self.head_node == other.head_node
    }
}
impl<I, P, TK> Eq for Parent<'_, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
}

impl<I, P, TK> std::hash::Hash for Parent<'_, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.root_node.hash(state);
        self.head_node.hash(state);
    }
}

impl<'i, I, P, TK> Parent<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    pub fn new(
        root_node: NodeIndex,
        head_node: NodeIndex,
        possibilities: Vec<Rc<SPPFTree<'i, I, P, TK>>>,
    ) -> Self {
        Self {
            root_node,
            head_node,
            possibilities: RefCell::new(possibilities),
        }
    }

    /// Number of possible solutions in this parent link.
    ///
    /// If there >1 solutions we have ambiguity along the input span covered by
    /// this parent link.
    pub fn solutions(&self) -> usize {
        self.possibilities
            .borrow()
            .iter()
            .map(|n| n.solutions())
            .sum()
    }

    /// Number of ambiguous nodes in the span covered by this parent link.
    /// If there is more than one possibility this parent link is ambiguous.
    #[allow(clippy::mutable_key_type)]
    pub fn ambiguities(&self, visited: &mut HashSet<Rc<Parent<'i, I, P, TK>>>) -> usize {
        let ambiguity: usize = if self.possibilities.borrow().len() > 1 {
            1
        } else {
            0
        };

        ambiguity
            + self
                .possibilities
                .borrow()
                .iter()
                .map(|n| n.ambiguities(visited))
                .sum::<usize>()
    }
}

/// A wrapper type around `SPPFTree` structure to provide a view of a
/// specific tree which index is given in idx.
pub struct Tree<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    idx: usize,
    root: Rc<SPPFTree<'i, I, P, TK>>,
}

impl<I, P, TK> Debug for Tree<'_, I, P, TK>
where
    I: Input + ?Sized + Debug,
    TK: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.root {
            SPPFTree::Term { token, .. } => write!(f, "{:#?}", token.value),
            SPPFTree::NonTerm { .. } => write!(f, "{:#?}", self.children()),
            SPPFTree::Empty => write!(f, "EMPTY"),
        }
    }
}

impl<'i, I, P, TK> Tree<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    pub fn new(root: Rc<SPPFTree<'i, I, P, TK>>, idx: usize) -> Self {
        Self { root, idx }
    }

    /// Return child nodes by disambiguating SPPFTree parent links based on the
    /// current tree index and weighted numbering system.
    pub fn children(&self) -> Vec<Tree<'i, I, P, TK>> {
        match *self.root {
            SPPFTree::Term { .. } | SPPFTree::Empty => vec![],
            SPPFTree::NonTerm { ref children, .. } => {
                let mut tree_idx = self.idx;
                // Calculate counter division based on weighted numbering
                // system. Basically, enumerating variations of children
                // solutions.
                let weights = children
                    .borrow()
                    .iter()
                    .map(|c| c.solutions())
                    .collect::<Vec<_>>();
                children
                    .borrow()
                    .iter()
                    .enumerate()
                    .map(|(idx, child)| {
                        let factor: usize = weights[(idx + 1)..].iter().product();
                        let tree_idx_residual = tree_idx / factor;
                        tree_idx %= factor;
                        let (root, new_tree_idx) =
                            Self::find_tree_root(&child.possibilities.borrow(), tree_idx_residual)
                                .expect("Tree index must be valid.");
                        Tree::new(root, new_tree_idx)
                    })
                    .collect()
            }
        }
    }

    /// Build an output of the tree using the given builder.
    pub fn build<B: LRBuilder<'i, I, GssHead<'i, I, S, TK>, S, P, TK>, S>(
        &self,
        builder: &mut B,
    ) -> B::Output
    where
        S: State,
        P: Copy,
    {
        let mut context = GssHead::default();
        self.build_inner(&mut context, builder);
        builder.get_result()
    }

    fn build_inner<B: LRBuilder<'i, I, C, S, P, TK>, C, S>(&self, context: &mut C, builder: &mut B)
    where
        C: Context<'i, I, S, TK> + Default,
        S: State,
        P: Copy,
    {
        match &*self.root {
            SPPFTree::Term { token, .. } => {
                context.set_span(Context::<I, S, TK>::span(&*self.root));
                builder.shift_action(context, token.clone())
            }
            SPPFTree::NonTerm { prod, .. } => {
                let children = self.children();
                children.iter().for_each(|c| {
                    c.build_inner(context, builder);
                });
                context.set_span(Context::<I, S, TK>::span(&*self.root));
                builder.reduce_action(context, *prod, children.len())
            }
            SPPFTree::Empty => (),
        }
    }

    /// For the given tree index finds the right tree root in the given slice of
    /// trees based on the number of solutions for each tree.
    #[allow(clippy::type_complexity)]
    fn find_tree_root(
        roots: &[Rc<SPPFTree<'i, I, P, TK>>],
        tree_idx: usize,
    ) -> Option<(Rc<SPPFTree<'i, I, P, TK>>, usize)> {
        if roots.is_empty() {
            return None;
        }
        let mut tree_idx = tree_idx;
        let mut root_idx = 0;
        let mut solutions = roots[root_idx].solutions();
        while solutions <= tree_idx {
            root_idx += 1;
            if root_idx >= roots.len() {
                // Tree index is out of bounds
                return None;
            }
            tree_idx -= solutions;
            solutions = roots[root_idx].solutions();
        }
        Some((Rc::clone(&roots[root_idx]), tree_idx))
    }

    // TODO: Implement iteration
}

/// Shared Packed Parse Forest (SPPF) returned by the GLR parser.
///
/// A forest is an ordered collection of trees. Basically, a wrapper around GSS
/// structure to provide information about number of trees/solutions,
/// ambiguities and to provide tree extraction/navigation.
///
/// Trees of the forest are ordered and each tree can be extracted as either an
/// eager or a lazy tree given its index.
#[derive(Debug)]
pub struct Forest<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    /// Root nodes of trees which are possible solutions.
    ///
    /// Each `SPPFTree` contains one or more trees lazily extracted using the
    /// `Tree` type.
    results: Vec<Rc<SPPFTree<'i, I, P, TK>>>,
}

impl<'i, I, P, TK> Forest<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    pub fn new(results: Vec<Rc<SPPFTree<'i, I, P, TK>>>) -> Self {
        Forest { results }
    }

    #[inline]
    pub fn get_first_tree(&self) -> Option<Tree<'i, I, P, TK>> {
        self.get_tree(0)
    }

    /// Extracts a tree with the given index
    pub fn get_tree(&self, idx: usize) -> Option<Tree<'i, I, P, TK>> {
        Tree::find_tree_root(&self.results, idx).map(|(root, idx)| Tree::new(root, idx))
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// The total number of trees/solutions in this forest.
    #[inline]
    pub fn solutions(&self) -> usize {
        self.results.iter().map(|n| n.solutions()).sum()
    }

    /// Total number of ambiguous places/nodes in this forest.
    ///
    /// Extracted trees are unambiguous but forests may have ambiguities.
    /// If there is >1 trees in the forest there are ambiguities.
    #[inline]
    pub fn ambiguities(&self) -> usize {
        #[allow(clippy::mutable_key_type)]
        let mut visited: HashSet<Rc<Parent<'i, I, P, TK>>> = HashSet::new();
        self.results
            .iter()
            .map(|n| n.ambiguities(&mut visited))
            .sum::<usize>()
            + if self.results.len() > 1 { 1 } else { 0 }
    }
}

/// Support for into_iter, i.e. iteration in for loops
pub struct ForestIntoIter<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    forest: Forest<'i, I, P, TK>,
    tree_idx: usize,
}

impl<'i, I, P, TK> IntoIterator for Forest<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    type Item = Tree<'i, I, P, TK>;
    type IntoIter = ForestIntoIter<'i, I, P, TK>;

    fn into_iter(self) -> Self::IntoIter {
        ForestIntoIter {
            forest: self,
            tree_idx: 0,
        }
    }
}

impl<'i, I, P, TK> Iterator for ForestIntoIter<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    type Item = Tree<'i, I, P, TK>;

    fn next(&mut self) -> Option<Self::Item> {
        let tree = self.forest.get_tree(self.tree_idx);
        if tree.is_some() {
            self.tree_idx += 1;
        }
        tree
    }
}

/// Support for iter()
impl<'i, I, P, TK> Forest<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    pub fn iter<'f>(&'f self) -> ForestIterator<'i, 'f, I, P, TK> {
        ForestIterator {
            forest: self,
            tree_idx: 0,
        }
    }
}

pub struct ForestIterator<'i, 'f, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    forest: &'f Forest<'i, I, P, TK>,
    tree_idx: usize,
}

impl<'i, I, P, TK> Iterator for ForestIterator<'i, '_, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    type Item = Tree<'i, I, P, TK>;

    fn next(&mut self) -> Option<Self::Item> {
        let tree = self.forest.get_tree(self.tree_idx);
        if tree.is_some() {
            self.tree_idx += 1;
        }
        tree
    }
}

/// For loop iteration over borrowed Forest
impl<'i, 'f, I, P, TK> IntoIterator for &'f Forest<'i, I, P, TK>
where
    I: Input + ?Sized,
    TK: Copy,
{
    type Item = Tree<'i, I, P, TK>;
    type IntoIter = ForestIterator<'i, 'f, I, P, TK>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
