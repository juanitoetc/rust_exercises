// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// I AM NOT DONE

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    /*  It is possible to declare a new variable with the same name
        as a previous variable. The first variable is "shadowed" by the second, 
        which means that the second variable’s value is 
        what the program sees when the variable is used. 

        We can shadow a variable by using the same variable’s name
        and repeating the use of the "let" keyword.
    */
    let number = 3;

    /*  Shadowing is different from marking a variable as mut, 
        because we’ll get a compile-time error if we accidentally 
        try to reassign to this variable without using the let keyword. 
        By using "let", we can perform a few transformations on a value but 
        have the variable be immutable after those transformations have been completed.
    */
    println!("Number plus two is : {}", number + 2);
}
