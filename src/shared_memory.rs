use std::{ptr, ffi::c_void};

use windows::{
  Win32::{
    System::Memory::{
      MapViewOfFile,
      UnmapViewOfFile,
      OpenFileMappingW,
      MEMORY_MAPPED_VIEW_ADDRESS,
      FILE_MAP_READ
    },
    Foundation::{
      CloseHandle,
      HANDLE
    }
  },
  core::HSTRING
};

use crate::{
  convert::SdkConverter,
  types::telemetry::TelemetryData
};


const DEFAULT_MAP_SIZE: usize = 32 * 1024;
const DEFAULT_MAP_NAME: &str = "Local\\SCSTelemetry";


pub struct SharedMemory {
  file_mapping_handle: HANDLE,
  mapped_view_address: MEMORY_MAPPED_VIEW_ADDRESS,
  converter: SdkConverter
}

impl Drop for SharedMemory {
  fn drop(&mut self) {
    self.dispose()
  }
}

impl SharedMemory {
  /// Create a [SharedMemory] by open Windows's mapped file.
  pub fn connect() -> Self {
    let h_map_file: HANDLE = unsafe {
      OpenFileMappingW(
        FILE_MAP_READ.0,
        false,
        &HSTRING::from(DEFAULT_MAP_NAME)
      ).expect(
        "Can't read shared memory, make sure the SCSTelemetry SDK is installed and the game is running."
      )
    };

    let p_buf: MEMORY_MAPPED_VIEW_ADDRESS = unsafe {
      MapViewOfFile(
        h_map_file,
        FILE_MAP_READ,
        0,
        0,
        DEFAULT_MAP_SIZE
      )
    };

    let exposed_ptr: *const c_void = ptr::from_exposed_addr::<c_void>(p_buf.Value.expose_addr());

    SharedMemory {
      file_mapping_handle: h_map_file,
      mapped_view_address: p_buf,
      converter: SdkConverter::new(exposed_ptr)
    }
  }

  /// Call [SdkConverter::convert] and return data.
  pub fn read(self: &mut Self) -> TelemetryData {
    self.converter.convert()
  }

  /// Unmap file and close handle.
  pub(self) fn dispose(self: &Self) {
    unsafe {
      UnmapViewOfFile(self.mapped_view_address).unwrap();

      CloseHandle(self.file_mapping_handle).unwrap();
    }
  }
}
