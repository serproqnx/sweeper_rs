
pub fn hello() {
	println!("Hello, world!");
}

pub fn hello_alt() {
	println!("What's up?");
}

pub mod german;

pub use german::hello as german_hello;