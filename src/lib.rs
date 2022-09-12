//! This crate provides a `pipe` macro to easily compose functions together
//! with a new syntax preventing nested calls which are hard to read and understand.
//!
//! example usage:
//! ```
//! use another_pipe_macro::pipe;
//!
//! let res = pipe!( "32" => str::parse::<i32> => Result::unwrap);
//!
//! assert_eq!(res, 32);
//! ```
//!
//! ## Should you use this crate?
//! No.
//!
//! It is meant as an exercice in writing macros and publishing to crates.io, and
//! possibly lacks features. If you want a more mature crate, use the [pipeline](https://crates.io/crates/pipeline) crate.

#[macro_export]
/// The `pipe` macro allows a new syntax to easily compose functions together.
///
/// example:
/// ```
/// use another_pipe_macro::pipe;
///
/// let res = pipe!(
///     "30"
///     => str::parse::<i32>
///     => Result::unwrap // works with methods using function call syntax
///     => |x| x + 2 // works with closures
/// );
///
/// assert_eq!(res, 32);
/// ```
///
/// The above example's `pipe` macro invocation expands to `(|x| x + 2)((Result::unwrap)((str::parse::<i32>)("30")))`
macro_rules! pipe {
    (@wrap $func:expr, $($tail:expr),*) => {
        ($func)(pipe!(@wrap $($tail),*))
    };
    (@wrap $x:expr) => {
        $x
    };
    (@reverse [] $($reversed:expr),* $(,)?) => {
        pipe!(@wrap $($reversed),*)
    };
    (@reverse [$head:expr, $($tail:expr),+] $($reversed:expr),* $(,)?) => {
        pipe!(@reverse [$($tail),+] $head, $($reversed),*)
    };
    (@reverse [$head:expr] $($reversed:expr),* $(,)?) => {
        pipe!(@reverse [] $head, $($reversed),*)
    };
    ($x:expr) => {
        $x
    };
    ($x:expr $(=> $functions:expr)+) => {
        pipe!(@reverse [$x, $($functions),+])
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn pipe_macro_result() {
        // Given
        let some_string = "32";
        let expected_result = 32;
        // When
        let r = pipe!(some_string => str::parse::<i32> => Result::unwrap);
        // Then
        assert_eq!(r, expected_result, "We are testing that the result given by the expansion of the macro 'r' (value: {}) is equal to our expected value: {}", r, expected_result);
    }
}
