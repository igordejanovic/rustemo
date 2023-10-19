use rustemo::rustemo_mod;

rustemo_mod!(json, "/src");
mod json_actions;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::json::JsonParser;
    use rustemo::Parser;
    use rustemo_compiler::{local_file, output_cmp};

    #[test]
    fn json_1() {
        let mut parser = JsonParser::new();
        let result = parser.parse_file(local_file!(file!(), "example1.json"));
        output_cmp!("src/example1.ast", format!("{:#?}", result));
    }

    #[test]
    fn json_2() {
        let mut parser = JsonParser::new();
        let result = parser.parse_file(local_file!(file!(), "example2.json"));
        output_cmp!("src/example2.ast", format!("{:#?}", result));
    }

    #[test]
    fn json_3() {
        let mut parser = JsonParser::new();
        let result = parser.parse_file(local_file!(file!(), "example3.json"));
        output_cmp!("src/example3.ast", format!("{:#?}", result));
    }

    #[test]
    fn json_4() {
        let mut parser = JsonParser::new();
        let result = parser.parse_file(local_file!(file!(), "example4.json"));
        output_cmp!("src/example4.ast", format!("{:#?}", result));
    }

    #[test]
    fn json_5() {
        let mut parser = JsonParser::new();
        let result = parser.parse_file(local_file!(file!(), "example5.json"));
        output_cmp!("src/example5.ast", format!("{:#?}", result));
    }
}
