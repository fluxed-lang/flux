use crate::ast::{get_type, Expr};
use crate::types::Type;

peg::parser!(pub grammar parser() for str {
    rule traced<T>(e: rule<T>) -> T =
        &(input:$([_]*) {
            #[cfg(feature = "trace")]
            println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
        })
        e:e()? {?
            #[cfg(feature = "trace")]
            println!("[PEG_TRACE_STOP]");
            e.ok_or("")
        }

    /// General statement matcher - attempts to match all statements in a file.
    pub(crate) rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    /// General statement matcher - attempts to match an expression, consuming whitespace around it.
    /// Expressions end with a new-line, or when EOF is reached.
    pub(crate) rule statement() -> Expr // hello i'm Ben and i like crisps and watching peppa pig :)
        = _ e:expression() _ ("\n"+ / ![_]) { e }

    /// Represents a language expression.
    pub(crate) rule expression() -> Expr
        = literal()
        / declaration()
        / assignment()
        / import()

    pub(crate) rule assignable() -> Expr
        = literal()

    /// Represents a primitive type name
    pub(crate) rule primitive_type() -> Expr
        = "int" { Expr::Type(Type::Int64.into()) }
        / "float" { Expr::Type(Type::Float64.into()) }
        / "bool" { Expr::Type(Type::Bool.into()) }
        / expected!("primitive")

    /// Represents a declaration of a new variable.
    pub(crate) rule declaration() -> Expr
        = "let" _ i:identifier()":" _ t:primitive_type() _ "=" _ e:assignable() {
            Expr::Declare(i, match t { Expr::Type(t) => t.into(), _ => panic!("parser returned an illegal expression type")}, e.into())
        }
        / "let" _ i:identifier() _ "=" _ e:assignable() { Expr::Declare(i, get_type(&e).into(), e.into()) }
        / expected!("declaration")

    /// Represents an assignment of an existing variable.
    pub(crate) rule assignment() -> Expr
        = !"let" i:identifier() _ "=" _ e:expression() { Expr::Assign(i, Box::new(e)) }
        / expected!("assignment")

    /// Represents a literal value.
    pub(crate) rule literal() -> Expr
        // match chars 0-9 in repeated order - integer type
        = int:$(['0'..='9']+) { Expr::Literal(int.to_owned(), Type::Int64.into()) }
        // match chars 0-9 in repeated order with a decimal point - float type
        / float:$(['0'..='9']+ "." ['0'..='9']+) { Expr::Literal(float.to_owned(), Type::Float64.into())}
        // matches true or false - todo: merge these
        / bool:$("true" / "false") { Expr::Literal(bool.to_owned(), Type::Bool.into()) }
        / expected!("literal")

    pub(crate) rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    pub(crate) rule import() -> Expr
        = "import" _ i:identifier() _ "from" _ n:$(['a'..='z']+) _ { Expr::Import(i, n.to_owned()) }
        / expected!("import")

    /// Whitespace and comment consumer rule
    rule _()
        = comment()+ / whitespace()

    /// Comment consumer rule.
    pub(crate) rule comment()
        = single_line_comment() / multi_line_comment()

    /// Matches a single line # comment.
    pub(crate) rule single_line_comment()
        = quiet!{"//" [^ '\n']* ("\n"+ / ![_])}

    /// Matches a single line
    pub(crate) rule multi_line_comment()
        = quiet!{"/*" [_]* "*/"}

    pub(crate) rule whitespace()
        = quiet!{[' ' | '\t']*}
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() {
        parser::whitespace(" ").expect("failed to match whitespace");
        parser::whitespace("x").expect_err("illegal whitespace match");
    }

    #[test]
    fn test_ident() {
        parser::identifier("x").expect("failed to match identifier");
        parser::identifier("1234").expect_err("illegal ident match");
    }

    #[test]
    fn test_assign() {
        parser::assignment("x = 1").expect("failed to match assignment");
        parser::assignment("1 = 1").expect_err("illegal assignment match");
    }

    #[test]
    fn test_declaration() {
        parser::declaration("let x = 1").expect("failed to match declaration");
        parser::declaration("x = 3").expect_err("illegal declaration match");
    }

    #[test]
    fn test_single_line_comment() {
        parser::single_line_comment("// hello world").expect("failed to match single line comment");
        parser::single_line_comment("let x = 1").expect_err("illegal single line comment match");
    }

    #[test]
    fn test_expression() {
        parser::expression("let x = 1").expect("failed to match expression");
    }

    #[test]
    fn test_import() {
        parser::import("import fox from fox").expect("failed to match import");
        parser::import("import fox from ./").expect_err("illegal import match");
    }
}
