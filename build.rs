use std::env;
use nasm_rs;

fn main() {
	if env::var("CARGO_CFG_TARGET_OS").unwrap() == "unix" {
		nasm_rs::compile_library("libasm_main.a", &["src/asm_main.asm"]).unwrap();
	} else {
		nasm_rs::compile_library("asm_main.o", &["src/asm_main.asm"]).unwrap();
	}
}