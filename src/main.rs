use std::io::{stdin, BufRead};

use rand::{prelude::SliceRandom, thread_rng};

const WORDS_LIST: &'static str = include_str!("word_list.txt");
const CHARS: [char; 26] = [
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
	't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
	let words = WORDS_LIST
		.lines()
		.map(|v| v.to_string())
		.collect::<Vec<String>>();
	let mut guessed: Vec<char> = vec![];
	println!("Type your word: ");
	loop {
		let word = stdin().lock().lines().next().unwrap().unwrap();
		if word.chars().into_iter().filter(|v| v == &'_').count() == 0 {
			if word.len() > 0 {
				println!("I win!");
			}
			break;
		}
		let guess = best_letter(&words, word, guessed.clone());
		match guess {
			Some(guess) => {
				guessed.push(guess.clone());
				println!("I think {} is the best letter.", guess);
				println!("Type your word: ");
			}
			None => break,
		}
	}
}

fn best_letter(words: &Vec<String>, word: String, guessed: Vec<char>) -> Option<&char> {
	println!("\n");
	let remaining_words = words
		.into_iter()
		.filter(|v| {
			if v.len() != word.len() {
				return false;
			}
			for i in 0..word.len() {
				let target = word.chars().nth(i).unwrap();
				let source = v.chars().nth(i).unwrap();
				if target == '_' {
					if guessed.contains(&source) {
						return false;
					} else {
						continue;
					}
				}
				if target != source {
					return false;
				}
			}
			true
		})
		.collect::<Vec<&String>>();
	println!("{} words are under consideration...", remaining_words.len());
	if remaining_words.len() == 1 {
		println!("Your word is {}!", remaining_words[0]);
		return None;
	} else if remaining_words.len() == 0 {
		println!("I admit defeat! I don't know any more words to ask you about.");
		return None;
	} else if remaining_words.len() < 5 {
		println!("I think your word might be one of {:?}", remaining_words);
	}
	let available = CHARS
		.iter()
		.filter(|v| !guessed.contains(v))
		.collect::<Vec<&char>>();
	let mut counts = available
		.into_iter()
		.map(|ch| {
			(
				ch,
				remaining_words
					.clone()
					.into_iter()
					.filter(|w| w.chars().collect::<Vec<char>>().contains(ch))
					.count(),
			)
		})
		.collect::<Vec<(&char, usize)>>();
	counts.sort_by(|a, b| a.1.cmp(&b.1));
	println!(
		"{:?}, {:?}",
		counts.first().unwrap(),
		counts.last().unwrap()
	);
	Some(counts.last().unwrap().0)
}
