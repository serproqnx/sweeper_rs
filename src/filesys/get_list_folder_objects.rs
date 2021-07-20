use super::get_path_usr_desktop;

use std::{fs, io};
use get_path_usr_desktop::get_path;

pub fn get_ls_usr_desktop() -> Result {
// pub fn get_ls_usr_desktop() -> Struct {
	let d_path = get_path(); 

	let entries = fs::read_dir(d_path)?
					.map(|res| res.map(|e| e.path()))
					.collect::<Result<Vec<_>, io::Error>>()?;

	
	// return entries;	

	Ok(entries)
}