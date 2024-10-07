fn main() {
    let s1 = 5;

    let len = calculate_length(s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: usize) -> usize {
    let length = s; // len() returns the length of a String

    length
}