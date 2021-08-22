// mod helpers;
// mod filesys;
mod clean;
mod messenger;

use clean::delete_logs;
use messenger::get_msg;
// use crate::messages::get_msg;
// use helpers::module_loaded as helpers_module_loaded;
// use filesys::get_list_of_data;

fn main() {
	//initialization
	let msg = get_msg();
	msg.test_impl();
	msg.started();

	//temp deleting logs
	let res = delete_logs();
	match res {
		Ok(_v) => msg.logs_deleted(),
		Err(e) => msg.error(e),
		// Err(e) => println!("{}{}", msg.error, e),
	}

	//deactivation
	msg.stopped();
}
