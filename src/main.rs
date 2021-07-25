mod helpers;
mod filesys;

use helpers::module_loaded as helpers_module_loaded;

use filesys::get_list_of_data;

fn main() {
	helpers_module_loaded();
	get_list_of_data();
}
