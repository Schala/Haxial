use nom::{
	AsChar,
	branch::alt,
	bytes::complete::{
		tag_no_case,
		take_till,
		take_while
	},
	character::complete::{
		char,
		multispace0,
		not_line_ending
	},
	combinator::{
		opt,
		value
	},
	error::ParseError,
	InputTakeAtPosition,
	IResult,
	multi::{
		count,
		many0,
		separated_list1
	},
	Parser,
	sequence::{
		delimited,
		pair,
		preceded,
		terminated,
		tuple
	},
};

use std::{
	collections::hash_map::DefaultHasher;
	hash::{
		Hash,
		Hasher
	},
	io
};

use thiserror::Error;

#[derive(Debug, Error)]
enum LexError<'a> {
	#[error("I/O error")]
	IO {
		#[from]
		source: io::Error,
	},
}

struct Record {
	name: String,
	fields: Vec<Variable>,
}

impl Hash for Record {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		for f in self.fields {
			f.hash(state);
		}
	}
}

struct Variable {
	name: String,
	value: Option<String>,
	kind: u64, // hash of the type
}

impl Hash for Variable {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		self.kind.hash(state);
	}
}

/// Parses an identifier
fn ident<'a>(input: T) -> IResult<T, &'a str, LexError<'a>>
where
	T: InputTakeAtPosition,
	<T as InputTakeAtPosition>::Item: AsChar,
{
	input.split_at_position_complete(|item| !item.is_alphanum() || (item.as_char() != '_'))
}

/// Parses an uninitialised variable declaration
fn var_decl_no_init<'a>(input: &'a str) -> IResult<&'a str, VarDecl, LexError<'a>> {
	let (input, decl) = preceded(tag_no_case("var"),
		tuple((
			terminated(ws(ident), ws(char(','))), // identifier
			terminated(ws(ident), ws(char(';'))))))(input)?; // type
	
	VarDecl {
		name: data.0.to_string(),
		kind: data.0.to_string(),
		value: None
	}
}

/// Parses a variable declaration initialised with a value
fn var_decl_init<'a>(input: &'a str) -> IResult<&'a str, VarDecl, LexError<'a>> {
	let (input, decl) = preceded(tag_no_case("var"),
		tuple((
			terminated(ws(ident), ws(char(','))), // identifier
			terminated(ws(ident), ws(char(','))))))(input)?; // type
	
	VarDecl {
		name: data.0.to_string(),
		kind: data.0.to_string(),
		value: None
	}
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// From https://github.com/Geal/nom/blob/master/doc/nom_recipes.md with minor edits
fn ws<'a, F, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	E: ParseError<&'a str>,
	F: Parser<&'a str, O, E>,
{
	delimited(multispace0, inner, multispace0)
}
