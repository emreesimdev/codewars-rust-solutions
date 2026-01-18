/*
  Title: Count of positives / sum of negatives
  Difficulty: 8 Kyu
  Link: https://www.codewars.com/kata/576bb71bbbcf0951d5000044
*/

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }
    
    let count = input.iter().filter(|&x| x > &0).count() as i32;
    
    let sum = input.iter().filter(|&x| x < &0).sum::<i32>();
    
    vec![count, sum]
}