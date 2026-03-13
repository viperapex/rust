// Compounnd Data Types
// arrays, tuples, slice


fn main(){
    // Arrays
    let numbers: [i32; 5 ] = [1,2,3,4,5];
    println!("Number Array:{:?}", numbers);
    //let mix = [1,2, "apple", true];
    //println!("Mix Array: {:?}", mix);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];

    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {:?}", fruits[0]);
    println!("Fruits Array: {:?}", fruits[1]);
    println!("Fruits Array: {:?}", fruits[2]);
    /////////////////////////////////////////////

    //Tuples
    let human = ("Alice", 30, false);
    println!("Human Tuple {:?}", human);
    
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);
 
    //Slices: "dynamically sized view into a contagious sequence of elements"
    //Slices: [1,2,3,4,5] 
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);  

    let animal_slices:&[&str] = &["Lion","Elephant","Crocodile"];
    println!("Number Slice: {:?}", animal_slices);

    //Strings Vs String Slices (&str)
    //String [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);
 
    // B- &str (String slice) - a reference
    let string: String = String::from("Hello, Megan!");
    let slice: &str = &string;
    println!("Slice Value: {}", slice);

    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);

}   
