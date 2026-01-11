/*
  Title: Grasshopper - Summation
  Difficulty: 8 Kyu
  Link: https://www.codewars.com/kata/55d24f55d7dd296eb9000030

  Note: 
  There are two ways to solve this.
  1. Iterative Way (Loop) -> Good for learning programming logic.
  2. Mathematical Way (Gauss Formula) -> n * (n + 1) / 2 -> Best for performance.
*/

fn summation(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}