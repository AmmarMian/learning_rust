---
layout: page
title: Rustlings
menu:
  main:
    weight: 2
mermaid: false
toc: true
---

Some notes about Rust from doing Rustlings

<!--more-->

## Borrowing versus Ownership

Borrowing means that we pass a reference to a variable, and the ownership remains with the original variable. This is useful when we want to pass a variable to a function, but we don't want to give the ownership to the function.

A typical function that only borrows a variable looks like this:
```rust
fn somefunc(s: &String) {
    // Do something with s
}
```
here we only pass a reference to the String variable.

Ownership means that we pass the ownership of a variable to a function, and the function is responsible for freeing the memory when it's done with it.

A typical function that takes ownership of a variable looks like this:
```rust
fn somefunc(s: String) {
    // Do something with s
}
```
here we pass the String variable itself. s no longer exists after the function call.

**If we want to keep the ownership outside, we have to transfer it again to the original variable thanks to return.**
for example:
```rust
fn creates_string() -> String {
    // This function creates a String and returns it,
    // transferring ownership of the new String to the caller.
    let s = String::from("Hello, Rust!");
    s // Ownership of the String `s` is moved out to the caller.
}

fn main() {
    let my_string = creates_string(); // Ownership of the returned String is moved to `my_string`.
    println!("{}", my_string); // This works fine, `my_string` is the owner of the String.
    // The String `my_string` owns is dropped here when `my_string` goes out of scope.
}
```

## Moved

When a function takes ownership of a variable, the original variable is considered "moved" at the point of the function call, not at the end of the function. This means the original variable is no longer valid after it has been passed to the function, and Rust's ownership rules ensure that it cannot be used accidentally after the move. The "drop" happens for the value at the end of the scope where the new owner (the function parameter) resides, not where the original variable was defined.

{{<info>}}
* A `match` expression move (takes ownership) of the value that matches the pattern. If we do not want to move the value, we can use a reference to the value in the pattern.
* The keyword `ref` is **super useful** when we want to take a reference to a value in a pattern without moving it. We can also add `mut` to make it mutable: `Some(ref mut val)`.
{{</info>}}



## Structs

Akin to classes but with a few tweaks:
* Immutable by default. We need to use the keyword `mut` to make a field mutable.
* Methods are defined in the `impl` block.


## Enums

Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values. For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle.
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
* Quit has no data associated with it at all.
* Move has named fields, like a struct does.
* Write includes a single String.
* ChangeColor includes three i32 values.

## Strings

* A string slice is a reference to part of a String, and it looks like this:
```rust
let string = String::from("Hello, world!");
const string_slice: &str = &string[0..5];
```

* In Rust, the + operator for string concatenation requires the left-hand operand to be of type String, not just a &str or a borrowed String. This is because the + operation is defined in such a way that it consumes the left-hand operand, appending the right-hand operand to it, and returns a new String. This means the left-hand side must be owned so it can be modified or "taken" by the operation.

## Modules

* Modules are a way to organize code in Rust. The advantages are:
 * They allow us to group related code together and keep it separate from other code.
 * They allow us to define public interfaces that hide implementation details.
  
* Using two "imports" at same time:
```rust
use std::{fmt::Display, io::Result};
```


## HashMap

* Similar to python dictionaries.
* To have different key types, use enum.


## Iterating over vecs:

Rust's borrowing rules mean that when you iterate over a vector with .iter(),
you get references to the elements of the vector, not the elements themselves.


## Option

* Returning nothing => `None`
* Returning something => `Some(value)`

## Results

To handle erors without exceptions as in other languages, we use the Result enum as an expected return type of a function. The Result enum has two variants:
* `Ok` - The value is present
* `Err` - An error has occurred

We can then match against the result to handle the error or the value.

Handling Result
Rust provides several methods to work with Result types, such as unwrap(), expect(), match expressions, and the ? operator, each serving different use cases and levels of error handling granularity.

* unwrap(): Retrieves the Ok value or panics if the result is Err.
expect(): Similar to unwrap(), but allows specifying an error message if the result is Err.
match: Provides detailed control over handling both Ok and Err cases.
* ? Operator: Propagates the error to the calling function, offering a concise way to handle errors in functions that return Result.
* Using the Result type effectively can lead to more reliable and clear error handling in Rust applications, making your code safer and more predictable.

Main function can also return a result to handle errors in the program. The signature would be: `fn main() -> Result<(), ErrorType> { ... }`

## Traits

### Traits as parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

## Lifetime annotations

* Needed when returning references from functions. Because the compiler needs to know the lifetime of the reference if ambiguous.
  
An example:
```rust
fn earliest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 < s2 { s1 } else { s2 }
}
```

Here, 'a is a named lifetime parameter. The annotations tell the compiler that both input references (s1 and s2) and the returned reference all share the same lifetime. This means the return value will be valid as long as both inputs are valid.


## Iterators

* Iterating over  vec with `iter` or `iter_mut` gives references to the elements. So if we ahd `&str`, now we have `&&str`.


## Cow

It is a smart pointer allowing to work with either owned or borrowed data. It is useful when we want to avoid unnecessary allocations.

If we pass a reference: `Cow::from(&mut s)`, the Cow will contain a reference to the string. If we pass an owned value: `Cow::from(s)`, the Cow will contain the owned value. This means a few things:
* When we pass a mutable reference, the data will be cloned if changed and will return a `Cow::Owned` value to a new owned value.
* If we do not modify the data, the `Cow` will return a `Cow::Borrowed` value with a reference to the original data.
* ¨If we pass a value, it will always return a `Cow::Owned` value. That is because the data is owned as usual.

## Multiple producers, single consumer (mspc)

When sending a message through a thread, the `spawn(move ||...)` takes ownership of the `tx` transmitter so we can't use it again. We need to clone it to use it in another thread.

See [here](https://doc.rust-lang.org/book/ch16-02-message-passing.html):
```rust
let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
```



