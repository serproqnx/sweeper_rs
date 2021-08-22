use std::fs;
// use crate::messages::get_msg;

pub fn delete_logs() -> std::io::Result<()> {
  struct Paths {
    temp_path: &'static str,
  }

  let path = Paths {
    temp_path: "C:\\temp\\TEST.txt",
  };

  // let dir_path_buf = fs::read_dir("C:\\temp\\");
  // let is_empty = &dir_path_buf.read_dir()?.next().is_none();

  
  fs::remove_file(&path.temp_path)?;

  Ok(())

  // println!("{}", &path.temp_path);
  // println!("delete logs module was connected");
}


