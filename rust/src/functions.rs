// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("Hello","Jane");

    // Bind function values to variables
    let get_sum = add(5,5);
    
}

fn greeting(greet: %str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3,3));
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}