use std::{fs, io::Read};

enum Author {
    Known(String),
    Unknown,
}

struct Book {
    name: String,
    description: String,
    author: Author,
    file: fs::File,
}

impl From<&str> for Author {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            return Author::Unknown;
        } else {
            return Author::Known(value.trim().into())
        }
    }
}

enum OpenBookError {
    InvalidMetadata,
    Unknown,
}

impl TryFrom<fs::File> for Book {
    type Error = OpenBookError;
    fn try_from(mut value: fs::File) -> Result<Self, Self::Error> {
        let mut input = String::new();
        value.read_to_string(&mut input).map_err(|_| OpenBookError::Unknown)?;
        let name = input.lines().nth(0).ok_or(OpenBookError::InvalidMetadata)?;
        let description = input.lines().nth(1).ok_or(OpenBookError::InvalidMetadata)?;
        let author = input.lines().nth(2).ok_or(OpenBookError::InvalidMetadata)?;

        Ok(Book {
            name: name.trim().into(),
            description: description.trim().into(),
            author: author.into(),
            file: value,
        })
    }
}

fn main() {

}
