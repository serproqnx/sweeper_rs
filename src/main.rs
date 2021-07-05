mod greeting {
	pub fn hello() {
		println!("Hello, world!");
	}

	pub fn hello_alt() {
		println!("What's up?");
	}
}

use greeting::{hello, hello_alt};

fn main() {
	hello();
	hello_alt();
}
