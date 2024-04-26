fn main() {
    const CONSTANT_VALUE: u32 = 42; // constant, type is required
    println!("{CONSTANT_VALUE}");

    // vars type is fixed
    let x = "Hello, World!"; // immutable
    println!("{x}");

    let mut y = "A"; // mutable
    println!("{y}");

    {
        let y = "B"; // interior scope has y immutable
        println!("{y}"); // print B
    }

    y = "C"; // back in exterior scope y is still mutable
    println!("{y}"); // print C

    // x = "hello" => error, value of immutable var cant be changes
    let x = "Hello, World?"; // re-assign immutable value
    println!("{x}");
}
