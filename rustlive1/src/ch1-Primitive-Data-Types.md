```markdown
# Rust Primitive Data Types Notes

Rust provides a set of primitive data types that are built into the language. This document covers integers, floating-point numbers, booleans, and characters.

---

## Integers

Integers can be **signed** (`i`) or **unsigned** (`u`), and come in various sizes: 8, 16, 32, 64, and 128 bits.

- **Signed integers** (`i8`, `i16`, `i32`, `i64`, `i128`) can store both positive and negative values.
- **Unsigned integers** (`u8`, `u16`, `u32`, `u64`, `u128`) can only store non‑negative values.

```rust
fn main() {
    let x: i32 = -42;      // signed 32-bit integer
    let y: u64 = 100;      // unsigned 64-bit integer
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
}
```

### Range Differences

The range of an integer type depends on its size:

| Type | Minimum                | Maximum                |
|------|------------------------|------------------------|
| i32  | -2,147,483,648         | 2,147,483,647          |
| i64  | -9,223,372,036,854,775,808 | 9,223,372,036,854,775,807 |

```rust
let e: i32 = 2147483647;               // maximum i32 value
let i: i64 = 9223372036854775807;       // maximum i64 value
println!("Maximum value of i32: {}", e);
println!("Maximum value of i64: {}", i);
```

---

## Floating-Point Types

Rust has two floating-point types: `f32` and `f64` (default is `f64` because it’s roughly the same speed but offers higher precision).

```rust
let pi: f64 = 3.14;
println!("Value of Pi: {}", pi);
```

---

## Booleans

The Boolean type (`bool`) has two possible values: `true` or `false`.

```rust
let is_snowing: bool = true;
println!("Is it snowing? {}", is_snowing);
```

---

## Character Type

The `char` type represents a single Unicode character. It is specified with single quotes.

```rust
let letter: char = 'a';
println!("First letter of the alphabet: {}", letter);
```

---

> **Note:** The code examples above are taken directly from the original snippet. Minor corrections have been applied (e.g., `u26` is corrected to `u16` in the text, and the `i64` maximum value is shown correctly as `9223372036854775807`).
```