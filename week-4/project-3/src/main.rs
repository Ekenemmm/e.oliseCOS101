use std::io;
fn main() {
    println!("P = Poundo Yam/Edinkaiko soup - N3,200");
    println!("F = Fried Rice and Chicken - N3,000");
    println!("A = Amala and Ewedu Soup - N2,500");
    println!("E = Eba and Egusi Soup - N2,000");
    println!("W = White Rice and Stew - N2,500");

    let mut food_input=String::new(); //meal choice of buyer
    let mut qty_input=String::new(); //how many portions or servings

    println!("Enter your meal of choice please (P,F,A,E,W):" );
    io::stdin().read_line(&mut food_input).expect("Failed to read line");
    let food = food_input.trim().to_uppercase(); //if buyer sends in p,f,a,e,w

    
    println!("Enter number of portions(quantity):");
    io::stdin().read_line(&mut qty_input).expect("Failed to read line");
    let quantity:i32 = qty_input.trim().parse().expect("Not a valid number"); 
    
    let price = match food.as_str () {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,

        _=> {
            println!("This meal is not available, sorry");
            return; 
        }
        
    };
    let mut total = price * quantity;

    if total > 10000 {
        let discount = total as f64 * 0.05; //5% percent discount 5/100=0.05
        total = total - discount as i32 ;
        println!("Congratulations, you've recieved a 5% discount of N{} on the house", discount);

    }
    println!("Total balance to be paid : N{} , thanks for your patronage",total);
     
}
