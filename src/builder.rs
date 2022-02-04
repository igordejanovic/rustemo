use crate::{
    lexer::{Lexer, Token},
    tree::TreeNode, parser::{ProdIndex, TermIndex},
};

/// This trait is implemented by all types that are in charge of building output
/// of the parsing process (e.g. a parse tree). Builder should keep its internal
/// stack of subresults, similar to the way LR parsing operates.
pub trait Builder {
    /// Lexer used to tokenizer input
    type Lexer: Lexer;

    /// A type produced by this builder. See `get_result`.
    type Output;

    fn new() -> Self;

    /// Called when LR shifting is taking place.
    ///
    /// # Arguments
    ///
    /// * `term_kind` - A kind of terminal.
    /// * `token` - A token recognized in the input.
    fn shift_action(&mut self, term_kind: TermIndex, token: Token<<Self::Lexer as Lexer>::Input>);

    /// Called when LR reduce is taking place.
    ///
    /// # Arguments
    ///
    /// * `prod_kind` - A production kind, used to decide the action to perform.
    /// * `prod_len` - A RHS length, used to pop appropriate number of
    /// subresults from the stack
    fn reduce_action(&mut self, prod_kind: ProdIndex, prod_len: usize, prod_str: &'static str);

    /// Returns the product of parsing. Usually the one and only element left on
    /// the result stack.
    fn get_result(&mut self) -> Self::Output;
}

/// TreeBuilder is the default builder that builds the parse tree.
pub struct TreeBuilder<L: Lexer> {
    res_stack: Vec<TreeNode<L::Input>>,
}

impl<L: Lexer> Builder for TreeBuilder<L> {
    type Output = TreeNode<L::Input>;
    type Lexer = L;

    fn new() -> Self {
        Self { res_stack: vec![] }
    }

    fn shift_action(&mut self, _term_kind: TermIndex, token: Token<L::Input>) {
        self.res_stack.push(TreeNode::TermNode(token))
    }

    fn reduce_action(&mut self, _prod_kind: ProdIndex, prod_len: usize, prod_str: &'static str) {
        let children = self.res_stack.split_off(self.res_stack.len() - prod_len);
        self.res_stack.push(TreeNode::NonTermNode {
            children,
            prod: prod_str,
        });
    }

    fn get_result(&mut self) -> Self::Output {
        self.res_stack.pop().unwrap()
    }
}
