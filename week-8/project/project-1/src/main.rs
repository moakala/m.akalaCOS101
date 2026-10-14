use std::io;
fn main() {
    let mut office_level = String::new();
    let mut academic_level = String::new();
    let mut legal_level = String::new();
    let mut teacher_level = String::new();

    println!("Welcome to the APS level checker!");
    println!("First we need to know your job history");
    println!("Have you worked in an office? (yes/no): ");
    io::stdin().read_line(&mut office_level).expect("Pls );
    office_level = office_level.trim().to_string();

    
}
