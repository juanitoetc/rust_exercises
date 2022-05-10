// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// I AM NOT DONE

fn main() {
    let x = 11;
    /*
    Rust is a statically typed language, it must know the types of all variables
    at compile time.
    The compiler can usually infer what type we want to use based
    on the value and how we use it.
    In cases when many types are possible: add a type annotation let guess: u32 = ... ;*/
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
