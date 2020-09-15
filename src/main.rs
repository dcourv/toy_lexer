use regex::Regex;
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// @NOTE: in future rust versions, will be able to use question mark here too
	let file_name = args().nth(1).expect("No input file specified");
	let file_str = read_to_string(Path::new(&file_name))?;

	let mut lexemes: Vec<&str> = Vec::new();
	let mut token_classes: Vec<&str> = Vec::new();

	let whitespace_regex = Regex::new(r"\s+").unwrap();
	let operator_regex = Regex::new(r"\+|-|/|\*").unwrap();
	let identifier_regex = Regex::new(r"[a-zA-Z][a-zA-Z\d]*").unwrap();
	let literal_regex = Regex::new(r"\d+((E(\+|-)?\d+)|(\.\d+))?").unwrap();

	// @OPTIMIZE ?
	let mut i = 0;
	while i < file_str.len() {
		// First eliminate any whitespace
		// @TODO eliminate array bounds check?
		if let Some(mtch) = whitespace_regex.find(&file_str[i..]) {
			if mtch.start() == 0 {
				i += mtch.end();
				continue;
			}
		}
		if let Some(mtch) = identifier_regex.find(&file_str[i..]) {
			// Rust is stupid and won't let me write this with an `&&` above
			// Make sure match is at start
			if mtch.start() == 0 {
				token_classes.push("identifier");
				lexemes.push(&file_str[i..i + mtch.end()]);
				i += mtch.end();
				continue;
			}
		}
		if let Some(mtch) = operator_regex.find(&file_str[i..]) {
			if mtch.start() == 0 {
				token_classes.push("operator");
				lexemes.push(&file_str[i..i + mtch.end()]);
				i += mtch.end();
				continue;
			}
		}
		if let Some(mtch) = literal_regex.find(&file_str[i..]) {
			if mtch.start() == 0 {
				token_classes.push("literal");
				lexemes.push(&file_str[i..i + mtch.end()]);
				i += mtch.end();
				continue;
			}
		} else {
			panic!("Lex error -- unrecognized token at character {}", i);
		}
	}

	println!("Lexemes: {:?}\nToken classes: {:?}", lexemes, token_classes);
	Ok(())
}
