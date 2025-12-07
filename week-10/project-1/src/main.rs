// laptop structure
struct Laptop {
    brand: String,
    price: u32,
    quantity: u32,
}

// methods for the struct
impl Laptop {
    // Method to calculate cost of 3 laptops
    fn cost_of_three(&self) -> u32 {
        self.price * 3
    }
}

fn main() {
    // C laptop objects
    let hp = Laptop {
        brand: "HP".to_string(),
        price: 650000,
        quantity: 10,
    };

    let ibm = Laptop {
        brand: "IBM".to_string(),
        price: 755000,
        quantity: 6,
    };

    let toshiba = Laptop {
        brand: "Toshiba".to_string(),
        price: 550000,
        quantity: 10,
    };

    let dell = Laptop {
        brand: "Dell".to_string(),
        price: 850000,
        quantity: 4,
    };

    //cost for 3 laptops of each brand
    let hp_cost = hp.cost_of_three();
    let ibm_cost = ibm.cost_of_three();
    let toshiba_cost = toshiba.cost_of_three();
    let dell_cost = dell.cost_of_three();

    // total cost
    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    // show result
    println!("Cost of 3 HP laptops: N{}", hp_cost);
    println!("Cost of 3 IBM laptops: N{}", ibm_cost);
    println!("Cost of 3 Toshiba laptops: N{}", toshiba_cost);
    println!("Cost of 3 Dell laptops: N{}", dell_cost);
    println!("Total cost for customer: N{}", total_cost);
}
