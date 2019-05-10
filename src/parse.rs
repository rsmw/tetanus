use std::{iter::Peekable, str::CharIndices};

use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    TooManyClosingBraces,
    TooFewClosingBraces,
}

pub type Result<T, E=Error> = std::result::Result<T, E>;

pub fn parse<'input>(input: &'input str) -> Result<Splice<'input>> {
    let chars = input.char_indices().peekable();

    let mut stream = Stream {
        input,
        chars,
    };

    let splice = stream.parse_splice()?;

    if let Some(&(_, '}')) = stream.chars.peek() {
        return Err(Error::TooManyClosingBraces);
    } else {
        assert_eq!(stream.chars.next(), None);
    }

    Ok(splice)
}

struct Stream<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Stream<'input> {
    fn parse_splice(&mut self) -> Result<Splice<'input>> {
        let mut items = vec![];

        while let Some(&(start, ch)) = self.chars.peek() {
            match ch {
                '\\' => {
                    let _ = self.chars.next().unwrap();

                    let start = start + '\\'.len_utf8();

                    let mut end = start;

                    while let Some(&(_, ch)) = self.chars.peek() {
                        if ch.is_alphabetic() {
                            end += ch.len_utf8();
                            let _ = self.chars.next();
                        } else {
                            break;
                        }
                    }

                    let name = &self.input[start .. end];

                    let body = if let Some(&(_, '{')) = self.chars.peek() {
                        self.parse_braces()?
                    } else {
                        Splice(Box::new([]))
                    };

                    items.push(Item::Tag {
                        name,
                        body,
                    });
                },

                '{' => {
                    let contents = self.parse_braces()?;
                    items.push(Item::Braces(contents));
                },

                '}' => break,

                _ => {
                    let mut end = start;

                    while let Some(&(_, ch)) = self.chars.peek() {
                        match ch {
                            '\\' | '{' | '}' => break,

                            _ => {
                                end += ch.len_utf8();
                                let _ = self.chars.next();
                            },
                        }
                    }

                    items.push(Item::Text({
                        &self.input[start .. end]
                    }));
                },
            }
        }

        Ok(Splice(items.into_boxed_slice()))
    }

    fn parse_braces(&mut self) -> Result<Splice<'input>> {
        match self.chars.next() {
            Some((_, '{')) => (),
            _ => panic!("Internal error: Expected opening brace"),
        }

        let splice = self.parse_splice()?;

        match self.chars.next() {
            Some((_, '}')) => Ok(splice),
            _ => Err(Error::TooFewClosingBraces),
        }
    }
}

#[test]
fn no_tags() {
    let input = "This is ordinary text";
    let output = Splice(vec![Item::Text(input)].into());
    assert_eq!(parse(input).unwrap(), output);
}

#[test]
fn nested_braces() {
    let original = "Enclosed";

    let mut input = original.to_owned();
    let mut output = Splice(vec![Item::Text(&original)].into());

    for _ in 0 .. 100 {
        input = format!("{{{}}}", input);
        output = Splice(vec![Item::Braces(output)].into());
        assert_eq!(parse(&input).unwrap(), output);
    }
}

#[test]
fn empty_tag() {
    let text = "\\test{}";
    assert_eq!(parse(text).unwrap(), Splice(vec!{
        Item::Tag {
            name: "test",
            body: Splice(vec![].into()),
        },
    }.into()));
}

#[test]
fn nested_braces_and_tags() {
    let input = "\\b{\\i{\\o{\\u{Have you read your \\book today?}}}}";

    let mut output = Splice(vec!{
        Item::Text("Have you read your "),
        Item::Tag {
            name: "book",
            body: Splice(vec![].into()),
        },
        Item::Text(" today?"),
    }.into());

    for &name in &["u", "o", "i", "b"] {
        output = Splice(vec!{
            Item::Tag {
                name: name,
                body: output,
            },
        }.into());
    }

    assert_eq!(parse(input).unwrap(), output);
}

#[test]
fn fmt_round_trip() {
    let inputs = vec!{
        "\\q{Surrounded by \\q{quotation marks!}}",
        "This is ordinary text containing no markup",
        "\\defun{\\greet{} \\print{Hello, world}}",

        // Don't test these -- \foo and \foo{} have the same representation
        //"\\hello\\world",
    };

    for input in inputs {
        let output = parse(input).unwrap();
        assert_eq!(&format!("{}", output), input);
    }
}

