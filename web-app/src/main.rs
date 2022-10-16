use std::io::{Error, ErrorKind};
use std::str;
use std::str::FromStr;

// ch02/src/main.rs
#[warn(dead_code)]
#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

fn main() {
    let address = String::from("Street 1");

    let a = add_postal_code(address);

    println!("{}", a);

    fn add_postal_code(mut address: String) -> String {
        address.push_str(", 1234 Kingston");
        address
    }
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);

    let x: &[u8] = &[b'a', b'b', b'c'];
    let stack_str: &str = str::from_utf8(x).unwrap();
    println!("stack_str: {}, x: {:?}", stack_str, x);
}
