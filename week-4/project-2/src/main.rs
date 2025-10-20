//problem on employee incentive

use std::io;

fn main() {
    let mut exp_input=String::new();
    let mut age_input=String::new();

    println!("Is the employee experienced? (yes/no)):");
    io::stdin().read_line(&mut exp_input).expect("Failed to read line");
    let experience = exp_input.trim().to_lowercase(); //incase they type YES or Yes 
                                                      // or basically any form of yes

    println!("Enter employee's age:");
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: i32 = age_input.trim().parse().expect("Not a valid number");

    
    let mut incentive: i32 = 0 ; 

    if experience == "yes" {
        if age >= 40 {
            incentive= 1560000;

        } else if age >=30 && age < 40{
            incentive= 1480000;

        }else if age < 28 {
            incentive = 1300000;
        }else {
            incentive = 0; // perhaps if theyre aged less than 28
                           // eg maybe a new hire in office fresh out of uni
        }
        }
        println!("The annual incentive is: N{}", incentive);
}