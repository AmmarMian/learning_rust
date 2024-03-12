// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.



fn main() {
    const ARRAY_SIZE: usize = 1000;
    let mut a: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    for i in 0..a.len() {
        a[i] = i as i32;
    };

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
