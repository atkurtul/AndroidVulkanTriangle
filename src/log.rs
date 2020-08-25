#![macro_use]

extern "C" {
  fn __android_log_print(log: i32, app: *const u8, fmt: *const u8, msg: *const u8);
}

pub fn a_log(mut msg : String) {
  msg.push_str("\0");
  unsafe { __android_log_print(4, "native-activity\0".as_ptr(), "%s\0".as_ptr(), msg.as_ptr()) };
}

#[macro_export]
macro_rules! alog {
    ($($arg:tt)*) => {
        a_log(format!($($arg)*));
    };
}