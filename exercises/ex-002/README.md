# Traits and Iterators

## Context

Imagine we are writing a web/HTTP service. For that purpose we need to generate
a HTTP response. Part of that response contains the HTTP headers. As you might
know, HTTP headers look like this:

```
Content-Type: text/html
Content-Length: 42
Cookie-Set: CookieValue1
Cookie-Set: CookieValue2
```

To simplify generating HTTP headers programmatically, we are going to implement
a `struct HttpHeaders` that can be used like this:

```rust
let mut headers = HttpHeaders::new();
headers.add("Content-Length", "42");
headers.add("Cookie-Set", "CookieValue1");
headers.add("Cookie-Set", "CookieValue2");
println!("{}", headers.to_string());
```

## Assignment 1

Open `src/assignment1.rs`. You see a struct `HttpHeaders`. Complete the
implementation. Choose an appropriate data structure. Why did you choose this
particual data structure? Can you explain your decision?

Verify the correctness of your implementation by running the test suite `cargo test --test
test_assignment1`.

## Assignment 2

Open `src/assignment2.rs`. Add your existing code from Assignment 1 and extend
it for method `iter`.  The `iter` method allows you to iterate over all
key/value pairs.

Depending on the data structure choosen, you might need to implement your own
`Iterator`.

Verify the correctness of your implementation by running the test suite `cargo test --test
test_assignment2`.

## Assignment 3

Open `src/assignment3.rs` and implement `to_string`.

Discuss a possible performance issue in case you based the implementation of
`to_string` on the `iter` method from Assignment 2? Can you think of a
solution?

Hint: It's about lifetimes.

Verify the correctness of your implementation by running the test suite `cargo test --test
test_assignment3`.

## Assignment 4

Open `src/assignment4.rs` and implement `iter_ref()`. We no longer want to
interate over `(String, String)` but instead over `(&str, &str)`.

Discuss advantages of this and fix your `to_string` method to use `iter_ref`.

Verify the correctness of your implementation by running the test suite `cargo test --test
test_assignment4`.
