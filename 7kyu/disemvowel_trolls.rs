/*
  Title: Disemvowel Trolls
  Difficulty: 7 Kyu
  Link: https://www.codewars.com/kata/52fba66badcd10859f00097e
*/

fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|x| match x {
            'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => false,
            _ => true,
    })
    .collect()
}