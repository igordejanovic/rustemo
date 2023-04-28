mod calculator01_ast_tests {

    use rustemo_compiler::output_cmp;

    use crate::ast_actions::{
        calculator01::Calculator01Parser,
        calculator02_ambig::Calculator02AmbigParser,
        calculator03_ambig_prodkind::Calculator03AmbigProdkindParser,
        calculator04_ambig_lhs::Calculator04AmbigLhsParser,
    };

    #[test]
    fn test_calculator01() {
        let result = Calculator01Parser::new().parse("2 + 3 * 7 + 6 * 3");
        output_cmp!(
            "src/ast_actions/calculator01.ast",
            format!("{:#?}", result)
        );
    }

    #[test]
    fn test_calculator02_ambig() {
        let result = Calculator02AmbigParser::new().parse("2 + 3 * 7 + 6 * 3");
        output_cmp!(
            "src/ast_actions/calculator02_ambig.ast",
            format!("{:#?}", result)
        );
    }

    #[test]
    fn test_calculator03_ambig_prodkind() {
        let result =
            Calculator03AmbigProdkindParser::new().parse("2 + 3 * 7 + 6 * 3");
        output_cmp!(
            "src/ast_actions/calculator03_ambig_prodkind.ast",
            format!("{:#?}", result)
        );
    }

    #[test]
    fn test_calculator04_ambig_lhs() {
        let result =
            Calculator04AmbigLhsParser::new().parse("2 + 3 * 7 + 6 * 3");
        output_cmp!(
            "src/ast_actions/calculator04_ambig_lhs.ast",
            format!("{:#?}", result)
        );
    }
}
