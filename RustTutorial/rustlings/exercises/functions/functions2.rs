// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// I AM NOT DONE

fn main() {
    call_me(3);
}

/*
    In function signatures, you must declare the type of each parameter. 
    Requires type annotations in function definitions (the compiler almost never
    needs to figure out what type you mean.
    When defining multiple parameters, separate the parameter declarations with commas.
*/

fn call_me(num: u16) {
    for i in 0..num {
        /*
            The println! macro puts the variable where the pair of
             curly brackets were in the format string.
        */
        println!("Ring! Call number {}", i + 1);
    }
}
