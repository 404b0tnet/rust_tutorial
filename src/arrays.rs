// Arrays - fixed list where eleemnts are the same data types

// use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    
    // re-assign value
    numbers[2] = 20;

    println!("Entire Array: {:?}",numbers);

    // get single value
    println!("Single Value: {}",numbers[0]);

    // get length of array
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}