// Println! macro
// https://doc.rust-lang.org/std/fmt/

fn main() {
    // Use {} to print the argument
    println!("{}", "Hello, world!");
    println!("{}, {}", 1, 2);

    // Positional arguments
    println!("{0}, {1}", "Alice", "Bob");

    // Named parameters
    println!(
        "{first}, {second}, {first}",
        first = "This is first",
        second = "This is second"
    );

    // Format character
    println!("Base 10: {}", 123450);
    println!("LowerExp: {:e}", 123450);
    println!("UpperExp: {:E}", 123450);
    println!("Base 8 Octal: {:o}", 123450);
    println!("Base 2 Binary: {:b}", 123450);
    println!("Base 16 Hexadecimal: {:x}", 123450);
    println!("Base 16 Hexadecimal: {:X}", 123450);

    // Print debug
    println!("{:?}", [1, 2, 3]);

    // Pretty print debug
    println!("{:#?}", [1, 2, 3]);
}
