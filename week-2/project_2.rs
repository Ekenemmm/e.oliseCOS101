fn main() {
	// quantities and amounts of the items, needed for further calculation
	let toshiba_qty = 2;
	let toshiba_amt = 450_000.0; 

	let mac_qty = 1;
	let mac_amt = 1_500_000.0;

	let hp_qty = 3;
	let hp_amt = 750_000.0;

	let dell_qty = 3;
	let dell_amt = 2_850_000.0;

	let acer_qty = 1;
	let acer_amt = 250_000.0;

	let total_qty = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty; // adding up the total quantities to be used in calculating average

	// sales calculation (qty * amount)
	let toshiba_sales = toshiba_qty as f64 * toshiba_amt;
    let mac_sales = mac_qty as f64 * mac_amt;
    let hp_sales = hp_qty as f64 * hp_amt;
    let dell_sales = dell_qty as f64 * dell_amt;
    let acer_sales = acer_qty as f64 * acer_amt;

    // Total value of sales
    let sum = toshiba_sales + mac_sales + hp_sales + dell_sales + acer_sales;

    // Average sales value
    let average = sum / total_qty as f64;

    // Output (final result of calculations)
    println!("P.M Okeke and Sons LTD Record"); // heading for easy identification
    println!("============================="); // faking an underline because rust doesnt support text formatting
    println!("Total Sales = ₦{:.2}", sum);
    println!("Average Sales = ₦{:.2}", average);
}