//! GLR parser implementation

use crate::{
    context::Context,
    glr::gss::Parent,
    input::Input,
    lexer::{Lexer, Token},
    location::Location,
    log,
    lr::{
        builder::SliceBuilder,
        parser::{Action, LRParser, ParserDefinition},
    },
    parser::{Parser, State},
    Result,
};
#[cfg(debug_assertions)]
use colored::*;
use petgraph::prelude::*;
use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Range,
    rc::Rc,
};

use super::gss::{Forest, GssGraph, GssHead, SPPFTree, TreeData};

/// The start of the reduction. For length 0 it will carry the node of the
/// reduction (empty reduction, thus the path is empty), while for len>0 it will
/// be the first edge along the reduction path.
#[derive(Debug)]
enum ReductionStart {
    Edge(EdgeIndex),
    Node(NodeIndex),
}

/// Used by the GLR algorithm to keep track of pending reductions.
#[derive(Debug)]
struct Reduction<P> {
    start: ReductionStart,

    /// The production to reduce by
    production: P,

    /// The length of the reduction path. Determined by the RHS of the grammar
    /// production for non right-nulled productions. For right-nulled production
    /// it will be the number of non-nullable symbol references on the left side
    /// of the production RHS.
    length: usize,
}

/// Reduction path is determined by the root node of the reduction together with
/// the path parent links containing sub-results (sub-trees).
#[derive(Debug)]
struct ReductionPath<'i, I: Input + ?Sized, P, TK: Copy> {
    /// Parents along the path
    parents: VecDeque<Rc<Parent<'i, I, P, TK>>>,

    /// The root of the reduction
    root_head: NodeIndex,
}

impl<'i, I: Input + ?Sized, P, TK: Copy> Display
    for ReductionPath<'i, I, P, TK>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pariter = self.parents.iter().rev();
        if let Some(parent) = pariter.next() {
            write!(f, "{}", parent.head_node.index())?;
            for parent in pariter {
                write!(f, " -> ")?;
                write!(f, "{}", parent.head_node.index())?;
            }
            write!(f, " -> ")?;
        }
        write!(f, "{}", self.root_head.index())
    }
}

type Content<'i, L, I, S, TK> =
    <<L as Lexer<'i, GssHead<'i, I, S, TK>, S, TK>>::Input as ToOwned>::Owned;

type LayoutParser<'i, I, S, P, TK, NTK, D, L> = Option<
    LRParser<
        'i,
        GssHead<'i, I, S, TK>,
        S,
        P,
        TK,
        NTK,
        D,
        L,
        SliceBuilder<'i, I>,
        I,
    >,
>;

pub struct GlrParser<
    'i,
    S: State,
    L: Lexer<'i, GssHead<'i, I, S, TK>, S, TK, Input = I>,
    P,
    TK: Default,
    NTK,
    D: ParserDefinition<S, P, TK, NTK> + 'static,
    I: Input + ?Sized,
    B,
> {
    /// Parser definition generated by Rustemo
    definition: &'static D,

    /// The file path if any or `<str>` if from str
    file_name: String,

    /// The owned input being parsed
    content: Option<Content<'i, L, I, S, TK>>,

    /// Layout parser if there is the Layout rule in the grammar. We keep the
    /// parser in a RefCell as we need it to be mutable during lookaheads
    /// finding.
    layout_parser: RefCell<LayoutParser<'i, I, S, P, TK, NTK, D, L>>,

    /// Is partial parse allowed, i.e. not requiring that the whole input is
    /// consumed. Use with care in GLR as it can lead to a *huge* number of
    /// possible solutions/trees.
    partial_parse: bool,
    start_position: usize,
    has_layout: bool,
    lexer: Rc<L>,

    phantom: PhantomData<(NTK, B)>,
}

impl<'i, S, L, P, TK, NTK, D, I, B> GlrParser<'i, S, L, P, TK, NTK, D, I, B>
where
    I: Input + ?Sized + Debug,
    L: Lexer<'i, GssHead<'i, I, S, TK>, S, TK, Input = I>,
    S: State + Ord + Debug,
    D: ParserDefinition<S, P, TK, NTK>,
    TK: Copy + Default + PartialEq + Ord + Debug + 'i,
    P: Copy + Debug + Into<NTK>,
{
    pub fn new(
        definition: &'static D,
        partial_parse: bool,
        has_layout: bool,
        lexer: L,
    ) -> Self {
        Self {
            file_name: "<str>".into(),
            content: None,
            layout_parser: RefCell::new(None),
            definition,
            partial_parse,
            start_position: 0,
            has_layout,
            lexer: Rc::new(lexer),
            phantom: PhantomData,
        }
    }

    /// Create pending shifts and reduction for the initial frontier.
    fn initial_process_frontier(
        &self,
        gss: &mut GssGraph<'i, I, S, P, TK>,
        frontier: &BTreeMap<TK, BTreeMap<S, NodeIndex>>,
        pending_reductions: &mut VecDeque<Reduction<P>>,
        pending_shifts: &mut Vec<(NodeIndex, S)>,
        accepted_heads: &mut Vec<NodeIndex>,
    ) {
        log!("\n{}", "Preparing frontier.".red());
        for subfrontier in frontier.values() {
            for (&state, head) in subfrontier {
                log!(
                    "\t{}",
                    format!("Processing head {}", head.index()).green()
                );
                for action in self.definition.actions(
                    state,
                    gss.head(*head).token_ahead().as_ref().unwrap().kind,
                ) {
                    match *action {
                        Action::Reduce(prod, length) => {
                            log!(
                                "\t{}: length = {}, production = {:?}",
                                "Register new reduction".green(),
                                length,
                                prod
                            );
                            if length == 0 {
                                pending_reductions.push_back(Reduction {
                                    start: ReductionStart::Node(*head),
                                    production: prod,
                                    length,
                                })
                            } else {
                                for edge in gss.backedges(*head) {
                                    pending_reductions.push_back(Reduction {
                                        start: ReductionStart::Edge(edge.id()),
                                        production: prod,
                                        length,
                                    })
                                }
                            }
                        }
                        Action::Shift(state) => {
                            log!(
                                "\t{}",
                                format!(
                                    "Adding head {} to pending shifts.",
                                    head.index()
                                )
                                .green()
                            );
                            pending_shifts.push((*head, state))
                        }
                        Action::Accept => {
                            log!(
                                "\t{}",
                                format!("Accepting head {}.", head.index())
                                    .red()
                            );
                            accepted_heads.push(*head)
                        }
                        Action::Error => break,
                    }
                }
            }
        }
    }

    /// From the current frontier base create full frontier with per-tokenkind
    /// sub-frontiers.
    ///
    /// If a head has multiple possible tokens ahead (lexical ambiguity) split
    /// the head and create a head per token thus enabling handling of lexical
    /// ambiguities using the same GLR mechanics.
    fn create_frontier(
        &self,
        gss: &mut GssGraph<'i, I, S, P, TK>,
        frontier_base: &BTreeMap<S, NodeIndex>,
        input: &'i L::Input,
    ) -> BTreeMap<TK, BTreeMap<S, NodeIndex>> {
        let mut frontier: BTreeMap<TK, BTreeMap<S, NodeIndex>> =
            BTreeMap::new();
        for &head_idx in frontier_base.values() {
            // Multiple heads are possible per state in case of lexical ambiguity.
            let head = gss.head(head_idx);
            if let Some(token) = &head.token_ahead() {
                // May happen after error recovery
                frontier
                    .entry(token.kind)
                    .or_default()
                    .insert(head.state(), head_idx);
            } else {
                // Find lookaheads
                log!(
                    "{}.",
                    format!("Finding lookaheads for head {}", head_idx.index())
                        .green()
                );
                let mut lookahead_tokens =
                    self.find_lookaheads(gss, head_idx, input);
                let head = gss.head_mut(head_idx);
                if let Some(token) = lookahead_tokens.pop() {
                    frontier
                        .entry(token.kind)
                        .or_default()
                        .insert(head.state(), head_idx);
                    head.set_token_ahead(token);
                    let state = head.state();
                    // If more tokens are found we have lexical ambiguity. Make
                    // a new head for each token.
                    for lookahead in lookahead_tokens {
                        frontier.entry(lookahead.kind).or_default().insert(
                            state,
                            self.head_for_lookahead(gss, head_idx, lookahead),
                        );
                    }
                } else {
                    log!("No lookaheads found. Killing head.");
                }
            }
        }
        frontier
    }

    /// Find all possible lookahead tokens for the given head. There can be more
    /// than one Token at the current location due to lexical ambiguity.
    ///
    /// If Layout rule is given in the grammar the layout parser will be used to
    /// skip whitespaces/comments before recognizing next tokens.
    fn find_lookaheads(
        &self,
        gss: &mut GssGraph<'i, I, S, P, TK>,
        head: NodeIndex,
        input: &'i I,
    ) -> Vec<Token<'i, I, TK>> {
        let head = gss.head_mut(head);
        let expected_tokens =
            self.definition.expected_token_kinds(head.state());

        let mut layout_parsing = true;
        loop {
            let tokens: Vec<_> = self
                .lexer
                .next_tokens(head, input, expected_tokens)
                .collect();

            if !tokens.is_empty() {
                return tokens;
            } else if layout_parsing {
                layout_parsing = false;
                if let Some(layout_parser) =
                    self.layout_parser.borrow_mut().as_ref()
                {
                    log!("\n{}", "*** Parsing layout".red().bold());
                    let current_state = head.state();
                    head.set_state(S::default_layout().unwrap());
                    let p = layout_parser.parse_with_context(head, input);
                    log!("Layout is {p:?}");
                    head.set_state(current_state);
                    if let Ok(Some(layout)) = p {
                        if layout.len() > 0 {
                            log!("Skipping layout: {layout:?}");
                            head.set_layout_ahead(Some(layout));
                            log!("\n{}", "*** Parsing content".red().bold());
                            continue;
                        }
                    }
                }
            }
            break;
        }

        // No tokens are found. For partial parsing return STOP if expected
        // even if we are not at the end of the input
        let stop_kind = <TK as Default>::default();
        if self.partial_parse
            && expected_tokens.iter().flatten().any(|&tk| tk == stop_kind)
        {
            vec![Token {
                kind: stop_kind,
                value: &input[0..0],
                location: head.location(),
            }]
        } else {
            vec![]
        }
    }

    /// Create a new head based on `head_idx` with the given lookahead.
    /// Used in lexical ambiguity.
    fn head_for_lookahead(
        &self,
        gss: &mut GssGraph<'i, I, S, P, TK>,
        head_idx: NodeIndex,
        lookahead: Token<'i, I, TK>,
    ) -> NodeIndex {
        let head = gss.head(head_idx);
        let new_head = gss.add_head(head.with_tok(lookahead));
        let new_parents: Vec<_> = gss
            .backedges(head_idx)
            .map(|e| (e.target(), Rc::clone(e.weight())))
            .collect();

        for (target, parent) in new_parents {
            // Copy all parent edges
            gss.add_parent(new_head, target, parent);
        }

        new_head
    }

    /// Starting from the queue of pending reduction execute reductions until no
    /// more reduction can be done. For each reduced head register shift
    /// operation if possible.
    fn reducer(
        &self,
        gss: &mut GssGraph<'i, I, S, P, TK>,
        pending_reductions: &mut VecDeque<Reduction<P>>,
        pending_shifts: &mut Vec<(NodeIndex, S)>,
        accepted_heads: &mut Vec<NodeIndex>,
        mut subfrontier: BTreeMap<S, NodeIndex>,
    ) {
        log!(
            "\n{}{}",
            "Reducing".red(),
            format!(
                " - {} pending reduction start(s)",
                if pending_reductions.is_empty() {
                    "no".to_owned()
                } else {
                    pending_reductions.len().to_string()
                }
            )
            .green()
        );
        while let Some(reduction) = pending_reductions.pop_front() {
            let production = reduction.production;
            for path in self.find_reduction_paths(gss, &reduction) {
                log!("{}: {:?}", "Reducing by production".green(), production);
                log!("\tPath: {path}");
                let root_head = gss.head(path.root_head);
                let start_head = gss.head(match reduction.start {
                    ReductionStart::Edge(e) => gss.start(e),
                    ReductionStart::Node(n) => n,
                });
                let start_position_before = start_head.position_before;
                let start_location_pos_before = start_head.location_pos_before;
                let token_kind_ahead =
                    start_head.token_ahead().as_ref().unwrap().kind;
                let root_state = root_head.state();
                let next_state =
                    self.definition.goto(root_state, production.into());

                let solution = Rc::new(SPPFTree::NonTerm {
                    prod: production,
                    data: TreeData {
                        range: Range {
                            start: root_head.position_ahead,
                            end: start_position_before,
                        },
                        location: Location {
                            start: root_head.location_pos_ahead,
                            end: Some(start_location_pos_before),
                        },
                        layout: root_head.layout_ahead(),
                    },
                    children: path.parents,
                });

                if let Some(head) = subfrontier.get(&next_state) {
                    log!(
                        "\t{}",
                        format!(
                            "Head {} with the same state already exists. \
                                  Ambiguity. \
                                  Just register new solution.",
                            head.index()
                        )
                        .green()
                    );
                    if let Some(edge) =
                        gss.add_solution(*head, path.root_head, solution)
                    {
                        log!("\t{}", "Parent link created.".green());
                        // Parent link was created it -> register all non-empty
                        // reductions
                        for &action in self
                            .definition
                            .actions(next_state, token_kind_ahead)
                            .iter()
                            .take_while(|a| !matches!(a, Action::Error))
                        {
                            match action {
                                Action::Reduce(prod, length) if length > 0 => {
                                    log!("\tRegister EMPTY reduction for production {:?}", prod);
                                    pending_reductions.push_back(Reduction {
                                        start: ReductionStart::Edge(edge),
                                        production: prod,
                                        length,
                                    })
                                }
                                _ => (),
                            }
                        }
                    }
                } else {
                    // No head with this state. We shall create one only if
                    // there is at least one action for the head. If not, we
                    // shall just report that.
                    let actions = self
                        .definition
                        .actions(next_state, token_kind_ahead)
                        .iter()
                        .take_while(|a| !matches!(a, Action::Error))
                        .collect::<Vec<_>>();
                    // No head with this state. Create one, create edge and
                    // register shifts and reductions.
                    if !actions.is_empty() {
                        let new_head = start_head.with_tok_state(
                            start_head.token_ahead().cloned().unwrap(),
                            next_state,
                        );
                        #[cfg(debug_assertions)]
                        let new_head_str = format!("{:?}", new_head);
                        let new_head_idx = gss.add_head(new_head);
                        subfrontier.insert(next_state, new_head_idx);
                        log!(
                            "\t{} {}: {}",
                            "Created head".green(),
                            new_head_idx.index(),
                            new_head_str
                        );
                        let edge = gss.add_solution(
                            new_head_idx,
                            path.root_head,
                            solution,
                        );

                        for &action in actions {
                            match action {
                                Action::Reduce(production, length) => {
                                    log!(
                                        "\t{}: length = {}, production = {:?}",
                                        "Register new reduction".green(),
                                        length,
                                        production
                                    );
                                    pending_reductions.push_back(Reduction {
                                        start: ReductionStart::Edge(
                                            edge.expect(
                                                "Edge must be created in add_solution!",
                                            ),
                                        ),
                                        production,
                                        length,
                                    })
                                }
                                Action::Shift(s) => {
                                    log!(
                                        "\t{}",
                                        format!(
                                            "Adding head {} to pending shifts.",
                                            new_head_idx.index()
                                        )
                                        .green()
                                    );
                                    pending_shifts.push((new_head_idx, s))
                                }
                                Action::Accept => {
                                    log!(
                                        "\t{}",
                                        format!(
                                            "Accepting head {}.",
                                            new_head_idx.index()
                                        )
                                        .red()
                                    );
                                    accepted_heads.push(new_head_idx)
                                }
                                Action::Error => panic!("Cannot happen!"),
                            }
                        }
                    } else {
                        log!("\tNo actions for reduced head. Not creating.");
                    }
                }
            }
        }
    }

    /// Do all pending shifts and create the next frontier base.
    fn shifter(
        &self,
        gss: &mut GssGraph<'i, I, S, P, TK>,
        pending_shifts: &mut Vec<(NodeIndex, S)>,
        frontier_idx: usize,
    ) -> BTreeMap<S, NodeIndex> {
        log!(
            "\n{}{}",
            "Shifting".red(),
            format!(
                " - {} pending shift(s)",
                if pending_shifts.is_empty() {
                    "no".to_owned()
                } else {
                    pending_shifts.len().to_string()
                }
            )
            .green()
        );
        let mut frontier_base = BTreeMap::new();
        while let Some((head_idx, state)) = pending_shifts.pop() {
            let head = gss.head(head_idx);
            let token = head.token_ahead().cloned().unwrap();
            let position = head.position() + token.value.len();
            log!(
                "{}",
                format!(
                    "Shifting head {} by token {:?}.",
                    head_idx.index(),
                    token.value
                )
                .green()
            );
            let shifted_head_idx = match frontier_base.get(&state) {
                Some(&shifted_head) => {
                    log!(
                        "\t{}",
                        "Head already exists. Adding new edge.".green()
                    );
                    shifted_head
                }
                None => {
                    let new_head = GssHead::new(
                        state,
                        frontier_idx,
                        // FIXME
                        position,
                        head.position()..position,
                        token.value.location_after(head.location()),
                        position,
                        position,
                        Default::default(),
                        Default::default(),
                        None,
                        None,
                    );
                    #[cfg(debug_assertions)]
                    let new_head_str = format!("{new_head:?}");
                    let new_head_idx = gss.add_head(new_head);
                    log!(
                        "\t{}: {new_head_str}",
                        format!(
                            "Creating new shifted head {}",
                            new_head_idx.index()
                        )
                        .green()
                    );
                    frontier_base.insert(state, new_head_idx);
                    new_head_idx
                }
            };
            gss.add_solution(
                shifted_head_idx,
                head_idx,
                Rc::new(SPPFTree::Term {
                    token,
                    data: TreeData {
                        // FIXME:
                        range: Default::default(),
                        location: Location {
                            start: Default::default(),
                            end: Default::default(),
                        },
                        layout: None,
                    },
                }),
            );
        }
        frontier_base
    }

    /// For the given reduction find all possible reduction paths by
    /// backtracing through the GSS for the reduction length.
    fn find_reduction_paths(
        &self,
        gss: &mut GssGraph<'i, I, S, P, TK>,
        reduction: &Reduction<P>,
    ) -> Vec<ReductionPath<'i, I, P, TK>> {
        let mut paths = vec![];
        match reduction.start {
            ReductionStart::Node(head) => {
                debug_assert!(reduction.length == 0);
                log!("\n{}", "Reducing EMPTY for head {head:?}".green());
                paths.push(ReductionPath {
                    parents: VecDeque::new(),
                    root_head: head,
                });
                return paths;
            }
            ReductionStart::Edge(start_edge) => {
                log!(
                    "{}",
                    format!(
                        "Finding reduction paths for length {} from head {}.",
                        reduction.length,
                        gss.start(start_edge).index()
                    )
                    .green()
                );
                #[derive(Debug)]
                struct PendingPath<'i, I: Input + ?Sized, P, TK: Copy> {
                    current_root: NodeIndex,
                    left_to_go: usize,
                    parents: VecDeque<Rc<Parent<'i, I, P, TK>>>,
                }
                let mut pending_paths: VecDeque<PendingPath<I, P, TK>> =
                    VecDeque::new();
                pending_paths.push_back(PendingPath {
                    current_root: gss.end(start_edge),
                    left_to_go: reduction.length - 1,
                    parents: VecDeque::from([gss.parent(start_edge)]),
                });

                while let Some(path) = pending_paths.pop_front() {
                    if path.left_to_go > 0 {
                        // We still have to traverse the path
                        for edge in gss.backedges(path.current_root) {
                            let mut new_ambiguities = path.parents.clone();
                            new_ambiguities.push_front(edge.weight().clone());
                            pending_paths.push_back(PendingPath {
                                current_root: edge.target(),
                                left_to_go: path.left_to_go - 1,
                                parents: new_ambiguities,
                            })
                        }
                    } else {
                        // The last traversal step
                        let path = ReductionPath {
                            parents: path.parents,
                            root_head: path.current_root,
                        };
                        log!("\t{}: {path}", "Found reduction path".green());
                        paths.push(path);
                    }
                }
            }
        }
        log!(
            "\t{}",
            format!("Reduction paths found: {}", paths.len()).green()
        );
        paths
    }

    fn create_forest(
        &self,
        gss: GssGraph<'i, I, S, P, TK>,
        accepted_heads: Vec<NodeIndex>,
    ) -> Forest<'i, I, P, TK>
    where
        TK: Copy,
    {
        Forest::new(
            accepted_heads
                .into_iter()
                .flat_map(|head| {
                    gss.backedges(head).flat_map(|p| {
                        p.weight()
                            .possibilities
                            .borrow()
                            .iter()
                            .map(|n| (*n).clone())
                            .collect::<Vec<_>>()
                    })
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl<'i, I, L, S, TK, NTK, P, D, B>
    Parser<'i, I, GssHead<'i, I, S, TK>, L, S, TK>
    for GlrParser<'i, S, L, P, TK, NTK, D, I, B>
where
    I: Input + ?Sized + Debug,
    L: Lexer<'i, GssHead<'i, I, S, TK>, S, TK, Input = I>,
    S: State + Debug + Ord,
    P: Copy + Debug + Into<NTK>,
    TK: Copy + Debug + Ord + Default + 'i,
    D: ParserDefinition<S, P, TK, NTK>,
{
    type Output = Forest<'i, I, P, TK>;

    fn parse(&self, input: &'i L::Input) -> Result<Self::Output> {
        let mut context = GssHead::default();
        context.set_position(self.start_position);
        self.parse_with_context(&mut context, input)
    }

    fn parse_with_context(
        &self,
        context: &mut GssHead<'i, I, S, TK>,
        input: &'i L::Input,
    ) -> Result<Self::Output> {
        let mut gss: GssGraph<'i, I, S, P, TK> = GssGraph::new();
        let start_head = gss.add_head(context.clone());
        if self.has_layout {
            *self.layout_parser.borrow_mut() = Some(LRParser::new(
                self.definition,
                S::default_layout().expect("Layout state not defined."),
                true,
                false,
                Rc::clone(&self.lexer),
                SliceBuilder::new(input),
            ))
        }

        log!("{}: {:?}", "Current state".green(), context.state());

        // Frontier represents the current "shift-level" or, starting from the
        // shifted nodes, frontier also has all the reduced nodes up to the next
        // shifted nodes which will form the basis for the next frontier. All
        // nodes with the same LR state belonging to a frontier are considered
        // equal, thus we use Map structure for quick access.
        //
        // This is the base of the frontier which is created before lookaheads
        // are found. The full frontier will be created by `create_frontier`
        // method.
        // The initial frontier base U0 has only the initial state 0.
        let mut frontier_idx = 0usize;
        let mut frontier_base: BTreeMap<S, NodeIndex> =
            BTreeMap::from([(context.state(), start_head)]);

        // Shifts that will be the basis of the next frontier base.
        let mut pending_shifts: Vec<(NodeIndex, S)> = vec![];

        // A queue of reductions that needs to be done.
        let mut pending_reductions: VecDeque<Reduction<P>> = VecDeque::new();

        let mut accepted_heads: Vec<NodeIndex> = vec![];

        while !frontier_base.is_empty() {
            // Create full frontier as a map where the key is a token ahead and
            // the value is sub-frontier for the given token. This is done to
            // support lexical ambiguity.
            let frontier =
                self.create_frontier(&mut gss, &frontier_base, input);
            // Create initial shifts/reductions for this frontier
            self.initial_process_frontier(
                &mut gss,
                &frontier,
                &mut pending_reductions,
                &mut pending_shifts,
                &mut accepted_heads,
            );
            for subfrontier in frontier.into_values() {
                // Reduce everything that is possible for this subfrontier
                self.reducer(
                    &mut gss,
                    &mut pending_reductions,
                    &mut pending_shifts,
                    &mut accepted_heads,
                    subfrontier,
                );
            }
            frontier_idx += 1;
            // Do shifts and create the next base frontier
            frontier_base =
                self.shifter(&mut gss, &mut pending_shifts, frontier_idx);
        }

        let forest = self.create_forest(gss, accepted_heads);
        Ok(forest)
    }

    fn parse_file<'a, F: AsRef<std::path::Path>>(
        &'a mut self,
        file: F,
    ) -> Result<Self::Output>
    where
        'a: 'i,
    {
        self.content = Some(L::Input::read_file(file.as_ref())?);
        self.file_name = file.as_ref().to_string_lossy().into();
        let parsed = self.parse(self.content.as_ref().unwrap().borrow());
        parsed
    }
}