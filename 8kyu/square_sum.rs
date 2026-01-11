/*
  Title: Square(n) Sum
  Difficulty: 8 Kyu
  Link: https://www.codewars.com/kata/515e271a311df0350d00000f
*/

fn square_sum(vec: Vec<i32>) -> i32 {
    let mut sq_sum = 0;
    for i in vec {
        sq_sum += i * i;
    }
    sq_sum
}