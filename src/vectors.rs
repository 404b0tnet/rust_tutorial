// Vectors - Resizeable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("Entire Vector: {:?}",numbers);
    
    // re-assign value
    numbers[2] = 20;

    // add to vector
    numbers.push(9);
    println!("Push 5 into Vector: {:?}",numbers);

    numbers.push(6);
    println!("Push 6 into Vector: {:?}",numbers);

    // pop off last value
    numbers.pop();

    println!("Pop value from Vector: {:?}",numbers);

    // get single value
    println!("Single Value: {}",numbers[0]);

    // get length of array
    println!("Vector Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers: {:?}", numbers);
}