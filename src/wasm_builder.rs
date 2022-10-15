use std::process::Command;

pub fn wasm_builder() {
  if cfg!(target_os = "windows") {
    Command::new("cmd")
      .arg("dev.sh")
      .status()
      .expect("failed to execute process in window");
  } else {
    Command::new("sh")
      .arg("dev.sh")
      .status()
      .expect("failed to execute process in linux");
  };
}