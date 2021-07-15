use std::{fs, io};

//super mod helpers;

mod ::helpers;


pub fn get_ls_usr_desktop() -> io::Result<()> {
	let mut entries = fs::read_dir(".")?
					.map(|res| res.map(|e| e.path()))
					.collect::<Result<Vec<_>, io::Error>>()?;

			// The order in which `read_dir` returns entries is not guaranteed. If reproducible
			// ordering is required the entries should be explicitly sorted.

			let ls = entries.sort();
			println!("{:?}", ls);
			// The entries have now been sorted by their path.

			Ok(())
}