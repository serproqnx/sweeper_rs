use std::{fs, io};

//super mod helpers;
// mod helpers;
use crate::helpers::get_path_usr_desktop;

pub fn get_ls_usr_desktop() -> io::Result<()> {
	let d_path = get_path_usr_desktop(); 
	// println!("{:?}", &d_path);
	let mut entries = fs::read_dir(d_path)?
					.map(|res| res.map(|e| e.path()))
					.collect::<Result<Vec<_>, io::Error>>()?;

			// The order in which `read_dir` returns entries is not guaranteed. If reproducible
			// ordering is required the entries should be explicitly sorted.
			// println!("{:?}", entries);
			for value in entries {
				println!("{:?}", value);
			}
			// The entries have now been sorted by their path.

			Ok(())
}