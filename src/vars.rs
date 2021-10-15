pub fn run() {
    let name = "Kavin";
    let mut age = 11;
    println!("My name is {} and I am {} years old", name, age);    
    age = 12;
    println!("My name is {} and I am {} years old", name, age);    

    const ID: i32 = 1126072;
    println!("ID: {}", ID);

    let ( my_name, my_age ) = ("Kavin", 11);
    println!("{} is {}", my_name, my_age );
}