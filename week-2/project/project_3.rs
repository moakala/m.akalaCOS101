fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// depreciation project 3
	let b = 1.0 - (r/100.0);
	let c = b.powf(t);
	let a = c*p;
	println!("Amount is {}",a);
	let ci = a - p;
	println!("Compound interest is {}", ci);
}
