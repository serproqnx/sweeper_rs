mod helpers;
mod filesys;
mod clean;

use clean::delete_logs;
use helpers::module_loaded as helpers_module_loaded;
use filesys::get_list_of_data;

fn main() {
	// helpers_module_loaded();
  delete_logs(); 
	// get_list_of_data();
}