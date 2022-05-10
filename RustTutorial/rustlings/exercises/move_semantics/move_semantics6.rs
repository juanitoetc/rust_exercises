// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string(); // returns a String value
    // let mut data = "Rust is great!".to_string();
    
    // Create a reference that refers to the value of "data" in get_char but does not own it.
    get_char(&data);

    // string_uppercase does not return anything and takes ownership 
    string_uppercase(data);
    // string_uppercase(&mut data);
}

/*
    These ampersands represent references, 
    and they allow you to refer to some value without taking ownership of it.

    When functions have references as parameters instead of the actual values, 
    we won’t need to return the values in order to give back ownership, 
    because we never had ownership.

    We call the action of creating a reference borrowing.
*/
// Should not take ownership
fn get_char(data: &String) -> char {
    // the value it points to will not be dropped when the reference stops being used.
    data.chars().last().unwrap()
}

// Should take ownership
// It is possible to modify a borrowed value with a mutable reference.
// In this case it is not a reference, but it would also work 

/*
    main:
        let mut data = "Rust is great!".to_string();
        string_uppercase(&mut data);
    function:
        fn string_uppercase(data: &mut String)
            *data = (*data).to_uppercase();
            println!("{}", data);
*/

// fn string_uppercase(data: &mut String) {
fn string_uppercase(mut data: String) {
    //*data = (*data).to_uppercase();
    data = data.to_uppercase();
    println!("{}", data);
}
