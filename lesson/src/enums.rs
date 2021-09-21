enum OS {
  Windows(u32, String),
  Mac(u32, String),
  Linux(u32, String),
}

pub fn run() {
  let linux = OS::Linux(1991, String::from("Linus"));
  let windows = OS::Windows(1985, String::from("MicroSoft"));
  let apple = OS::Mac(2001, String::from("Apple"));
}
