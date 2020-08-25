#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputEvent(pub usize);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KeyEvent(usize);

impl KeyEvent {
  pub fn action(self) -> i32 {
    unsafe { AKeyEvent_getAction(self) }
  }
  pub fn flags(self) -> i32 {
    unsafe { AKeyEvent_getFlags(self) }
  }
  pub fn key_code(self) -> i32 {
    unsafe { AKeyEvent_getKeyCode(self) }
  }
  pub fn scan_code(self) -> i32 {
    unsafe { AKeyEvent_getScanCode(self) }
  }
  pub fn meta_state(self) -> i32 {
    unsafe { AKeyEvent_getMetaState(self) }
  }
  pub fn repeat_count(self) -> i32 {
    unsafe { AKeyEvent_getRepeatCount(self) }
  }
  pub fn down_time(self) -> i64 {
    unsafe { AKeyEvent_getDownTime(self) }
  }
  pub fn event_time(self) -> i64 {
    unsafe { AKeyEvent_getEventTime(self) }
  }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MotionEvent(usize);

impl MotionEvent {
  pub fn action(self) -> i32 {
    unsafe { AMotionEvent_getAction(self) }
  }
  pub fn flags(self) -> i32 {
    unsafe { AMotionEvent_getFlags(self) }
  }
  pub fn meta_state(self) -> i32 {
    unsafe { AMotionEvent_getMetaState(self) }
  }
  pub fn button_state(self) -> i32 {
    unsafe { AMotionEvent_getButtonState(self) }
  }
  pub fn edge_flags(self) -> i32 {
    unsafe { AMotionEvent_getEdgeFlags(self) }
  }
  pub fn down_time(self) -> i64 {
    unsafe { AMotionEvent_getDownTime(self) }
  }
  pub fn event_time(self) -> i64 {
    unsafe { AMotionEvent_getEventTime(self) }
  }
  pub fn xoffset(self) -> f32 {
    unsafe { AMotionEvent_getXOffset(self) }
  }
  pub fn yoffset(self) -> f32 {
    unsafe { AMotionEvent_getYOffset(self) }
  }
  pub fn xprecision(self) -> f32 {
    unsafe { AMotionEvent_getXPrecision(self) }
  }
  pub fn yprecision(self) -> f32 {
    unsafe { AMotionEvent_getYPrecision(self) }
  }
  pub fn pointer_count(self) -> usize {
    unsafe { AMotionEvent_getPointerCount(self) }
  }
  pub fn pointer_id(self, pointer_index: usize) -> i32 {
    unsafe { AMotionEvent_getPointerId(self, pointer_index) }
  }
  pub fn tool_type(self, pointer_index: usize) -> i32 {
    unsafe { AMotionEvent_getToolType(self, pointer_index) }
  }
  pub fn raw_x(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getRawX(self, pointer_index) }
  }
  pub fn raw_y(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getRawY(self, pointer_index) }
  }
  pub fn x(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getX(self, pointer_index) }
  }
  pub fn y(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getY(self, pointer_index) }
  }
  pub fn pressure(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getPressure(self, pointer_index) }
  }
  pub fn size(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getSize(self, pointer_index) }
  }
  pub fn touch_major(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getTouchMajor(self, pointer_index) }
  }
  pub fn touch_minor(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getTouchMinor(self, pointer_index) }
  }
  pub fn tool_major(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getToolMajor(self, pointer_index) }
  }
  pub fn tool_minor(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getToolMinor(self, pointer_index) }
  }
  pub fn orientation(self, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getOrientation(self, pointer_index) }
  }
  pub fn axis_value(self, axis: i32, pointer_index: usize) -> f32 {
    unsafe { AMotionEvent_getAxisValue(self, axis, pointer_index) }
  }
  pub fn history_size(self) -> usize {
    unsafe { AMotionEvent_getHistorySize(self) }
  }
  pub fn historical_event_time(self, history_index: usize) -> i64 {
    unsafe { AMotionEvent_getHistoricalEventTime(self, history_index) }
  }
  pub fn historical_raw_x(self, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalRawX(self, pointer_index, history_index) }
  }
  pub fn historical_raw_y(self, pointer_index: usize,history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalRawY(self, pointer_index, history_index) }
  }
  pub fn historical_x(self, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalX(self, pointer_index, history_index) }
  }
  pub fn historical_y(self, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalY(self, pointer_index, history_index) }
  }
  pub fn historical_pressure(self, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalPressure(self, pointer_index, history_index) }
  }
  pub fn historical_size(self, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalSize(self, pointer_index, history_index) }
  }
  pub fn historical_touch_major(self, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalTouchMajor(self, pointer_index, history_index) }
  }
  pub fn historical_touch_minor(self, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalTouchMinor(self, pointer_index, history_index) }
  }
  pub fn historical_tool_major(self, pointer_index: usize,history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalToolMajor(self, pointer_index, history_index) }
  }
  pub fn historical_tool_minor(self, pointer_index: usize,history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalToolMinor(self, pointer_index, history_index) }
  }
  pub fn historical_orientation(self, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalOrientation(self, pointer_index, history_index) }
  }
  pub fn historical_axis_value(self, axis: i32, pointer_index: usize, history_index: usize) -> f32 {
    unsafe { AMotionEvent_getHistoricalAxisValue(self, axis, pointer_index, history_index) }
  }
}

impl InputEvent {
  pub fn typ3(self) -> i32 {
    unsafe { AInputEvent_getType(self) }
  }
  pub fn device_id(self) -> i32 {
    unsafe { AInputEvent_getDeviceId(self) }
  }
  pub fn source(self) -> i32 {
    unsafe { AInputEvent_getSource(self) }
  }
}

extern "C" {
  fn AInputEvent_getType(event: InputEvent) -> i32;
  fn AInputEvent_getDeviceId(event: InputEvent) -> i32;
  fn AInputEvent_getSource(event: InputEvent) -> i32;
}
extern "C" {
  fn AKeyEvent_getAction(key_event: KeyEvent) -> i32;
  fn AKeyEvent_getFlags(key_event: KeyEvent) -> i32;
  fn AKeyEvent_getKeyCode(key_event: KeyEvent) -> i32;
  fn AKeyEvent_getScanCode(key_event: KeyEvent) -> i32;
  fn AKeyEvent_getMetaState(key_event: KeyEvent) -> i32;
  fn AKeyEvent_getRepeatCount(key_event: KeyEvent) -> i32;
  fn AKeyEvent_getDownTime(key_event: KeyEvent) -> i64;
  fn AKeyEvent_getEventTime(key_event: KeyEvent) -> i64;
}
extern "C" {
  fn AMotionEvent_getAction(motion_event: MotionEvent) -> i32;
  fn AMotionEvent_getFlags(motion_event: MotionEvent) -> i32;
  fn AMotionEvent_getMetaState(motion_event: MotionEvent) -> i32;
  fn AMotionEvent_getButtonState(motion_event: MotionEvent) -> i32;
  fn AMotionEvent_getEdgeFlags(motion_event: MotionEvent) -> i32;
  fn AMotionEvent_getDownTime(motion_event: MotionEvent) -> i64;
  fn AMotionEvent_getEventTime(motion_event: MotionEvent) -> i64;
  fn AMotionEvent_getXOffset(motion_event: MotionEvent) -> f32;
  fn AMotionEvent_getYOffset(motion_event: MotionEvent) -> f32;
  fn AMotionEvent_getXPrecision(motion_event: MotionEvent) -> f32;
  fn AMotionEvent_getYPrecision(motion_event: MotionEvent) -> f32;
  fn AMotionEvent_getPointerCount(motion_event: MotionEvent) -> usize;
  fn AMotionEvent_getPointerId(motion_event: MotionEvent, pointer_index: usize) -> i32;
  fn AMotionEvent_getToolType(motion_event: MotionEvent, pointer_index: usize) -> i32;
  fn AMotionEvent_getRawX(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getRawY(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getX(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getY(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getPressure(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getSize(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getTouchMajor(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getTouchMinor(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getToolMajor(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getToolMinor(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getOrientation(motion_event: MotionEvent, pointer_index: usize) -> f32;
  fn AMotionEvent_getAxisValue(motion_event: MotionEvent, axis: i32, pointer_index: usize) -> f32;
  fn AMotionEvent_getHistorySize(motion_event: MotionEvent) -> usize;
  fn AMotionEvent_getHistoricalEventTime(motion_event: MotionEvent, history_index: usize) -> i64;
  fn AMotionEvent_getHistoricalRawX(motion_event: MotionEvent, pointer_index: usize, history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalRawY(motion_event: MotionEvent, pointer_index: usize,history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalX(motion_event: MotionEvent, pointer_index: usize, history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalY(motion_event: MotionEvent, pointer_index: usize, history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalPressure(motion_event: MotionEvent, pointer_index: usize, history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalSize(motion_event: MotionEvent, pointer_index: usize, history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalTouchMajor(motion_event: MotionEvent, pointer_index: usize, history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalTouchMinor(motion_event: MotionEvent, pointer_index: usize, history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalToolMajor(motion_event: MotionEvent, pointer_index: usize,history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalToolMinor(motion_event: MotionEvent, pointer_index: usize,history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalOrientation(motion_event: MotionEvent, pointer_index: usize, history_index: usize) -> f32;
  fn AMotionEvent_getHistoricalAxisValue(motion_event: MotionEvent, axis: i32, pointer_index: usize, history_index: usize) -> f32;
}

