# styxc_parser

Converts a stream of tokens into an AST representing the source code.

## Steps

1. Conversion - The parser attempts to construct a valid AST from the tokens it receives.
2. Validation and semantic analysis - The parser ensures the AST is valid by comparing identifiers, and then types.

    - Identifier validation ensures identifiers exist in the current scope they are used in. This includes functions and variables.
    - Type validation ensures the correct types are parsed to methods and variables.

    This step is evaluated by methods in the [`styxc_ast`]("../styxc_ast) crate.
