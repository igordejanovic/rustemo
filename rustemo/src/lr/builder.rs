use crate::{
    builder::Builder,
    lexer::{Context, Input, Token}, location::Location,
};

/// A builder variant for LR parsing.
///
/// Builder should keep its internal stack of subresults, similar to the way LR
/// parsing operates.
pub trait LRBuilder<'i, I: Input + ?Sized, P, TK>: Builder {
    /// Called when LR shifting is taking place.
    ///
    /// # Arguments
    ///
    /// * `term_idx` - A terminal unique identifier - index.
    /// * `token` - A token recognized in the input.
    fn shift_action(
        &mut self,
        context: &mut Context<'i, I>,
        token: Token<'i, I, TK>,
    );

    /// Called when LR reduce is taking place.
    ///
    /// # Arguments
    ///
    /// * `prod_idx` - A production unique identifier, used to decide the action
    ///                to perform.
    /// * `prod_len` - A RHS length, used to pop appropriate number of
    ///                subresults from the stack
    fn reduce_action(
        &mut self,
        context: &mut Context<'i, I>,
        prod: P,
        prod_len: usize,
    );
}

/// TreeBuilder is a builder that builds a generic concrete parse tree.
pub struct TreeBuilder<'i, I: Input + ?Sized, P, TK> {
    res_stack: Vec<TreeNode<'i, I, P, TK>>,
}

impl<'i, I: Input + ?Sized, P, TK> Builder for TreeBuilder<'i, I, P, TK> {
    type Output = TreeNode<'i, I, P, TK>;

    fn new() -> Self {
        Self { res_stack: vec![] }
    }

    fn get_result(&mut self) -> Self::Output {
        self.res_stack.pop().unwrap()
    }
}

impl<'i, I: Input + ?Sized, P, TK> LRBuilder<'i, I, P, TK>
    for TreeBuilder<'i, I, P, TK>
{
    fn shift_action(
        &mut self,
        context: &mut Context<'i, I>,
        token: Token<'i, I, TK>,
    ) {
        self.res_stack.push(TreeNode::TermNode {
            token,
            layout: context.layout,
        })
    }

    fn reduce_action(
        &mut self,
        context: &mut Context<'i, I>,
        prod: P,
        prod_len: usize,
    ) {
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
            location: context.location,
            layout,
        });
    }
}

#[derive(Debug)]
pub enum TreeNode<'i, I: Input + ?Sized, P, TK> {
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

/// This builder returns a slice of the matched input. If no match is possible
/// `None` is returned.
pub struct SliceBuilder<'i, I: Input + ?Sized>(Option<&'i I>);
impl<'i, I: Input + ?Sized> Builder for SliceBuilder<'i, I> {
    type Output = Option<&'i I>;

    fn new() -> Self {
        Self(None)
    }

    fn get_result(&mut self) -> Self::Output {
        self.0
    }
}

impl<'i, I: Input + ?Sized, P, TK> LRBuilder<'i, I, P, TK>
    for SliceBuilder<'i, I>
{
    fn shift_action(
        &mut self,
        _context: &mut Context<'i, I>,
        _token: Token<'i, I, TK>,
    ) {
        // We do nothing on shift
    }

    fn reduce_action(
        &mut self,
        context: &mut Context<'i, I>,
        _prod: P,
        _prod_len: usize,
    ) {
        // On reduce, save the slice of the input.
        self.0 = Some(context.input.slice(&context.range));
    }
}

impl<'i, I: Input + ?Sized> Default for SliceBuilder<'i, I> {
    fn default() -> Self {
        Self::new()
    }
}
