use core::num;
/// 사용자 정의 에러 타입
/// 내가 직접 에러를 만들고 싶을 때
use std::error::Error;
use std::fmt;

struct EvenIntegerError {}

#[test]
fn example_1() {
    let numerator = 6;
    let denominator = 3;
    let answer = example_1_inner(&numerator, &denominator);
    dbg!(answer);
}

fn example_1_inner<'a>(numer: &'a i32, denom: &'a i32) -> i32 {
    let answer: i32 = numer / denom;
    answer
}