use std::io;

fn main() {
    let mut office_level = String::new();
    let mut academic_level = String::new();
    let mut legal_level = String::new();
    let mut teacher_level = String::new();

    let aps_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    println!("Welcome to the APS level checker!");
    println!("First we need to know your job history.");


    println!("Have you worked in an office before? (yes/no): ");
    io::stdin().read_line(&mut office_level).expect("Pls pick yes/no");
    let office_level = office_level.trim().to_lowercase();

    if office_level == "yes" {
        println!("Pls pick one of the following office jobs or equivalents:");
        println!("1. Intern (Press 1)");
        println!("2. Administrator (Press 2)");
        println!("3. Senior Administrator (Press 3)");
        println!("4. Office Manager (Press 4)");
        println!("5. Director (Press 5)");
        println!("6. CEO (Press 6)");

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).expect("Failed to read number");
        let choice: usize = choice_str.trim().parse().expect("Invalid number try again");

        if choice >= 1 && choice <= 6 {
            println!("Your Public Servant Level is: {} in the office field", aps_levels[choice - 1]);
        } else {
            println!("Invalid selection.");
        }
    } else if office_level == "no" {
        println!("You do not have experience in an office setting.");
    } else {
        println!("Invalid input. Please enter 'yes' or 'no'.");
    }

    println!("Have you worked in an academic setting before? (yes/no): ");
    io::stdin().read_line(&mut academic_level).expect("Pls pick yes/no");
    let academic_level = academic_level.trim().to_lowercase();

    if academic_level == "yes" {
        println!("Pls pick one ofasma the following academic jobs or equivalents:");
        println!("1. Placement (Press 1)");
        println!("2. Research Assistant (Press 2)");
        println!("3. PhD Candidate (Press 3)");
        println!("4. Post-Doc Researcher (Press 4)");
        println!("5. Senior Lecturer (Press 5)");
        println!("6. Dean (Press 6)");

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).expect("Failed to read number");
        let choice: usize = choice_str.trim().parse().expect("Invalid number try again");

        if choice >= 1 && choice <= 6 {
            println!("Your Public Servant Level is: {} in the academic field", aps_levels[choice - 1]);
        } else {
            println!("Invalid selection.");
        }
    } else if academic_level == "no" {
        println!("You do not have experience in an academic setting.");
    } else {
        println!("Invalid input. Please enter 'yes' or 'no'.");
    }

    println!("Have you worked in a legal setting before? (yes/no): ");
    io::stdin().read_line(&mut legal_level).expect("Pls pick yes/no");
    let legal_level = legal_level.trim().to_lowercase();

    if legal_level == "yes" {
        println!("Pls pick one of the following legal jobs or equivalents:");
        println!("1. Intern–Paralegal (Press 1)");
        println!("2. Junior Associate (Press 2)");
        println!("3. Associate (Press 3)");
        println!("4. Senior Associate 1-2 (Press 4)");
        println!("5. Senior Associate 3-4 (Press 5)");
        println!("6. Partner (Press 6)");

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).expect("Failed to read number");
        let choice: usize = choice_str.trim().parse().expect("Invalid number try again");

        if choice >= 1 && choice <= 6 {
            println!("Your Public Servant Level is: {} in the legal field", aps_levels[choice - 1]);
        } else {
            println!("Invalid selection.");
        }
    } else if legal_level == "no" {
        println!("You do not have experience in a legal setting.");
    } else {
        println!("Invalid input. Please enter 'yes' or 'no'.");
    }

    println!("Have you worked as a teacher before? (yes/no): ");
    io::stdin().read_line(&mut teacher_level).expect("Pls pick yes/no");
    let teacher_level = teacher_level.trim().to_lowercase();

    if teacher_level == "yes" {
        println!("Pls pick one of the following teaching jobs or equivalents:");
        println!("1. Classroom Teacher (Press 1)");
        println!("2. Snr Teacher (Press 2)");
        println!("3. Leading Teacher (Press 3)");
        println!("4. Deputy Principal (Press 4)");
        println!("5. Principal (Press 5)");

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).expect("Failed to read number");
        let choice: usize = choice_str.trim().parse().expect("Invalid number try again");

        if choice >= 1 && choice <= 5 {
            println!("Your Public Servant Level is: {} in the teaching field", aps_levels[choice]);
        } else {
            println!("Invalid selection.");
        }
    } else if teacher_level == "no" {
        println!("You do not have experience as a teacher.");
    } else {
        println!("Invalid input. Please enter 'yes' or 'no'.");
    }
}