#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

/// Computes the factorial of the input!
///
/// ```
/// use testing_rust::factorial;
///
/// assert_eq!(2, factorial(2));
/// assert_eq!(24, factorial(4));
/// ```
pub fn factorial(input: i64) -> i64 {
    let mut f: i64 = 1;
    for i in 1..input {
        f = f.saturating_add(f.saturating_mul(i));
    }
    f
}

/// Returns the factorial of a number that's passed as a byte slice
/// This is a pretty contrived example, but it's meant to be used as
/// an example of where fuzzing would be useful
pub fn factorial_of_str(input: &[u8]) -> Result<i64, &'static str> {
  let string_input = std::str::from_utf8(input).map_err(|_| "input is not valid utf8")?;
  let int = string_input.parse().map_err(|_| "input is not a valid integer")?;
  Ok(factorial(int))
}


// This entire test module will be excluded from the normal builds. Tests don't actually
// _need_ to be inside of a test module like this, but it does make it easier to keep things
// clean and organized.
#[cfg(test)]
mod test {
    use super::factorial;

    #[test]
    fn factorial_of_5_is_120() {
        assert_eq!(120, factorial(5));
    }

    // Uses the quickcheck crate for property-based tests
    // https://github.com/BurntSushi/quickcheck
    //
    // This function will be automatically called with a variety of inputs. Since this is a unit function,
    // the test will pass as long as it doesn't panic. You could also return a `Testable` value like a `bool`
    #[quickcheck]
    fn factorial_does_not_panic(input: i64) {
        factorial(input);
    }

    #[test]
    fn factorial_returns_max_value_for_any_input_over_20() {
        assert_eq!(i64::max_value(), factorial(21));
        assert_eq!(i64::max_value(), factorial(42));
        assert_eq!(i64::max_value(), factorial(900));
    }
}
