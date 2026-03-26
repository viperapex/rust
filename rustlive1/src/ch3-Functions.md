```markdown
# Functions in Rust

Functions are the building blocks of Rust programs. This guide covers how to define and use functions, parameters, return values, and the difference between expressions and statements.

---

## Function Basics

- **Entry point**: Every Rust program starts with the `main` function.
- **Naming convention**: Use **snake_case** for function and variable names (e.g., `hello_world`, `tell_height`).  
  (kebab-case like `hello-world` is **not** allowed in Rust.)

```rust
fn main() {
    hello_world();                     // function call
    tell_height(182);
    human_id("Joel", 55, 182.2);
}
```

---

## Defining a Function

Use the `fn` keyword, followed by the name, parentheses `()`, and a block `{}`.

```rust
fn hello_world() {
    println!("Hello, Rust!");
}
```

> **Note:** Rust supports **hoisting** — you can call a function anywhere in the code, even before its definition.

---

## Functions with Parameters

Parameters are specified in the function signature with their types.

```rust
fn tell_height(height: u32) {
    println!("My height is {} cm.", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old, and my height is {} cm.",
        name, age, height
    );
}
```

- Each parameter must have an explicit type.
- Multiple parameters are separated by commas.

---

## Returning Values

To return a value, use `->` followed by the return type. The last expression in the function (without a semicolon) is returned implicitly.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b                     // no semicolon → this is the return value
}
```

You can also use an explicit `return` keyword, but the implicit return is idiomatic in Rust.

```rust
let y = add(4, 6);
println!("Value of y is: {}", y);
```

---

## Expressions vs. Statements

Understanding the distinction is key to writing Rust code.

- **Statement**: An instruction that performs an action but **does not return a value**.  
  Examples:
  - Variable declaration: `let x = 5;`
  - Function definition: `fn foo() {}`
  - Control flow without a value: `if condition { ... }` (when used as a statement)

- **Expression**: Anything that **evaluates to a value**.  
  Examples:
  - Literals: `5`, `true`, `"hello"`
  - Function calls: `add(3,4)`
  - Blocks: `{ let a = 1; let b = 2; a + b }`
  - `if` used as an expression: `if x > 0 { 1 } else { 0 }`

**Key rule**: In Rust, almost all statements end with a semicolon `;`, while expressions do **not** end with a semicolon when they are being returned.

### Example of a block expression

```rust
let _x: i32 = {
    let price = 5;
    let qty = 10;
    price * qty          // no semicolon → the value of the block
};
println!("Result is {}", _x);   // 50
```

The block `{ ... }` evaluates to the last expression inside it.

---

## Practical Example: BMI Calculator

This function calculates Body Mass Index (BMI) given weight (kg) and height (m).

```rust
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

// Usage:
let weight = 70.0;
let height = 1.82;
let bmi = calculate_bmi(weight, height);
println!("Your BMI is: {:.2}", bmi);
```

The function returns the computed BMI value, which can be used directly in expressions.

---

## Summary

- Functions are defined with `fn name() { ... }`.
- Parameters must have explicit types.
- Return type is specified with `-> Type`.
- The last expression in a function is returned automatically.
- Blocks (`{ ... }`) are expressions and can be used to compute values.
- Use snake_case for naming.

Mastering functions is essential for organizing code and reusing logic effectively in Rust.
```