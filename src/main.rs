use rayon::prelude::*;
use std::io::{stdin, BufRead};

const WORDS_LIST: &'static str = include_str!("word_list.txt");
const CHARS: [char; 26] = [
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
	't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
	let words = WORDS_LIST
		.lines()
		.par_bridge()
		.map(|v| v.to_string())
		.collect::<Vec<String>>();
	let mut guessed: Vec<char> = vec![];
	let mut last_length: Option<usize> = None;
	println!("Type your word: ");
	loop {
		let word = loop {
			let word = stdin().lock().lines().next().unwrap().unwrap();
			match last_length {
				Some(len) => {
					if len == word.len() {
						let un_guessed_count = word
							.chars()
							.filter(|ch| ch != &'_' && !guessed.contains(ch))
							.count();
						if un_guessed_count > 0 {
							println!("This word contains characters we haven't guessed yet, try putting it in again.");
							continue;
						} else {
							break word;
						}
					} else {
						println!("This word isn't the same length...");
						continue;
					}
				}
				None => {
					last_length = Some(word.len());
					guessed
						.append(&mut word.chars().filter(|ch| ch != &'_').collect::<Vec<char>>());
					break word;
				}
			}
		};
		if word.chars().par_bridge().filter(|v| v == &'_').count() == 0 {
			if word.len() > 0 {
				println!("I win!");
			}
			break;
		}
		let guess = best_letter(&words, word, guessed.clone());
		match guess {
			Ok(guess) => {
				guessed.push(guess.clone());
				println!("I think {} is the best letter.", guess);
				println!("Type your word: ");
			}
			Err(_) => break,
		}
	}
}

fn best_letter(words: &Vec<String>, word: String, guessed: Vec<char>) -> Result<&char, bool> {
	println!("\n");
	let remaining_words = words
		.into_par_iter()
		.filter(|v| {
			if v.len() != word.len() {
				return false;
			}
			let error_count = word
				.chars()
				.into_iter()
				.enumerate()
				.filter(|position| {
					let target = position.1;
					let source = v.chars().nth(position.0).unwrap();
					if target == '_' {
						if guessed.contains(&source) {
							return true;
						} else {
							return false;
						}
					}
					if target != source {
						return true;
					}
					false
				})
				.count();
			error_count == 0
		})
		.collect::<Vec<&String>>();
	if remaining_words.len() == 1 {
		println!("Your word is {}!", remaining_words[0]);
		return Err(true);
	} else if remaining_words.len() == 0 {
		println!("I admit defeat! I don't know any more words to ask you about.");
		if guessed.len() == 0 {
			println!("Is that even a word? It's pretty long...");
		}
		return Err(false);
	} else if remaining_words.len() < 5 {
		println!("I think your word might be one of {:?}", remaining_words);
	} else {
		println!("{} words are under consideration...", remaining_words.len());
	}
	let available = CHARS
		.par_iter()
		.filter(|v| !guessed.contains(v))
		.collect::<Vec<&char>>();
	let mut counts = available
		.into_par_iter()
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
		.filter(|(_, count)| count < &remaining_words.len())
		.collect::<Vec<(&char, usize)>>();
	counts.sort_by(|a, b| a.1.cmp(&b.1));
	println!(
		"{:?}, {:?}",
		counts.first().unwrap(),
		counts.last().unwrap()
	);
	Ok(counts.last().unwrap().0)
}

#[test]
fn check_every_word() {
	let words = WORDS_LIST
		.lines()
		.map(|v| v.to_string())
		.collect::<Vec<String>>();
	let successes = words
		.clone()
		.par_iter()
		.map(|target| {
			let mut guessed: Vec<char> = vec![];
			let correct: bool = {
				let mut correct = false;
				for _ in 0..26 {
					let word = target
						.chars()
						.into_iter()
						.map(|ch| if guessed.contains(&ch) { ch } else { '_' })
						.collect::<String>();
					let guess = best_letter(&words, word, guessed.clone());
					match guess {
						Ok(g) => {
							guessed.push(g.clone());
						}
						Err(correct_) => {
							correct = correct_;
							break;
						}
					};
				}
				correct
			};
			correct
		})
		.collect::<Vec<bool>>();
	println!("{}", successes.iter().filter(|v| **v).count())
}
