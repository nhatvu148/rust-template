use std::str;

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

fn main() {
    let address = String::from("Street 1");

    let a = add_postal_code(address);

    println!("{}", a);

    fn add_postal_code(mut address: String) -> String {
        address.push_str(", 1234 Kingston");
        address
    }
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);

    let x: &[u8] = &[b'a', b'b', b'c'];
    let stack_str: &str = str::from_utf8(x).unwrap();
    println!("stack_str: {}, x: {:?}", stack_str, x);
}
