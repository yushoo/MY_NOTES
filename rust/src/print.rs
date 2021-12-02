pub fn run() {
    // Print to console
    println!("Hello from the printrs file");
    // Can't directly print integer
    println!("Number: {}", 1);
    // Multiple placeholder
    println!("{} is from {}", "Brad","Mass");
    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}","Brad", "Mass","code");
    // Named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");
    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    // Placeholder for debuf trait
    println!("{:?}",(12, true,"hello"));
    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}