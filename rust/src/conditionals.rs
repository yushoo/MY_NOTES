// Conditionals - Used to check the condition of something and act on the result

pub fn run(){
    let age = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;


    // If/Else
    if age >= 21 && cehck_id{
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id{
        println!("Bartender: Leave");
    } else {
        println!("check id");
    }

    // Shorthand if
    let is_of_age = if age>= 21 { true } else { false };
}