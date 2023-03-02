mod calculator01_calc_tests {
    use rustemo_tools::output_cmp;

    use crate::calc_actions::calculator01::Calculator01Parser;

    #[test]
    fn test_calculator01_1() {
        let result = Calculator01Parser::parse("2 + 3 * 7 + 6 * 3");
        assert_eq!(result.unwrap(), 41f32);
    }

    #[test]
    fn test_calculator01_2() {
        let result = Calculator01Parser::parse("2 + ( 3  * 7 ) + 2 * 4");
        assert_eq!(result.unwrap(), 31f32);
    }

    #[test]
    fn test_calculator01_error() {
        let result = Calculator01Parser::parse("2 + ( 3  *  + 7 ) + 2 * 4");
        assert!(result.is_err());
        output_cmp!(
            "src/calc_actions/calculator01.err",
            result.unwrap_err().to_string()
        );
    }
}

mod calculator02_tests {
    use crate::calc_actions::calculator02_ambig::Calculator02AmbigParser;

    #[test]
    fn test_calculator2_1() {
        let result =
            Calculator02AmbigParser::parse("7 + 56.4 / 3 + 5 / 2 * (7 - 1)");
        assert_eq!(result.unwrap(), 40.800003f32);
    }
}

mod calculator03_tests {
    use crate::calc_actions::calculator03_ambig_prodkind::Calculator03AmbigProdkindParser;

    #[test]
    fn test_calculator2_1() {
        let result = Calculator03AmbigProdkindParser::parse(
            "7 + 56.4 / 3 + 5 / 2 * (7 - 1)",
        );
        assert_eq!(result.unwrap(), 40.800003f32);
    }
}

mod calculator04_tests {
    use crate::calc_actions::calculator04_ambig_lhs::Calculator04AmbigLhsParser;

    #[test]
    fn test_calculator2_1() {
        let result =
            Calculator04AmbigLhsParser::parse("7 + 56.4 / 3 + 5 / 2 * (7 - 1)");
        assert_eq!(result.unwrap(), 40.800003f32);
    }
}
