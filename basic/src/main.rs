// importing libraries (called crates)
use std::io; // :: is path seperator

fn main() {
    const CONSTANT_VALUE: u32 = 42; // constant, type is required
    println!("{CONSTANT_VALUE} //constant whose value cant be changed");

    // vars type is fixed
    let x = "Hello, World!"; // immutable
    println!("{x} //immutable var");

    let mut y = "A"; // mutable
    println!("{y} //mutable var");

    {
        let y = "B"; // interior scope has y immutable
        println!("{y} //var defined in interior scope"); // print B
    }

    y = "C"; // back in exterior scope y is still mutable
    println!("{y} //does not change exterior scope"); // print C

    // x = -7 => error, value of immutable var cant be changes
    // i8, i16, i32, i64 and i128 are possible signed numeric types
    // u8, u16, u32, u64 and u128 are possible unsigned numeric types
    // i32->int
    // defualt integer type is i32
    // i'n' has twice as much of range as u'n'
    let x: i32 = -7; // re-assign immutable value with specified data type (signed 32 bit integer or default integer)
    println!("{x} //32 bit signed integer");

    // f32 and f64 are the possible floating-point values (signed)
    // f32->float, f64->double
    // f32 is deafult
    let x: f32 = -7.6;
    println!("{x} //32 bit floating point value");

    // true and false are bool values
    // '!' is the not operator
    let b: bool = !true;
    println!("{b} //boolean");

    // char is defined with ' (single qoutes)
    let c: char = ';';
    println!("{c} //character");

    // tuple is defined with ()
    // the two dont have the same type
    // tuple is statically allocated
    let mut _tup1 = (1, true, 's'); // type is (i32, bool, char)
    let _tup2 = (false, 2, 's'); // type is (bool, i32, char)

    println!("{} //tuple elem", _tup1.0); // for some reason tuple's elements cant be put into '{}'

    let z = _tup1.2; // altho it can be assigned to a var
    println!("{z} //tuple elem by assigning to var"); // and then printed

    _tup1.2 = 'c'; // can't change type of var at location
    let z = _tup1.2;
    println!("{z} //changed tuple elem by assigning to var");

    // arr is defined with []
    // all elem types need to be same
    // also statically allocated
    let mut arr = [1, 3, 4, 5, 6, 7];
    println!("{} //array elem", arr[2]);

    arr[0] = 5;
    println!("{} //changed array elem", arr[0]);

    let x: u8 = 4;
    let y: i32 = x as i32; // some types can be interconverted
    let z: i16 = x.into(); // again only works for certain types
    println!("{x}, {y}, {z} //type casting without error");

    let mut input = String::new(); // this is string constructor

    // read_line takes a mutable pointer to which 'String' we want to change, expect is the value printed if theres an error
    // &mut input, creates pointer to change value of input (&input by default is immutable)
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input = input.trim().parse().unwrap(); // trim removes trailing \n, parse parses to string, unwrap unwraps the value into the variable
    println!("{input} //inputted value");

    let u = 27_000_u64; // '_' in numbers is ignored, type at end works as well

    //u = -12_000_i64; this will give an error and types cant be changed after assignment

    let v = 1323 as i64;
    let z = (u as i64) / v; // automatic rounding off
    println!("{z} //basic arithemetic");

    // Overflows can happen, thats why conversions should be done from small to large and not vice-versa
    let exp = (i32::MAX as i64) + 1;
    let z = exp as i32;
    println!("{z} //overflow"); // this gives i32::MIN as answer

    let mut int_input = String::new();
    io::stdin()
        .read_line(&mut int_input)
        .expect("failed to read line");
    let mut int: i64 = int_input.trim().parse().unwrap();
    int = int + 2;
    println!("{int} //added 2 to inputted value");
}
