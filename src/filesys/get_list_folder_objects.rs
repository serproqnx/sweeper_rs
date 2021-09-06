// use super::get_path_usr_desktop;

// use std::{fs, io};
// // use std::path::{PathBuf};
// // use get_path_usr_desktop::get_path;
// use std::fs::DirEntry;

// pub fn get_ls_usr_desktop() -> io::Result<Vec<DirEntry>> {
// pub fn get_ls_usr_desktop() -> Struct {
// 	let d_path = get_path(); 

// 	let entries = fs::read_dir(d_path)?
// 					.map(|res| res.map(|e| e))
// 					.collect::<Result<Vec<_>, io::Error>>()?;

	
// 	// return Result(entries);	
// 		Ok(entries)
// }