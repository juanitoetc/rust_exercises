// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

// I AM NOT DONE

fn main() {
    let mut x = 3;
    /* Variables are immutable only by default; 
    they can be mutable by adding "mut" in front of the variable name.
    Adding "mut" also conveys intent to future readers of the code 
    by indicating that other parts of the code will be changing 
    this variable’s value. When to use? with large data structures, 
    when mutating an instance in place may be faster than copying and 
    returning newly allocated instances.*/
    
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
