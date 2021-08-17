// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

use std::convert::TryFrom;

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match u64::try_from(value) {
            Err(_) => Err(CreationError::Negative),
            Ok(0) => Err(CreationError::Zero),
            Ok(val) => Ok(PositiveNonzeroInteger(val))
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
