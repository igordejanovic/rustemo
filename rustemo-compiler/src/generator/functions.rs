use super::{ParserGenerator, PartGenerator};

use crate::error::Result;

pub(crate) struct FunctionPartGenerator {}

impl FunctionPartGenerator {
    pub fn new() -> Self {
        FunctionPartGenerator {}
    }
}

impl<'g, 's> PartGenerator<'g, 's> for FunctionPartGenerator {
    fn header(
        &self,
        _generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        todo!()
    }

    fn symbols(
        &self,
        _generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        todo!()
    }

    fn types(
        &self,
        _generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        todo!()
    }

    fn lexer_definition(
        &self,
        _generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        todo!()
    }

    fn parser_definition(
        &self,
        _generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        todo!()
    }

    fn builder(
        &self,
        _generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        todo!()
    }
}
