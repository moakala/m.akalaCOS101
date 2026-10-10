fn main() {
    loop {
        println!("Welcome to PAU CAFE");
        println!("What would you like to order?");
        println!("If you want to order Poundo Yam/Edinkaiko Soup please type P");
        println!("If you want to order Fried Rice & Chicken please type F");
        println!("If you want to order Amala & Ewedu Soup please type A");
        println!("If you want to order Eba & Egusi Soup please type E");
        println!("If you want to order White Rice & Stew please type W");
        
        let mut food = String::new();
        std::io::stdin().read_line(&mut food).expect("Please enter a valid order");
        let order = food.trim();

        println!("How many portions would you like to order?");
        let mut quantity = String::new();
        std::io::stdin().read_line(&mut quantity).expect("Please enter a valid number");
        let quantity: f32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid quantity entered! Retrying...");
                continue;
            }
        };

        let price_portion: f32;

        if order == "P" || order == "p" {
            price_portion = 3200.0;
        } else if order == "F" || order == "f" {
            price_portion = 3000.0;
        } else if order == "A" || order == "a" {
            price_portion = 2500.0;
        } else if order == "E" || order == "e" {
            price_portion = 2000.0;
        } else if order == "W" || order == "w" {
            price_portion = 2500.0;
        } else {
            println!("Invalid order entered! Try again.");
            continue; // Replaced 'return' with 'continue' to stay in the loop
        }

        let mut total_price = price_portion * quantity;

        if total_price > 10000.0 {
            let discount: f32 = total_price * 0.05;
            println!("You are eligible for a discount of ₦{}", discount);
            total_price -= discount;
            println!("Your new total price after discount is: ₦{}", total_price);
        } else {
            println!("You are not eligible for a discount.");
            println!("Your total price is: ₦{}", total_price);
        } // Fixed missing brace here

        println!("\nIf you like to order more, please type Y, if not type N");
        let mut again = String::new();
        std::io::stdin().read_line(&mut again).expect("Failed to read input");
        let choice = again.trim();

        if choice == "Y" || choice == "y" {
            continue;
        } else if choice == "N" || choice == "n" {
            println!("Thank you for your order!");
            break;
        } else {
            println!("Invalid input. Exiting the program.");
            break;
        }
    }
}