#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]


fn variables() {
    // * Mutability

    let x = 5;
    // x = 6;              // illegal

    let mut x = 5;
    x = 6;

    // * Constants

    const LIFE: u16 = 21 * 2;   // eval'd at compile time, can be global

    // * Shadowing

    let mut x = "hey";
    x = "hi";
    // x = 5;              // illegal, cannot mutate variable type
    let mut x = 5;      // ok (shadowed)
    {
        let mut x = 10;     // vars are block scoped, this will not affect outside x
    }
}


fn types() {
    // * Scakars

    // Integers
    let i: i8;      let i: u8;
    let i: i16;     let i: u16;
    let i: i32;     let i: u32;
    let i: i64;     let i: u64;
    let i: i128;    let i: u128;
    let i: isize;   let i: usize;   // 32-bit on 32-bit arch, 64-bit on 64-bit arch, etc.

    let i = 42u8;   // literal with type
    let i = 0xff + 0o77 + 0b1111_0000;  // base N
    let i = b'A';   // byte literal

    // let i: u8 = 200 * 2;     // overflow is an error

    // Floats
    let f: f32;     let b: bool;

    // Boolean
    let f: f64;

    // Char
    let c: char;
    c = '💻';       // single-quote, supports unicode

    // * Compound types

    // Tuples
    let t: (i32, f64, char) = (42, 3.14, '💻');   // fixed length, supports different types

    let (n, f, c) = t;      // destructuring access
    let n = t.0;            // index access
    let unit = ();          // empty/nothing returned represented with 0-len tuple

    // Arrays
    let a: [i32; 5] = [1, 1, 2, 3, 5];    // fixed length, one type only
    let a = ['A'; 5];           // short-hand, like Python's 'A' * 5

    let x = a[0];       // always-on bounds checking at runtime
}


fn functions(arg: i32) -> i32 {    // type annotations required!
    arg + 1;    // statement, no return val
    arg + 1     // expression, last one is automatically returned
}


fn control_flow(n: i32) {
    // * If else

    if n < 2 {
        // do stuff
    }
    else if n == 3 {
        // do stuff
    }
    else {
        // do stuff
    }

    let x = if n > 0 { n } else { -1 };     // ternary equivalent

    // let x = if n > 0 { 1 } else { "boo!" };  // illegal, types must match
    // if 0 { /* do stuff */ };                 // condition must be boolean

    // * Loops

    loop { break; }             // effectively while (true)
    while n != 0 { break; }
    for elem in 1..=5 {};

    let x = loop {
        break 42;           // return values via break
    };

    'my_tag: loop {
        loop {
            break 'my_tag;  // break for tagged outer loops (!!)
        }
    }
}


fn fib(n: i32) -> i128 {
    // Naive Nth fibonacci generator for fun

    let mut n0: i128 = 1;
    let mut n1: i128 = 1;
    
    for i in 3..=n {
        let next = n0 + n1;
        n0 = n1;
        n1 = next;
    }

    n1
}


fn main() {
    variables();
    types();
    let x = functions(1);

    // Cmment
    /* 
        Block comment
     */

    control_flow(0);

    let n = 100;
    println!("{n}th fibonacci number: {}", fib(n));
}
