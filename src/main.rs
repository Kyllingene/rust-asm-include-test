#[link(name = "asm_main")]
extern "C" {
	fn return_one() -> u64;
}

fn main() {
    println!("Hello, world!");
	println!("{}", unsafe{return_one()})
}
