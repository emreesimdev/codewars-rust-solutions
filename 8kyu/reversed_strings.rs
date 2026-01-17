/*
  Title: Reversed Strings
  Difficulty: 8 Kyu
  Link: https://www.codewars.com/kata/5168bb5dfe9a00b126000018
*/

fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}