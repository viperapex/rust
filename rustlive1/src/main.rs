//Primitive data types
//int, float, bool

//Integer
//Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
// i8, i16, i32, i64, i128: Signes integers.
// u8, u26, u32, u64, u128: Unsigned integers.
fn main(){
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
//diff bet i32 (32 bits) and i64(64 bits)
//range  :
// i32 - 2147483647
// i64 - 9223372036854775807
    let e: i32 = 2147483647;
    let i : i64 = 223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);
    // ==========================================
    // Floats [Ffloating Point Types]
    // f32, f64
        let pi: f64 = 3.14;
        println!("Value of Pi: {}", pi);
    // Boolean Values : true, false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // Character Type - char
    let letter: char = 'a';
    println!("First letter of the aplhabet: {}", letter);
    

}