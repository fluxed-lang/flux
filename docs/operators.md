# Operators

Styx, like most other languages, defines two type of operation expressions:

- Unary expressions (e.g. `!x`, `*y`)
  - Prefix unary expressions
  - Postfix unary expressions
- Binary expressions (e.g. `x + y`, `x * y`)

These operations can be combined with other expressions to form more complex expressions.

## Precedence Table

| Operator                                           | Precedence | Associativity |
|----------------------------------------------------|------------|---------------|
| Postfix unary operations (e.g. `[1]`, `()`, `x.y`) | 2          | N/A           |
| Prefix unary operations (e.g. `&`, `*`, `~`)       | 3          | N/A           |
| Cast, `as`                                         | 4          | Left to Right |
| `*`, `/`, `%`                                      | 5          | Left to Right |
| `+`, `-`                                           | 6          | Left to Right |
| `<<`, `>>`                                         | 7          | Left to Right |
| `&`                                                | 8          | Left to Right |
| `^`                                                | 9          | Left to Right |
| `\|`                                               | 10         | Left to Right |
| `<`, `>`, `<=`, `>=`                               | 11         | Right to Left |
| `==`, `!=`                                         | 12         | Right to Left |
| `&&`                                               | 13         | Left to Right |
| `\|\|`                                             | 14         | Left to Right |
| Assignment operators (e.g. `=`, `+=`, `&=`)        | 15         | Right to Left |

## Unary expressions

### Prefix unary expressions

- `!x` - logical NOT
- `~x` - bitwise NOT
- `&x` - address-of
- `*x` - dereference

### Postfix unary expressions

- `x.y` - path access
- `x()` - function call
- `x[1234]` - array indexing
