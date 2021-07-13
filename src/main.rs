// mod greeting;
// use greeting::{hello as hi, hello_alt, german_hello};

mod helpers;
use helpers::{get_path_usr_desktop};

fn main() {
	// hi();
	// hello_alt();
	// german_hello();
	let path = get_path_usr_desktop();
	println!("{}", path);
}
