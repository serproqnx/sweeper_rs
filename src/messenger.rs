pub struct Messages {
  pub program_started: &'static str,
  pub logs_were_deleted: &'static str,
  pub program_stopped: &'static str,
  pub test_impl: &'static str,
  pub error: &'static str,
}

fn p(txt: &'static str) {
  println!("{}", txt);
}

fn p_err(txt: &'static str, err_txt: std::io::Error) {
  println!("{}{}", txt, err_txt);
}

pub fn get_msg() -> Messages {
  let msg = Messages {
    program_started: "\n🚀 Sweeper started  ===============\n",
    logs_were_deleted: "Logs were deleted",
    program_stopped: "\n👍 Sweeper stopped  ===============\n",
    test_impl: "",
    error: "💥 Error: "
  };

  // println!("{}", msg.program_started);

  impl Messages {
    // fn started(text: &'static str) {
    // fn prnt(txt: &'static str) {println!("{}", txt);}

    // pub fn started(&self) {println!("{}", self.test_impl);}
    pub fn started(&self) {
      p(self.program_started);
    }
    pub fn stopped(&self) {
      p(self.program_stopped);
    }
    pub fn logs_deleted(&self) {
      p(self.logs_were_deleted);
    }
    pub fn test_impl(&self) {
      p(self.test_impl);
    }
    pub fn error(&self, err_txt: std::io::Error) {
      p_err(self.error, err_txt);
    }
  }

  msg
}
