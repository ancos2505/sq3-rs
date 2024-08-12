use super::LiteralParser;

#[test]
fn ok_on_parse_valid_numeric_literals() {
    let test_cases = vec![
        "123", "-456", "3.14", ".5", "-.5", "1.23e10", "1.23E-5", "0x1A3F", "0XabCD",
    ];
    for case in test_cases {
        let res = LiteralParser::new(case).run();
        assert!(res.is_ok());
        // match res {
        //     Ok(result) => eprintln!("{}: {:?}", case, result),
        //     Err(err) => eprintln!("Failed to parse: {case}. Reason: {err}"),
        // }
    }
}

#[test]
// #[ignore = "Todo"]
fn err_on_parse_invalid() {
    let test_cases = vec![
        "+-323", "-+123", "12🦀34", ">456", ".3.14", ".5.", "-.5.", "1.23.e10", "1.23.E-5",
        "0x1A.3F", "0XabC-D",
    ];

    for case in test_cases {
        let res = LiteralParser::new(case).run();
        assert!(res.is_err());
    }
}
