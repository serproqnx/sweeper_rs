// use crate::filesys::get_path_usr_desktop::get_usr_paths;

pub struct Paths {
	pub temp_path: &'static str,
}

impl Paths {
	pub fn load_usr_paths(&self) {
		// let a = get_usr_paths();
		//println!("{:?}", a);
	}
}

pub fn get_paths() -> Paths {

  let path = Paths {
    temp_path: "C:\\temp\\TEST.txt",
  };

	&path.load_usr_paths();

	path

}