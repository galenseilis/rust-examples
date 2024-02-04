# Goals
- Getting familiar with core concepts and syntax.
- Data types & data structures
- Ownership & borrowing
- Allocating data to memory and accessing data fro memory
- Standard library
- Generic, traits, lifetimes, ...


# Why learn Rust?
- Fast
- Rich type system
- No garbage collator (faster runtime)
- Useful compiler output
- Memory safety
- ...

# Variables
- Assigned using `let` keyword.
- Print to standard output by `print!()` or `println!()`.
- Scope of variable is defined by the block of code in which it is declared.
- Function is named block of code that is reusable.
- Shadowing allows a variable to be re-declared in the same scope with the same name.

# Numbers

## Integer Types

- Signed integers: Can represent both positive and negative values.
- Unsigned Integer: Always positive integers.

| Length | Signed | Unsigned |
| --- | --- | --- |
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` | 
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

## Default Types

- Integers: `i32`
- Floats: `f64`

## Binary Number System

42

| 4 | 2 |
| --- | --- |
| 10^1 | 10^0 |

$$\left( 4 \times 10^1 \right) + \left( 2 \times 10^0 \right) = 42$$


