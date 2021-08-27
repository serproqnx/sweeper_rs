mod clean;
mod messenger;
mod paths;
mod filesys;

use clean::delete_logs;
use messenger::get_msg;

fn main() {
	//initialization
	let msg = get_msg();
	// msg.test_impl();
	msg.started();

	//temp deleting logs
	let res = delete_logs();
	match res {
		Ok(_v) => msg.logs_deleted(),
		Err(e) => msg.error(e),
	}

	//deactivation
	msg.stopped();
}
