// Vectors - resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];

     // Can't add to array but can re-assign value
     numbers[2] = 20;

     numbers.push(5);
     numbers.push(6);

     numbers.pop();

     println!("{:?}",numbers);
 
     // get single val
     println!("single value: {}",numbers[0]);
 
     println!("Vector Length: {}", numbers.len());
 
     // Vectors are stack allocated
     println!("Vectors occupies {} bytes", std::mem::size_of_val(&numbers));
 
     // Get Slice
     let slice: &[i32] = &numbers[0..2];
     println!("Slice: {:?}",slice);

     // Loop through vector values
     for x in numbers.iter() {
         println!("Number: {}", x);
     }

     // Loop & mutate values
     for x in numbers.iter_mut() {
         *x *= 2;
     }

     println!("Numbers Vec: {:?}", numbers);

}