use std::io;

fn trapezium_area() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter height: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let height: f64 = input1.trim().parse().expect("Invalid number");

    println!("Enter base 1: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1: f64 = input2.trim().parse().expect("Invalid number");

    println!("Enter base 2: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2: f64 = input3.trim().parse().expect("Invalid number");

    let area = (height / 2.0) * (base1 + base2);
    println!("Area of Trapezium = {}\n", area);
}

fn rhombus_area() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter diagonal 1: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let d1: f64 = input1.trim().parse().expect("Invalid number");

    println!("Enter diagonal 2: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let d2: f64 = input2.trim().parse().expect("Invalid number");

    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}\n", area);
}

fn parallelogram_area() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter base: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Invalid number");

    println!("Enter altitude: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude: f64 = input2.trim().parse().expect("Invalid number");

    let area = base * altitude;
    println!("Area of Parallelogram = {}\n", area);
}

fn cube_area() {
    let mut input1 = String::new();

    println!("Enter length of the side: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let side: f64 = input1.trim().parse().expect("Invalid number");

    let area = 6.0 * (side * side);
    println!("Area of Cube = {}\n", area);
}

fn cylinder_volume() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter radius: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius: f64 = input1.trim().parse().expect("Invalid number");

    println!("Enter height: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height: f64 = input2.trim().parse().expect("Invalid number");

    const PI: f64 = 3.14159;
    let volume = PI * (radius * radius) * height;
    println!("Volume of Cylinder = {}\n", volume);
}

fn main() {
    loop {
        println!("       MTH 101 CALCULATOR PROGRAM       ");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");
        println!("Select an equation (1-6): ");

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).expect("Failed to read input");
        let choice: i32 = match choice_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number");
                continue;
            }
        };

        // Calling the appropriate function based on user selection
        if choice == 1 {
            trapezium_area();
        } else if choice == 2 {
            rhombus_area();
        } else if choice == 3 {
            parallelogram_area();
        } else if choice == 4 {
            cube_area();
        } else if choice == 5 {
            cylinder_volume();
        } else if choice == 6 {
            println!("Exiting program. Thank you for using the calculator!");
            break;
        } else {
            println!("Invalid choice! Please select between 1 and 6.");
        }
    }
}