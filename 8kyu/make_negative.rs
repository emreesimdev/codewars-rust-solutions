/*
  Title: Return Negative
  Difficulty: 8 Kyu
  Link: https://www.codewars.com/kata/55685cd7ad70877c23000102

  Description:
  In this simple assignment you are given a number and have to make it negative. 
  But maybe the number is already negative?
*/

fn make_negative(n: i32) -> i32 {
    if n > 0 {
        -n
    } else {
        n
    }
}