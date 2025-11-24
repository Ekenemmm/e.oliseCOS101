use std::fs::File;
use std::io::Write;

fn main() {
    // Vector of student records (Name, Matric No, Department, Level)
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
        ("Shania Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];

    // Display students
    println!("PAU SMIS - Student Records");
    println!("Name | Matric No | Department | Level");
    println!("---------------------------------------");

    for student in &students {
        println!("{} | {} | {} | {}", student.0, student.1, student.2, student.3);
    }

    // Save to file
    let mut file = File::create("students.txt").unwrap();

    file.write_all(b"Name, Matric No, Department, Level\n").unwrap();

    for student in students {
        let line = format!("{}, {}, {}, {}\n", student.0, student.1, student.2, student.3);
        file.write_all(line.as_bytes()).unwrap();
    }

    println!("Student data saved to students.txt successfully!");
}
