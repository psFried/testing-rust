

/// Computes the factorial of the input!
///
/// ```
/// use testing_rust::factorial;
///
/// assert_eq!(2, factorial(2));
/// assert_eq!(24, factorial(4));
/// ```
pub fn factorial(input: i64) -> i64 {
    let mut f = 1;
    for i in 1..input {
        f += i * f;
    }
    f
}
