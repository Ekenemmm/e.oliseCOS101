use std::io::Read;

fn open_file(x: &str) {
    // We use the parameter 'x' so it opens the correct file for the role
    match std::fs::File::open(x) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("\n--- Displaying Structure for {} ---\n", x);
            println!("{}", contents);
        }
        Err(_) => {
            println!("\nERROR: The file '{}' was not found in the project directory.", x);
            println!("Please run pg_dump to create it first!\n");
        }
    }
}

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter Status...");
        println!("1) Administrator (Full DB)");
        println!("2) Project Manager (Project Table)");
        println!("3) Employee (Staff Table)");
        println!("4) Customer (Customer Table)");
        println!("5) Vendor (Dataplan Table)");
        println!("0) Exit");
        println!("Select a number from 1 - 5:");

        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let status: i8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match status {
            1 => open_file("globacom_db.sql"), // Fixed typo from globalcom to globacom
            2 => open_file("project_tb.sql"),
            3 => open_file("staff_tb.sql"),
            4 => open_file("customer_tb.sql"),
            5 => open_file("dataplan_tb.sql"),
            0 => break,
            _ => println!("Invalid status entered"),
        }
    }
}