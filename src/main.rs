use regex::Regex;
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

// @TODO fix nasty control flow

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// @NOTE: in future rust versions, will be able to use question mark here too
	let file_name = args().nth(1).expect("No input file specified");
	let file_cnts = read_to_string(Path::new(&file_name))?;

	let mut cnts_to_read = &file_cnts[..];

	let mut lexemes: Vec<&str> = Vec::new();
	let mut token_classes: Vec<&str> = Vec::new();

	// NB: hopefully my home-coded DFA implementation will only start at zero, but
	// for the time being I am using anchors (won't have to implement them in my
	// version). \A is the beginning of a string
	// NB: because of | operators, I'm putting everything after \A in parens, i.e.
	// \A(unanchored_regex)

	let newline_regex = Regex::new(r"\A\n+").unwrap();
	let whitespace_regex = Regex::new(r"\A(\s+)").unwrap();
	let operator_regex = Regex::new(r"\A(\+|-|/|\*|\^|%)").unwrap();
	let identifier_regex = Regex::new(r"\A([a-zA-Z][a-zA-Z\d]*)").unwrap();
	let literal_regex = Regex::new(r"\A(\d+((E(\+|-)?\d+)|(\.\d+))?)").unwrap();

	// NOTE: in the future, keep track of line and column info?
	let mut chr_idx: usize = 0;
	let mut line_idx: usize = 0;

	// Some(match) = expression
	while cnts_to_read.len() > 0 {
		// @TODO think about multiline strings in the future
		if let Some(mtch) = newline_regex.find(cnts_to_read) {
			cnts_to_read = &cnts_to_read[mtch.end()..];
			chr_idx = 0;
			line_idx += 1;
		}
		// @TODO think about writing numbers with spaces
		// First eliminate any whitespace
		else if let Some(mtch) = whitespace_regex.find(cnts_to_read) {
			cnts_to_read = &cnts_to_read[mtch.end()..];
			chr_idx += mtch.end();
		} else if let Some(mtch) = identifier_regex.find(cnts_to_read) {
			// Rust is stupid and won't let me write this with an `&&` above
			// Make sure match is at start
			token_classes.push("identifier");
			lexemes.push(&cnts_to_read[..mtch.end()]);
			cnts_to_read = &cnts_to_read[mtch.end()..];
			chr_idx += mtch.end();
		} else if let Some(mtch) = operator_regex.find(cnts_to_read) {
			token_classes.push("operator");
			lexemes.push(&cnts_to_read[..mtch.end()]);
			cnts_to_read = &cnts_to_read[mtch.end()..];
			chr_idx += mtch.end();
		} else if let Some(mtch) = literal_regex.find(cnts_to_read) {
			token_classes.push("literal");
			lexemes.push(&cnts_to_read[..mtch.end()]);
			cnts_to_read = &cnts_to_read[mtch.end()..];
			chr_idx += mtch.end();
		} else {
			// @DEBUG
			for i in 0..lexemes.len() {
				print!("[{} {}] ", token_classes[i], lexemes[i]);
			}
			println!("");
			panic!(
				"Lex error -- unrecognized token at line {}:{}",
				line_idx, chr_idx
			);
		}
	}

	for i in 0..lexemes.len() {
		print!("[{} {}] ", token_classes[i], lexemes[i]);
	}
	println!("");
	Ok(())
}
