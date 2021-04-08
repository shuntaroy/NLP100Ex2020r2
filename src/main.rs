//! 言語処理100本ノック
//! 第1章: 準備運動
//!
use nlp100ex2020r2::chapter1::*;
// use nlp100ex2020r2::chapter2::*;
// use nlp100ex2020r2::chapter3::*;
// use nlp100ex2020r2::chapter4::*;
// use nlp100ex2020r2::chapter5::*;
// use nlp100ex2020r2::chapter6::*;
// use nlp100ex2020r2::chapter7::*;

fn main() {
    println!("Chapter 1");
    println!("Exercise 00: {}", &ex00("stressed"));
    println!("Exercise 01: {}", &ex01("パタトクカシーー"));
    println!("Exercise 02: {}", &ex02("パトカー", "タクシー"));
    println!("Exercise 03: {:?}", &ex03("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."));
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test00() {
//         assert_eq!("desserts", ex00("stressed"))
//     }
//     #[test]
//     fn test01() {
//         assert_eq!("パトカー", ex01("パタトクカシーー"))
//     }
// }
