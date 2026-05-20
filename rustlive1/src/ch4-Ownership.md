```markdown
# Ownership in Rust

Ownership is Rust’s most unique feature and enables memory safety without a garbage collector. It consists of three core rules:

1. **Each value in Rust has a variable that’s its owner.**
2. **There can be only one owner at a time.**
3. **When the owner goes out of scope, the value is dropped (memory freed).**

---

## The Stack vs. Heap

- **Stack**: Stores values with a known, fixed size. Fast and automatically managed.
- **Heap**: Stores dynamic data (like `String`). The owner’s variable holds a pointer to the heap data.

When ownership rules are violated, Rust prevents memory issues like double‑free, dangling pointers, or memory leaks.

---

## Move Semantics

When a value is assigned to another variable or passed to a function, ownership **moves** (unless the type implements the `Copy` trait).

### Example: Move of a `String` (heap data)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;               // s1 is moved to s2

    // println!("{}", s1);     // ❌ error: s1 no longer owns the value
    println!("{}", s2);        // ✅ ok
}
```

### Example: Copy of an integer (stack data)

```rust
fn main() {
    let x = 5;
    let y = x;                 // x is copied (integers implement Copy)
    println!("{}", x);         // ✅ still valid
    println!("{}", y);
}
```

Types that are `Copy` (simple scalars like integers, booleans, characters) are automatically copied; heap types like `String` are moved.

---

## Ownership and Functions

Passing a value to a function transfers ownership to the function’s parameter. The original variable is no longer usable after the call.

```rust
fn main() {
    let s = String::from("Rust");
    take_ownership(s);         // s moved into the function

    // println!("{}", s);      // ❌ error: s is no longer valid
}

fn take_ownership(str: String) {
    println!("{}", str);
}                              // str dropped here (memory freed)
```

---

## Returning Ownership

A function can return ownership back to the caller.

```rust
fn main() {
    let s1 = gives_ownership();         // s1 takes ownership of the returned value
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 moved, then returned to s3
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string                         // returned, ownership moves out
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string                            // returned, ownership moves out
}
```

---

## Borrowing (References)

To avoid moving ownership every time we pass data, we can **borrow** using references (`&`). A reference allows a function to read (or mutate) a value without taking ownership.

### Immutable Borrowing

```rust
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);   // we pass a reference, not ownership
    println!("Length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}                                    // s goes out of scope, but nothing is dropped because it’s a reference
```

The `&s1` syntax creates a reference that **borrows** the value. The original `s1` remains valid.

### Mutable Borrowing

We can also borrow a value mutably with `&mut`, but there are strict rules:

- Only **one mutable reference** to a particular piece of data in a scope.
- No immutable references while a mutable reference exists (to prevent data races).

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

---

## Dangling References

Rust guarantees that references never dangle (point to freed memory). The compiler checks that any reference is valid for its entire lifetime.

```rust
fn main() {
    let reference = dangle();      // ❌ error: reference points to a dropped value
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s                             // s is dropped here, returning a reference to it is invalid
}
```

Instead, return the `String` directly (transfer ownership) to avoid dangling.

---

## The Provided Code Example

```rust
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);      // borrows s1 immutably
    println!("Length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()                               // can read s, but cannot modify it
}
```

**Explanation**:
- `s1` owns the `String` `"RUST"`.
- `calculate_length` takes an immutable reference `&String`, so ownership stays with `s1`.
- After the function returns, `s1` can still be used in `println!`.

---

## Rules Summary

| Rule | Description |
|------|-------------|
| **Move** | Assignment or passing a value transfers ownership, except for `Copy` types. |
| **Borrow** | Use `&` to reference a value without taking ownership. |
| **Mutable Borrow** | Use `&mut` to get a mutable reference; only one allowed at a time. |
| **Lifetime** | References must always be valid; Rust prevents dangling references. |

These rules are enforced at compile time, ensuring memory safety without runtime overhead.
```