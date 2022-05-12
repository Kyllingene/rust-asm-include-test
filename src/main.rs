#[link(name = "return_one")]
extern "C" {
	fn return_one() -> u64;
}

fn main() {
    println!("Hello, world!");
	println!("{}", unsafe{return_one()})
}
