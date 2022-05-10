// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// I AM NOT DONE

fn main() {
    /*
        Functions can have parameters, which are special variables 
        that are part of a function’s signature.
        When a function has parameters, provide it with concrete values.
    */
    call_me(5);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
