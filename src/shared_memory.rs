use std::{ffi::c_void, ptr};

use thiserror::Error;
use ::windows::{
  core::HSTRING,
  Win32::{
    Foundation::{
      CloseHandle,
      HANDLE
    },
    System::Memory::{
      MapViewOfFile,
      OpenFileMappingW,
      UnmapViewOfFile,
      FILE_MAP_READ,
      MEMORY_MAPPED_VIEW_ADDRESS
    }
  }
};

use crate::{
  convert::SdkConverter,
  types::telemetry::TelemetryData
};


const DEFAULT_MAP_SIZE: usize = 32 * 1024;
const DEFAULT_MAP_NAME: &str = "Local\\SCSTelemetry";


#[derive(Error, Debug)]
pub enum SharedMemError {
  #[error("can't read shared memory, make sure the SCSTelemetry SDK is installed and the game is running.")]
  NotFound(#[from] ::windows::core::Error)
}


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
  pub fn connect() -> Result<Self, SharedMemError> {
    let h_map_file = unsafe {
      OpenFileMappingW(
        FILE_MAP_READ.0,
        false,
        &HSTRING::from(DEFAULT_MAP_NAME)
      )
    };

    match h_map_file {
      Ok(h_map_file) => {
        let p_buf: MEMORY_MAPPED_VIEW_ADDRESS = unsafe {
          MapViewOfFile(
            h_map_file,
            FILE_MAP_READ,
            0,
            0,
            DEFAULT_MAP_SIZE
          )
        };
  
        let exposed_ptr: *const c_void = ptr::with_exposed_provenance::<c_void>(p_buf.Value.addr());
  
        return Ok(
          SharedMemory {
            file_mapping_handle: h_map_file,
            mapped_view_address: p_buf,
            converter: SdkConverter::new(exposed_ptr)
          }
        )
      },
      Err(err) => {
        return Err(SharedMemError::NotFound(err));
      },
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
