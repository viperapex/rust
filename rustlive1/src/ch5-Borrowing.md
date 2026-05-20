````markdown
# Borrowing in Rust

Borrowing is one of Rust’s core concepts for managing memory safely without a garbage collector. It allows functions and methods to access data **without taking ownership**.

This guide covers immutable borrowing, mutable borrowing, borrowing rules, and how borrowing works with methods.

---

## Example Program

```rust
fn main() {
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Immutable borrow to check balance
    account.check_balance();

    // Mutable borrow to modify balance
    account.withdraw(45.5);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );

        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner,
            self.balance
        );
    }
}
```

---

## Ownership Refresher

Rust follows three ownership rules:

1. Every value has one owner.
2. A value can only have one owner at a time.
3. When the owner goes out of scope, the value is dropped.

Example:

```rust
let account = BankAccount {
    owner: "Alice".to_string(),
    balance: 150.55,
};
```

`account` owns:

- `owner` → `String`
- `balance` → `f64`

---

## What is Borrowing?

Borrowing allows code to use data temporarily **without transferring ownership**.

Rust supports two kinds of borrowing:

- **Immutable borrowing** → `&T`
- **Mutable borrowing** → `&mut T`

---

## Immutable Borrowing (`&self`)

Immutable borrowing gives **read-only access** to data.

Method:

```rust
fn check_balance(&self) {
    println!(
        "Account owned by {} has a balance of {}",
        self.owner,
        self.balance
    );
}
```

Equivalent form:

```rust
fn check_balance(self: &BankAccount)
```

Usage:

```rust
account.check_balance();
```

Characteristics:

- Can read data
- Cannot modify data
- Ownership remains with original variable

Example:

```rust
let acc_ref = &account;

println!("{}", acc_ref.balance);
```

Multiple immutable borrows are allowed:

```rust
let a = &account;
let b = &account;
let c = &account;
```

This is valid because all references only read data.

---

## Mutable Borrowing (`&mut self`)

Mutable borrowing gives **write access** to data.

Method:

```rust
fn withdraw(&mut self, amount: f64) {
    self.balance -= amount;
}
```

Equivalent form:

```rust
fn withdraw(self: &mut BankAccount, amount: f64)
```

Usage:

```rust
account.withdraw(45.5);
```

Balance changes:

```text
150.55 → 105.05
```

Characteristics:

- Can read and modify
- Ownership stays with original variable
- Only one mutable borrow allowed at a time

Example:

```rust
let acc_mut = &mut account;

acc_mut.balance -= 20.0;
```

---

## Borrowing Rules

Rust enforces rules at compile time to prevent memory errors.

### Rule 1: Many immutable borrows allowed

Valid:

```rust
let a = &account;
let b = &account;
```

Both can read safely.

---

### Rule 2: Only one mutable borrow allowed

Valid:

```rust
let a = &mut account;
```

Invalid:

```rust
let a = &mut account;
let b = &mut account;
```

Rust error:

```text
cannot borrow as mutable more than once
```

Reason:

Two writers could create data races.

---

### Rule 3: Mutable and immutable borrows cannot coexist

Invalid:

```rust
let a = &account;
let b = &mut account;
```

Why?

`a` reads while `b` modifies.

Rust prevents this conflict.

---

## Method Receivers in Rust

Methods use special receiver types.

### Ownership Receiver (`self`)

```rust
fn consume(self)
```

Takes ownership.

Example:

```rust
fn close_account(self) {
    println!("Account closed");
}
```

After calling:

```rust
account.close_account();
```

`account` becomes unusable.

---

### Immutable Receiver (`&self`)

```rust
fn check_balance(&self)
```

- Read-only access
- Ownership retained

Used for:

```rust
account.check_balance();
```

---

### Mutable Receiver (`&mut self`)

```rust
fn withdraw(&mut self)
```

- Allows modification
- Ownership retained

Used for:

```rust
account.withdraw(45.5);
```

---

## Program Flow

### Step 1 — Create account

```rust
let mut account = BankAccount {
    owner: "Alice".to_string(),
    balance: 150.55,
};
```

State:

```text
Owner: Alice
Balance: 150.55
```

---

### Step 2 — Check balance

```rust
account.check_balance();
```

Output:

```text
Account owned by Alice has a balance of 150.55
```

Uses immutable borrowing.

---

### Step 3 — Withdraw money

```rust
account.withdraw(45.5);
```

Output:

```text
Withdrawing 45.5 from account owned by Alice
```

New balance:

```text
105.05
```

Uses mutable borrowing.

---

## Why Borrowing Exists

Borrowing provides:

- Memory safety
- No dangling pointers
- No null references
- Data race prevention
- No garbage collector overhead

Rust performs these checks at compile time.

---

## Summary

| Receiver | Meaning | Ownership | Modification |
|----------|----------|------------|--------------|
| `self` | Takes ownership | Moved | Yes |
| `&self` | Immutable borrow | Retained | No |
| `&mut self` | Mutable borrow | Retained | Yes |

Key points:

- Use `&self` for reading.
- Use `&mut self` for modification.
- Multiple immutable borrows are allowed.
- Only one mutable borrow exists at a time.
- Mutable and immutable borrows cannot coexist.

Borrowing enables Rust’s memory safety without needing a garbage collector.
````
