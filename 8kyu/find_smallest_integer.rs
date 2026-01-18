/*
  Title: Find the smallest integer in the array
  Difficulty: 8 Kyu
  Link: https://www.codewars.com/kata/55a2d7ebe362935a210000b2
*/

fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}