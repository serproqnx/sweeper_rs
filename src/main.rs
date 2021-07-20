// mod greeting;
// use greeting::{hello as hi, hello_alt, german_hello};

mod helpers;
mod filesys;

use helpers::module_loaded as helpers_module_loaded;

use filesys::get_list_of_data;

fn main() {
	helpers_module_loaded();
	get_list_of_data();
  // hi();
  // hello_alt();
  // german_hello();
  // let path = get_path_usr_desktop();
  // println!("{}", path);

  // get_ls_usr_desktop().ok();
}
