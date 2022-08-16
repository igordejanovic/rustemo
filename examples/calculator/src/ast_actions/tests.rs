mod calculator01_ast_tests {

    use crate::{
        ast_actions::{
            calculator01::Calculator01Parser,
            calculator02_ambig::Calculator02AmbigParser,
            calculator03_ambig_prodkind::Calculator03AmbigProdkindParser,
            calculator04_ambig_lhs::Calculator04AmbigLhsParser,
        },
        output_cmp,
    };

    #[test]
    fn test_calculator01() {
        let result = Calculator01Parser::parse_str("2 + 3 * 7 + 6 * 3");
        output_cmp!(
            "src/ast_actions/calculator01.ast",
            format!("{:#?}", result)
        );
    }

    #[test]
    fn test_calculator02_ambig() {
        let result = Calculator02AmbigParser::parse_str("2 + 3 * 7 + 6 * 3");
        output_cmp!(
            "src/ast_actions/calculator02_ambig.ast",
            format!("{:#?}", result)
        );
    }

    #[test]
    fn test_calculator03_ambig_prodkind() {
        let result =
            Calculator03AmbigProdkindParser::parse_str("2 + 3 * 7 + 6 * 3");
        output_cmp!(
            "src/ast_actions/calculator03_ambig_prodkind.ast",
            format!("{:#?}", result)
        );
    }

    #[test]
    fn test_calculator04_ambig_lhs() {
        let result = Calculator04AmbigLhsParser::parse_str("2 + 3 * 7 + 6 * 3");
        output_cmp!(
            "src/ast_actions/calculator04_ambig_lhs.ast",
            format!("{:#?}", result)
        );
    }
}
