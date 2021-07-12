mod greeting;
use greeting::{hello as hi, hello_alt, german_hello};

fn main() {
	hi();
	hello_alt();
	german_hello();
}
