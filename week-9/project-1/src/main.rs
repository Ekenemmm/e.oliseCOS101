use std::fs::File;
use std::io::Write;

fn main() {
    // Vectors for drink categories
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Create file
    let mut file = File::create("drinks.txt").unwrap();

    // Write headers
    file.write_all(b"Lager Drinks:\n").unwrap();
    for drink in lager {
        file.write_all(format!("{}\n", drink).as_bytes()).unwrap();
    }

    file.write_all(b"\nStout Drinks:\n").unwrap();
    for drink in stout {
        file.write_all(format!("{}\n", drink).as_bytes()).unwrap();
    }

    file.write_all(b"\nNon-Alcoholic Drinks:\n").unwrap();
    for drink in non_alcoholic {
        file.write_all(format!("{}\n", drink).as_bytes()).unwrap();
    }

    println!("Drinks saved to drinks.txt successfully!");
}
