// variables hold primitive data or references to data
// variables are immutable by defaults
// rust is a block-scoped lanaguge

pub fn run(){
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // define constant -> type is needed "i32"
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}