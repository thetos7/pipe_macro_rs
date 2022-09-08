#[macro_export]
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
