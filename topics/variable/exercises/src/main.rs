#![allow(unused)]
// Exercise: Fix the code to make it compile and pass the assertions

//constant
const NUM: u32 = 1;

fn main() {
    // Exercise 1: Make this variable mutable
    let count = 1;
    count += 1;

    println!("count: {count}");

    // immutable variable as default
    // use mut keyword to make it mutable
    let mut x = 1;
    x += 1;

    // Type inference
    let y: i32 = -1;
    let z -1;

    // Shadowing
    let x: i32 = 1;
    let x: i32 = 2;
    let x: bool = true;

    // Type placeholder
    let x: _ = true;

    //println
    let x = 1;
    printl("x is {}", x);
    //Inline 
    println("x is {x}");
    // Positional arguments
    let z = x + x;
    println("{0} + {0} = {1}", x,x+x);
    // debug 
    println("DEBUG: x {:?}",x);
    println("DEBUG: x {:#?}",x);
}
