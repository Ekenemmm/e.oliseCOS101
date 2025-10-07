fn main() {
	let p:f64 = 520_000_000.00; //  principal amount
	let r:f64 = 10.00; // interest rate per year
    let n:i32 =5; // no of years

	//compund interest calculation
	let a = p * (1.0 + (r / 100.0)).powi(n); // raises value to the power of n
	let ci = a - p;  //  compound interest= amount - principal
	println!("Ibeju Local Government Chairman Project"); // heading for easy identification
    println!("======================================="); // faking an underline because rust doesnt support text formatting
	println!("Compound Amount after {} years =  ₦{:.2}",n, a);// displays result of first calc with 2 d.p
	println!("Compund Interest =  ₦{:.2}", ci); // display final result with 2 d.p
}