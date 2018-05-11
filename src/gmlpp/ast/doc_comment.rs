use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::super::tokenizer::{Token, Tokens};
use error::Error;

#[derive(Clone, Debug)]
pub struct DocComment(Vec<String>);

impl Display for DocComment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in self.0.iter() {
            write!(f, "/// {}", line)?;
        }
        Ok(())
    }
}

impl Fragment for DocComment {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        let mut comments = vec![];
        while let [Token::DocComment(ref comment), Token::EOL] = tokens[..2] {
            comments.push(comment.clone());
            tokens.skip(2);
        }
        Ok(DocComment(comments))
    }
}
