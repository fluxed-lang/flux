# Flux Documentation

Flux is a general-purpose, multi-paradigm programming language. Designed as an intermediary between [nodejs](https://nodejs.org) and [python](https://python.org), and lower level languages like [Rust](https://rust-lang.org) and [C++](https://wikipedia.org/C++), it provides a friendly programming experience while maintaining the speed of a low-level language.

## Basics

Flux currently supports 5 primitive variable types:

-   Int
-   Float
-   Bool
-   Char
-   String

> Both numerical types are 64 bits wide, and are both signed.

Once the type system is up to scratch, you will be able to build more complex types from these primitives, similar to TypeScript.

## Control Flow

Like any other language, Flux provides a means to control the flow of your program's logic.

```styx
if expression {
    # do something
}
if another_expression {
    # do something else
} else {
    # or even this!
}
```

`if` expressions can come at any point (including as an inline statement), and `else` expressions must follow an `if` expression.

```styx
let x = 1
let y = if x == 1 { 1 } else { 2 }
```

Inline if expressions are often represented as "ternary" expressions, but, for ease of use, these do not exist in Flux.

## Variables

Like all languages aiming for Turing-complete goodness, Flux supports variables. These must be statically typed, unless a type can be inferred at compile time.

```
let x = 1
mut y = 2
y = x + 1

const C = 299_792_458
# look how far i went!
let distance = C * 4
```

### Mutability

Variables can have one of three mutability states. These all allow for better memory safety and reduces the opportunities for bugs to sneak into your code.

-   Mutable
-   Immutable
-   Constant

> While you may not think there is a difference between immutable and constant variables (bit of a misnomer, I know), the size of a constant must be known at compile-time, unlike a mutable variable.

## Loops

There are three types of loops available in Flux:

-   Unconditional
-   Conditional
-   Iterative

More commonly, you may know the last two as `while` and `for`. You can think of the unconditional loop, written as `loop {}`, as a `while` loop that repeats forever, unless you break out of it.

## Functions

Function declaration in Flux is relatively simple and requires little syntactic sugar:

```
foo x: int -> {
    # do something
}

bar (x: int, y: int) -> {}

foo 2
bar 3, 4
```

Function parameters must also specify their mutability:

```
foo x: int -> {
    x += 1 # This won't compile
}

bar x: mut int -> {
    x += 1 # This will
}
```

### Quirks

When writing a function that mutates a variable, you may encounter some issues.

```
let x = 1

do_a_thing x: mut int -> {
    # The x in this function is not the same as the x outside the function.
    x += 1
}

# Unless...
do_a_thing x
```
