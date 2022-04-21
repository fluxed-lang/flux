# fluxc_types

Contains the type system definitions for Flux.

## Overview

The Flux type system takes heavy inspiration from that of TypeScript's.

## Built-in Types

### Primitive Types

Built-in types which can be used to construct more complex types.

### Integers

-   int128
-   int64
-   int32
-   int16
-   int8
-   uint128
-   uint64
-   uint32
-   uint16
-   uint8

> Integer types also have aliases formatted as i64, i32 etc. Unsigned integer types are formatted as u64, u32 etc.

### Floats

-   float128
-   float64
-   float32

> Float types also have aliases formatted as f64, f32 etc.

### Lists

-   int128[3]
-   int64[4]

### Text

-   char
-   string

## High-level Types

-   Intersection
-   Union
-   Pick
-   Omit
-   Exclude

## Conditional Types

> Fetch the type of an array.
>
> ```flux
> type T = T extends U[] ? infer U : never;
> ```

## Casting

> Cast the type of one object to another.
>
> ```flux
> let x: int64 = 2
> let y: int32 = x as int64
> ```
