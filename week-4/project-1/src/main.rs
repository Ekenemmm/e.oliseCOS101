use std::io;

fn main() {
    let mut input1=String::new();
    let mut input2=String::new();
    let mut input3=String::new();

    println!("Enter value of a:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of c:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f64 = input3.trim().parse().expect("Not a valid number");

    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("Two distinct roots: {} and {}", root1, root2);

    } else if discriminant == 0.0 {
        let root= -b / (2.0 * a);
        println! ("one real root: {}", root);

    }else {
        println!("There is no real root");
    }


}
