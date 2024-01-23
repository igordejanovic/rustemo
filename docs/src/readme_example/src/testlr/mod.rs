#[rustfmt::skip]
mod calclr;
mod calclr_actions;
use self::calclr::CalclrParser;
use rustemo::Parser;

#[test]
fn test_lr() {
    let result = CalclrParser::new().parse("2 + 3 * 4 + 1").unwrap();

    // As the parsing is deterministic now we have just 1 solution which is
    // automatically evaluated using the default builder and provided actions
    assert_eq!(result, 15);
}
