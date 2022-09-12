# another_pipe_macro
This crate provides a `pipe` macro to easily compose functions together
with a new syntax preventing nested calls which are hard to read and understand.

example usage:
```
use another_pipe_macro::pipe;

let res = pipe!( "32" => str::parse::<i32> => Result::unwrap);

assert_eq!(res, 32);
```

## Should you use this crate?
No.

It is meant as an exercice in writing macros and publishing to crates.io, and
possibly lacks features. If you want a more mature crate, use the [pipeline](https://crates.io/crates/pipeline) crate.