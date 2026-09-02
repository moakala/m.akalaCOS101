fn main() {
	Let p:f64 = 520000000.0
	Let r:f64 = 10.0
	Let t:f64 = 5.0

	/// simple interest
	let a = p * (1.0 + (r/100.0)) ^ t)
	println!("Amount is {}",a);
	Let ci = a - p;
	println!("Compound interest is {}", ci)
}
