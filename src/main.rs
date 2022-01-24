use rayon::prelude::*;
use std::io::{stdin, BufRead};

enum SnowmanResult {
	Success(String),
	NoMoreWords(bool),
	Considering(Vec<String>, char),
	ConsideringMany(usize, char),
	UnknownError,
}

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
			SnowmanResult::Considering(words, guess) => {
				println!("I'm considering {}", words.join(", "));
				guessed.push(guess.clone());
				println!("I think {} is the best letter.", guess);
				println!("Type your word: ");
			}
			SnowmanResult::ConsideringMany(count, guess) => {
				println!("I'm considering {} words", count);
				guessed.push(guess.clone());
				println!("I think {} is the best letter.", guess);
				println!("Type your word: ");
			}
			SnowmanResult::Success(word) => {
				println!("I win!");
				println!("The word was {}", word);
				break;
			}
			SnowmanResult::NoMoreWords(l) => {
				println!("I lose!");
				println!("I couldn't find any words that fit the word you gave me.");
				if l {
					println!("Was that even a word? It looked very long...");
				}
				break;
			}
			SnowmanResult::UnknownError => {
				println!("I lose!");
				println!("I don't know what happened, but I lost.");
				break;
			}
		}
	}
}

fn best_letter(words: &Vec<String>, word: String, guessed: Vec<char>) -> SnowmanResult {
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
		return SnowmanResult::Success(remaining_words[0].clone());
	} else if remaining_words.len() == 0 {
		return SnowmanResult::NoMoreWords(guessed.len() == 0);
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
	if counts.len() == 0 {
		SnowmanResult::UnknownError
	} else {
		if cfg!(not(test)) {
			println!(
				"{:?}, {:?}",
				counts.first().unwrap(),
				counts.last().unwrap(),
			);
		}
		if remaining_words.len() < 5 {
			SnowmanResult::Considering(
				remaining_words.iter().map(|v| (*v).clone()).collect(),
				*counts.last().unwrap().0,
			)
		} else {
			SnowmanResult::ConsideringMany(remaining_words.len(), *counts.last().unwrap().0)
		}
	}
}

#[test]
fn check_every_word() {
	let now = Instant::now();
	let words = WORDS_LIST
		.lines()
		.map(|v| v.to_string())
		.filter(|v| v.chars().all(|c| c.is_ascii_lowercase()))
		.collect::<Vec<String>>();
	let successes = words
		.clone()
		.par_iter()
		.map(|target| {
			let mut guessed: Vec<char> = vec![];
			let (correct, guesses): (bool, u8) = {
				let mut guesses = 0;
				let correct;
				loop {
					guesses += 1;
					let word = target
						.chars()
						.into_iter()
						.map(|ch| if guessed.contains(&ch) { ch } else { '_' })
						.collect::<String>();
					let guess = best_letter(&words, word, guessed.clone());
					match guess {
						SnowmanResult::Considering(_, g) | SnowmanResult::ConsideringMany(_, g) => {
							guessed.push(g.clone());
						}
						SnowmanResult::Success(_) => {
							correct = guesses <= 6;
							break;
						}
						_ => {
							correct = false;
							break;
						}
					};
				}
				(correct, guesses)
			};
			(target.clone(), correct, guesses)
		})
		.collect::<Vec<(String, bool, u8)>>();
	eprintln!("Finished in {}s", now.elapsed().as_secs());
	eprintln!(
		"{} victories out of {} words",
		successes.iter().filter(|v| v.1 && v.2 < 5).count(),
		words.len()
	);
	let sorted = {
		let mut successes = successes.clone();
		successes.sort_by(|a, b| a.2.cmp(&b.2));
		successes
	};
	eprintln!(
		"The hardest word was {}, with {} guesses.",
		sorted.last().unwrap().0,
		sorted.last().unwrap().2
	);
	eprintln!(
		"The easiest word was {}, with {} guess(es).",
		sorted.first().unwrap().0,
		sorted.first().unwrap().2
	);
}
