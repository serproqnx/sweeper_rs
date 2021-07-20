mod get_list_folder_objects;
mod get_path_usr_desktop;

use std::fs;

// use get_path_usr_desktop::get_path;
use get_list_folder_objects::get_ls_usr_desktop;

pub fn get_list_of_data() {
	let entries = get_ls_usr_desktop();

	for value in entries {
		let attr = fs::metadata(value);
		println!("{:?}", attr.created().unwrap());
	}
}