use crate::filesys::get_path_usr_desktop::get_usr_paths;
use directories::UserDirs;

pub struct Paths {
	pub temp_path: &'static str,	
	pub user_paths: UserDirs,
}

// impl Paths {
// 	pub fn load_usr_downloads(self) {
// 		self.user_paths.download_dir
// 		// println!("{:?}", paths);


// 		// match paths {
// 		// 	Some(p) => println!("{:?}", p),
// 		// 	None => println!("none"),
// 		// }
// 	}
// }

pub fn get_paths() -> Paths {
  let path_db = Paths {
    temp_path: "C:\\temp\\TEST.txt",
		user_paths: get_usr_paths(),
  };

	// println!("{:?}", path_db.user_paths);

	// &path.load_usr_paths();

	path_db
}
