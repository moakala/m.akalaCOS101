fn main() {
	// average project 2
	let t:f64 = 450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 750000.00;
	let d:f64 = 2850000.00;
	let a:f64 = 250000.00;
	let total_cost:f64 = t * 2.00 + m* 1.00 + h * 3.00 + d * 3.00 + a * 1.00;
	println!("Total cost is {}", total_cost);
	let avg_cost:f64 = total_cost / 10.0;
	println!("Average cost is {}", avg_cost);
}
