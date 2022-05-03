// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` for a hint.

// I AM NOT DONE

fn main() {
    let strslcMyString = "World";

    /*
        A ‘string’ is a sequence of Unicode scalar values encoded as a stream 
        of UTF-8 bytes. All strings are guaranteed to be a valid encoding of UTF-8 
        sequences. Rust has two main types of strings: &str and String.
        strslcMyString is &str, called ‘string slices’. Fixed size. cannot be mutated.
        Statically allocated. they do not support indexing (because of the UTF-8 codification)
        Something similar is strslcMyString.chars().nth(1);
    */
    println!("Hello {strslcMyString}!");
}
