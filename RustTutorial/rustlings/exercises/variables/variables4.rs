// variables4.rs
// Make me compile! Execute the command `rustlings hint variables4` if you want a hint :)

// I AM NOT DONE

fn main() {
    let x: i32 = 100;
    /* If we don’t add the type annotation here, Rust will display 
    an error, which means the compiler needs an initialization.
    32bit signed integer. 
        Length	Signed	Unsigned
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize

    "arch" means architecture depedent. 
    isize and usize types depend on the architecture of the computer your program
    is running on: 
        - 64 bits if you’re on a 64-bit architecture
        - 32 bits if you’re on a 32-bit architecture.
    */
    println!("Number {}", x);
}
