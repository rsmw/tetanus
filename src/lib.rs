pub mod parse;

#[derive(Clone, Debug, PartialEq)]
pub struct Splice<'input>(Box<[Item<'input>]>);

#[derive(Clone, Debug, PartialEq)]
pub enum Item<'input> {
    Text(&'input str),

    Tag {
        name: &'input str,
        body: Splice<'input>,
    },

    Braces(Splice<'input>),
}

use std::fmt;

impl<'input> fmt::Display for Splice<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in self.0.iter() {
            write!(f, "{}", &*item)?;
        }

        Ok(())
    }
}

impl<'input> fmt::Display for Item<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Item::Text(ref s) => write!(f, "{}", s),

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

