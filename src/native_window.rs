
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NativeWindow(usize);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NativeWindowBuffer(usize);

impl NativeWindow {
  pub fn acquire(self) {
    unsafe { ANativeWindow_acquire(self) }
  }
  pub fn release(self) {
    unsafe { ANativeWindow_release(self) }
  }
  pub fn width(self) -> u32 {
    unsafe { ANativeWindow_getWidth(self) }
  }
  pub fn height(self) -> u32 {
    unsafe { ANativeWindow_getHeight(self) }
  }
  pub fn format(self) -> i32 {
    unsafe { ANativeWindow_getFormat(self) }
  }
  pub fn set_buffers_geometry(self, width: i32, height: i32, format: i32) -> i32 {
    unsafe { ANativeWindow_setBuffersGeometry(self, width, height, format) }
  }
  pub fn lock(self, out_buffer: NativeWindowBuffer, in_out_dirty_bounds: &mut [i32;4]) -> i32 {
    unsafe {  ANativeWindow_lock(self, out_buffer, in_out_dirty_bounds) }
  }
  // pub fn unlock_and_post(self) -> i32 {
  //   unsafe { ANativeWindow_unlockAndPost(self) }
  // }
  // pub fn set_buffers_transform(self, transform: i32)  -> i32 {
  //   unsafe { ANativeWindow_setBuffersTransform(self, transform) }
  // }
  // pub fn set_buffers_data_space(self, data_space: i32) -> i32 {
  //   unsafe { ANativeWindow_setBuffersDataSpace(self, data_space) }
  // }
  // pub fn get_buffers_data_space(self) -> i32 {
  //   unsafe { ANativeWindow_getBuffersDataSpace(self) }
  // }
  // pub fn set_framerate(self, frame_rate: f32, compatibility: i8) -> i32 {
  //   unsafe { ANativeWindow_setFrameRate(self, frame_rate, compatibility) }
  // }
  // pub fn try_allocate_buffers(self) {
  //   unsafe { ANativeWindow_tryAllocateBuffers(self) }
  // }
  pub fn as_void(self) -> *mut std::ffi::c_void {
    self.0 as _
  }
}

extern "C" {
  fn ANativeWindow_acquire(window: NativeWindow);
  fn ANativeWindow_release(window: NativeWindow);
  fn ANativeWindow_getWidth(window: NativeWindow) -> u32;
  fn ANativeWindow_getHeight(window: NativeWindow) -> u32;
  fn ANativeWindow_getFormat(window: NativeWindow) -> i32;
  fn ANativeWindow_setBuffersGeometry(window: NativeWindow, width: i32, height: i32, format: i32) -> i32;
  fn ANativeWindow_lock(window: NativeWindow, outBuffer: NativeWindowBuffer, inOutDirtyBounds: &mut [i32;4]) -> i32;
  fn ANativeWindow_unlockAndPost(window: NativeWindow) -> i32;
  fn ANativeWindow_setBuffersTransform(window: NativeWindow, transform: i32) -> i32;
  fn ANativeWindow_setBuffersDataSpace(window: NativeWindow, dataSpace: i32) -> i32;
  fn ANativeWindow_getBuffersDataSpace(window: NativeWindow) -> i32;
  fn ANativeWindow_setFrameRate(window: NativeWindow, frameRate: f32, compatibility: i8) -> i32;
  fn ANativeWindow_tryAllocateBuffers(window: NativeWindow);
}