use std::io::Write;

fn main() {
    let names = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matric_numbers = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001",
    ];

    let departments = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let levels = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("pau_smis.txt").expect("create failed");

    file.write_all("PAU SMIS\n".as_bytes()).expect("write failed");
    file.write_all("Student Name\t\tMatric Number\tDepartment\tLevel\n".as_bytes()).expect("write failed");

    for i in 0..names.len() {
        file.write_all(names[i].as_bytes()).expect("write failed");
        file.write_all("\t\t".as_bytes()).expect("write failed");
        file.write_all(matric_numbers[i].as_bytes()).expect("write failed");
        file.write_all("\t\t".as_bytes()).expect("write failed");
        file.write_all(departments[i].as_bytes()).expect("write failed");
        file.write_all("\t\t".as_bytes()).expect("write failed");
        file.write_all(levels[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("\nStudent records successfully saved!");
}