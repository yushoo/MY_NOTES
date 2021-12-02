// Arrays = Fixed list where elements are the same data types

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let mut mutNumbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Can't add to array but can re-assign value
    mutNumbers[2] = 20;

    println!("{:?}",numbers);

    // get single val
    println!("single value: {}",numbers[0]);

    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}",slice);

}