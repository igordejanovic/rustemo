use super::{arrays::ArrayPartGenerator, ParserGenerator, PartGenerator};

use crate::error::Result;
use syn::parse_quote;

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
    fn parser_header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let states_count = generator.table.states.len();
        Ok(parse_quote! {
            const STATE_COUNT: usize = #states_count;
        })
    }

    fn parser_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.parser_definition(generator)
    }

    fn delegate(&self) -> &dyn PartGenerator<'g, 's> {
        &self.delegate
    }
}
