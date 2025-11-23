use std::io;

fn main() {
    println!("===== EY EXPERIENCE CHECKER =====");

    // Ask how many candidates
    println!("Enter the number of candidates:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_candidates: usize = input.trim().parse().expect("invalid"); 

    // Create an empty vector to store tuples (name, years_of_experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    // Input candidate details
    for i in 0..num_candidates {
        let mut name = String::new();
        let mut years_input = String::new();

        println!("Enter the name of candidate {}:", i + 1);
        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim().to_string();

        println!("Enter years of programming experience for {}:", name);
        io::stdin().read_line(&mut years_input).expect("Failed to read years");
        let years: u32 = years_input.trim().parse().expect("invalid"); 

        // Push tuple into vector
        candidates.push((name, years));
    }

    // Find the candidate with the highest experience
    let mut max_candidate = &candidates[0];
    for i in 1..candidates.len() {
        if candidates[i].1 > max_candidate.1 {
            max_candidate = &candidates[i];
        }
    }

    println!(
        "The person with the highest years of programming experience is {} with {} years.",
        max_candidate.0, max_candidate.1
    );
}
