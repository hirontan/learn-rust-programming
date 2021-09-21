enum OS {
  Windows(u32, String),
  Mac(u32, String),
  Linux(u32, String),
}

pub fn run() {
  let linux = OS::Linux(1991, String::from("Linus"));
  print_os_info(linux);
  let windows = OS::Windows(1985, String::from("MicroSoft"));
  print_os_info(windows);
  let apple = OS::Mac(2001, String::from("Apple"));
  print_os_info(apple);
}

// OSの種類に応じて実体を切り替える
fn print_os_info(os: OS) {
  match os {
    OS::Windows(year, who) => {
      println!("Windows: first release in {} by {}", year, who);
    }
    OS::Mac(year, who) => {
      println!("Mac: first release in {} by {}", year, who);
    }
    OS::Linux(year, who) => {
      println!("Linux: first release in {} by {}", year, who);
    }
  }
}
