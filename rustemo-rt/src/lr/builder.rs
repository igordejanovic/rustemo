use crate::{
    builder::Builder,
    index::{ProdIndex, StateIndex, TermIndex},
    lexer::{Context, Token},
};

/// A builder variant for LR parsing.
///
/// Builder should keep its internal stack of subresults, similar to the way LR
/// parsing operates.
pub trait LRBuilder<I, LO>: Builder {
    /// Called when LR shifting is taking place.
    ///
    /// # Arguments
    ///
    /// * `term_idx` - A terminal unique identifier - index.
    /// * `token` - A token recognized in the input.
    fn shift_action(
        &mut self,
        context: &Context<I, LO, StateIndex>,
        term_idx: TermIndex,
        token: Token<I>,
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
        prod_str: &'static str,
    );
}

/// TreeBuilder is the default builder that builds the parse tree.
pub struct TreeBuilder<I> {
    res_stack: Vec<TreeNode<I>>,
}

impl<I> Builder for TreeBuilder<I> {
    type Output = TreeNode<I>;

    fn new() -> Self {
        Self { res_stack: vec![] }
    }

    fn get_result(&mut self) -> Self::Output {
        self.res_stack.pop().unwrap()
    }
}

impl<I, LO> LRBuilder<I, LO> for TreeBuilder<I> {
    fn shift_action(
        &mut self,
        _context: &Context<I, LO, StateIndex>,
        _term_idx: TermIndex,
        token: Token<I>,
    ) {
        self.res_stack.push(TreeNode::TermNode(token))
    }

    fn reduce_action(
        &mut self,
        _context: &Context<I, LO, StateIndex>,
        _prod_idx: ProdIndex,
        prod_len: usize,
        prod_str: &'static str,
    ) {
        let children =
            self.res_stack.split_off(self.res_stack.len() - prod_len);
        self.res_stack.push(TreeNode::NonTermNode {
            children,
            prod: prod_str,
        });
    }
}

#[derive(Debug)]
pub enum TreeNode<I> {
    TermNode(Token<I>),
    NonTermNode {
        prod: &'static str,
        children: Vec<TreeNode<I>>,
    },
}
