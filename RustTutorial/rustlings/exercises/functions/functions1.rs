// functions1.rs
// Make me compile! Execute `rustlings hint functions1` for hints :)

// I AM NOT DONE

fn main() {
    /*
        Main function is the entry point of many programs
        "fn" keyword, allows to declare new functions.

        Rust uses snake case as the conventional style for function and variable names:
        - All letters are lowercase and underscores separate words.

        How to declare a function:
        - "fn"
        - function name
        - set of parentheses
        - curly brackets tell the compiler where the function body begins and ends.
    */
    call_me();
}

/*  Definition of call_me.
    Rust doesn’t care where you define your functions (before or after the main), 
    only that they’re defined somewhere.
*/

fn call_me(){
    println!("Hello main!");
}