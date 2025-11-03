fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki,L] Lagos";

    println!("Name: {}", name);
    println!("University : {}, \nAdress: {}", uni, addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nschool: {}", department, school);
}
