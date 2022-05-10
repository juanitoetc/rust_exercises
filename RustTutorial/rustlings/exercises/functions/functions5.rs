// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// I AM NOT DONE

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    /*
        In Rust, the return value of the function is synonymous 
        with the value of the final expression in the block of
        the body of a function.

        Define the type of the return value. In the body of the function
        the return values has no semicolon because it’s an expression whose 
        value we want to return.

        Another solution is to write 
            return (num * num); 
        Here we have a semicolon.
    */
    num * num
}
