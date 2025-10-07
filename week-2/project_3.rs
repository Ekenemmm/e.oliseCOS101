fn main () {
	let p : f64 = 210_000.0; // original price of the television
	let r : f64 = 5.0; // depreciation rate
	let n : i32 = 3; //number of years of depreciation

	// applying depreciation formula a = p * (1 - (r/100))
	let a = p * (1.0 - (r/100.0)).powi(n); // calculate the depreciated value of Ms Akudo's TV after three years

	println!("Ms. Akudo Ijezie's TV Depreciation Record"); // heading for easier identification
	println!("========================================="); // fake underline
	println!("Depreciated Value of TV = ₦{:.2}", a); // displays the depreciated value
    println!("Total Depreciation =  ₦{:.2}", p - a); // finds the total value lost, through subtration and displays it
}