/*
  Title: Vowel Count
  Difficulty: 7 Kyu
  Link: https://www.codewars.com/kata/54ff3102c1bad923760001f3
*/

fn get_count(string: &str) -> usize {
  let mut vowels_count: usize = 0;

  string.chars()
    .filter(|x| match x {
        'a'|'e'|'i'|'o'|'u' => true,
        _ => false,
    })
    .count()
}