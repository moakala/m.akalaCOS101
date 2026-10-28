struct Company {
    name: String,
    founded: u32,
    shares: u32,
    liabilities: u32,
}

impl Company {
    fn leverage(&self) -> u32 {
        self.shares + self.liabilities
    }
}

fn main() {
    let companies = vec![
        Company {
            name: String::from("Cadbury Nigeria Plc"),
            founded: 1965,
            shares: 1500,
            liabilities: 500,
        },
        Company {
            name: String::from("Champion Breweries Plc"),
            founded: 1974,
            shares: 1200,
            liabilities: 400,
        },
        Company {
            name: String::from("Dangote Sugar Refinery Plc"),
            founded: 2005,
            shares: 3000,
            liabilities: 1000,
        },
        Company {
            name: String::from("Flour Mills Nigeria Plc"),
            founded: 1960,
            shares: 2500,
            liabilities: 800,
        },
        Company {
            name: String::from("Nestle Nigeria Plc"),
            founded: 1961,
            shares: 4000,
            liabilities: 1500,
        },
    ];

    println!("{:<30} | {:<10} | {:<10} | {:<15} | {:<10}", "Company Name", "Founded", "Shares", "Liabilities", "Leverage");
    println!("----------------------------------------------------------------------------------");

    for c in &companies {
        println!("{:<30} | {:<10} | {:<10} | {:<15} | {:<10}", c.name, c.founded, c.shares, c.liabilities, c.leverage());
    }
}