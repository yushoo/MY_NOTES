pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    let name = "fungus";
    let status = "100%";

    // println!("Args: {:?}",args);
    // println!("Command: {}",command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } elsse if command == "status" {
        println!("Status is {}", status);
    }