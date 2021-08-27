use chrono::{DateTime, Local};

pub struct Messages {
  pub program_started: &'static str,
  pub logs_were_deleted: &'static str,
  pub program_stopped: &'static str,
  pub test_impl: &'static str,
	pub try_delete_logs: &'static str,
  pub error: &'static str,
}

impl Messages {
	pub fn started(&self) 
		{	p(self.program_started) }
	pub fn stopped(&self) 
		{	p(self.program_stopped) }
	pub fn logs_deleted(&self) 
		{ p(self.logs_were_deleted) }
	pub fn test_impl(&self) 
		{	p(self.test_impl) }
	pub fn try_delete_logs(&self) 
		{	p(self.try_delete_logs)	}
	pub fn error(&self, err_txt: std::io::Error) 
		{ p_err(self.error, err_txt) }
}

fn p(txt: &'static str) {
	let now: DateTime<Local> = Local::now();
	println!("[{}] - {}", now.format("%T"), txt);
}

fn p_err(txt: &'static str, err_txt: std::io::Error) {
	let now: DateTime<Local> = Local::now();
	println!("[{}] - {} {}", now.format("%T"), txt, err_txt);
}

pub fn get_msg() -> Messages {
  let msg = Messages {
		program_started: "🚀 Sweeper started  ===============",
    logs_were_deleted: " ˫✔️  Logs were deleted",
    program_stopped: "👍 Sweeper stopped  ===============",
    test_impl: "",
		try_delete_logs: "🧹 Trying to delete logs...",
    error: " ˫💥 Error:",
  };

  // println!("{}", msg.program_started);
	

  msg
}
