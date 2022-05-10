// variables6.rs
// Make me compile! Execute the command `rustlings hint variables6` if you want a hint :)

// I AM NOT DONE

const NUMBER : u16 = 3;
fn main() {
    /*
        Like immutable variables, constants are values that are bound
        to a name and are not allowed to change, 
        but there are a few differences between constants and variables:

        - It is not allowed to use mut with constants (they’re always immutable).
        - Declare constants using the const keyword
        - The type of the value must be annotated
        - Constants can be declared in any scope (including the global scope)
        - Value set only to a constant expression, not a result computed at runtime.
          ie: const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    */

    /*
        Rust’s naming convention for constants is: 
            - All uppercase with underscores between words
    */
    println!("Number {}", NUMBER);
}
