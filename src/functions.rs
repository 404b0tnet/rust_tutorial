// functions - used to store blocks of code for re-use

pub fn run(){
    greeting("Hello", "James");

    // bind function values to variables
    let get_sum = add(3, 3);
    println!("Sum: {}", get_sum);

    // Closure
    let num3: i32 = 10;
    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to mee you!", greet, name);
}

fn add(num1: i32, num2: i32) -> i32{
    num1 + num2
}