#![feature(test)]

extern crate test;

use sha2::{Sha256, Digest};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufRead};

const USER: &str = "zifan";
const SALT: &str = "8934029034";
const FILE: &str = "dictionary.txt";
const HASH: &str = "1ca6004d870d5c9dcf2ffd231046a9015072a518c708040a02bf8b5b3a4e18b2";

fn parse_hex(input: char) -> u8 {
    match input {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!("not valid hex string"),
    }
}

fn parse_hex_str(s: &str) -> Vec<u8> {
    let mut acc = None;
    let mut goal = Vec::new();
    for c in s.chars() {
        let y = parse_hex(c);
        if let Some(x) = acc {
            goal.push((x << 4) + y);
            acc = None;
        } else {
            acc = Some(y);
        }
    }

    goal
}

// want to map each char to a list of subs
fn char_swap(c: char) -> Vec<char> {
    let mut res = vec![c];
    if c.is_alphabetic() {
        res.push(c.to_ascii_uppercase());
    }

    match c {
        'a' => {
            res.push('4');
            res.push('@')
        },
        'l' => {
            res.push('7');
            res.push('1')
        },
        's' => res.push('5'),
        'e' => res.push('3'),
        'i' => res.push('1'),
        'o' => res.push('0'),
        _ => (),
    }

    res
}

fn string_build(possible_chars: &Vec<Vec<char>>) -> Vec<String> {
    let mut res = vec![String::from("")];
    for chars in possible_chars {
        let new_res: Vec<Vec<String>> = res.into_iter().map(|s| {
            chars.into_iter()
                 .map(|c| {
                     let mut x = String::from(s.clone());
                     x.push(*c);
                     x
                 })
                .collect()
        }).collect();
        res = new_res.into_iter().flatten().collect();
    }

    res
}

fn generate_permutations(password: &str) -> Vec<String> {
    let p_chars: Vec<Vec<char>> = password.chars().map(char_swap).collect();
    string_build(&p_chars)
}

fn check_str(goal: &[u8], password: &str) -> bool {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(format!("{},{},{}", USER, password, SALT).as_bytes());
    for _ in 0..256 {
        let mut hasher = Sha256::new();
        hasher.input(&bytes);
        bytes.clear();
        bytes.extend_from_slice(hasher.result().as_slice())
    }

    bytes[..] == goal[..]
}

// about 0.12 s per passwd
fn check_permutations(goal: &[u8], s: &str) -> Option<String> {
    for perm in generate_permutations(s) {
        if check_str(goal, &perm) {
            println!("Found match {}", perm);
            return Some(String::from(perm))
        }
    }
    None
}

fn main() {
    let goal = parse_hex_str(HASH);
    let file = File::open(FILE).unwrap();

    let password: Option<String> = BufReader::new(file)
        .lines()
        .find_map(|line| {
            let l = line.unwrap();
            check_permutations(&goal, &l)
        });

    if let Some(p) = password {
        println!("Found match: {}", p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let c = 'a'; // want a, A, 4, @
        assert_eq!(vec!['a', 'A', '4', '@'], char_swap(c));
    }

    #[test]
    fn test_string_build() {
        let p_chars = vec![vec!['a', 'A'], vec!['b', 'B']];
        assert_eq!(vec![String::from("ab"), String::from("Ab"), String::from("aB"), String::from("AB")].sort(),
                   string_build(&p_chars).sort());
    }

    #[test]
    fn test_perms() {
        let example = "facebook";
        assert_eq!(generate_permutations(&example).len(), 1728);
    }

    #[bench]
    fn bench_password_hash_check(b: &mut test::Bencher) {
        let ex = "facebook";
        let goal = parse_hex_str(HASH);
        b.iter(|| check_permutations(&goal, &ex));
    }
}
