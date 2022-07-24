use core::num;
/// 사용자 정의 에러 타입
/// 내가 직접 에러를 만들고 싶을 때
use std::fmt;

struct EvenIntegerError;

impl fmt::Display for EvenIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The answer cannot be an even integer..")
    }
}

impl fmt::Debug for EvenIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The answer cannot be an even integer..")
    }
}

impl std::error::Error for EvenIntegerError {}

/// 반환하는 에러 타입이 하나만 존재할 때는 아래와 같이 구현 가능.
#[test]
fn example_1() {
    let numerator = 6;
    let denominator = 0;
    let answer = example_1_inner(&numerator, &denominator).unwrap();
    dbg!(answer);
}

fn example_1_inner<'a>(numer: &'a i32, denom: &'a i32) -> Result<i32, EvenIntegerError> {
    let answer: i32 = numer / denom;
    if answer % 2 == 0 {
        Err(EvenIntegerError)
    } else {
        Ok(answer)
    }
}