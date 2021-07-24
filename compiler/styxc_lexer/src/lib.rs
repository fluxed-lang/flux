extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct IdentParser;

#[cfg(test)]
mod tests {
    use pest::Span;

    use super::*;

    #[test]
    fn test_ident() {
        // x
        let mut res = IdentParser::parse(Rule::ident, "x").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule ident");

        assert_eq!(res.as_rule(), Rule::ident);
        assert_eq!(res.as_span(), Span::new("x", 0, 1).unwrap());
        assert_eq!(res.as_str(), "x");

        // someFunc_1234
        let mut res =
            IdentParser::parse(Rule::ident, "someFunc_1234").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule ident");

        assert_eq!(res.as_rule(), Rule::ident);
        assert_eq!(res.as_span(), Span::new("someFunc_1234", 0, 13).unwrap());
        assert_eq!(res.as_str(), "someFunc_1234");
    }

    #[test]
    fn test_int() {
        // 1234
        let mut res = IdentParser::parse(Rule::int, "1234").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("1234", 0, 4).unwrap());
        assert_eq!(res.as_str(), "1234");

        // -4321
        let mut res = IdentParser::parse(Rule::int, "-4321").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("-4321", 0, 5).unwrap());
        assert_eq!(res.as_str(), "-4321");

        // 0b1011101
        let mut res =
            IdentParser::parse(Rule::int, "0b1011101").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0b1011101", 0, 9).unwrap());
        assert_eq!(res.as_str(), "0b1011101");

        // -0d123456890
        let mut res =
            IdentParser::parse(Rule::int, "-0d123456890").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("-0d123456890", 0, 12).unwrap());
        assert_eq!(res.as_str(), "-0d123456890");

        // 0o1234567
        let mut res =
            IdentParser::parse(Rule::int, "0o1234567").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0o1234567", 0, 9).unwrap());
        assert_eq!(res.as_str(), "0o1234567");

        // 0xffff
        let mut res = IdentParser::parse(Rule::int, "0xffff").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0xffff", 0, 6).unwrap());
        assert_eq!(res.as_str(), "0xffff");
    }

    #[test]
    fn test_float() {
        // 1234.5
        let mut res = IdentParser::parse(Rule::float, "1234.5").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule float");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("1234.5", 0, 6).unwrap());
        assert_eq!(res.as_str(), "1234.5");

        // -543.21
        let mut res =
            IdentParser::parse(Rule::float, "-543.21").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule float");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("-543.21", 0, 7).unwrap());
        assert_eq!(res.as_str(), "-543.21");

        // 23e7
        let mut res = IdentParser::parse(Rule::float, "23e7").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule float");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("23e7", 0, 4).unwrap());
        assert_eq!(res.as_str(), "23e7");

        // 32e-72
        let mut res = IdentParser::parse(Rule::float, "32e-72").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule float");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("32e-72", 0, 6).unwrap());
        assert_eq!(res.as_str(), "32e-72");
    }

    #[test]
    fn test_char() {
        // 'a'
        let mut res = IdentParser::parse(Rule::char, "'a'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule char");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'a'", 0, 3).unwrap());
        assert_eq!(res.as_str(), "'a'");

        // '\n'
        let mut res = IdentParser::parse(Rule::char, "'\\n'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule char");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'\\n'", 0, 4).unwrap());
        assert_eq!(res.as_str(), "'\\n'");

        // '\uFF0F'
        let mut res = IdentParser::parse(Rule::char, "'\\uFF0F'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule char");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'\\uFF0F'", 0, 8).unwrap());
        assert_eq!(res.as_str(), "'\\uFF0F'");
    }

    #[test]
    fn test_string() {
        // "hello world"
        let mut res = IdentParser::parse(Rule::string, "\"hello world\"").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule string");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(res.as_span(), Span::new("\"hello world\"", 0, 13).unwrap());
        assert_eq!(res.as_str(), "\"hello world\"");

        // "hello, \u60ff"
        let mut res = IdentParser::parse(Rule::string, "\"hello, \\u60ff\"").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule string");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(res.as_span(), Span::new("\"hello, \\u60ff\"", 0, 15).unwrap());
        assert_eq!(res.as_str(), "\"hello, \\u60ff\"");

        // hello, 🦊
        let mut res = IdentParser::parse(Rule::string, "\"hello, 🦊\"").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule string");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(res.as_span(), Span::new("\"hello, 🦊\"", 0, 13).unwrap());
        assert_eq!(res.as_str(), "\"hello, 🦊\"");
    }
}
