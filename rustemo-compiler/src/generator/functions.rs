use super::{arrays::ArrayPartGenerator, ParserGenerator, PartGenerator};

use crate::error::Result;

pub(crate) struct FunctionPartGenerator {
    delegate: ArrayPartGenerator,
}

impl FunctionPartGenerator {
    pub fn new() -> Self {
        FunctionPartGenerator {
            delegate: ArrayPartGenerator::new(),
        }
    }
}

impl<'g, 's> PartGenerator<'g, 's> for FunctionPartGenerator {
    fn header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.header(generator)
    }

    fn parser_header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.parser_definition(generator)
    }

    fn symbols(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.symbols(generator)
    }

    fn types(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.types(generator)
    }

    fn parser(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.parser(generator)
    }

    fn lexer_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.lexer_definition(generator)
    }

    fn parser_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.parser_definition(generator)
    }

    fn builder(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.builder(generator)
    }
}
