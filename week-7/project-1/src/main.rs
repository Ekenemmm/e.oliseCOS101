use std::io;

fn main() {
    println!("===== Area and Volume Calculator =====");

    loop { //so the user can perform multiple calculations
        println!("Do you want to calculate area or volume ? (A/V)"); //area or volume
        let mut choice= String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().to_uppercase();

        if choice == "A" {
            println!("Which shape's area do you want to calculate? (1,2,3,4)");
            println!("1 - trapezium");
            println!("2 - rhombus");
            println!("3 - parallelogram");
            println!("4 - cube");

            let mut area_choice = String::new();
            io::stdin().read_line(&mut area_choice).expect("Failed to read input");
            let area_choice = area_choice.trim();

            match area_choice {
                "1" => area_trapezium(),
                "2" => area_rhombus(),
                "3" => area_parallelogram(),
                "4" => area_cube(),
                _ => println!("Apologies, this calculation is unavailable"), //if they input another option that is not currently in my code
            }


        }

        else if choice == "V" {
            volume_cylinder();
        }
        println!("Do you want to do another calculation? (Y/N)"); //allows for multiple calculations
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read input");
        
        if again.trim().to_uppercase() == "N" { //to stop the program if user is done
            println!("Thanks for using ekenem's calculator. See ya!");
            break;
        }
    }
}
 //  AREA FORMULAS  preferred this way for readability

// Trapezium: Area = height/2 * (base1 + base2)
fn area_trapezium() {
    let mut h_in = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut h_in).expect("Failed to read");
    let h: f64 = h_in.trim().parse().expect("Not a number");

    let mut b1_in = String::new();
    println!("Enter base1:");
    io::stdin().read_line(&mut b1_in).expect("Failed to read");
    let b1: f64 = b1_in.trim().parse().expect("Not a number");

    let mut b2_in = String::new();
    println!("Enter base2:");
    io::stdin().read_line(&mut b2_in).expect("Failed to read");
    let b2: f64 = b2_in.trim().parse().expect("Not a number");

    let area = h / 2.0 * (b1 + b2);
    println!("Area of Trapezium = {:.2}", area);
}

// Rhombus: Area = 1/2 x diagonal1 x diagonal2
fn area_rhombus() {
    let mut d1_in = String::new();
    println!("Enter diagonal1:");
    io::stdin().read_line(&mut d1_in).expect("Failed to read");
    let d1: f64 = d1_in.trim().parse().expect("Not a number");

    let mut d2_in = String::new();
    println!("Enter diagonal2:");
    io::stdin().read_line(&mut d2_in).expect("Failed to read");
    let d2: f64 = d2_in.trim().parse().expect("Not a number");

    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {:.2}", area);
}

// Parallelogram: Area = base x altitude
fn area_parallelogram() {
    let mut b_in = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut b_in).expect("Failed to read");
    let base: f64 = b_in.trim().parse().expect("Not a number");

    let mut a_in = String::new();
    println!("Enter altitude:");
    io::stdin().read_line(&mut a_in).expect("Failed to read");
    let altitude: f64 = a_in.trim().parse().expect("Not a number");

    let area = base * altitude;
    println!("Area of Parallelogram = {:.2}", area);
}

// Cube: Area = 6 x side^2
fn area_cube() {
    let mut s_in = String::new();
    println!("Enter side length:");
    io::stdin().read_line(&mut s_in).expect("Failed to read");
    let side: f64 = s_in.trim().parse().expect("Not a number");

    let area = 6.0 * side * side;
    println!("Area of Cube = {:.2}", area);
}


// Cylinder: Volume = pi x radius^2 x height
fn volume_cylinder() {
    let mut r_in = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut r_in).expect("Failed to read");
    let radius: f64 = r_in.trim().parse().expect("Not a number");

    let mut h_in = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut h_in).expect("Failed to read");
    let height: f64 = h_in.trim().parse().expect("Not a number");

    let volume = 3.142 * radius * radius * height;
    println!("Volume of Cylinder = {:.2}", volume);
}