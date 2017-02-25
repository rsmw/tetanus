use std::str::Chars;
use std::iter::Peekable;

#[derive(Clone, Debug, PartialEq)]
pub struct Splice(Vec<Item>);

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Str(String),
    Tag {
        name: String,
        body: Splice,
    },
    Braces(Splice),
}

#[derive(Copy, Clone, Debug)]
pub enum ParseErr {
    TooManyClosingBraces,
    TooFewClosingBraces,
}

pub fn parse(input: &str) -> Result<Splice, ParseErr> {
    let mut stream = input.chars().peekable();
    let splice = parse_splice(&mut stream)?;
    if let Some(&'}') = stream.peek() {
        return Err(ParseErr::TooManyClosingBraces);
    } else {
        assert_eq!(stream.next(), None);
    }
    Ok(splice)
}

type Stream<'input> = Peekable<Chars<'input>>;

fn parse_splice(stream: &mut Stream) -> Result<Splice, ParseErr> {
    let mut items = vec![];

    while let Some(&ch) = stream.peek() {
        match ch {
            '\\' => {
                let _ = stream.next();

                let mut name = String::new();
                while let Some(&ch) = stream.peek() {
                    if ch.is_alphabetic() {
                        name.push(ch);
                        let _ = stream.next();
                    } else {
                        break;
                    }
                }

                let mut body = Splice(vec![]);
                if let Some(&'{') = stream.peek() {
                    body = parse_braces(stream)?;
                }

                items.push(Item::Tag {
                    name: name,
                    body: body,
                });
            },

            '{' => {
                items.push(Item::Braces(parse_braces(stream)?));
            },

            '}' => break,

            _ => {
                let mut text = String::new();
                while let Some(&ch) = stream.peek() {
                    match ch {
                        '\\' | '{' | '}' => break,
                        _ => {
                            text.push(ch);
                            let _ = stream.next();
                        },
                    }
                }
                items.push(Item::Str(text));
            },
        }
    }

    Ok(Splice(items))
}

fn parse_braces(stream: &mut Stream) -> Result<Splice, ParseErr> {
    assert_eq!(stream.next(), Some('{'));
    let splice = parse_splice(stream)?;
    match stream.next() {
        Some('}') => Ok(splice),
        _ => Err(ParseErr::TooFewClosingBraces),
    }
}

mod display {
    use std::fmt;
    use super::*;

    impl fmt::Display for Splice {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for item in self.0.iter() {
                write!(f, "{}", item)?;
            }

            Ok(())
        }
    }

    impl fmt::Display for Item {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &Item::Str(ref s) => write!(f, "{}", s),

                &Item::Tag { ref name, ref body } => {
                    write!(f, "\\{}{{", name)?;
                    for item in body.0.iter() {
                        write!(f, "{}", item)?;
                    }
                    write!(f, "}}")
                },

                &Item::Braces(ref contents) => {
                    write!(f, "{{{}}}", contents)
                },
            }
        }
    }
}

#[test]
fn no_tags() {
    let input = "This is ordinary text";
    let output = Splice(vec![Item::Str(input.to_owned())]);
    assert_eq!(parse(input).unwrap(), output);
}

#[test]
fn nested_braces() {
    let mut input = "Enclosed".to_owned();
    let mut output = Splice(vec![Item::Str(input.clone())]);

    for _ in 0 .. 100 {
        input = format!("{{{}}}", input);
        output = Splice(vec![Item::Braces(output)]);
        assert_eq!(parse(&input).unwrap(), output);
    }
}

#[test]
fn empty_tag() {
    let text = "\\test{}";
    assert_eq!(parse(text).unwrap(), Splice(vec!{
        Item::Tag {
            name: "test".into(),
            body: Splice(vec![]),
        },
    }));
}

#[test]
fn nested_braces_and_tags() {
    let input = "\\b{\\i{\\o{\\u{Have you read your \\book today?}}}}";

    let mut output = Splice(vec!{
        Item::Str("Have you read your ".to_owned()),
        Item::Tag {
            name: "book".to_owned(),
            body: Splice(vec![]),
        },
        Item::Str(" today?".to_owned()),
    });

    for &name in &["u", "o", "i", "b"] {
        output = Splice(vec!{
            Item::Tag {
                name: name.to_owned(),
                body: output,
            },
        });
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
