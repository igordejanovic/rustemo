use crate::lexer::Token;

#[derive(Debug)]
pub enum TreeNode<I> {
    TermNode(Token<I>),
    NonTermNode {
        prod: &'static str,
        children: Vec<TreeNode<I>>,
    },
}
