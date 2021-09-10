mod clean;
mod messenger;
mod paths;
mod filesys;

use clean::delete_usr_downloads;
use messenger::get_msg;

fn main() {
	//initialization
	let msg = get_msg();
	msg.started();

	//temp deleting logs
	let res = delete_usr_downloads();
	match res {
		Ok(_v) => msg.logs_deleted(),
		Err(e) => msg.error(e),
	}

	//deactivation
	msg.stopped();
}
