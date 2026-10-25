use std::io::Write;

fn main() {
    let sn = vec!["1", "2", "3", "4", "5"];

    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let mut file = std::fs::File::create("Government records.txt").expect("create failed");

    file.write_all("S/N\tName of Commissioner\tGeopolitical Zone\tMinistry\n".as_bytes()).expect("write failed");

    for i in 0..commissioners.len() {
        file.write_all(sn[i].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(commissioners[i].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(zones[i].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(ministries[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("\nGovernment records successfully saved!");
}