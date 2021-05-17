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
    pub rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    /// General statement matcher - attempts to match an expression, consuming whitespace around it.
    /// Expressions end with a new-line, or when EOF is reached.
    rule statement() -> Expr // hello i'm Ben and i like crisps and watching peppa pig :)
        = _ e:expression() _ ("\n"+ / ![_]) { e }

    /// Represents a language expression.
    rule expression() -> Expr
        = literal()
        / declaration()
        / assignment()
        / import()

    rule assignable() -> Expr
        = literal()

    /// Represents a primitive type name
    rule primitive_type() -> Expr
        = "int" { Expr::Type(Type::Int64.into()) }
        / "float" { Expr::Type(Type::Float64.into()) }
        / "bool" { Expr::Type(Type::Bool.into()) }
        / expected!("primitive")

    /// Represents a declaration of a new variable.
    rule declaration() -> Expr
        = "let" _ i:identifier()":" _ t:primitive_type() _ "=" _ e:assignable() {
            Expr::Declare(i, match t { Expr::Type(t) => t.into(), _ => panic!("parser returned an illegal expression type")}, e.into())
        }
        / "let" _ i:identifier() _ "=" _ e:assignable() { Expr::Declare(i, get_type(&e).into(), e.into()) }
        / expected!("declaration")

    /// Represents an assignment of an existing variable.
    rule assignment() -> Expr
        = !"let" i:identifier() _ "=" _ e:expression() { Expr::Assign(i, Box::new(e)) }
        / expected!("assignment")

    /// Represents a literal value.
    rule literal() -> Expr
        // match chars 0-9 in repeated order - integer type
        = int:$(['0'..='9']+) { Expr::Literal(int.to_owned(), Type::Int64.into()) }
        // match chars 0-9 in repeated order with a decimal point - float type
        / float:$(['0'..='9']+ "." ['0'..='9']+) { Expr::Literal(float.to_owned(), Type::Float64.into())}
        // matches true or false - todo: merge these
        / bool:$("true" / "false") { Expr::Literal(bool.to_owned(), Type::Bool.into()) }
        / expected!("literal")

    rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule import() -> Expr
        = "import" _ i:identifier() _ "from" _ n:$(['a'..='z']) _ { Expr::Import(i, n.to_owned()) }
        / expected!("import")

    /// Whitespace and comment consumer rule
    rule _()
        = comment()+ / whitespace()

    /// Comment consumer rule.
    rule comment()
        = single_line_comment() / multi_line_comment()

    /// Matches a single line # comment.
    rule single_line_comment()
        = quiet!{"//" [^ '\n']* ("\n"+ / ![_])}

    /// Matches a single line
    rule multi_line_comment()
        = quiet!{"/*" [_]* "*/"}

    rule whitespace()
        = quiet!{[' ' | '\t']*}
});
