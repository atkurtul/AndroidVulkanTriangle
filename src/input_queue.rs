
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputQueue(usize);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Looper(usize);

pub use crate::event::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LooperPollData(i32, i32, usize, i32);


pub type LooperCallback = extern fn(i32, i32, usize);

impl InputQueue {
  pub fn attach_looper(self, looper: Looper, ident: i32, callback: LooperCallback, data: usize) {
    unsafe { AInputQueue_attachLooper(self, looper, ident, callback, data) }
  }
  pub fn detach_looper(self) {
    unsafe { AInputQueue_detachLooper(self) }
  }
  pub fn has_events(self) -> i32 {
     unsafe { AInputQueue_hasEvents(self) }
  }
  pub fn get_event(self) -> Option<InputEvent> {
    unsafe {
      let mut event = InputEvent(0);
      match AInputQueue_getEvent(self, &mut event) {
        r if r >= 0 => Some(event),
        _ => None
      }
    }
  }
  pub fn pre_dispatch_event(self, event: InputEvent) -> i32 {
    unsafe { AInputQueue_preDispatchEvent(self, event) }
  }
  pub fn finish_event(self, event: InputEvent, handled: i32) {
    unsafe { AInputQueue_finishEvent(self, event, handled) }
  }
}

impl Looper {
  pub fn for_thread() -> Looper {
    unsafe { ALooper_forThread() }
  }
  pub fn prepare(opts: i32) -> Looper {
    unsafe { ALooper_prepare(opts) }
  }
  pub fn acquire(self) {
    unsafe { ALooper_acquire(self) }
  }
  pub fn release(self) {
    unsafe { ALooper_release(self) }
  }
  pub fn poll_once(self, timeout: i32) -> LooperPollData {
    unsafe { 
      let mut data = LooperPollData(0, 0, 0, 0);
      data.3 = ALooper_pollOnce(self, timeout, &mut data.0, &mut data.1, &mut data.2);
      data
    }
  }
  pub fn poll_all(self, timeout: i32) -> LooperPollData {
    unsafe { 
      let mut data = LooperPollData(0, 0, 0, 0);
      data.3 = ALooper_pollAll(self, timeout, &mut data.0, &mut data.1, &mut data.2);
      data
    }
  }
  pub fn wake(self) {
    unsafe { ALooper_wake(self) }
  }
  pub fn add_fd(self, fd: i32, ident: i32, events: i32, callback: LooperCallback, data: usize) -> i32 {
    unsafe { ALooper_addFd(self, fd, ident, events, callback, data) }
  }
  pub fn remove_fd(self, fd: i32) -> i32 {
    unsafe { ALooper_removeFd(self, fd) }
  }
}

extern "C" {
  fn AInputQueue_attachLooper(queue: InputQueue, looper: Looper, ident: i32, callback: LooperCallback, data: usize);
  fn AInputQueue_detachLooper(queue: InputQueue);
  fn AInputQueue_hasEvents(queue: InputQueue) -> i32;
  fn AInputQueue_getEvent(queue: InputQueue, event: *mut InputEvent) -> i32;
  fn AInputQueue_preDispatchEvent(queue: InputQueue, event: InputEvent) -> i32;
  fn AInputQueue_finishEvent(queue: InputQueue, event: InputEvent, handled: i32);
}

extern "C" {
  fn ALooper_forThread() -> Looper;
  fn ALooper_prepare(opts: i32) -> Looper;
  fn ALooper_acquire(looper: Looper);
  fn ALooper_release(looper: Looper);
  fn ALooper_pollOnce(looper: Looper, timeout: i32, fd: *mut i32, events: *mut i32, data: *mut usize) -> i32;
  fn ALooper_pollAll(looper: Looper, timeout: i32, fd: *mut i32, events: *mut i32, data: *mut usize) -> i32;
  fn ALooper_wake(looper: Looper);
  fn ALooper_addFd(looper: Looper, fd: i32, ident: i32, events: i32, callback: LooperCallback, data: usize) -> i32;
  fn ALooper_removeFd(looper: Looper, fd: i32) -> i32;
}