// use std::{fs, io};

mod helpers;

use get_path_usr_desktop as get_dskt_path;


pub fn get_ls_usr_desktop() {
	let temp = get_dskt_path();
	println!("{}", temp);
}