# Error handling

## Assignment 0

What are your options when it comes to dealing with "errors" or partial
functions in Rust? _Partial function_ is the mathematical term for functions
that do not produce a value for _some_ input values.

Can you find three (or four) different approaches in how to deal with errors?

Before starting with the assignments 1-4 below, try to come up with at least
three different approaches. Write them down. In the following assignments we
will implement each of these approaches.

## Assignment 1

Open up `src/sqrt.rs`. You will find a partially implemented function `sqrt`
(square root). Finish it's implementation such that it returns the correct
square root for positive inputs. What can you do for inputs that are not
positive? Run the test suite to figure out what behaviour is intended and
complete it's implementation.

Hint: Run tests with `cargo test --test test_sqrt`.

## Assignment 2

Fix `sqrt2` in `src/sqrt2.rs`. Return `None` in case the function is not
defined at the input value, otherwise `Some(x)`.

Run tests with `cargo test --test test_sqrt2`.

## Assignment 3

Implement `sqrt3` in `src/sqrt3.rs`. Instead of using an `Option<f32>`, use a
`Result` instead.

Also create a test suite for `sqrt3` in file `tests/test_sqrt3.rs`. You can
base this on `tests/test_sqrt2.rs`.

## Assignment 4a (advanced)

What are the downsides of the approaches we have implemented in Assignment 1-3?
Write them down!

## Assignment 4b (advanced)

Open `src/sqrt4.rs`. Create a new type `NonNegativeFloat` and implement `sqrt`
on this type. The purpose of this new type is to encapsulate a number that is
known to be non-negative. You cannot construct a value of this type for a
negative number. This means, that your `sqrt` implementation cannot fail.

Uncomment and run the test suite `tests/test_sqrt4.rs` to ensure correct
behaviour.

## Assignment 4c (advanced)

Implement the `TryFrom` trait for `NonNegativeFloat`, and add one more test
case to `tests/test_sqrt4.rs`.

## Assignment 5

Think about the consequences of the `newtype` approach that we learned in
Assignment 4. If you want, call someone of the team and discuss the pros and
cons of this approach.
