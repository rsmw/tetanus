use std::str::Chars;
use std::iter::Peekable;

pub type Splice = Vec<Item>;

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
    TooManyRightBraces,
    TooFewRightBraces,
}

pub fn parse(input: &str) -> Result<Splice, ParseErr> {
    let mut stream = input.chars().peekable();
    let splice = parse_splice(&mut stream)?;
    if let Some(&'}') = stream.peek() {
        return Err(ParseErr::TooManyRightBraces);
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

                let mut body = vec![];
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

    Ok(items)
}

fn parse_braces(stream: &mut Stream) -> Result<Splice, ParseErr> {
    assert_eq!(stream.next(), Some('{'));
    let splice = parse_splice(stream)?;
    match stream.next() {
        Some('}') => Ok(splice),
        _ => Err(ParseErr::TooFewRightBraces),
    }
}

#[test]
fn no_tags() {
    let input = "This is ordinary text";
    assert_eq!(parse(input).unwrap(), vec![Item::Str(input.to_owned())]);
}

#[test]
fn nested_braces() {
    let mut input = "Enclosed".to_owned();
    let mut output = vec![Item::Str(input.clone())];

    for _ in 0 .. 100 {
        input = format!("{{{}}}", input);
        output = vec![Item::Braces(output)];
        assert_eq!(parse(&input).unwrap(), output);
    }
}

#[test]
fn empty_tag() {
    let text = "\\test{}";
    assert_eq!(parse(text).unwrap(), vec!{
        Item::Tag {
            name: "test".into(),
            body: vec![],
        },
    });
}

#[test]
fn nested_braces_and_tags() {
    let input = "\\b{\\i{\\o{\\u{Have you read your \\book today?}}}}";
    let output = vec!{
        Item::Tag {
            name: "b".to_owned(),
            body: vec!{
                Item::Tag {
                    name: "i".to_owned(),
                    body: vec!{
                        Item::Tag {
                            name: "o".to_owned(),
                            body: vec!{
                                Item::Tag {
                                    name: "u".to_owned(),
                                    body: vec!{
                                        Item::Str("Have you read your ".into()),
                                        Item::Tag {
                                            name: "book".to_owned(),
                                            body: vec![],
                                        },
                                        Item::Str(" today?".to_owned()),
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    };

    assert_eq!(parse(input).unwrap(), output);
}
