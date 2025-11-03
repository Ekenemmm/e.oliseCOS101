use std::io;

fn main() {
    println!("==================== MENU ===================="); //menu for user
    println!("P = Poundo Yam/Edinkaiko Soup  - N3200");
    println!("F = Fried Rice & Chicken       - N3000");
    println!("A = Amala & Ewedu Soup         - N2500");
    println!("E = Eba & Egusi Soup           - N2000");
    println!("W = White Rice & Stew          - N2500");
    println!("==============================================");

    let mut food_input = String::new(); // variables for storing user input
    let mut qty_input = String::new();

    println!("Enter the type of food (P/F/A/E/W): "); //ask for meal choice 
    io::stdin().read_line(&mut food_input).expect("Failed to read line");
    let food = food_input.trim().to_uppercase(); //converts all to uppercase and removes whitespace so all cases work

    println!("Enter quantity: ");
    io::stdin().read_line(&mut qty_input).expect("Failed to read line");
    let quantity: i32 = qty_input.trim().parse().expect("Not a valid number"); //convert string to integer

    let price = match food.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => { //incase the user asks for a meal thats not sold or unavailable
            println!("Invalid food choice!");
            return;
        }
    };

    let mut total = price * quantity; //calculate total amount before discount

    if total > 10000 {
        let discount = (total as f64 * 0.05) as i32; // converts total to f64 before multiplying by 0.05, then convert back to i32 to avoid error
        total = total - discount;
        println!("You got a 5% discount of N{}", discount);
    }

    println!("Total amount to pay: N{}", total); // what the user will see
}