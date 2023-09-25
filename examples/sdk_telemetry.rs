use scs_sdk_telemetry::shared_memory::SharedMemory;

fn main() {
  let mut shared_mem: SharedMemory = SharedMemory::connect();

  println!("{:#?}", shared_mem.read());
  
  #[cfg(feature = "json")]
  println!("{:#?}", shared_mem.read().to_json().unwrap().to_string());
}
