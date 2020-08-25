
pub use crate::native_window::*;
pub use crate::input_queue::*;

pub use crate::log::*;
pub struct AAssetManager(usize);


pub use std::thread;
pub use std::sync::mpsc;

macro_rules! alog {
    ($($arg:tt)*) => {
        a_log(format!($($arg)*));
    };
}

struct AndroidApp {
  msg: mpsc::Receiver<MainThreadMsg>,
  window: NativeWindow,
  queue: InputQueue,
  looper: Looper
}

#[repr(C)]
pub struct ANativeActivity<'a> {
    pub callbacks: &'a mut ANativeActivityCallbacks,
    pub vm: usize,
    pub env: usize,
    pub clazz: usize,
    pub internal_data_path: usize,
    pub external_data_path: usize,
    pub sdk_version: i32,
    pub instance: std::mem::ManuallyDrop<Box<mpsc::Sender<MainThreadMsg>>>,
    pub asset_manager: usize,
    pub obb_path: usize,
}

  // 8  // struct ANativeActivityCallbacks* callbacks;
  // 8  // JavaVM* vm;
  // 8  // JNIEnv* env;
  // 8  // jobject clazz;
  // 8  // const char* internalDataPath;
  // 8  // const char* externalDataPath;
  // 4  // int32_t sdkVersion;
  // 8  // void* instance;
  // 8  // AAssetManager* assetManager;
  // 8  // const char* obbPath;

#[repr(C)]
pub struct ANativeActivityCallbacks {
    pub start: fn(&mut ANativeActivity),
    pub resume: fn(&mut ANativeActivity),
    pub save_instance_state: fn(&mut ANativeActivity, &'static mut usize),
    pub pause: fn(&mut ANativeActivity),
    pub stop: fn(&mut ANativeActivity),
    pub destroy: fn(&mut ANativeActivity),
    pub window_focus_changed: fn(&mut ANativeActivity, i32),
    pub native_window_created: fn(&mut ANativeActivity, NativeWindow),
    pub native_window_resized: fn(&mut ANativeActivity, NativeWindow),
    pub native_window_redraw_needed: fn(&mut ANativeActivity, NativeWindow),
    pub native_window_destroyed: fn(&mut ANativeActivity, NativeWindow),
    pub input_queue_created: fn(&mut ANativeActivity, InputQueue),
    pub input_queue_destroyed: fn(&mut ANativeActivity, InputQueue),
    pub content_rect_changed: fn(&mut ANativeActivity, &[i32;4]),
    pub configuration_changed: fn(&mut ANativeActivity),
    pub low_memory: fn(&mut ANativeActivity),
}

pub enum MainThreadMsg {
  Start(),
  Resume(),
  SaveInstanceState(&'static mut usize),
  Pause(),
  Stop(),
  Destroy(),
  WindowFocusChanged(i32),
  NativeWindowCreated(NativeWindow),
  NativeWindowResized(NativeWindow),
  NativeWindowRedrawNeeded(NativeWindow),
  NativeWindowDestroyed(NativeWindow),
  InputQueueCreated(InputQueue),
  InputQueueDestroyed(InputQueue),
  ContentRectChanged([i32;4]),
  ConfigurationChanged(),
  LowMemory(),
}

impl MainThreadMsg {
  pub fn as_str(&self) -> &str {
    match self {
      MainThreadMsg::Start() => "Start",
      MainThreadMsg::Resume() => "Resume",
      MainThreadMsg::SaveInstanceState(_) => "SaveInstanceState",
      MainThreadMsg::Pause() => "Pause",
      MainThreadMsg::Stop() => "Stop",
      MainThreadMsg::Destroy() => "Destroy",
      MainThreadMsg::WindowFocusChanged(_) => "WindowFocusChanged",
      MainThreadMsg::NativeWindowCreated(_) => "NativeWindowCreated",
      MainThreadMsg::NativeWindowResized(_) => "NativeWindowResized",
      MainThreadMsg::NativeWindowRedrawNeeded(_) => "NativeWindowRedrawNeeded",
      MainThreadMsg::NativeWindowDestroyed(_) => "NativeWindowDestroyed",
      MainThreadMsg::InputQueueCreated(_) => "InputQueueCreated",
      MainThreadMsg::InputQueueDestroyed(_) => "InputQueueDestroyed",
      MainThreadMsg::ContentRectChanged(_) => "ContentRectChanged",
      MainThreadMsg::ConfigurationChanged() => "ConfigurationChanged",
      MainThreadMsg::LowMemory() => "LowMemory",
    }
  }
}

#[no_mangle]
pub unsafe extern fn android_activity_entry(activity: &mut ANativeActivity) {
  alog!("Entering main");
  alog!("{:?}", std::mem::size_of::<ANativeActivity>());
  alog!("{:?}", std::mem::size_of::<Box<mpsc::Sender<MainThreadMsg>>>());
  activity.callbacks.start = start;
  activity.callbacks.resume = resume;
  activity.callbacks.save_instance_state = save_instance_state;
  activity.callbacks.pause = pause;
  activity.callbacks.stop = stop;
  activity.callbacks.destroy = destroy;
  activity.callbacks.window_focus_changed = window_focus_changed;
  activity.callbacks.native_window_created = native_window_created;
  activity.callbacks.native_window_resized = native_window_resized;
  activity.callbacks.native_window_redraw_needed = native_window_redraw_needed;
  activity.callbacks.native_window_destroyed = native_window_destroyed;
  activity.callbacks.input_queue_created = input_queue_created;
  activity.callbacks.input_queue_destroyed = input_queue_destroyed;
  activity.callbacks.content_rect_changed = content_rect_changed;
  activity.callbacks.configuration_changed = configuration_changed;
  activity.callbacks.low_memory = low_memory;
  alog!("Calling rust main");
  let (tx, rx) = mpsc::channel();
  //std::mem::forget(activity.instance);
  activity.instance = std::mem::ManuallyDrop::new(Box::new(tx));
  alog!("Spawning thread");
  std::thread::spawn(|| {
    crate::rust_main(rx);
  });
}

fn start(activity: &mut ANativeActivity) {
  alog!("start");
  activity.instance.send(MainThreadMsg::Start()).unwrap();
}
fn resume(activity: &mut ANativeActivity) {
  alog!("resume");
  activity.instance.send(MainThreadMsg::Resume()).unwrap();
}
fn save_instance_state(activity: &mut ANativeActivity, size: &'static mut usize) {
  alog!("save_instance_state");
  activity.instance.send(MainThreadMsg::SaveInstanceState(size)).unwrap();
}
fn pause(activity: &mut ANativeActivity) {
  alog!("pause");
  activity.instance.send(MainThreadMsg::Pause()).unwrap();
}
fn stop(activity: &mut ANativeActivity) {
  alog!("stop");
  activity.instance.send(MainThreadMsg::Stop()).unwrap();
}
fn destroy(activity: &mut ANativeActivity) {
  alog!("destroy");
  activity.instance.send(MainThreadMsg::Destroy()).unwrap();
}
fn window_focus_changed(activity: &mut ANativeActivity, focus: i32) {
  alog!("window_focus_changed");
  activity.instance.send(MainThreadMsg::WindowFocusChanged(focus)).unwrap();
}
fn native_window_created(activity: &mut ANativeActivity, window: NativeWindow) {
  alog!("native_window_created");
  activity.instance.send(MainThreadMsg::NativeWindowCreated(window)).unwrap();
}
fn native_window_resized(activity: &mut ANativeActivity, window: NativeWindow) {
  alog!("native_window_resized");
  activity.instance.send(MainThreadMsg::NativeWindowResized(window)).unwrap();
}
fn native_window_redraw_needed(activity: &mut ANativeActivity, window: NativeWindow) {
  alog!("native_window_redraw_needed");
  activity.instance.send(MainThreadMsg::NativeWindowRedrawNeeded(window)).unwrap();
}
fn native_window_destroyed(activity: &mut ANativeActivity, window: NativeWindow) {
  alog!("native_window_destroyed");
  activity.instance.send(MainThreadMsg::NativeWindowDestroyed(window)).unwrap();
}
fn input_queue_created(activity: &mut ANativeActivity, queue: InputQueue) {
  alog!("input_queue_created");
  alog!("Sending queue {:?}", queue);
  activity.instance.send(MainThreadMsg::InputQueueCreated(queue)).unwrap();
}
fn input_queue_destroyed(activity: &mut ANativeActivity, queue: InputQueue) {
  alog!("input_queue_destroyed");
  activity.instance.send(MainThreadMsg::InputQueueDestroyed(queue)).unwrap();
}
fn content_rect_changed(activity: &mut ANativeActivity, rect: &[i32;4]) {
  alog!("content_rect_changed");
  activity.instance.send(MainThreadMsg::ContentRectChanged(rect.clone())).unwrap();
}
fn configuration_changed(activity: &mut ANativeActivity) {
  alog!("configuration_changed");
  activity.instance.send(MainThreadMsg::ConfigurationChanged()).unwrap();
}
fn low_memory(activity: &mut ANativeActivity) {
  alog!("low_memory");
  activity.instance.send(MainThreadMsg::LowMemory()).unwrap();
}

  // activity.callbacks.start = |act| act.instance.send(MainThreadMsg::Start()).unwrap();
  // activity.callbacks.resume = |act| act.instance.send(MainThreadMsg::Resume()).unwrap();
  // activity.callbacks.save_instance_state = |act, size| act.instance.send(MainThreadMsg::SaveInstanceState(size)).unwrap();
  // activity.callbacks.pause = |act| act.instance.send(MainThreadMsg::Pause()).unwrap();
  // activity.callbacks.stop = |act| act.instance.send(MainThreadMsg::Stop()).unwrap();
  // activity.callbacks.destroy = |act| act.instance.send(MainThreadMsg::Destroy()).unwrap();
  // activity.callbacks.window_focus_changed = |act, focus| act.instance.send(MainThreadMsg::WindowFocusChanged(focus)).unwrap();
  // activity.callbacks.native_window_created = |act, window| act.instance.send(MainThreadMsg::NativeWindowCreated(window)).unwrap();
  // activity.callbacks.native_window_resized = |act, window| act.instance.send(MainThreadMsg::NativeWindowResized(window)).unwrap();
  // activity.callbacks.native_window_redraw_needed = |act, window| act.instance.send(MainThreadMsg::NativeWindowRedrawNeeded(window)).unwrap();
  // activity.callbacks.native_window_destroyed = |act, window| act.instance.send(MainThreadMsg::NativeWindowDestroyed(window)).unwrap();
  // activity.callbacks.input_queue_created = |act, queue| act.instance.send(MainThreadMsg::InputQueueCreated(queue)).unwrap();
  // activity.callbacks.input_queue_destroyed = |act, queue| act.instance.send(MainThreadMsg::InputQueueDestroyed(queue)).unwrap();
  // activity.callbacks.content_rect_changed = |act, rect| act.instance.send(MainThreadMsg::ContentRectChanged(rect.clone())).unwrap();
  // activity.callbacks.configuration_changed = |act| act.instance.send(MainThreadMsg::ConfigurationChanged()).unwrap();
  // activity.callbacks.low_memory = |act| act.instance.send(MainThreadMsg::LowMemory()).unwrap();