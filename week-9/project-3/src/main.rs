fn main() {


    // Names of Commissioners
    let commissioners = vec![
        "Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye",
    ];

    // Ministries
    let ministries = vec![
        "Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum",
    ];

    // Geopolitical Zones
    let zones = vec![
        "South West", "North East", "South South", "South West", "South East",
    ];

    println!("--- MERGED EFCC DATASET ---\n");
    println!("{:<5} {:<30} {:<20} {:<15}", "S/N", "COMMISSIONER", "MINISTRY", "ZONE");

    // Merge by iterating using indexing
    for i in 0..commissioners.len() {
        // Access using indexing 
        let name = commissioners[i];
        let ministry = ministries[i];
        let zone = zones[i];

        println!("{:<5} {:<30} {:<20} {:<15}", i + 1, name, ministry, zone);
    }
}
