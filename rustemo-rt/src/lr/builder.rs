use crate::{
    builder::Builder,
    index::{ProdIndex, StateIndex},
    lexer::{Context, Input, Token},
};

/// A builder variant for LR parsing.
///
/// Builder should keep its internal stack of subresults, similar to the way LR
/// parsing operates.
pub trait LRBuilder<'i, I: Input<'i>, LO, TK: Copy>: Builder {
    /// Called when LR shifting is taking place.
    ///
    /// # Arguments
    ///
    /// * `term_idx` - A terminal unique identifier - index.
    /// * `token` - A token recognized in the input.
    fn shift_action(
        &mut self,
        context: &Context<I, LO, StateIndex>,
        token: Token<I, TK>,
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
        context: &Context<I, LO, StateIndex>,
        prod_idx: ProdIndex,
        prod_len: usize,
    );
}

/// TreeBuilder is a builder that builds a generic parse tree.
pub struct TreeBuilder<'i, I: Input<'i>, TK: Copy> {
    res_stack: Vec<TreeNode<I, TK>>,
}

impl<'i, I: Input<'i>, TK: Copy> Builder for TreeBuilder<'i, I, TK> {
    type Output = TreeNode<I, TK>;

    fn new() -> Self {
        Self { res_stack: vec![] }
    }

    fn get_result(&mut self) -> Self::Output {
        self.res_stack.pop().unwrap()
    }
}

impl<'i, I: Input<'i>, LO, TK: Clone + Copy> LRBuilder<'i, I, LO, TK> for TreeBuilder<'i, I, TK> {
    fn shift_action(
        &mut self,
        context: &Context<I, LO, StateIndex>,
        token: Token<I, TK>,
    ) {
        self.res_stack.push(TreeNode::TermNode {
            token,
            position: context.start_pos,
        })
    }

    fn reduce_action(
        &mut self,
        context: &Context<I, LO, StateIndex>,
        prod_idx: ProdIndex,
        prod_len: usize,
    ) {
        let children =
            self.res_stack.split_off(self.res_stack.len() - prod_len);
        self.res_stack.push(TreeNode::NonTermNode {
            children,
            prod_idx,
            position: context.start_pos,
        });
    }
}

#[derive(Debug)]
pub enum TreeNode<'i, I: Input<'i>, TK: Copy> {
    TermNode {
        token: Token<'i, I, TK>,
        position: usize,
    },
    NonTermNode {
        prod_idx: ProdIndex,
        position: usize,
        children: Vec<TreeNode<'i, I, TK>>,
    },
}
