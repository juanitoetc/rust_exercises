// if1.rs

// I AM NOT DONE

pub fn bigger(a: i32, b: i32) -> i32 {
    /*
        An if expression allows you to branch your code depending on conditions. 
        -  "if" expressions start with the keyword "if", followed by a condition. 
        -  Place block of code to execute if the condition is true
            immediately after the condition inside curly brackets
        -  Can also include an else expression
    */
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    // Execute `rustlings hint if1` for hints
    if (a > b){
        a
    }
    else{
        b
    }

}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
