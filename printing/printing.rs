fn main() {
    // replace the {} with the value of the argument
    println!("{} days", 31);

    // positional arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments can be used
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // special formatting can be specified after a `:`
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // right-align text with a specified width
    println!("{number:0>5}", number=1); // "     1"
    println!("{number:>5}", number=1); // "     1"
    println!("{number:0>width$}", number=1, width=6); // "     1"

    // left align text with a specified width
    println!("{number:<width$}", number=1, width=6); // "     1"

     // Rust even checks to make sure the correct number of arguments are used.
     println!("My name is {0}, {1} {0}", "Bond", "James");

     // For Rust 1.58 and above, you can directly capture the argument
    let number: f64 = 3.141592;
    let width: usize = 3;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);


    println!("Hello {} is {2:.*}",   "x", 4, 0.01);
    println!("Hello {} is {number:.prec$}", "x", prec = 6, number = 0.01);
    println!("Hello {1} is {2:.*}",  5, "x", 0.01);
    println!("Hello {} is {:.*}",    "x", 5, 0.01);
}