//Ownership Rules
//---------------------------
//1 Each value in Rust has a variable that's its owner.
//2 There can be only one owner at a time
//3 When the owner goes out of scope, the value will be dropped

fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' id '{}.", s1, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
