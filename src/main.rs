//mod print;
fn main() {
    println!("Hello, world!");
    //print::run();

    //Positional Arguements
    /*Indexing--
        Kavin - 0
        California - 1
        Kevin - 2
    */
    println!("I am {0}, I live in {1} and {2}'s best friend is {0}", "Kavin", "California", "Kevin");

    //Named Arguements
    println!("{name} is a the most loved programming language, {name} is available on replit", name="Rust");

    //Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Debug Traits
    println!("{:?}", (12, true, "hello"));

    //Basic Maths
    println!("10 + 10 = {}", 10 + 10);
}
