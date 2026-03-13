```markdown
# Compound Data Types in Rust

Compound data types group multiple values into one type. Rust has three primary compound types: **arrays**, **tuples**, and **slices**.

---

## Arrays

An array is a fixed‑length collection of elements of the **same type**. The type signature is `[T; N]` where `T` is the element type and `N` is a compile‑time constant (the length).

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
println!("Number Array: {:?}", numbers);
```

- All elements must have the same type (e.g., trying to mix types like `[1, 2, "apple", true]` would cause a compile error).
- Elements are accessed by indexing, starting at `0`:

```rust
let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
println!("First fruit: {}", fruits[0]); // Apple
println!("Second fruit: {}", fruits[1]); // Banana
```

Arrays are stack‑allocated and have a fixed size known at compile time.

---

## Tuples

A tuple is a fixed‑length collection that can hold values of **different types**. Tuples are written as a comma‑separated list inside parentheses.

```rust
let human = ("Alice", 30, false);
println!("Human Tuple: {:?}", human); // ("Alice", 30, false)
```

- Tuples can contain any combination of types, including other compound types:

```rust
let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
println!("My Mix Tuple: {:?}", my_mix_tuple);
```

- Access tuple elements either by index (using dot notation, e.g., `human.0`) or by destructuring.

---

## Slices

A slice is a **dynamically‑sized view** into a contiguous sequence of elements. Slices are references to a portion of an array (or another slice) and do not have ownership. They are written as `&[T]`.

```rust
let number_slices: &[i32] = &[1, 2, 3, 4, 5];
println!("Number Slice: {:?}", number_slices);
```

- Slices are useful for borrowing a part of a collection without copying.
- They can also be created from arrays or vectors:

```rust
let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
println!("Animal Slice: {:?}", animal_slices);
```

---

## Strings vs. String Slices (`String` and `&str`)

Rust has two main string types:

### `String` – Owned, Growable String

- Stored on the heap, can be mutated and grown.
- Created with `String::from` or `to_string()`.

```rust
let mut stone_cold = String::from("Hell, ");
stone_cold.push_str("Yeah!"); // Appends a string slice
println!("Stone cold says: {}", stone_cold); // "Hell, Yeah!"
```

### `&str` – String Slice

- A reference to a string (or part of one). Often called a “string slice”.
- It is a view into a `String` or a string literal.

```rust
let string = String::from("Hello, Megan!");
let slice: &str = &string;          // Full string slice
println!("Slice Value: {}", slice);
```

- You can take a slice of a specific part using range syntax:

```rust
let string = String::from("Hello, World!");
let slice = &string[0..5];           // "Hello"
println!("Slice Value: {}", slice);
```

- String literals (e.g., `"hello"`) are of type `&str`.

---

## Key Differences

| Feature               | Array                 | Tuple                 | Slice                 |
|-----------------------|-----------------------|-----------------------|-----------------------|
| Fixed length?         | Yes                   | Yes                   | No (view)             |
| Same type elements?   | Yes                   | No                    | Yes                   |
| Owns data?            | Yes                   | Yes                   | No (reference)        |
| Example               | `[i32; 5]`            | `(i32, bool, &str)`   | `&[i32]`              |

Understanding these compound types is essential for effective memory management and data organization in Rust.
```