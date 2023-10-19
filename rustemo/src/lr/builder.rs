use crate::{
    builder::Builder, context::Context, input::Input, lexer::Token,
    location::Location, parser::State,
};
use core::fmt::Debug;

/// A builder variant for LR parsing.
///
/// Builder should keep its internal stack of subresults, similar to the way LR
/// parsing operates.
pub trait LRBuilder<'i, I, C, S, P, TK>: Builder
where
    I: Input + ?Sized,
    C: Context<'i, I, S, TK>,
    S: State,
{
    /// Called when LR shifting is taking place.
    ///
    /// # Arguments
    ///
    /// * `term_idx` - A terminal unique identifier - index.
    /// * `token` - A token recognized in the input.
    fn shift_action(&mut self, context: &mut C, token: Token<'i, I, TK>);

    /// Called when LR reduce is taking place.
    ///
    /// # Arguments
    ///
    /// * `prod_idx` - A production unique identifier, used to decide the action
    ///                to perform.
    /// * `prod_len` - A RHS length, used to pop appropriate number of
    ///                subresults from the stack
    fn reduce_action(&mut self, context: &mut C, prod: P, prod_len: usize);
}

/// TreeBuilder is a builder that builds a generic concrete parse tree.
pub struct TreeBuilder<'i, I, P, TK>
where
    I: Input + ?Sized,
{
    res_stack: Vec<TreeNode<'i, I, P, TK>>,
}

impl<'i, I, P, TK> TreeBuilder<'i, I, P, TK>
where
    I: Input + ?Sized,
{
    pub fn new() -> Self {
        Self { res_stack: vec![] }
    }
}

impl<'i, I, P, TK> Default for TreeBuilder<'i, I, P, TK>
where
    I: Input + ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'i, I, P, TK> Builder for TreeBuilder<'i, I, P, TK>
where
    I: Input + ?Sized,
{
    type Output = TreeNode<'i, I, P, TK>;

    fn get_result(&mut self) -> Self::Output {
        self.res_stack.pop().unwrap()
    }
}

impl<'i, I, C, S, P, TK> LRBuilder<'i, I, C, S, P, TK>
    for TreeBuilder<'i, I, P, TK>
where
    I: Input + ?Sized,
    C: Context<'i, I, S, TK>,
    S: State,
{
    fn shift_action(&mut self, context: &mut C, token: Token<'i, I, TK>) {
        self.res_stack.push(TreeNode::TermNode {
            token,
            layout: context.layout_ahead(),
        })
    }

    fn reduce_action(&mut self, context: &mut C, prod: P, prod_len: usize) {
        let children;
        let layout;
        if prod_len > 0 {
            children =
                self.res_stack.split_off(self.res_stack.len() - prod_len);
            layout = match children[0] {
                TreeNode::TermNode { layout, .. } => layout,
                TreeNode::NonTermNode { layout, .. } => layout,
            };
        } else {
            children = vec![];
            layout = None;
        }
        self.res_stack.push(TreeNode::NonTermNode {
            children,
            prod,
            location: context.location(),
            layout,
        });
    }
}

/// A node in the generic tree produced by [`TreeBuilder`]
#[derive(Debug)]
pub enum TreeNode<'i, I, P, TK>
where
    I: Input + ?Sized,
{
    TermNode {
        token: Token<'i, I, TK>,
        layout: Option<&'i I>,
    },
    NonTermNode {
        prod: P,
        location: Location,
        children: Vec<TreeNode<'i, I, P, TK>>,
        layout: Option<&'i I>,
    },
}

/// Returns a slice of the matched input. If no match is possible `None` is
/// returned.
///
/// This is used by default for layout parsing where we don't need to keep the
/// structure of the parsed layout but we need just the content as a slice of
/// the input.
pub struct SliceBuilder<'i, I: ?Sized> {
    input: &'i I,
    slice: Option<&'i I>,
}

impl<'i, I> SliceBuilder<'i, I>
where
    I: Input + ?Sized,
{
    pub fn new(input: &'i I) -> Self {
        Self { input, slice: None }
    }
}

impl<'i, I> Builder for SliceBuilder<'i, I>
where
    I: Input + ?Sized,
{
    type Output = Option<&'i I>;

    fn get_result(&mut self) -> Self::Output {
        self.slice
    }
}

impl<'i, I, C, S, P, TK> LRBuilder<'i, I, C, S, P, TK> for SliceBuilder<'i, I>
where
    I: Input + ?Sized,
    C: Context<'i, I, S, TK>,
    S: State,
{
    fn shift_action(&mut self, _context: &mut C, _token: Token<'i, I, TK>) {
        // We do nothing on shift
    }

    fn reduce_action(&mut self, context: &mut C, _prod: P, _prod_len: usize) {
        // On reduce, save the slice of the input.
        self.slice = Some(&self.input[context.range()]);
    }
}
