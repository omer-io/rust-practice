use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    email: String, 
}

fn main() {
    let person = Person {
        name: String::from("Malik"),
        age: 22,
        email:String::from("hello@gmail.com"),
    };

    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {serialized}");

    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {deserialized:?}");
}