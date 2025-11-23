use std::io;

fn main() {
    println!("===== APS LEVEL CHECKER =====");

    let mut profession = String::new();
    let mut title = String::new();
    let mut years_input = String::new();

    println!("Enter your profession (admin / academic / lawyer / teacher): ");
    io::stdin()
        .read_line(&mut profession)
        .expect("Failed to read profession");
    let profession = profession.trim().to_lowercase();

    println!("Enter your job title: ");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read title");
    let title = title.trim().to_lowercase();

    println!("Enter years of work experience: ");
    io::stdin()
        .read_line(&mut years_input)
        .expect("Failed to read years");
    let years: u32 = match years_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for years.");
            return;
        }
    };

    // Vector of tuples: (profession, job title, min years, max years, APS level)
    let roles: Vec<(&str, &str, u32, u32, &str)> = vec![
        ("admin", "intern", 0, 2, "APS 1-2"),
        ("admin", "administrator", 3, 5, "APS 3-5"),
        ("admin", "senior administrator", 5, 8, "APS 5-8"),
        ("academic", "research assistant", 3, 5, "APS 3-5"),
        ("academic", "phd candidate", 5, 8, "APS 5-8"),
        ("academic", "post-doc researcher", 8, 10, "EL1 8-10"),
        ("lawyer", "junior associate", 3, 5, "APS 3-5"),
        ("lawyer", "associate lawyer", 5, 8, "APS 5-8"),
        ("lawyer", "senior associate", 8, 10, "EL1 8-10"),
        ("lawyer", "partner", 10, 13, "EL2 10-13"),
        ("teacher", "classroom teacher", 3, 5, "APS 3-5"),
        ("teacher", "senior teacher", 5, 8, "APS 5-8"),
        ("teacher", "leading teacher", 8, 10, "EL1 8-10"),
        ("teacher", "deputy principal", 10, 13, "EL2 10-13"),
    ];

    // Loop through vector using index
    let mut aps_found = "No matching APS level found.";
    for i in 0..roles.len() {
        let role = roles[i]; // access tuple by index
        let prof = role.0;
        let job = role.1;
        let min_years = role.2;
        let max_years = role.3;
        let aps = role.4;

        if prof == profession && job == title && years >= min_years && years <= max_years {
            // Use match to output APS level
            match aps {
                "APS 1-2" => aps_found = "APS Level: APS 1-2",
                "APS 3-5" => aps_found = "APS Level: APS 3-5",
                "APS 5-8" => aps_found = "APS Level: APS 5-8",
                "EL1 8-10" => aps_found = "APS Level: EL1 8-10",
                "EL2 10-13" => aps_found = "APS Level: EL2 10-13",
                _ => aps_found = "APS Level found but not listed!",
            }
            break; // stop the loop once found
        }
    }

    println!("{}", aps_found);
}
