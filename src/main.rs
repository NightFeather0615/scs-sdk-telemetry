#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]
#![feature(default_free_fn)]


mod shared_memory;
mod convert;
mod types;


use shared_memory::SharedMemory;


fn main() {
  let mut shared_mem: SharedMemory = SharedMemory::connect();

  println!("{:#?}", shared_mem.read());

  shared_mem.dispose();
}
