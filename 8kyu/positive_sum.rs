/*
  Title: Sum of Positive
  Difficulty: 8 Kyu
  Link: https://www.codewars.com/kata/5715eaedb436cf5606000381
*/

fn positive_sum(slice: &[i32]) -> i32 {
    let mut sum = 0; 
    
    for i in slice { 
        if *i > 0 {  
            sum += *i; 
        }
    }
    
    sum 
}