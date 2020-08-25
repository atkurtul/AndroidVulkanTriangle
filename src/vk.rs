#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types, dead_code, improper_ctypes)]
pub use std::ptr::null;
pub use std::ptr::null_mut;
pub use std::ffi::c_void;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct IndirectCommandsLayoutNV(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PerformanceConfigurationINTEL(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ValidationCacheEXT(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DebugUtilsMessengerEXT(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DisplayModeKHR(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DisplayKHR(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DescriptorSetLayout(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DeviceMemory(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PipelineCache(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PrivateDataSlotEXT(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Event(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CommandBuffer(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct AccelerationStructureKHR(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Buffer(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct BufferView(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Fence(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RenderPass(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SamplerYcbcrConversion(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Queue(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DebugReportCallbackEXT(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct QueryPool(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Device(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PhysicalDevice(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Semaphore(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ImageView(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Instance(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ShaderModule(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PipelineLayout(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Image(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Sampler(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DescriptorSet(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DescriptorPool(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DescriptorUpdateTemplate(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Framebuffer(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Pipeline(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CommandPool(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SwapchainKHR(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SurfaceKHR(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct HANDLE(pub usize);
pub type RROutput = u64;
pub type xcb_visualid_t = u32;
pub type xcb_window_t = u32;
pub type CAMetalLayer = c_void;
pub type ANativeWindow = c_void;
pub type xcb_connection_t = c_void;
pub type VisualID = u32;
pub type Display = c_void;
pub type AHardwareBuffer = c_void;
pub type HMONITOR = *const c_void;
pub type wl_surface = c_void;
pub type wl_display = c_void;
pub type HWND = *const c_void;
pub type SECURITY_ATTRIBUTES = c_void;
pub type HINSTANCE = *const c_void;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PrivateDataSlotCreateFlagsEXT(pub i32);
impl PrivateDataSlotCreateFlagsEXT {

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PrivateDataSlotCreateFlagsEXT {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectCommandsLayoutUsageFlagsNV(pub i32);
impl IndirectCommandsLayoutUsageFlagsNV {
	pub const EXPLICIT_PREPROCESS_NV : Self = Self(0x00000001);
	pub const INDEXED_SEQUENCES_NV : Self = Self(0x00000002);
	pub const UNORDERED_SEQUENCES_NV : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for IndirectCommandsLayoutUsageFlagsNV {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectStateFlagsNV(pub i32);
impl IndirectStateFlagsNV {
	pub const FLAG_FRONTFACE_NV : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for IndirectStateFlagsNV {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectCommandsTokenTypeNV(pub i32);
impl IndirectCommandsTokenTypeNV {
	pub const SHADER_GROUP_NV : Self = Self(0);
	pub const STATE_FLAGS_NV : Self = Self(1);
	pub const INDEX_BUFFER_NV : Self = Self(2);
	pub const VERTEX_BUFFER_NV : Self = Self(3);
	pub const PUSH_CONSTANT_NV : Self = Self(4);
	pub const DRAW_INDEXED_NV : Self = Self(5);
	pub const DRAW_NV : Self = Self(6);
	pub const DRAW_TASKS_NV : Self = Self(7);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CoverageReductionModeNV(pub i32);
impl CoverageReductionModeNV {
	pub const MERGE_NV : Self = Self(0);
	pub const TRUNCATE_NV : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationFeatureEnableEXT(pub i32);
impl ValidationFeatureEnableEXT {
	pub const GPU_ASSISTED_EXT : Self = Self(0);
	pub const GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT : Self = Self(1);
	pub const BEST_PRACTICES_EXT : Self = Self(2);
	pub const DEBUG_PRINTF_EXT : Self = Self(3);
	pub const SYNCHRONIZATION_VALIDATION_EXT : Self = Self(4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceValueTypeINTEL(pub i32);
impl PerformanceValueTypeINTEL {
	pub const UINT32_INTEL : Self = Self(0);
	pub const UINT64_INTEL : Self = Self(1);
	pub const FLOAT_INTEL : Self = Self(2);
	pub const BOOL_INTEL : Self = Self(3);
	pub const STRING_INTEL : Self = Self(4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceParameterTypeINTEL(pub i32);
impl PerformanceParameterTypeINTEL {
	pub const HW_COUNTERS_SUPPORTED_INTEL : Self = Self(0);
	pub const STREAM_MARKER_VALIDS_INTEL : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueryPoolSamplingModeINTEL(pub i32);
impl QueryPoolSamplingModeINTEL {
	pub const MANUAL_INTEL : Self = Self(0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ComponentTypeNV(pub i32);
impl ComponentTypeNV {
	pub const FLOAT16_NV : Self = Self(0);
	pub const FLOAT32_NV : Self = Self(1);
	pub const FLOAT64_NV : Self = Self(2);
	pub const SINT8_NV : Self = Self(3);
	pub const SINT16_NV : Self = Self(4);
	pub const SINT32_NV : Self = Self(5);
	pub const SINT64_NV : Self = Self(6);
	pub const UINT8_NV : Self = Self(7);
	pub const UINT16_NV : Self = Self(8);
	pub const UINT32_NV : Self = Self(9);
	pub const UINT64_NV : Self = Self(10);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineCreationFeedbackFlagsEXT(pub i32);
impl PipelineCreationFeedbackFlagsEXT {
	pub const VALID_EXT : Self = Self(0x00000001);
	pub const APPLICATION_PIPELINE_CACHE_HIT_EXT : Self = Self(0x00000002);
	pub const BASE_PIPELINE_ACCELERATION_EXT : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PipelineCreationFeedbackFlagsEXT {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FullScreenExclusiveEXT(pub i32);
impl FullScreenExclusiveEXT {
	pub const DEFAULT_EXT : Self = Self(0);
	pub const ALLOWED_EXT : Self = Self(1);
	pub const DISALLOWED_EXT : Self = Self(2);
	pub const APPLICATION_CONTROLLED_EXT : Self = Self(3);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MemoryOverallocationBehaviorAMD(pub i32);
impl MemoryOverallocationBehaviorAMD {
	pub const DEFAULT_AMD : Self = Self(0);
	pub const ALLOWED_AMD : Self = Self(1);
	pub const DISALLOWED_AMD : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TimeDomainEXT(pub i32);
impl TimeDomainEXT {
	pub const DEVICE_EXT : Self = Self(0);
	pub const CLOCK_MONOTONIC_EXT : Self = Self(1);
	pub const CLOCK_MONOTONIC_RAW_EXT : Self = Self(2);
	pub const QUERY_PERFORMANCE_COUNTER_EXT : Self = Self(3);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineCompilerControlFlagsAMD(pub i32);
impl PipelineCompilerControlFlagsAMD {

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PipelineCompilerControlFlagsAMD {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueueGlobalPriorityEXT(pub i32);
impl QueueGlobalPriorityEXT {
	pub const LOW_EXT : Self = Self(128);
	pub const MEDIUM_EXT : Self = Self(256);
	pub const HIGH_EXT : Self = Self(512);
	pub const REALTIME_EXT : Self = Self(1024);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BuildAccelerationStructureFlagsKHR(pub i32);
impl BuildAccelerationStructureFlagsKHR {
	pub const ALLOW_UPDATE_NV : Self = Self(0x00000001);
	pub const ALLOW_UPDATE_KHR : Self = Self(0x00000001);
	pub const ALLOW_COMPACTION_NV : Self = Self(0x00000002);
	pub const ALLOW_COMPACTION_KHR : Self = Self(0x00000002);
	pub const PREFER_FAST_TRACE_NV : Self = Self(0x00000004);
	pub const PREFER_FAST_TRACE_KHR : Self = Self(0x00000004);
	pub const PREFER_FAST_BUILD_NV : Self = Self(0x00000008);
	pub const PREFER_FAST_BUILD_KHR : Self = Self(0x00000008);
	pub const LOW_MEMORY_NV : Self = Self(0x00000010);
	pub const LOW_MEMORY_KHR : Self = Self(0x00000010);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for BuildAccelerationStructureFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GeometryInstanceFlagsKHR(pub i32);
impl GeometryInstanceFlagsKHR {
	pub const TRIANGLE_CULL_DISABLE_NV : Self = Self(0x00000001);
	pub const TRIANGLE_FACING_CULL_DISABLE_KHR : Self = Self(0x00000001);
	pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV : Self = Self(0x00000002);
	pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR : Self = Self(0x00000002);
	pub const FORCE_OPAQUE_NV : Self = Self(0x00000004);
	pub const FORCE_OPAQUE_KHR : Self = Self(0x00000004);
	pub const FORCE_NO_OPAQUE_NV : Self = Self(0x00000008);
	pub const FORCE_NO_OPAQUE_KHR : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for GeometryInstanceFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GeometryFlagsKHR(pub i32);
impl GeometryFlagsKHR {
	pub const OPAQUE_NV : Self = Self(0x00000001);
	pub const OPAQUE_KHR : Self = Self(0x00000001);
	pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV : Self = Self(0x00000002);
	pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for GeometryFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccelerationStructureMemoryRequirementsTypeKHR(pub i32);
impl AccelerationStructureMemoryRequirementsTypeKHR {
	pub const OBJECT_NV : Self = Self(0);
	pub const OBJECT_KHR : Self = Self(0);
	pub const BUILD_SCRATCH_NV : Self = Self(1);
	pub const BUILD_SCRATCH_KHR : Self = Self(1);
	pub const UPDATE_SCRATCH_NV : Self = Self(2);
	pub const UPDATE_SCRATCH_KHR : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CopyAccelerationStructureModeKHR(pub i32);
impl CopyAccelerationStructureModeKHR {
	pub const CLONE_NV : Self = Self(0);
	pub const CLONE_KHR : Self = Self(0);
	pub const COMPACT_NV : Self = Self(1);
	pub const COMPACT_KHR : Self = Self(1);
	pub const SERIALIZE_KHR : Self = Self(2);
	pub const DESERIALIZE_KHR : Self = Self(3);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GeometryTypeKHR(pub i32);
impl GeometryTypeKHR {
	pub const TRIANGLES_NV : Self = Self(0);
	pub const TRIANGLES_KHR : Self = Self(0);
	pub const AABBS_KHR : Self = Self(1);
	pub const AABBS_NV : Self = Self(1);
	pub const INSTANCES_KHR : Self = Self(1000150000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RayTracingShaderGroupTypeKHR(pub i32);
impl RayTracingShaderGroupTypeKHR {
	pub const GENERAL_NV : Self = Self(0);
	pub const GENERAL_KHR : Self = Self(0);
	pub const TRIANGLES_HIT_GROUP_KHR : Self = Self(1);
	pub const TRIANGLES_HIT_GROUP_NV : Self = Self(1);
	pub const PROCEDURAL_HIT_GROUP_NV : Self = Self(2);
	pub const PROCEDURAL_HIT_GROUP_KHR : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShadingRatePaletteEntryNV(pub i32);
impl ShadingRatePaletteEntryNV {
	pub const NO_INVOCATIONS_NV : Self = Self(0);
	pub const e16_INVOCATIONS_PER_PIXEL_NV : Self = Self(1);
	pub const e8_INVOCATIONS_PER_PIXEL_NV : Self = Self(2);
	pub const e4_INVOCATIONS_PER_PIXEL_NV : Self = Self(3);
	pub const e2_INVOCATIONS_PER_PIXEL_NV : Self = Self(4);
	pub const e1_INVOCATION_PER_PIXEL_NV : Self = Self(5);
	pub const e1_INVOCATION_PER_2X1_PIXELS_NV : Self = Self(6);
	pub const e1_INVOCATION_PER_1X2_PIXELS_NV : Self = Self(7);
	pub const e1_INVOCATION_PER_2X2_PIXELS_NV : Self = Self(8);
	pub const e1_INVOCATION_PER_4X2_PIXELS_NV : Self = Self(9);
	pub const e1_INVOCATION_PER_2X4_PIXELS_NV : Self = Self(10);
	pub const e1_INVOCATION_PER_4X4_PIXELS_NV : Self = Self(11);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationCacheHeaderVersionEXT(pub i32);
impl ValidationCacheHeaderVersionEXT {
	pub const ONE_EXT : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CoverageModulationModeNV(pub i32);
impl CoverageModulationModeNV {
	pub const NONE_NV : Self = Self(0);
	pub const RGB_NV : Self = Self(1);
	pub const ALPHA_NV : Self = Self(2);
	pub const RGBA_NV : Self = Self(3);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DebugUtilsMessageSeverityFlagsEXT(pub i32);
impl DebugUtilsMessageSeverityFlagsEXT {
	pub const VERBOSE_EXT : Self = Self(0x00000001);
	pub const INFO_EXT : Self = Self(0x00000010);
	pub const WARNING_EXT : Self = Self(0x00000100);
	pub const ERROR_EXT : Self = Self(0x00001000);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DebugUtilsMessageSeverityFlagsEXT {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShaderModuleCreateFlags(pub i32);
impl ShaderModuleCreateFlags {

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ShaderModuleCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BufferCreateFlags(pub i32);
impl BufferCreateFlags {
	pub const SPARSE_BINDING : Self = Self(0x00000001);
	pub const SPARSE_RESIDENCY : Self = Self(0x00000002);
	pub const SPARSE_ALIASED : Self = Self(0x00000004);
	pub const PROTECTED : Self = Self(0x00000008);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR : Self = Self(0x00000010);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT : Self = Self(0x00000010);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY : Self = Self(0x00000010);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for BufferCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SemaphoreWaitFlags(pub i32);
impl SemaphoreWaitFlags {
	pub const ANY_KHR : Self = Self(0x00000001);
	pub const ANY : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SemaphoreWaitFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueryPipelineStatisticFlags(pub i32);
impl QueryPipelineStatisticFlags {
	pub const INPUT_ASSEMBLY_VERTICES : Self = Self(0x00000001);
	pub const INPUT_ASSEMBLY_PRIMITIVES : Self = Self(0x00000002);
	pub const VERTEX_SHADER_INVOCATIONS : Self = Self(0x00000004);
	pub const GEOMETRY_SHADER_INVOCATIONS : Self = Self(0x00000008);
	pub const GEOMETRY_SHADER_PRIMITIVES : Self = Self(0x00000010);
	pub const CLIPPING_INVOCATIONS : Self = Self(0x00000020);
	pub const CLIPPING_PRIMITIVES : Self = Self(0x00000040);
	pub const FRAGMENT_SHADER_INVOCATIONS : Self = Self(0x00000080);
	pub const TESSELLATION_CONTROL_SHADER_PATCHES : Self = Self(0x00000100);
	pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS : Self = Self(0x00000200);
	pub const COMPUTE_SHADER_INVOCATIONS : Self = Self(0x00000400);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for QueryPipelineStatisticFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FenceCreateFlags(pub i32);
impl FenceCreateFlags {
	pub const SIGNALED : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for FenceCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageViewCreateFlags(pub i32);
impl ImageViewCreateFlags {
	pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT : Self = Self(0x00000001);
	pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ImageViewCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PrimitiveTopology(pub i32);
impl PrimitiveTopology {
	pub const POINT_LIST : Self = Self(0);
	pub const LINE_LIST : Self = Self(1);
	pub const LINE_STRIP : Self = Self(2);
	pub const TRIANGLE_LIST : Self = Self(3);
	pub const TRIANGLE_STRIP : Self = Self(4);
	pub const TRIANGLE_FAN : Self = Self(5);
	pub const LINE_LIST_WITH_ADJACENCY : Self = Self(6);
	pub const LINE_STRIP_WITH_ADJACENCY : Self = Self(7);
	pub const TRIANGLE_LIST_WITH_ADJACENCY : Self = Self(8);
	pub const TRIANGLE_STRIP_WITH_ADJACENCY : Self = Self(9);
	pub const PATCH_LIST : Self = Self(10);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SamplerReductionMode(pub i32);
impl SamplerReductionMode {
	pub const WEIGHTED_AVERAGE_EXT : Self = Self(0);
	pub const WEIGHTED_AVERAGE : Self = Self(0);
	pub const MIN_EXT : Self = Self(1);
	pub const MIN : Self = Self(1);
	pub const MAX_EXT : Self = Self(2);
	pub const MAX : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DebugReportObjectTypeEXT(pub i32);
impl DebugReportObjectTypeEXT {
	pub const UNKNOWN_EXT : Self = Self(0);
	pub const INSTANCE_EXT : Self = Self(1);
	pub const PHYSICAL_DEVICE_EXT : Self = Self(2);
	pub const DEVICE_EXT : Self = Self(3);
	pub const QUEUE_EXT : Self = Self(4);
	pub const SEMAPHORE_EXT : Self = Self(5);
	pub const COMMAND_BUFFER_EXT : Self = Self(6);
	pub const FENCE_EXT : Self = Self(7);
	pub const DEVICE_MEMORY_EXT : Self = Self(8);
	pub const BUFFER_EXT : Self = Self(9);
	pub const IMAGE_EXT : Self = Self(10);
	pub const EVENT_EXT : Self = Self(11);
	pub const QUERY_POOL_EXT : Self = Self(12);
	pub const BUFFER_VIEW_EXT : Self = Self(13);
	pub const IMAGE_VIEW_EXT : Self = Self(14);
	pub const SHADER_MODULE_EXT : Self = Self(15);
	pub const PIPELINE_CACHE_EXT : Self = Self(16);
	pub const PIPELINE_LAYOUT_EXT : Self = Self(17);
	pub const RENDER_PASS_EXT : Self = Self(18);
	pub const PIPELINE_EXT : Self = Self(19);
	pub const DESCRIPTOR_SET_LAYOUT_EXT : Self = Self(20);
	pub const SAMPLER_EXT : Self = Self(21);
	pub const DESCRIPTOR_POOL_EXT : Self = Self(22);
	pub const DESCRIPTOR_SET_EXT : Self = Self(23);
	pub const FRAMEBUFFER_EXT : Self = Self(24);
	pub const COMMAND_POOL_EXT : Self = Self(25);
	pub const SURFACE_KHR_EXT : Self = Self(26);
	pub const SWAPCHAIN_KHR_EXT : Self = Self(27);
	pub const DEBUG_REPORT_EXT : Self = Self(28);
	pub const DEBUG_REPORT_CALLBACK_EXT_EXT : Self = Self(28);
	pub const DISPLAY_KHR_EXT : Self = Self(29);
	pub const DISPLAY_MODE_KHR_EXT : Self = Self(30);
	pub const VALIDATION_CACHE_EXT : Self = Self(33);
	pub const VALIDATION_CACHE_EXT_EXT : Self = Self(33);
	pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT : Self = Self(1000085000);
	pub const DESCRIPTOR_UPDATE_TEMPLATE_EXT : Self = Self(1000085000);
	pub const SAMPLER_YCBCR_CONVERSION_KHR_EXT : Self = Self(1000156000);
	pub const SAMPLER_YCBCR_CONVERSION_EXT : Self = Self(1000156000);
	pub const ACCELERATION_STRUCTURE_NV_EXT : Self = Self(1000165000);
	pub const ACCELERATION_STRUCTURE_KHR_EXT : Self = Self(1000165000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceQueueCreateFlags(pub i32);
impl DeviceQueueCreateFlags {
	pub const PROTECTED : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DeviceQueueCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MemoryPropertyFlags(pub i32);
impl MemoryPropertyFlags {
	pub const DEVICE_LOCAL : Self = Self(0x00000001);
	pub const HOST_VISIBLE : Self = Self(0x00000002);
	pub const HOST_COHERENT : Self = Self(0x00000004);
	pub const HOST_CACHED : Self = Self(0x00000008);
	pub const LAZILY_ALLOCATED : Self = Self(0x00000010);
	pub const PROTECTED : Self = Self(0x00000020);
	pub const DEVICE_COHERENT_AMD : Self = Self(0x00000040);
	pub const DEVICE_UNCACHED_AMD : Self = Self(0x00000080);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for MemoryPropertyFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceCounterScopeKHR(pub i32);
impl PerformanceCounterScopeKHR {
	pub const QUERY_SCOPE_COMMAND_BUFFER_KHR : Self = Self(0);
	pub const COMMAND_BUFFER_KHR : Self = Self(0);
	pub const QUERY_SCOPE_RENDER_PASS_KHR : Self = Self(1);
	pub const RENDER_PASS_KHR : Self = Self(1);
	pub const QUERY_SCOPE_COMMAND_KHR : Self = Self(2);
	pub const COMMAND_KHR : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DiscardRectangleModeEXT(pub i32);
impl DiscardRectangleModeEXT {
	pub const INCLUSIVE_EXT : Self = Self(0);
	pub const EXCLUSIVE_EXT : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ColorComponentFlags(pub i32);
impl ColorComponentFlags {
	pub const R : Self = Self(0x00000001);
	pub const G : Self = Self(0x00000002);
	pub const B : Self = Self(0x00000004);
	pub const A : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ColorComponentFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SampleCountFlags(pub i32);
impl SampleCountFlags {
	pub const e1 : Self = Self(0x00000001);
	pub const e2 : Self = Self(0x00000002);
	pub const e4 : Self = Self(0x00000004);
	pub const e8 : Self = Self(0x00000008);
	pub const e16 : Self = Self(0x00000010);
	pub const e32 : Self = Self(0x00000020);
	pub const e64 : Self = Self(0x00000040);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SampleCountFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageAspectFlags(pub i32);
impl ImageAspectFlags {
	pub const COLOR : Self = Self(0x00000001);
	pub const DEPTH : Self = Self(0x00000002);
	pub const STENCIL : Self = Self(0x00000004);
	pub const METADATA : Self = Self(0x00000008);
	pub const PLANE_0_KHR : Self = Self(0x00000010);
	pub const PLANE_0 : Self = Self(0x00000010);
	pub const PLANE_1 : Self = Self(0x00000020);
	pub const PLANE_1_KHR : Self = Self(0x00000020);
	pub const PLANE_2_KHR : Self = Self(0x00000040);
	pub const PLANE_2 : Self = Self(0x00000040);
	pub const MEMORY_PLANE_0_EXT : Self = Self(0x00000080);
	pub const MEMORY_PLANE_1_EXT : Self = Self(0x00000100);
	pub const MEMORY_PLANE_2_EXT : Self = Self(0x00000200);
	pub const MEMORY_PLANE_3_EXT : Self = Self(0x00000400);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ImageAspectFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Format(pub i32);
impl Format {
	pub const UNDEFINED : Self = Self(0);
	pub const R4G4_UNORM_PACK8 : Self = Self(1);
	pub const R4G4B4A4_UNORM_PACK16 : Self = Self(2);
	pub const B4G4R4A4_UNORM_PACK16 : Self = Self(3);
	pub const R5G6B5_UNORM_PACK16 : Self = Self(4);
	pub const B5G6R5_UNORM_PACK16 : Self = Self(5);
	pub const R5G5B5A1_UNORM_PACK16 : Self = Self(6);
	pub const B5G5R5A1_UNORM_PACK16 : Self = Self(7);
	pub const A1R5G5B5_UNORM_PACK16 : Self = Self(8);
	pub const R8_UNORM : Self = Self(9);
	pub const R8_SNORM : Self = Self(10);
	pub const R8_USCALED : Self = Self(11);
	pub const R8_SSCALED : Self = Self(12);
	pub const R8_UINT : Self = Self(13);
	pub const R8_SINT : Self = Self(14);
	pub const R8_SRGB : Self = Self(15);
	pub const R8G8_UNORM : Self = Self(16);
	pub const R8G8_SNORM : Self = Self(17);
	pub const R8G8_USCALED : Self = Self(18);
	pub const R8G8_SSCALED : Self = Self(19);
	pub const R8G8_UINT : Self = Self(20);
	pub const R8G8_SINT : Self = Self(21);
	pub const R8G8_SRGB : Self = Self(22);
	pub const R8G8B8_UNORM : Self = Self(23);
	pub const R8G8B8_SNORM : Self = Self(24);
	pub const R8G8B8_USCALED : Self = Self(25);
	pub const R8G8B8_SSCALED : Self = Self(26);
	pub const R8G8B8_UINT : Self = Self(27);
	pub const R8G8B8_SINT : Self = Self(28);
	pub const R8G8B8_SRGB : Self = Self(29);
	pub const B8G8R8_UNORM : Self = Self(30);
	pub const B8G8R8_SNORM : Self = Self(31);
	pub const B8G8R8_USCALED : Self = Self(32);
	pub const B8G8R8_SSCALED : Self = Self(33);
	pub const B8G8R8_UINT : Self = Self(34);
	pub const B8G8R8_SINT : Self = Self(35);
	pub const B8G8R8_SRGB : Self = Self(36);
	pub const R8G8B8A8_UNORM : Self = Self(37);
	pub const R8G8B8A8_SNORM : Self = Self(38);
	pub const R8G8B8A8_USCALED : Self = Self(39);
	pub const R8G8B8A8_SSCALED : Self = Self(40);
	pub const R8G8B8A8_UINT : Self = Self(41);
	pub const R8G8B8A8_SINT : Self = Self(42);
	pub const R8G8B8A8_SRGB : Self = Self(43);
	pub const B8G8R8A8_UNORM : Self = Self(44);
	pub const B8G8R8A8_SNORM : Self = Self(45);
	pub const B8G8R8A8_USCALED : Self = Self(46);
	pub const B8G8R8A8_SSCALED : Self = Self(47);
	pub const B8G8R8A8_UINT : Self = Self(48);
	pub const B8G8R8A8_SINT : Self = Self(49);
	pub const B8G8R8A8_SRGB : Self = Self(50);
	pub const A8B8G8R8_UNORM_PACK32 : Self = Self(51);
	pub const A8B8G8R8_SNORM_PACK32 : Self = Self(52);
	pub const A8B8G8R8_USCALED_PACK32 : Self = Self(53);
	pub const A8B8G8R8_SSCALED_PACK32 : Self = Self(54);
	pub const A8B8G8R8_UINT_PACK32 : Self = Self(55);
	pub const A8B8G8R8_SINT_PACK32 : Self = Self(56);
	pub const A8B8G8R8_SRGB_PACK32 : Self = Self(57);
	pub const A2R10G10B10_UNORM_PACK32 : Self = Self(58);
	pub const A2R10G10B10_SNORM_PACK32 : Self = Self(59);
	pub const A2R10G10B10_USCALED_PACK32 : Self = Self(60);
	pub const A2R10G10B10_SSCALED_PACK32 : Self = Self(61);
	pub const A2R10G10B10_UINT_PACK32 : Self = Self(62);
	pub const A2R10G10B10_SINT_PACK32 : Self = Self(63);
	pub const A2B10G10R10_UNORM_PACK32 : Self = Self(64);
	pub const A2B10G10R10_SNORM_PACK32 : Self = Self(65);
	pub const A2B10G10R10_USCALED_PACK32 : Self = Self(66);
	pub const A2B10G10R10_SSCALED_PACK32 : Self = Self(67);
	pub const A2B10G10R10_UINT_PACK32 : Self = Self(68);
	pub const A2B10G10R10_SINT_PACK32 : Self = Self(69);
	pub const R16_UNORM : Self = Self(70);
	pub const R16_SNORM : Self = Self(71);
	pub const R16_USCALED : Self = Self(72);
	pub const R16_SSCALED : Self = Self(73);
	pub const R16_UINT : Self = Self(74);
	pub const R16_SINT : Self = Self(75);
	pub const R16_SFLOAT : Self = Self(76);
	pub const R16G16_UNORM : Self = Self(77);
	pub const R16G16_SNORM : Self = Self(78);
	pub const R16G16_USCALED : Self = Self(79);
	pub const R16G16_SSCALED : Self = Self(80);
	pub const R16G16_UINT : Self = Self(81);
	pub const R16G16_SINT : Self = Self(82);
	pub const R16G16_SFLOAT : Self = Self(83);
	pub const R16G16B16_UNORM : Self = Self(84);
	pub const R16G16B16_SNORM : Self = Self(85);
	pub const R16G16B16_USCALED : Self = Self(86);
	pub const R16G16B16_SSCALED : Self = Self(87);
	pub const R16G16B16_UINT : Self = Self(88);
	pub const R16G16B16_SINT : Self = Self(89);
	pub const R16G16B16_SFLOAT : Self = Self(90);
	pub const R16G16B16A16_UNORM : Self = Self(91);
	pub const R16G16B16A16_SNORM : Self = Self(92);
	pub const R16G16B16A16_USCALED : Self = Self(93);
	pub const R16G16B16A16_SSCALED : Self = Self(94);
	pub const R16G16B16A16_UINT : Self = Self(95);
	pub const R16G16B16A16_SINT : Self = Self(96);
	pub const R16G16B16A16_SFLOAT : Self = Self(97);
	pub const R32_UINT : Self = Self(98);
	pub const R32_SINT : Self = Self(99);
	pub const R32_SFLOAT : Self = Self(100);
	pub const R32G32_UINT : Self = Self(101);
	pub const R32G32_SINT : Self = Self(102);
	pub const R32G32_SFLOAT : Self = Self(103);
	pub const R32G32B32_UINT : Self = Self(104);
	pub const R32G32B32_SINT : Self = Self(105);
	pub const R32G32B32_SFLOAT : Self = Self(106);
	pub const R32G32B32A32_UINT : Self = Self(107);
	pub const R32G32B32A32_SINT : Self = Self(108);
	pub const R32G32B32A32_SFLOAT : Self = Self(109);
	pub const R64_UINT : Self = Self(110);
	pub const R64_SINT : Self = Self(111);
	pub const R64_SFLOAT : Self = Self(112);
	pub const R64G64_UINT : Self = Self(113);
	pub const R64G64_SINT : Self = Self(114);
	pub const R64G64_SFLOAT : Self = Self(115);
	pub const R64G64B64_UINT : Self = Self(116);
	pub const R64G64B64_SINT : Self = Self(117);
	pub const R64G64B64_SFLOAT : Self = Self(118);
	pub const R64G64B64A64_UINT : Self = Self(119);
	pub const R64G64B64A64_SINT : Self = Self(120);
	pub const R64G64B64A64_SFLOAT : Self = Self(121);
	pub const B10G11R11_UFLOAT_PACK32 : Self = Self(122);
	pub const E5B9G9R9_UFLOAT_PACK32 : Self = Self(123);
	pub const D16_UNORM : Self = Self(124);
	pub const X8_D24_UNORM_PACK32 : Self = Self(125);
	pub const D32_SFLOAT : Self = Self(126);
	pub const S8_UINT : Self = Self(127);
	pub const D16_UNORM_S8_UINT : Self = Self(128);
	pub const D24_UNORM_S8_UINT : Self = Self(129);
	pub const D32_SFLOAT_S8_UINT : Self = Self(130);
	pub const BC1_RGB_UNORM_BLOCK : Self = Self(131);
	pub const BC1_RGB_SRGB_BLOCK : Self = Self(132);
	pub const BC1_RGBA_UNORM_BLOCK : Self = Self(133);
	pub const BC1_RGBA_SRGB_BLOCK : Self = Self(134);
	pub const BC2_UNORM_BLOCK : Self = Self(135);
	pub const BC2_SRGB_BLOCK : Self = Self(136);
	pub const BC3_UNORM_BLOCK : Self = Self(137);
	pub const BC3_SRGB_BLOCK : Self = Self(138);
	pub const BC4_UNORM_BLOCK : Self = Self(139);
	pub const BC4_SNORM_BLOCK : Self = Self(140);
	pub const BC5_UNORM_BLOCK : Self = Self(141);
	pub const BC5_SNORM_BLOCK : Self = Self(142);
	pub const BC6H_UFLOAT_BLOCK : Self = Self(143);
	pub const BC6H_SFLOAT_BLOCK : Self = Self(144);
	pub const BC7_UNORM_BLOCK : Self = Self(145);
	pub const BC7_SRGB_BLOCK : Self = Self(146);
	pub const ETC2_R8G8B8_UNORM_BLOCK : Self = Self(147);
	pub const ETC2_R8G8B8_SRGB_BLOCK : Self = Self(148);
	pub const ETC2_R8G8B8A1_UNORM_BLOCK : Self = Self(149);
	pub const ETC2_R8G8B8A1_SRGB_BLOCK : Self = Self(150);
	pub const ETC2_R8G8B8A8_UNORM_BLOCK : Self = Self(151);
	pub const ETC2_R8G8B8A8_SRGB_BLOCK : Self = Self(152);
	pub const EAC_R11_UNORM_BLOCK : Self = Self(153);
	pub const EAC_R11_SNORM_BLOCK : Self = Self(154);
	pub const EAC_R11G11_UNORM_BLOCK : Self = Self(155);
	pub const EAC_R11G11_SNORM_BLOCK : Self = Self(156);
	pub const ASTC_4x4_UNORM_BLOCK : Self = Self(157);
	pub const ASTC_4x4_SRGB_BLOCK : Self = Self(158);
	pub const ASTC_5x4_UNORM_BLOCK : Self = Self(159);
	pub const ASTC_5x4_SRGB_BLOCK : Self = Self(160);
	pub const ASTC_5x5_UNORM_BLOCK : Self = Self(161);
	pub const ASTC_5x5_SRGB_BLOCK : Self = Self(162);
	pub const ASTC_6x5_UNORM_BLOCK : Self = Self(163);
	pub const ASTC_6x5_SRGB_BLOCK : Self = Self(164);
	pub const ASTC_6x6_UNORM_BLOCK : Self = Self(165);
	pub const ASTC_6x6_SRGB_BLOCK : Self = Self(166);
	pub const ASTC_8x5_UNORM_BLOCK : Self = Self(167);
	pub const ASTC_8x5_SRGB_BLOCK : Self = Self(168);
	pub const ASTC_8x6_UNORM_BLOCK : Self = Self(169);
	pub const ASTC_8x6_SRGB_BLOCK : Self = Self(170);
	pub const ASTC_8x8_UNORM_BLOCK : Self = Self(171);
	pub const ASTC_8x8_SRGB_BLOCK : Self = Self(172);
	pub const ASTC_10x5_UNORM_BLOCK : Self = Self(173);
	pub const ASTC_10x5_SRGB_BLOCK : Self = Self(174);
	pub const ASTC_10x6_UNORM_BLOCK : Self = Self(175);
	pub const ASTC_10x6_SRGB_BLOCK : Self = Self(176);
	pub const ASTC_10x8_UNORM_BLOCK : Self = Self(177);
	pub const ASTC_10x8_SRGB_BLOCK : Self = Self(178);
	pub const ASTC_10x10_UNORM_BLOCK : Self = Self(179);
	pub const ASTC_10x10_SRGB_BLOCK : Self = Self(180);
	pub const ASTC_12x10_UNORM_BLOCK : Self = Self(181);
	pub const ASTC_12x10_SRGB_BLOCK : Self = Self(182);
	pub const ASTC_12x12_UNORM_BLOCK : Self = Self(183);
	pub const ASTC_12x12_SRGB_BLOCK : Self = Self(184);
	pub const PVRTC1_2BPP_UNORM_BLOCK_IMG : Self = Self(1000054000);
	pub const PVRTC1_4BPP_UNORM_BLOCK_IMG : Self = Self(1000054001);
	pub const PVRTC2_2BPP_UNORM_BLOCK_IMG : Self = Self(1000054002);
	pub const PVRTC2_4BPP_UNORM_BLOCK_IMG : Self = Self(1000054003);
	pub const PVRTC1_2BPP_SRGB_BLOCK_IMG : Self = Self(1000054004);
	pub const PVRTC1_4BPP_SRGB_BLOCK_IMG : Self = Self(1000054005);
	pub const PVRTC2_2BPP_SRGB_BLOCK_IMG : Self = Self(1000054006);
	pub const PVRTC2_4BPP_SRGB_BLOCK_IMG : Self = Self(1000054007);
	pub const ASTC_4x4_SFLOAT_BLOCK_EXT : Self = Self(1000066000);
	pub const ASTC_5x4_SFLOAT_BLOCK_EXT : Self = Self(1000066001);
	pub const ASTC_5x5_SFLOAT_BLOCK_EXT : Self = Self(1000066002);
	pub const ASTC_6x5_SFLOAT_BLOCK_EXT : Self = Self(1000066003);
	pub const ASTC_6x6_SFLOAT_BLOCK_EXT : Self = Self(1000066004);
	pub const ASTC_8x5_SFLOAT_BLOCK_EXT : Self = Self(1000066005);
	pub const ASTC_8x6_SFLOAT_BLOCK_EXT : Self = Self(1000066006);
	pub const ASTC_8x8_SFLOAT_BLOCK_EXT : Self = Self(1000066007);
	pub const ASTC_10x5_SFLOAT_BLOCK_EXT : Self = Self(1000066008);
	pub const ASTC_10x6_SFLOAT_BLOCK_EXT : Self = Self(1000066009);
	pub const ASTC_10x8_SFLOAT_BLOCK_EXT : Self = Self(1000066010);
	pub const ASTC_10x10_SFLOAT_BLOCK_EXT : Self = Self(1000066011);
	pub const ASTC_12x10_SFLOAT_BLOCK_EXT : Self = Self(1000066012);
	pub const ASTC_12x12_SFLOAT_BLOCK_EXT : Self = Self(1000066013);
	pub const G8B8G8R8_422_UNORM_KHR : Self = Self(1000156000);
	pub const G8B8G8R8_422_UNORM : Self = Self(1000156000);
	pub const B8G8R8G8_422_UNORM : Self = Self(1000156001);
	pub const B8G8R8G8_422_UNORM_KHR : Self = Self(1000156001);
	pub const G8_B8_R8_3PLANE_420_UNORM : Self = Self(1000156002);
	pub const G8_B8_R8_3PLANE_420_UNORM_KHR : Self = Self(1000156002);
	pub const G8_B8R8_2PLANE_420_UNORM_KHR : Self = Self(1000156003);
	pub const G8_B8R8_2PLANE_420_UNORM : Self = Self(1000156003);
	pub const G8_B8_R8_3PLANE_422_UNORM : Self = Self(1000156004);
	pub const G8_B8_R8_3PLANE_422_UNORM_KHR : Self = Self(1000156004);
	pub const G8_B8R8_2PLANE_422_UNORM : Self = Self(1000156005);
	pub const G8_B8R8_2PLANE_422_UNORM_KHR : Self = Self(1000156005);
	pub const G8_B8_R8_3PLANE_444_UNORM : Self = Self(1000156006);
	pub const G8_B8_R8_3PLANE_444_UNORM_KHR : Self = Self(1000156006);
	pub const R10X6_UNORM_PACK16_KHR : Self = Self(1000156007);
	pub const R10X6_UNORM_PACK16 : Self = Self(1000156007);
	pub const R10X6G10X6_UNORM_2PACK16_KHR : Self = Self(1000156008);
	pub const R10X6G10X6_UNORM_2PACK16 : Self = Self(1000156008);
	pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16 : Self = Self(1000156009);
	pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR : Self = Self(1000156009);
	pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 : Self = Self(1000156010);
	pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR : Self = Self(1000156010);
	pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 : Self = Self(1000156011);
	pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR : Self = Self(1000156011);
	pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR : Self = Self(1000156012);
	pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 : Self = Self(1000156012);
	pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 : Self = Self(1000156013);
	pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR : Self = Self(1000156013);
	pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR : Self = Self(1000156014);
	pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 : Self = Self(1000156014);
	pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR : Self = Self(1000156015);
	pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 : Self = Self(1000156015);
	pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR : Self = Self(1000156016);
	pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 : Self = Self(1000156016);
	pub const R12X4_UNORM_PACK16_KHR : Self = Self(1000156017);
	pub const R12X4_UNORM_PACK16 : Self = Self(1000156017);
	pub const R12X4G12X4_UNORM_2PACK16_KHR : Self = Self(1000156018);
	pub const R12X4G12X4_UNORM_2PACK16 : Self = Self(1000156018);
	pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR : Self = Self(1000156019);
	pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16 : Self = Self(1000156019);
	pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR : Self = Self(1000156020);
	pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 : Self = Self(1000156020);
	pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR : Self = Self(1000156021);
	pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 : Self = Self(1000156021);
	pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR : Self = Self(1000156022);
	pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 : Self = Self(1000156022);
	pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR : Self = Self(1000156023);
	pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 : Self = Self(1000156023);
	pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR : Self = Self(1000156024);
	pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 : Self = Self(1000156024);
	pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR : Self = Self(1000156025);
	pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 : Self = Self(1000156025);
	pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 : Self = Self(1000156026);
	pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR : Self = Self(1000156026);
	pub const G16B16G16R16_422_UNORM_KHR : Self = Self(1000156027);
	pub const G16B16G16R16_422_UNORM : Self = Self(1000156027);
	pub const B16G16R16G16_422_UNORM_KHR : Self = Self(1000156028);
	pub const B16G16R16G16_422_UNORM : Self = Self(1000156028);
	pub const G16_B16_R16_3PLANE_420_UNORM_KHR : Self = Self(1000156029);
	pub const G16_B16_R16_3PLANE_420_UNORM : Self = Self(1000156029);
	pub const G16_B16R16_2PLANE_420_UNORM_KHR : Self = Self(1000156030);
	pub const G16_B16R16_2PLANE_420_UNORM : Self = Self(1000156030);
	pub const G16_B16_R16_3PLANE_422_UNORM_KHR : Self = Self(1000156031);
	pub const G16_B16_R16_3PLANE_422_UNORM : Self = Self(1000156031);
	pub const G16_B16R16_2PLANE_422_UNORM_KHR : Self = Self(1000156032);
	pub const G16_B16R16_2PLANE_422_UNORM : Self = Self(1000156032);
	pub const G16_B16_R16_3PLANE_444_UNORM_KHR : Self = Self(1000156033);
	pub const G16_B16_R16_3PLANE_444_UNORM : Self = Self(1000156033);
	pub const A4R4G4B4_UNORM_PACK16_EXT : Self = Self(1000340000);
	pub const A4B4G4R4_UNORM_PACK16_EXT : Self = Self(1000340001);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndexType(pub i32);
impl IndexType {
	pub const UINT16 : Self = Self(0);
	pub const UINT32 : Self = Self(1);
	pub const NONE_NV : Self = Self(1000165000);
	pub const NONE_KHR : Self = Self(1000165000);
	pub const UINT8_EXT : Self = Self(1000265000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CommandBufferResetFlags(pub i32);
impl CommandBufferResetFlags {
	pub const RELEASE_RESOURCES : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for CommandBufferResetFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueryResultFlags(pub i32);
impl QueryResultFlags {
	pub const e64 : Self = Self(0x00000001);
	pub const WAIT : Self = Self(0x00000002);
	pub const WITH_AVAILABILITY : Self = Self(0x00000004);
	pub const PARTIAL : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for QueryResultFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FormatFeatureFlags(pub i32);
impl FormatFeatureFlags {
	pub const SAMPLED_IMAGE : Self = Self(0x00000001);
	pub const STORAGE_IMAGE : Self = Self(0x00000002);
	pub const STORAGE_IMAGE_ATOMIC : Self = Self(0x00000004);
	pub const UNIFORM_TEXEL_BUFFER : Self = Self(0x00000008);
	pub const STORAGE_TEXEL_BUFFER : Self = Self(0x00000010);
	pub const STORAGE_TEXEL_BUFFER_ATOMIC : Self = Self(0x00000020);
	pub const VERTEX_BUFFER : Self = Self(0x00000040);
	pub const COLOR_ATTACHMENT : Self = Self(0x00000080);
	pub const COLOR_ATTACHMENT_BLEND : Self = Self(0x00000100);
	pub const DEPTH_STENCIL_ATTACHMENT : Self = Self(0x00000200);
	pub const BLIT_SRC : Self = Self(0x00000400);
	pub const BLIT_DST : Self = Self(0x00000800);
	pub const SAMPLED_IMAGE_FILTER_LINEAR : Self = Self(0x00001000);
	pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT : Self = Self(0x00002000);
	pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG : Self = Self(0x00002000);
	pub const TRANSFER_SRC_KHR : Self = Self(0x00004000);
	pub const TRANSFER_SRC : Self = Self(0x00004000);
	pub const TRANSFER_DST : Self = Self(0x00008000);
	pub const TRANSFER_DST_KHR : Self = Self(0x00008000);
	pub const SAMPLED_IMAGE_FILTER_MINMAX : Self = Self(0x00010000);
	pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT : Self = Self(0x00010000);
	pub const MIDPOINT_CHROMA_SAMPLES_KHR : Self = Self(0x00020000);
	pub const MIDPOINT_CHROMA_SAMPLES : Self = Self(0x00020000);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR : Self = Self(0x00040000);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER : Self = Self(0x00040000);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR : Self = Self(0x00080000);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER : Self = Self(0x00080000);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR : Self = Self(0x00100000);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT : Self = Self(0x00100000);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR : Self = Self(0x00200000);
	pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE : Self = Self(0x00200000);
	pub const DISJOINT_KHR : Self = Self(0x00400000);
	pub const DISJOINT : Self = Self(0x00400000);
	pub const COSITED_CHROMA_SAMPLES_KHR : Self = Self(0x00800000);
	pub const COSITED_CHROMA_SAMPLES : Self = Self(0x00800000);
	pub const FRAGMENT_DENSITY_MAP_EXT : Self = Self(0x01000000);
	pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR : Self = Self(0x20000000);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for FormatFeatureFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ColorSpaceKHR(pub i32);
impl ColorSpaceKHR {
	pub const COLORSPACE_SRGB_NONLINEAR_KHR : Self = Self(0);
	pub const SRGB_NONLINEAR_KHR : Self = Self(0);
	pub const DISPLAY_P3_NONLINEAR_EXT : Self = Self(1000104001);
	pub const EXTENDED_SRGB_LINEAR_EXT : Self = Self(1000104002);
	pub const DCI_P3_LINEAR_EXT : Self = Self(1000104003);
	pub const DISPLAY_P3_LINEAR_EXT : Self = Self(1000104003);
	pub const DCI_P3_NONLINEAR_EXT : Self = Self(1000104004);
	pub const BT709_LINEAR_EXT : Self = Self(1000104005);
	pub const BT709_NONLINEAR_EXT : Self = Self(1000104006);
	pub const BT2020_LINEAR_EXT : Self = Self(1000104007);
	pub const HDR10_ST2084_EXT : Self = Self(1000104008);
	pub const DOLBYVISION_EXT : Self = Self(1000104009);
	pub const HDR10_HLG_EXT : Self = Self(1000104010);
	pub const ADOBERGB_LINEAR_EXT : Self = Self(1000104011);
	pub const ADOBERGB_NONLINEAR_EXT : Self = Self(1000104012);
	pub const PASS_THROUGH_EXT : Self = Self(1000104013);
	pub const EXTENDED_SRGB_NONLINEAR_EXT : Self = Self(1000104014);
	pub const DISPLAY_NATIVE_AMD : Self = Self(1000213000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccessFlags(pub i32);
impl AccessFlags {
	pub const INDIRECT_COMMAND_READ : Self = Self(0x00000001);
	pub const INDEX_READ : Self = Self(0x00000002);
	pub const VERTEX_ATTRIBUTE_READ : Self = Self(0x00000004);
	pub const UNIFORM_READ : Self = Self(0x00000008);
	pub const INPUT_ATTACHMENT_READ : Self = Self(0x00000010);
	pub const SHADER_READ : Self = Self(0x00000020);
	pub const SHADER_WRITE : Self = Self(0x00000040);
	pub const COLOR_ATTACHMENT_READ : Self = Self(0x00000080);
	pub const COLOR_ATTACHMENT_WRITE : Self = Self(0x00000100);
	pub const DEPTH_STENCIL_ATTACHMENT_READ : Self = Self(0x00000200);
	pub const DEPTH_STENCIL_ATTACHMENT_WRITE : Self = Self(0x00000400);
	pub const TRANSFER_READ : Self = Self(0x00000800);
	pub const TRANSFER_WRITE : Self = Self(0x00001000);
	pub const HOST_READ : Self = Self(0x00002000);
	pub const HOST_WRITE : Self = Self(0x00004000);
	pub const MEMORY_READ : Self = Self(0x00008000);
	pub const MEMORY_WRITE : Self = Self(0x00010000);
	pub const COMMAND_PREPROCESS_READ_NV : Self = Self(0x00020000);
	pub const COMMAND_PREPROCESS_WRITE_NV : Self = Self(0x00040000);
	pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT : Self = Self(0x00080000);
	pub const CONDITIONAL_RENDERING_READ_EXT : Self = Self(0x00100000);
	pub const ACCELERATION_STRUCTURE_READ_KHR : Self = Self(0x00200000);
	pub const ACCELERATION_STRUCTURE_READ_NV : Self = Self(0x00200000);
	pub const ACCELERATION_STRUCTURE_WRITE_NV : Self = Self(0x00400000);
	pub const ACCELERATION_STRUCTURE_WRITE_KHR : Self = Self(0x00400000);
	pub const SHADING_RATE_IMAGE_READ_NV : Self = Self(0x00800000);
	pub const FRAGMENT_DENSITY_MAP_READ_EXT : Self = Self(0x01000000);
	pub const TRANSFORM_FEEDBACK_WRITE_EXT : Self = Self(0x02000000);
	pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT : Self = Self(0x04000000);
	pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT : Self = Self(0x08000000);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for AccessFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineStageFlags(pub i32);
impl PipelineStageFlags {
	pub const TOP_OF_PIPE : Self = Self(0x00000001);
	pub const DRAW_INDIRECT : Self = Self(0x00000002);
	pub const VERTEX_INPUT : Self = Self(0x00000004);
	pub const VERTEX_SHADER : Self = Self(0x00000008);
	pub const TESSELLATION_CONTROL_SHADER : Self = Self(0x00000010);
	pub const TESSELLATION_EVALUATION_SHADER : Self = Self(0x00000020);
	pub const GEOMETRY_SHADER : Self = Self(0x00000040);
	pub const FRAGMENT_SHADER : Self = Self(0x00000080);
	pub const EARLY_FRAGMENT_TESTS : Self = Self(0x00000100);
	pub const LATE_FRAGMENT_TESTS : Self = Self(0x00000200);
	pub const COLOR_ATTACHMENT_OUTPUT : Self = Self(0x00000400);
	pub const COMPUTE_SHADER : Self = Self(0x00000800);
	pub const TRANSFER : Self = Self(0x00001000);
	pub const BOTTOM_OF_PIPE : Self = Self(0x00002000);
	pub const HOST : Self = Self(0x00004000);
	pub const ALL_GRAPHICS : Self = Self(0x00008000);
	pub const ALL_COMMANDS : Self = Self(0x00010000);
	pub const COMMAND_PREPROCESS_NV : Self = Self(0x00020000);
	pub const CONDITIONAL_RENDERING_EXT : Self = Self(0x00040000);
	pub const TASK_SHADER_NV : Self = Self(0x00080000);
	pub const MESH_SHADER_NV : Self = Self(0x00100000);
	pub const RAY_TRACING_SHADER_NV : Self = Self(0x00200000);
	pub const RAY_TRACING_SHADER_KHR : Self = Self(0x00200000);
	pub const SHADING_RATE_IMAGE_NV : Self = Self(0x00400000);
	pub const FRAGMENT_DENSITY_PROCESS_EXT : Self = Self(0x00800000);
	pub const TRANSFORM_FEEDBACK_EXT : Self = Self(0x01000000);
	pub const ACCELERATION_STRUCTURE_BUILD_NV : Self = Self(0x02000000);
	pub const ACCELERATION_STRUCTURE_BUILD_KHR : Self = Self(0x02000000);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PipelineStageFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AcquireProfilingLockFlagsKHR(pub i32);
impl AcquireProfilingLockFlagsKHR {

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for AcquireProfilingLockFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DescriptorType(pub i32);
impl DescriptorType {
	pub const SAMPLER : Self = Self(0);
	pub const COMBINED_IMAGE_SAMPLER : Self = Self(1);
	pub const SAMPLED_IMAGE : Self = Self(2);
	pub const STORAGE_IMAGE : Self = Self(3);
	pub const UNIFORM_TEXEL_BUFFER : Self = Self(4);
	pub const STORAGE_TEXEL_BUFFER : Self = Self(5);
	pub const UNIFORM_BUFFER : Self = Self(6);
	pub const STORAGE_BUFFER : Self = Self(7);
	pub const UNIFORM_BUFFER_DYNAMIC : Self = Self(8);
	pub const STORAGE_BUFFER_DYNAMIC : Self = Self(9);
	pub const INPUT_ATTACHMENT : Self = Self(10);
	pub const INLINE_UNIFORM_BLOCK_EXT : Self = Self(1000138000);
	pub const ACCELERATION_STRUCTURE_NV : Self = Self(1000165000);
	pub const ACCELERATION_STRUCTURE_KHR : Self = Self(1000165000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FramebufferCreateFlags(pub i32);
impl FramebufferCreateFlags {
	pub const IMAGELESS_KHR : Self = Self(0x00000001);
	pub const IMAGELESS : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for FramebufferCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PhysicalDeviceType(pub i32);
impl PhysicalDeviceType {
	pub const OTHER : Self = Self(0);
	pub const INTEGRATED_GPU : Self = Self(1);
	pub const DISCRETE_GPU : Self = Self(2);
	pub const VIRTUAL_GPU : Self = Self(3);
	pub const CPU : Self = Self(4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SubpassContents(pub i32);
impl SubpassContents {
	pub const INLINE : Self = Self(0);
	pub const SECONDARY_COMMAND_BUFFERS : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageType(pub i32);
impl ImageType {
	pub const e1D : Self = Self(0);
	pub const e2D : Self = Self(1);
	pub const e3D : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InternalAllocationType(pub i32);
impl InternalAllocationType {
	pub const EXECUTABLE : Self = Self(0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ScopeNV(pub i32);
impl ScopeNV {
	pub const DEVICE_NV : Self = Self(1);
	pub const WORKGROUP_NV : Self = Self(2);
	pub const SUBGROUP_NV : Self = Self(3);
	pub const QUEUE_FAMILY_NV : Self = Self(5);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CommandBufferUsageFlags(pub i32);
impl CommandBufferUsageFlags {
	pub const ONE_TIME_SUBMIT : Self = Self(0x00000001);
	pub const RENDER_PASS_CONTINUE : Self = Self(0x00000002);
	pub const SIMULTANEOUS_USE : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for CommandBufferUsageFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MemoryHeapFlags(pub i32);
impl MemoryHeapFlags {
	pub const DEVICE_LOCAL : Self = Self(0x00000001);
	pub const MULTI_INSTANCE_KHR : Self = Self(0x00000002);
	pub const MULTI_INSTANCE : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for MemoryHeapFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShaderFloatControlsIndependence(pub i32);
impl ShaderFloatControlsIndependence {
	pub const e32_ONLY_KHR : Self = Self(0);
	pub const e32_ONLY : Self = Self(0);
	pub const ALL_KHR : Self = Self(1);
	pub const ALL : Self = Self(1);
	pub const NONE_KHR : Self = Self(2);
	pub const NONE : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceConfigurationTypeINTEL(pub i32);
impl PerformanceConfigurationTypeINTEL {
	pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL : Self = Self(0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CommandBufferLevel(pub i32);
impl CommandBufferLevel {
	pub const PRIMARY : Self = Self(0);
	pub const SECONDARY : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CommandPoolResetFlags(pub i32);
impl CommandPoolResetFlags {
	pub const RELEASE_RESOURCES : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for CommandPoolResetFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SurfaceCounterFlagsEXT(pub i32);
impl SurfaceCounterFlagsEXT {
	pub const VBLANK_EXT : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SurfaceCounterFlagsEXT {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageLayout(pub i32);
impl ImageLayout {
	pub const UNDEFINED : Self = Self(0);
	pub const GENERAL : Self = Self(1);
	pub const COLOR_ATTACHMENT_OPTIMAL : Self = Self(2);
	pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL : Self = Self(3);
	pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL : Self = Self(4);
	pub const SHADER_READ_ONLY_OPTIMAL : Self = Self(5);
	pub const TRANSFER_SRC_OPTIMAL : Self = Self(6);
	pub const TRANSFER_DST_OPTIMAL : Self = Self(7);
	pub const PREINITIALIZED : Self = Self(8);
	pub const PRESENT_SRC_KHR : Self = Self(1000001002);
	pub const SHARED_PRESENT_KHR : Self = Self(1000111000);
	pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR : Self = Self(1000117000);
	pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL : Self = Self(1000117000);
	pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR : Self = Self(1000117001);
	pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL : Self = Self(1000117001);
	pub const SHADING_RATE_OPTIMAL_NV : Self = Self(1000164003);
	pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT : Self = Self(1000218000);
	pub const DEPTH_ATTACHMENT_OPTIMAL_KHR : Self = Self(1000241000);
	pub const DEPTH_ATTACHMENT_OPTIMAL : Self = Self(1000241000);
	pub const DEPTH_READ_ONLY_OPTIMAL_KHR : Self = Self(1000241001);
	pub const DEPTH_READ_ONLY_OPTIMAL : Self = Self(1000241001);
	pub const STENCIL_ATTACHMENT_OPTIMAL_KHR : Self = Self(1000241002);
	pub const STENCIL_ATTACHMENT_OPTIMAL : Self = Self(1000241002);
	pub const STENCIL_READ_ONLY_OPTIMAL_KHR : Self = Self(1000241003);
	pub const STENCIL_READ_ONLY_OPTIMAL : Self = Self(1000241003);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineCreateFlags(pub i32);
impl PipelineCreateFlags {
	pub const DISABLE_OPTIMIZATION : Self = Self(0x00000001);
	pub const ALLOW_DERIVATIVES : Self = Self(0x00000002);
	pub const DERIVATIVE : Self = Self(0x00000004);
	pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR : Self = Self(0x00000008);
	pub const VIEW_INDEX_FROM_DEVICE_INDEX : Self = Self(0x00000008);
	pub const DISPATCH_BASE_KHR : Self = Self(0x00000010);
	pub const DISPATCH_BASE : Self = Self(0x00000010);
	pub const DEFER_COMPILE_NV : Self = Self(0x00000020);
	pub const CAPTURE_STATISTICS_KHR : Self = Self(0x00000040);
	pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR : Self = Self(0x00000080);
	pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT : Self = Self(0x00000100);
	pub const EARLY_RETURN_ON_FAILURE_EXT : Self = Self(0x00000200);
	pub const LIBRARY_KHR : Self = Self(0x00000800);
	pub const RAY_TRACING_SKIP_TRIANGLES_KHR : Self = Self(0x00001000);
	pub const RAY_TRACING_SKIP_AABBS_KHR : Self = Self(0x00002000);
	pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR : Self = Self(0x00004000);
	pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR : Self = Self(0x00008000);
	pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR : Self = Self(0x00010000);
	pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR : Self = Self(0x00020000);
	pub const INDIRECT_BINDABLE_NV : Self = Self(0x00040000);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PipelineCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceOverrideTypeINTEL(pub i32);
impl PerformanceOverrideTypeINTEL {
	pub const NULL_HARDWARE_INTEL : Self = Self(0);
	pub const FLUSH_GPU_CACHES_INTEL : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ObjectType(pub i32);
impl ObjectType {
	pub const UNKNOWN : Self = Self(0);
	pub const INSTANCE : Self = Self(1);
	pub const PHYSICAL_DEVICE : Self = Self(2);
	pub const DEVICE : Self = Self(3);
	pub const QUEUE : Self = Self(4);
	pub const SEMAPHORE : Self = Self(5);
	pub const COMMAND_BUFFER : Self = Self(6);
	pub const FENCE : Self = Self(7);
	pub const DEVICE_MEMORY : Self = Self(8);
	pub const BUFFER : Self = Self(9);
	pub const IMAGE : Self = Self(10);
	pub const EVENT : Self = Self(11);
	pub const QUERY_POOL : Self = Self(12);
	pub const BUFFER_VIEW : Self = Self(13);
	pub const IMAGE_VIEW : Self = Self(14);
	pub const SHADER_MODULE : Self = Self(15);
	pub const PIPELINE_CACHE : Self = Self(16);
	pub const PIPELINE_LAYOUT : Self = Self(17);
	pub const RENDER_PASS : Self = Self(18);
	pub const PIPELINE : Self = Self(19);
	pub const DESCRIPTOR_SET_LAYOUT : Self = Self(20);
	pub const SAMPLER : Self = Self(21);
	pub const DESCRIPTOR_POOL : Self = Self(22);
	pub const DESCRIPTOR_SET : Self = Self(23);
	pub const FRAMEBUFFER : Self = Self(24);
	pub const COMMAND_POOL : Self = Self(25);
	pub const SURFACE_KHR : Self = Self(1000000000);
	pub const SWAPCHAIN_KHR : Self = Self(1000001000);
	pub const DISPLAY_KHR : Self = Self(1000002000);
	pub const DISPLAY_MODE_KHR : Self = Self(1000002001);
	pub const DEBUG_REPORT_CALLBACK_EXT : Self = Self(1000011000);
	pub const DESCRIPTOR_UPDATE_TEMPLATE : Self = Self(1000085000);
	pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR : Self = Self(1000085000);
	pub const DEBUG_UTILS_MESSENGER_EXT : Self = Self(1000128000);
	pub const SAMPLER_YCBCR_CONVERSION_KHR : Self = Self(1000156000);
	pub const SAMPLER_YCBCR_CONVERSION : Self = Self(1000156000);
	pub const VALIDATION_CACHE_EXT : Self = Self(1000160000);
	pub const ACCELERATION_STRUCTURE_NV : Self = Self(1000165000);
	pub const ACCELERATION_STRUCTURE_KHR : Self = Self(1000165000);
	pub const PERFORMANCE_CONFIGURATION_INTEL : Self = Self(1000210000);
	pub const DEFERRED_OPERATION_KHR : Self = Self(1000268000);
	pub const INDIRECT_COMMANDS_LAYOUT_NV : Self = Self(1000277000);
	pub const PRIVATE_DATA_SLOT_EXT : Self = Self(1000295000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageCreateFlags(pub i32);
impl ImageCreateFlags {
	pub const SPARSE_BINDING : Self = Self(0x00000001);
	pub const SPARSE_RESIDENCY : Self = Self(0x00000002);
	pub const SPARSE_ALIASED : Self = Self(0x00000004);
	pub const MUTABLE_FORMAT : Self = Self(0x00000008);
	pub const CUBE_COMPATIBLE : Self = Self(0x00000010);
	pub const e2D_ARRAY_COMPATIBLE_KHR : Self = Self(0x00000020);
	pub const e2D_ARRAY_COMPATIBLE : Self = Self(0x00000020);
	pub const SPLIT_INSTANCE_BIND_REGIONS_KHR : Self = Self(0x00000040);
	pub const SPLIT_INSTANCE_BIND_REGIONS : Self = Self(0x00000040);
	pub const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR : Self = Self(0x00000080);
	pub const BLOCK_TEXEL_VIEW_COMPATIBLE : Self = Self(0x00000080);
	pub const EXTENDED_USAGE_KHR : Self = Self(0x00000100);
	pub const EXTENDED_USAGE : Self = Self(0x00000100);
	pub const DISJOINT_KHR : Self = Self(0x00000200);
	pub const DISJOINT : Self = Self(0x00000200);
	pub const ALIAS_KHR : Self = Self(0x00000400);
	pub const ALIAS : Self = Self(0x00000400);
	pub const PROTECTED : Self = Self(0x00000800);
	pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT : Self = Self(0x00001000);
	pub const CORNER_SAMPLED_NV : Self = Self(0x00002000);
	pub const SUBSAMPLED_EXT : Self = Self(0x00004000);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ImageCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExternalSemaphoreFeatureFlags(pub i32);
impl ExternalSemaphoreFeatureFlags {
	pub const EXPORTABLE_KHR : Self = Self(0x00000001);
	pub const EXPORTABLE : Self = Self(0x00000001);
	pub const IMPORTABLE_KHR : Self = Self(0x00000002);
	pub const IMPORTABLE : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ExternalSemaphoreFeatureFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StencilOp(pub i32);
impl StencilOp {
	pub const KEEP : Self = Self(0);
	pub const ZERO : Self = Self(1);
	pub const REPLACE : Self = Self(2);
	pub const INCREMENT_AND_CLAMP : Self = Self(3);
	pub const DECREMENT_AND_CLAMP : Self = Self(4);
	pub const INVERT : Self = Self(5);
	pub const INCREMENT_AND_WRAP : Self = Self(6);
	pub const DECREMENT_AND_WRAP : Self = Self(7);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SamplerMipmapMode(pub i32);
impl SamplerMipmapMode {
	pub const NEAREST : Self = Self(0);
	pub const LINEAR : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageViewType(pub i32);
impl ImageViewType {
	pub const e1D : Self = Self(0);
	pub const e2D : Self = Self(1);
	pub const e3D : Self = Self(2);
	pub const CUBE : Self = Self(3);
	pub const e1D_ARRAY : Self = Self(4);
	pub const e2D_ARRAY : Self = Self(5);
	pub const CUBE_ARRAY : Self = Self(6);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SystemAllocationScope(pub i32);
impl SystemAllocationScope {
	pub const COMMAND : Self = Self(0);
	pub const OBJECT : Self = Self(1);
	pub const CACHE : Self = Self(2);
	pub const DEVICE : Self = Self(3);
	pub const INSTANCE : Self = Self(4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueueFlags(pub i32);
impl QueueFlags {
	pub const GRAPHICS : Self = Self(0x00000001);
	pub const COMPUTE : Self = Self(0x00000002);
	pub const TRANSFER : Self = Self(0x00000004);
	pub const SPARSE_BINDING : Self = Self(0x00000008);
	pub const PROTECTED : Self = Self(0x00000010);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for QueueFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SubpassDescriptionFlags(pub i32);
impl SubpassDescriptionFlags {
	pub const PER_VIEW_ATTRIBUTES_NVX : Self = Self(0x00000001);
	pub const PER_VIEW_POSITION_X_ONLY_NVX : Self = Self(0x00000002);
	pub const FRAGMENT_REGION_QCOM : Self = Self(0x00000004);
	pub const SHADER_RESOLVE_QCOM : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SubpassDescriptionFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueryType(pub i32);
impl QueryType {
	pub const OCCLUSION : Self = Self(0);
	pub const PIPELINE_STATISTICS : Self = Self(1);
	pub const TIMESTAMP : Self = Self(2);
	pub const TRANSFORM_FEEDBACK_STREAM_EXT : Self = Self(1000028004);
	pub const PERFORMANCE_QUERY_KHR : Self = Self(1000116000);
	pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR : Self = Self(1000150000);
	pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV : Self = Self(1000165000);
	pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR : Self = Self(1000165000);
	pub const PERFORMANCE_QUERY_INTEL : Self = Self(1000210000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SharingMode(pub i32);
impl SharingMode {
	pub const EXCLUSIVE : Self = Self(0);
	pub const CONCURRENT : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineCacheHeaderVersion(pub i32);
impl PipelineCacheHeaderVersion {
	pub const ONE : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AttachmentStoreOp(pub i32);
impl AttachmentStoreOp {
	pub const STORE : Self = Self(0);
	pub const DONT_CARE : Self = Self(1);
	pub const NONE_QCOM : Self = Self(1000301000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueryControlFlags(pub i32);
impl QueryControlFlags {
	pub const PRECISE : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for QueryControlFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DebugReportFlagsEXT(pub i32);
impl DebugReportFlagsEXT {
	pub const INFORMATION_EXT : Self = Self(0x00000001);
	pub const WARNING_EXT : Self = Self(0x00000002);
	pub const PERFORMANCE_WARNING_EXT : Self = Self(0x00000004);
	pub const ERROR_EXT : Self = Self(0x00000008);
	pub const DEBUG_EXT : Self = Self(0x00000010);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DebugReportFlagsEXT {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ComponentSwizzle(pub i32);
impl ComponentSwizzle {
	pub const IDENTITY : Self = Self(0);
	pub const ZERO : Self = Self(1);
	pub const ONE : Self = Self(2);
	pub const R : Self = Self(3);
	pub const G : Self = Self(4);
	pub const B : Self = Self(5);
	pub const A : Self = Self(6);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineBindPoint(pub i32);
impl PipelineBindPoint {
	pub const GRAPHICS : Self = Self(0);
	pub const COMPUTE : Self = Self(1);
	pub const RAY_TRACING_NV : Self = Self(1000165000);
	pub const RAY_TRACING_KHR : Self = Self(1000165000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SparseImageFormatFlags(pub i32);
impl SparseImageFormatFlags {
	pub const SINGLE_MIPTAIL : Self = Self(0x00000001);
	pub const ALIGNED_MIP_SIZE : Self = Self(0x00000002);
	pub const NONSTANDARD_BLOCK_SIZE : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SparseImageFormatFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DynamicState(pub i32);
impl DynamicState {
	pub const VIEWPORT : Self = Self(0);
	pub const SCISSOR : Self = Self(1);
	pub const LINE_WIDTH : Self = Self(2);
	pub const DEPTH_BIAS : Self = Self(3);
	pub const BLEND_CONSTANTS : Self = Self(4);
	pub const DEPTH_BOUNDS : Self = Self(5);
	pub const STENCIL_COMPARE_MASK : Self = Self(6);
	pub const STENCIL_WRITE_MASK : Self = Self(7);
	pub const STENCIL_REFERENCE : Self = Self(8);
	pub const VIEWPORT_W_SCALING_NV : Self = Self(1000087000);
	pub const DISCARD_RECTANGLE_EXT : Self = Self(1000099000);
	pub const SAMPLE_LOCATIONS_EXT : Self = Self(1000143000);
	pub const VIEWPORT_SHADING_RATE_PALETTE_NV : Self = Self(1000164004);
	pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV : Self = Self(1000164006);
	pub const EXCLUSIVE_SCISSOR_NV : Self = Self(1000205001);
	pub const LINE_STIPPLE_EXT : Self = Self(1000259000);
	pub const CULL_MODE_EXT : Self = Self(1000267000);
	pub const FRONT_FACE_EXT : Self = Self(1000267001);
	pub const PRIMITIVE_TOPOLOGY_EXT : Self = Self(1000267002);
	pub const VIEWPORT_WITH_COUNT_EXT : Self = Self(1000267003);
	pub const SCISSOR_WITH_COUNT_EXT : Self = Self(1000267004);
	pub const VERTEX_INPUT_BINDING_STRIDE_EXT : Self = Self(1000267005);
	pub const DEPTH_TEST_ENABLE_EXT : Self = Self(1000267006);
	pub const DEPTH_WRITE_ENABLE_EXT : Self = Self(1000267007);
	pub const DEPTH_COMPARE_OP_EXT : Self = Self(1000267008);
	pub const DEPTH_BOUNDS_TEST_ENABLE_EXT : Self = Self(1000267009);
	pub const STENCIL_TEST_ENABLE_EXT : Self = Self(1000267010);
	pub const STENCIL_OP_EXT : Self = Self(1000267011);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CullModeFlags(pub i32);
impl CullModeFlags {
	pub const NONE : Self = Self(0);
	pub const FRONT : Self = Self(0x00000001);
	pub const BACK : Self = Self(0x00000002);
	pub const FRONT_AND_BACK : Self = Self(0x00000003);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for CullModeFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CompareOp(pub i32);
impl CompareOp {
	pub const NEVER : Self = Self(0);
	pub const LESS : Self = Self(1);
	pub const EQUAL : Self = Self(2);
	pub const LESS_OR_EQUAL : Self = Self(3);
	pub const GREATER : Self = Self(4);
	pub const NOT_EQUAL : Self = Self(5);
	pub const GREATER_OR_EQUAL : Self = Self(6);
	pub const ALWAYS : Self = Self(7);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineCacheCreateFlags(pub i32);
impl PipelineCacheCreateFlags {
	pub const EXTERNALLY_SYNCHRONIZED_EXT : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PipelineCacheCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RenderPassCreateFlags(pub i32);
impl RenderPassCreateFlags {
	pub const TRANSFORM_QCOM : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for RenderPassCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VkResult(pub i32);
impl VkResult {
	pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR : Self = Self(-1000257000);
	pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS : Self = Self(-1000257000);
	pub const ERROR_INVALID_DEVICE_ADDRESS_EXT : Self = Self(-1000257000);
	pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT : Self = Self(-1000255000);
	pub const ERROR_NOT_PERMITTED_EXT : Self = Self(-1000174001);
	pub const ERROR_FRAGMENTATION_EXT : Self = Self(-1000161000);
	pub const ERROR_FRAGMENTATION : Self = Self(-1000161000);
	pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT : Self = Self(-1000158000);
	pub const ERROR_INCOMPATIBLE_VERSION_KHR : Self = Self(-1000150000);
	pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR : Self = Self(-1000072003);
	pub const ERROR_INVALID_EXTERNAL_HANDLE : Self = Self(-1000072003);
	pub const ERROR_OUT_OF_POOL_MEMORY : Self = Self(-1000069000);
	pub const ERROR_OUT_OF_POOL_MEMORY_KHR : Self = Self(-1000069000);
	pub const ERROR_INVALID_SHADER_NV : Self = Self(-1000012000);
	pub const ERROR_VALIDATION_FAILED_EXT : Self = Self(-1000011001);
	pub const ERROR_INCOMPATIBLE_DISPLAY_KHR : Self = Self(-1000003001);
	pub const ERROR_OUT_OF_DATE_KHR : Self = Self(-1000001004);
	pub const ERROR_NATIVE_WINDOW_IN_USE_KHR : Self = Self(-1000000001);
	pub const ERROR_SURFACE_LOST_KHR : Self = Self(-1000000000);
	pub const ERROR_UNKNOWN : Self = Self(-13);
	pub const ERROR_FRAGMENTED_POOL : Self = Self(-12);
	pub const ERROR_FORMAT_NOT_SUPPORTED : Self = Self(-11);
	pub const ERROR_TOO_MANY_OBJECTS : Self = Self(-10);
	pub const ERROR_INCOMPATIBLE_DRIVER : Self = Self(-9);
	pub const ERROR_FEATURE_NOT_PRESENT : Self = Self(-8);
	pub const ERROR_EXTENSION_NOT_PRESENT : Self = Self(-7);
	pub const ERROR_LAYER_NOT_PRESENT : Self = Self(-6);
	pub const ERROR_MEMORY_MAP_FAILED : Self = Self(-5);
	pub const ERROR_DEVICE_LOST : Self = Self(-4);
	pub const ERROR_INITIALIZATION_FAILED : Self = Self(-3);
	pub const ERROR_OUT_OF_DEVICE_MEMORY : Self = Self(-2);
	pub const ERROR_OUT_OF_HOST_MEMORY : Self = Self(-1);
	pub const SUCCESS : Self = Self(0);
	pub const NOT_READY : Self = Self(1);
	pub const TIMEOUT : Self = Self(2);
	pub const EVENT_SET : Self = Self(3);
	pub const EVENT_RESET : Self = Self(4);
	pub const INCOMPLETE : Self = Self(5);
	pub const SUBOPTIMAL_KHR : Self = Self(1000001003);
	pub const THREAD_IDLE_KHR : Self = Self(1000268000);
	pub const THREAD_DONE_KHR : Self = Self(1000268001);
	pub const OPERATION_DEFERRED_KHR : Self = Self(1000268002);
	pub const OPERATION_NOT_DEFERRED_KHR : Self = Self(1000268003);
	pub const PIPELINE_COMPILE_REQUIRED_EXT : Self = Self(1000297000);
	pub const ERROR_PIPELINE_COMPILE_REQUIRED_EXT : Self = Self(1000297000);
	pub fn err_str(self) -> &'static str {
		match self {
			VkResult::OPERATION_DEFERRED_KHR => "OPERATION_DEFERRED_KHR",
			VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => "ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT",
			VkResult::ERROR_NOT_PERMITTED_EXT => "ERROR_NOT_PERMITTED_EXT",
			VkResult::ERROR_INVALID_SHADER_NV => "ERROR_INVALID_SHADER_NV",
			VkResult::ERROR_LAYER_NOT_PRESENT => "ERROR_LAYER_NOT_PRESENT",
			VkResult::ERROR_MEMORY_MAP_FAILED => "ERROR_MEMORY_MAP_FAILED",
			VkResult::ERROR_OUT_OF_DEVICE_MEMORY => "ERROR_OUT_OF_DEVICE_MEMORY",
			VkResult::EVENT_SET => "EVENT_SET",
			VkResult::PIPELINE_COMPILE_REQUIRED_EXT => "PIPELINE_COMPILE_REQUIRED_EXT",
			VkResult::ERROR_OUT_OF_HOST_MEMORY => "ERROR_OUT_OF_HOST_MEMORY",
			VkResult::ERROR_OUT_OF_POOL_MEMORY => "ERROR_OUT_OF_POOL_MEMORY",
			VkResult::ERROR_EXTENSION_NOT_PRESENT => "ERROR_EXTENSION_NOT_PRESENT",
			VkResult::ERROR_INCOMPATIBLE_DISPLAY_KHR => "ERROR_INCOMPATIBLE_DISPLAY_KHR",
			VkResult::ERROR_INCOMPATIBLE_VERSION_KHR => "ERROR_INCOMPATIBLE_VERSION_KHR",
			VkResult::ERROR_OUT_OF_DATE_KHR => "ERROR_OUT_OF_DATE_KHR",
			VkResult::INCOMPLETE => "INCOMPLETE",
			VkResult::ERROR_FORMAT_NOT_SUPPORTED => "ERROR_FORMAT_NOT_SUPPORTED",
			VkResult::EVENT_RESET => "EVENT_RESET",
			VkResult::ERROR_TOO_MANY_OBJECTS => "ERROR_TOO_MANY_OBJECTS",
			VkResult::THREAD_DONE_KHR => "THREAD_DONE_KHR",
			VkResult::THREAD_IDLE_KHR => "THREAD_IDLE_KHR",
			VkResult::TIMEOUT => "TIMEOUT",
			VkResult::OPERATION_NOT_DEFERRED_KHR => "OPERATION_NOT_DEFERRED_KHR",
			VkResult::SUCCESS => "SUCCESS",
			VkResult::NOT_READY => "NOT_READY",
			VkResult::ERROR_FEATURE_NOT_PRESENT => "ERROR_FEATURE_NOT_PRESENT",
			VkResult::ERROR_INCOMPATIBLE_DRIVER => "ERROR_INCOMPATIBLE_DRIVER",
			VkResult::ERROR_FRAGMENTED_POOL => "ERROR_FRAGMENTED_POOL",
			VkResult::ERROR_UNKNOWN => "ERROR_UNKNOWN",
			VkResult::ERROR_INITIALIZATION_FAILED => "ERROR_INITIALIZATION_FAILED",
			VkResult::ERROR_INVALID_EXTERNAL_HANDLE => "ERROR_INVALID_EXTERNAL_HANDLE",
			VkResult::ERROR_FRAGMENTATION => "ERROR_FRAGMENTATION",
			VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => "ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS",
			VkResult::ERROR_DEVICE_LOST => "ERROR_DEVICE_LOST",
			VkResult::ERROR_SURFACE_LOST_KHR => "ERROR_SURFACE_LOST_KHR",
			VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR => "ERROR_NATIVE_WINDOW_IN_USE_KHR",
			VkResult::SUBOPTIMAL_KHR => "SUBOPTIMAL_KHR",
			VkResult::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => "ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT",
			VkResult::ERROR_VALIDATION_FAILED_EXT => "ERROR_VALIDATION_FAILED_EXT",
			_ => "UNKNOWN"
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlendFactor(pub i32);
impl BlendFactor {
	pub const ZERO : Self = Self(0);
	pub const ONE : Self = Self(1);
	pub const SRC_COLOR : Self = Self(2);
	pub const ONE_MINUS_SRC_COLOR : Self = Self(3);
	pub const DST_COLOR : Self = Self(4);
	pub const ONE_MINUS_DST_COLOR : Self = Self(5);
	pub const SRC_ALPHA : Self = Self(6);
	pub const ONE_MINUS_SRC_ALPHA : Self = Self(7);
	pub const DST_ALPHA : Self = Self(8);
	pub const ONE_MINUS_DST_ALPHA : Self = Self(9);
	pub const CONSTANT_COLOR : Self = Self(10);
	pub const ONE_MINUS_CONSTANT_COLOR : Self = Self(11);
	pub const CONSTANT_ALPHA : Self = Self(12);
	pub const ONE_MINUS_CONSTANT_ALPHA : Self = Self(13);
	pub const SRC_ALPHA_SATURATE : Self = Self(14);
	pub const SRC1_COLOR : Self = Self(15);
	pub const ONE_MINUS_SRC1_COLOR : Self = Self(16);
	pub const SRC1_ALPHA : Self = Self(17);
	pub const ONE_MINUS_SRC1_ALPHA : Self = Self(18);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FenceImportFlags(pub i32);
impl FenceImportFlags {
	pub const TEMPORARY_KHR : Self = Self(0x00000001);
	pub const TEMPORARY : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for FenceImportFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageUsageFlags(pub i32);
impl ImageUsageFlags {
	pub const TRANSFER_SRC : Self = Self(0x00000001);
	pub const TRANSFER_DST : Self = Self(0x00000002);
	pub const SAMPLED : Self = Self(0x00000004);
	pub const STORAGE : Self = Self(0x00000008);
	pub const COLOR_ATTACHMENT : Self = Self(0x00000010);
	pub const DEPTH_STENCIL_ATTACHMENT : Self = Self(0x00000020);
	pub const TRANSIENT_ATTACHMENT : Self = Self(0x00000040);
	pub const INPUT_ATTACHMENT : Self = Self(0x00000080);
	pub const SHADING_RATE_IMAGE_NV : Self = Self(0x00000100);
	pub const FRAGMENT_DENSITY_MAP_EXT : Self = Self(0x00000200);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ImageUsageFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AttachmentLoadOp(pub i32);
impl AttachmentLoadOp {
	pub const LOAD : Self = Self(0);
	pub const CLEAR : Self = Self(1);
	pub const DONT_CARE : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BufferUsageFlags(pub i32);
impl BufferUsageFlags {
	pub const TRANSFER_SRC : Self = Self(0x00000001);
	pub const TRANSFER_DST : Self = Self(0x00000002);
	pub const UNIFORM_TEXEL_BUFFER : Self = Self(0x00000004);
	pub const STORAGE_TEXEL_BUFFER : Self = Self(0x00000008);
	pub const UNIFORM_BUFFER : Self = Self(0x00000010);
	pub const STORAGE_BUFFER : Self = Self(0x00000020);
	pub const INDEX_BUFFER : Self = Self(0x00000040);
	pub const VERTEX_BUFFER : Self = Self(0x00000080);
	pub const INDIRECT_BUFFER : Self = Self(0x00000100);
	pub const CONDITIONAL_RENDERING_EXT : Self = Self(0x00000200);
	pub const RAY_TRACING_NV : Self = Self(0x00000400);
	pub const RAY_TRACING_KHR : Self = Self(0x00000400);
	pub const TRANSFORM_FEEDBACK_BUFFER_EXT : Self = Self(0x00000800);
	pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT : Self = Self(0x00001000);
	pub const SHADER_DEVICE_ADDRESS_KHR : Self = Self(0x00020000);
	pub const SHADER_DEVICE_ADDRESS_EXT : Self = Self(0x00020000);
	pub const SHADER_DEVICE_ADDRESS : Self = Self(0x00020000);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for BufferUsageFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BorderColor(pub i32);
impl BorderColor {
	pub const FLOAT_TRANSPARENT_BLACK : Self = Self(0);
	pub const INT_TRANSPARENT_BLACK : Self = Self(1);
	pub const FLOAT_OPAQUE_BLACK : Self = Self(2);
	pub const INT_OPAQUE_BLACK : Self = Self(3);
	pub const FLOAT_OPAQUE_WHITE : Self = Self(4);
	pub const INT_OPAQUE_WHITE : Self = Self(5);
	pub const FLOAT_CUSTOM_EXT : Self = Self(1000287003);
	pub const INT_CUSTOM_EXT : Self = Self(1000287004);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShaderCorePropertiesFlagsAMD(pub i32);
impl ShaderCorePropertiesFlagsAMD {

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ShaderCorePropertiesFlagsAMD {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FrontFace(pub i32);
impl FrontFace {
	pub const COUNTER_CLOCKWISE : Self = Self(0);
	pub const CLOCKWISE : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SamplerAddressMode(pub i32);
impl SamplerAddressMode {
	pub const REPEAT : Self = Self(0);
	pub const MIRRORED_REPEAT : Self = Self(1);
	pub const CLAMP_TO_EDGE : Self = Self(2);
	pub const CLAMP_TO_BORDER : Self = Self(3);
	pub const MIRROR_CLAMP_TO_EDGE_KHR : Self = Self(4);
	pub const MIRROR_CLAMP_TO_EDGE : Self = Self(4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationFeatureDisableEXT(pub i32);
impl ValidationFeatureDisableEXT {
	pub const ALL_EXT : Self = Self(0);
	pub const SHADERS_EXT : Self = Self(1);
	pub const THREAD_SAFETY_EXT : Self = Self(2);
	pub const API_PARAMETERS_EXT : Self = Self(3);
	pub const OBJECT_LIFETIMES_EXT : Self = Self(4);
	pub const CORE_CHECKS_EXT : Self = Self(5);
	pub const UNIQUE_HANDLES_EXT : Self = Self(6);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DescriptorUpdateTemplateType(pub i32);
impl DescriptorUpdateTemplateType {
	pub const DESCRIPTOR_SET_KHR : Self = Self(0);
	pub const DESCRIPTOR_SET : Self = Self(0);
	pub const PUSH_DESCRIPTORS_KHR : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PolygonMode(pub i32);
impl PolygonMode {
	pub const FILL : Self = Self(0);
	pub const LINE : Self = Self(1);
	pub const POINT : Self = Self(2);
	pub const FILL_RECTANGLE_NV : Self = Self(1000153000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CoarseSampleOrderTypeNV(pub i32);
impl CoarseSampleOrderTypeNV {
	pub const DEFAULT_NV : Self = Self(0);
	pub const CUSTOM_NV : Self = Self(1);
	pub const PIXEL_MAJOR_NV : Self = Self(2);
	pub const SAMPLE_MAJOR_NV : Self = Self(3);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LogicOp(pub i32);
impl LogicOp {
	pub const CLEAR : Self = Self(0);
	pub const AND : Self = Self(1);
	pub const AND_REVERSE : Self = Self(2);
	pub const COPY : Self = Self(3);
	pub const AND_INVERTED : Self = Self(4);
	pub const NO_OP : Self = Self(5);
	pub const XOR : Self = Self(6);
	pub const OR : Self = Self(7);
	pub const NOR : Self = Self(8);
	pub const EQUIVALENT : Self = Self(9);
	pub const INVERT : Self = Self(10);
	pub const OR_REVERSE : Self = Self(11);
	pub const COPY_INVERTED : Self = Self(12);
	pub const OR_INVERTED : Self = Self(13);
	pub const NAND : Self = Self(14);
	pub const SET : Self = Self(15);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CommandPoolCreateFlags(pub i32);
impl CommandPoolCreateFlags {
	pub const TRANSIENT : Self = Self(0x00000001);
	pub const RESET_COMMAND_BUFFER : Self = Self(0x00000002);
	pub const PROTECTED : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for CommandPoolCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DescriptorBindingFlags(pub i32);
impl DescriptorBindingFlags {
	pub const UPDATE_AFTER_BIND_EXT : Self = Self(0x00000001);
	pub const UPDATE_AFTER_BIND : Self = Self(0x00000001);
	pub const UPDATE_UNUSED_WHILE_PENDING_EXT : Self = Self(0x00000002);
	pub const UPDATE_UNUSED_WHILE_PENDING : Self = Self(0x00000002);
	pub const PARTIALLY_BOUND_EXT : Self = Self(0x00000004);
	pub const PARTIALLY_BOUND : Self = Self(0x00000004);
	pub const VARIABLE_DESCRIPTOR_COUNT : Self = Self(0x00000008);
	pub const VARIABLE_DESCRIPTOR_COUNT_EXT : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DescriptorBindingFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Filter(pub i32);
impl Filter {
	pub const NEAREST : Self = Self(0);
	pub const LINEAR : Self = Self(1);
	pub const CUBIC_EXT : Self = Self(1000015000);
	pub const CUBIC_IMG : Self = Self(1000015000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineShaderStageCreateFlags(pub i32);
impl PipelineShaderStageCreateFlags {
	pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT : Self = Self(0x00000001);
	pub const REQUIRE_FULL_SUBGROUPS_EXT : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PipelineShaderStageCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShaderStageFlags(pub i32);
impl ShaderStageFlags {
	pub const VERTEX : Self = Self(0x00000001);
	pub const TESSELLATION_CONTROL : Self = Self(0x00000002);
	pub const TESSELLATION_EVALUATION : Self = Self(0x00000004);
	pub const GEOMETRY : Self = Self(0x00000008);
	pub const FRAGMENT : Self = Self(0x00000010);
	pub const ALL_GRAPHICS : Self = Self(0x0000001F);
	pub const COMPUTE : Self = Self(0x00000020);
	pub const TASK_NV : Self = Self(0x00000040);
	pub const MESH_NV : Self = Self(0x00000080);
	pub const RAYGEN_NV : Self = Self(0x00000100);
	pub const RAYGEN_KHR : Self = Self(0x00000100);
	pub const ANY_HIT_NV : Self = Self(0x00000200);
	pub const ANY_HIT_KHR : Self = Self(0x00000200);
	pub const CLOSEST_HIT_NV : Self = Self(0x00000400);
	pub const CLOSEST_HIT_KHR : Self = Self(0x00000400);
	pub const MISS_NV : Self = Self(0x00000800);
	pub const MISS_KHR : Self = Self(0x00000800);
	pub const INTERSECTION_NV : Self = Self(0x00001000);
	pub const INTERSECTION_KHR : Self = Self(0x00001000);
	pub const CALLABLE_KHR : Self = Self(0x00002000);
	pub const CALLABLE_NV : Self = Self(0x00002000);
	pub const ALL : Self = Self(0x7FFFFFFF);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ShaderStageFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExternalSemaphoreHandleTypeFlags(pub i32);
impl ExternalSemaphoreHandleTypeFlags {
	pub const OPAQUE_FD_KHR : Self = Self(0x00000001);
	pub const OPAQUE_FD : Self = Self(0x00000001);
	pub const OPAQUE_WIN32_KHR : Self = Self(0x00000002);
	pub const OPAQUE_WIN32 : Self = Self(0x00000002);
	pub const OPAQUE_WIN32_KMT_KHR : Self = Self(0x00000004);
	pub const OPAQUE_WIN32_KMT : Self = Self(0x00000004);
	pub const D3D12_FENCE_KHR : Self = Self(0x00000008);
	pub const D3D12_FENCE : Self = Self(0x00000008);
	pub const D3D11_FENCE : Self = Self(0x00000008);
	pub const SYNC_FD : Self = Self(0x00000010);
	pub const SYNC_FD_KHR : Self = Self(0x00000010);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ExternalSemaphoreHandleTypeFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SamplerCreateFlags(pub i32);
impl SamplerCreateFlags {
	pub const SUBSAMPLED_EXT : Self = Self(0x00000001);
	pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SamplerCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PipelineExecutableStatisticFormatKHR(pub i32);
impl PipelineExecutableStatisticFormatKHR {
	pub const BOOL32_KHR : Self = Self(0);
	pub const INT64_KHR : Self = Self(1);
	pub const UINT64_KHR : Self = Self(2);
	pub const FLOAT64_KHR : Self = Self(3);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DescriptorSetLayoutCreateFlags(pub i32);
impl DescriptorSetLayoutCreateFlags {
	pub const PUSH_DESCRIPTOR_KHR : Self = Self(0x00000001);
	pub const UPDATE_AFTER_BIND_POOL_EXT : Self = Self(0x00000002);
	pub const UPDATE_AFTER_BIND_POOL : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DescriptorSetLayoutCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImageTiling(pub i32);
impl ImageTiling {
	pub const OPTIMAL : Self = Self(0);
	pub const LINEAR : Self = Self(1);
	pub const DRM_FORMAT_MODIFIER_EXT : Self = Self(1000158000);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AttachmentDescriptionFlags(pub i32);
impl AttachmentDescriptionFlags {
	pub const MAY_ALIAS : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for AttachmentDescriptionFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SemaphoreType(pub i32);
impl SemaphoreType {
	pub const BINARY_KHR : Self = Self(0);
	pub const BINARY : Self = Self(0);
	pub const TIMELINE_KHR : Self = Self(1);
	pub const TIMELINE : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DependencyFlags(pub i32);
impl DependencyFlags {
	pub const BY_REGION : Self = Self(0x00000001);
	pub const VIEW_LOCAL_KHR : Self = Self(0x00000002);
	pub const VIEW_LOCAL : Self = Self(0x00000002);
	pub const DEVICE_GROUP_KHR : Self = Self(0x00000004);
	pub const DEVICE_GROUP : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DependencyFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StencilFaceFlags(pub i32);
impl StencilFaceFlags {
	pub const FRONT : Self = Self(0x00000001);
	pub const BACK : Self = Self(0x00000002);
	pub const STENCIL_FRONT_AND_BACK : Self = Self(0x00000003);
	pub const FRONT_AND_BACK : Self = Self(0x00000003);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for StencilFaceFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PointClippingBehavior(pub i32);
impl PointClippingBehavior {
	pub const ALL_CLIP_PLANES_KHR : Self = Self(0);
	pub const ALL_CLIP_PLANES : Self = Self(0);
	pub const USER_CLIP_PLANES_ONLY_KHR : Self = Self(1);
	pub const USER_CLIP_PLANES_ONLY : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CompositeAlphaFlagsKHR(pub i32);
impl CompositeAlphaFlagsKHR {
	pub const OPAQUE_KHR : Self = Self(0x00000001);
	pub const PRE_MULTIPLIED_KHR : Self = Self(0x00000002);
	pub const POST_MULTIPLIED_KHR : Self = Self(0x00000004);
	pub const INHERIT_KHR : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for CompositeAlphaFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DescriptorPoolCreateFlags(pub i32);
impl DescriptorPoolCreateFlags {
	pub const FREE_DESCRIPTOR_SET : Self = Self(0x00000001);
	pub const UPDATE_AFTER_BIND_EXT : Self = Self(0x00000002);
	pub const UPDATE_AFTER_BIND : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DescriptorPoolCreateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TessellationDomainOrigin(pub i32);
impl TessellationDomainOrigin {
	pub const UPPER_LEFT_KHR : Self = Self(0);
	pub const UPPER_LEFT : Self = Self(0);
	pub const LOWER_LEFT_KHR : Self = Self(1);
	pub const LOWER_LEFT : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SamplerYcbcrModelConversion(pub i32);
impl SamplerYcbcrModelConversion {
	pub const RGB_IDENTITY_KHR : Self = Self(0);
	pub const RGB_IDENTITY : Self = Self(0);
	pub const YCBCR_IDENTITY_KHR : Self = Self(1);
	pub const YCBCR_IDENTITY : Self = Self(1);
	pub const YCBCR_709_KHR : Self = Self(2);
	pub const YCBCR_709 : Self = Self(2);
	pub const YCBCR_601_KHR : Self = Self(3);
	pub const YCBCR_601 : Self = Self(3);
	pub const YCBCR_2020_KHR : Self = Self(4);
	pub const YCBCR_2020 : Self = Self(4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ViewportCoordinateSwizzleNV(pub i32);
impl ViewportCoordinateSwizzleNV {
	pub const POSITIVE_X_NV : Self = Self(0);
	pub const NEGATIVE_X_NV : Self = Self(1);
	pub const POSITIVE_Y_NV : Self = Self(2);
	pub const NEGATIVE_Y_NV : Self = Self(3);
	pub const POSITIVE_Z_NV : Self = Self(4);
	pub const NEGATIVE_Z_NV : Self = Self(5);
	pub const POSITIVE_W_NV : Self = Self(6);
	pub const NEGATIVE_W_NV : Self = Self(7);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SamplerYcbcrRange(pub i32);
impl SamplerYcbcrRange {
	pub const ITU_FULL_KHR : Self = Self(0);
	pub const ITU_FULL : Self = Self(0);
	pub const ITU_NARROW_KHR : Self = Self(1);
	pub const ITU_NARROW : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DisplayPowerStateEXT(pub i32);
impl DisplayPowerStateEXT {
	pub const OFF_EXT : Self = Self(0);
	pub const SUSPEND_EXT : Self = Self(1);
	pub const ON_EXT : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ChromaLocation(pub i32);
impl ChromaLocation {
	pub const COSITED_EVEN_KHR : Self = Self(0);
	pub const COSITED_EVEN : Self = Self(0);
	pub const MIDPOINT_KHR : Self = Self(1);
	pub const MIDPOINT : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SubgroupFeatureFlags(pub i32);
impl SubgroupFeatureFlags {
	pub const BASIC : Self = Self(0x00000001);
	pub const VOTE : Self = Self(0x00000002);
	pub const ARITHMETIC : Self = Self(0x00000004);
	pub const BALLOT : Self = Self(0x00000008);
	pub const SHUFFLE : Self = Self(0x00000010);
	pub const SHUFFLE_RELATIVE : Self = Self(0x00000020);
	pub const CLUSTERED : Self = Self(0x00000040);
	pub const QUAD : Self = Self(0x00000080);
	pub const PARTITIONED_NV : Self = Self(0x00000100);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SubgroupFeatureFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RasterizationOrderAMD(pub i32);
impl RasterizationOrderAMD {
	pub const STRICT_AMD : Self = Self(0);
	pub const RELAXED_AMD : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SparseMemoryBindFlags(pub i32);
impl SparseMemoryBindFlags {
	pub const METADATA : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SparseMemoryBindFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationCheckEXT(pub i32);
impl ValidationCheckEXT {
	pub const ALL_EXT : Self = Self(0);
	pub const SHADERS_EXT : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DisplayPlaneAlphaFlagsKHR(pub i32);
impl DisplayPlaneAlphaFlagsKHR {
	pub const OPAQUE_KHR : Self = Self(0x00000001);
	pub const GLOBAL_KHR : Self = Self(0x00000002);
	pub const PER_PIXEL_KHR : Self = Self(0x00000004);
	pub const PER_PIXEL_PREMULTIPLIED_KHR : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DisplayPlaneAlphaFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PeerMemoryFeatureFlags(pub i32);
impl PeerMemoryFeatureFlags {
	pub const COPY_SRC_KHR : Self = Self(0x00000001);
	pub const COPY_SRC : Self = Self(0x00000001);
	pub const COPY_DST_KHR : Self = Self(0x00000002);
	pub const COPY_DST : Self = Self(0x00000002);
	pub const GENERIC_SRC_KHR : Self = Self(0x00000004);
	pub const GENERIC_SRC : Self = Self(0x00000004);
	pub const GENERIC_DST_KHR : Self = Self(0x00000008);
	pub const GENERIC_DST : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PeerMemoryFeatureFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MemoryAllocateFlags(pub i32);
impl MemoryAllocateFlags {
	pub const DEVICE_MASK_KHR : Self = Self(0x00000001);
	pub const DEVICE_MASK : Self = Self(0x00000001);
	pub const DEVICE_ADDRESS_KHR : Self = Self(0x00000002);
	pub const DEVICE_ADDRESS : Self = Self(0x00000002);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR : Self = Self(0x00000004);
	pub const DEVICE_ADDRESS_CAPTURE_REPLAY : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for MemoryAllocateFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceDiagnosticsConfigFlagsNV(pub i32);
impl DeviceDiagnosticsConfigFlagsNV {
	pub const ENABLE_SHADER_DEBUG_INFO_NV : Self = Self(0x00000001);
	pub const ENABLE_RESOURCE_TRACKING_NV : Self = Self(0x00000002);
	pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DeviceDiagnosticsConfigFlagsNV {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ToolPurposeFlagsEXT(pub i32);
impl ToolPurposeFlagsEXT {
	pub const VALIDATION_EXT : Self = Self(0x00000001);
	pub const PROFILING_EXT : Self = Self(0x00000002);
	pub const TRACING_EXT : Self = Self(0x00000004);
	pub const ADDITIONAL_FEATURES_EXT : Self = Self(0x00000008);
	pub const MODIFYING_FEATURES_EXT : Self = Self(0x00000010);
	pub const DEBUG_REPORTING_EXT : Self = Self(0x00000020);
	pub const DEBUG_MARKERS_EXT : Self = Self(0x00000040);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ToolPurposeFlagsEXT {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VendorId(pub i32);
impl VendorId {
	pub const VIV : Self = Self(0x10001);
	pub const VSI : Self = Self(0x10002);
	pub const KAZAN : Self = Self(0x10003);
	pub const CODEPLAY : Self = Self(0x10004);
	pub const MESA : Self = Self(0x10005);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExternalMemoryHandleTypeFlags(pub i32);
impl ExternalMemoryHandleTypeFlags {
	pub const OPAQUE_FD_KHR : Self = Self(0x00000001);
	pub const OPAQUE_FD : Self = Self(0x00000001);
	pub const OPAQUE_WIN32 : Self = Self(0x00000002);
	pub const OPAQUE_WIN32_KHR : Self = Self(0x00000002);
	pub const OPAQUE_WIN32_KMT : Self = Self(0x00000004);
	pub const OPAQUE_WIN32_KMT_KHR : Self = Self(0x00000004);
	pub const D3D11_TEXTURE : Self = Self(0x00000008);
	pub const D3D11_TEXTURE_KHR : Self = Self(0x00000008);
	pub const D3D11_TEXTURE_KMT_KHR : Self = Self(0x00000010);
	pub const D3D11_TEXTURE_KMT : Self = Self(0x00000010);
	pub const D3D12_HEAP_KHR : Self = Self(0x00000020);
	pub const D3D12_HEAP : Self = Self(0x00000020);
	pub const D3D12_RESOURCE_KHR : Self = Self(0x00000040);
	pub const D3D12_RESOURCE : Self = Self(0x00000040);
	pub const HOST_ALLOCATION_EXT : Self = Self(0x00000080);
	pub const HOST_MAPPED_FOREIGN_MEMORY_EXT : Self = Self(0x00000100);
	pub const DMA_BUF_EXT : Self = Self(0x00000200);
	pub const ANDROID_HARDWARE_BUFFER_ANDROID : Self = Self(0x00000400);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ExternalMemoryHandleTypeFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExternalMemoryFeatureFlags(pub i32);
impl ExternalMemoryFeatureFlags {
	pub const DEDICATED_ONLY_KHR : Self = Self(0x00000001);
	pub const DEDICATED_ONLY : Self = Self(0x00000001);
	pub const EXPORTABLE_KHR : Self = Self(0x00000002);
	pub const EXPORTABLE : Self = Self(0x00000002);
	pub const IMPORTABLE_KHR : Self = Self(0x00000004);
	pub const IMPORTABLE : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ExternalMemoryFeatureFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExternalFenceFeatureFlags(pub i32);
impl ExternalFenceFeatureFlags {
	pub const EXPORTABLE_KHR : Self = Self(0x00000001);
	pub const EXPORTABLE : Self = Self(0x00000001);
	pub const IMPORTABLE_KHR : Self = Self(0x00000002);
	pub const IMPORTABLE : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ExternalFenceFeatureFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SemaphoreImportFlags(pub i32);
impl SemaphoreImportFlags {
	pub const TEMPORARY_KHR : Self = Self(0x00000001);
	pub const TEMPORARY : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SemaphoreImportFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LineRasterizationModeEXT(pub i32);
impl LineRasterizationModeEXT {
	pub const DEFAULT_EXT : Self = Self(0);
	pub const RECTANGULAR_EXT : Self = Self(1);
	pub const BRESENHAM_EXT : Self = Self(2);
	pub const RECTANGULAR_SMOOTH_EXT : Self = Self(3);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceGroupPresentModeFlagsKHR(pub i32);
impl DeviceGroupPresentModeFlagsKHR {
	pub const LOCAL_KHR : Self = Self(0x00000001);
	pub const REMOTE_KHR : Self = Self(0x00000002);
	pub const SUM_KHR : Self = Self(0x00000004);
	pub const LOCAL_MULTI_DEVICE_KHR : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DeviceGroupPresentModeFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlendOverlapEXT(pub i32);
impl BlendOverlapEXT {
	pub const UNCORRELATED_EXT : Self = Self(0);
	pub const DISJOINT_EXT : Self = Self(1);
	pub const CONJOINT_EXT : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DriverId(pub i32);
impl DriverId {
	pub const AMD_PROPRIETARY_KHR : Self = Self(1);
	pub const AMD_PROPRIETARY : Self = Self(1);
	pub const AMD_OPEN_SOURCE_KHR : Self = Self(2);
	pub const AMD_OPEN_SOURCE : Self = Self(2);
	pub const MESA_RADV_KHR : Self = Self(3);
	pub const MESA_RADV : Self = Self(3);
	pub const NVIDIA_PROPRIETARY_KHR : Self = Self(4);
	pub const NVIDIA_PROPRIETARY : Self = Self(4);
	pub const INTEL_PROPRIETARY_WINDOWS_KHR : Self = Self(5);
	pub const INTEL_PROPRIETARY_WINDOWS : Self = Self(5);
	pub const INTEL_OPEN_SOURCE_MESA_KHR : Self = Self(6);
	pub const INTEL_OPEN_SOURCE_MESA : Self = Self(6);
	pub const IMAGINATION_PROPRIETARY_KHR : Self = Self(7);
	pub const IMAGINATION_PROPRIETARY : Self = Self(7);
	pub const QUALCOMM_PROPRIETARY : Self = Self(8);
	pub const QUALCOMM_PROPRIETARY_KHR : Self = Self(8);
	pub const ARM_PROPRIETARY_KHR : Self = Self(9);
	pub const ARM_PROPRIETARY : Self = Self(9);
	pub const GOOGLE_SWIFTSHADER_KHR : Self = Self(10);
	pub const GOOGLE_SWIFTSHADER : Self = Self(10);
	pub const GGP_PROPRIETARY_KHR : Self = Self(11);
	pub const GGP_PROPRIETARY : Self = Self(11);
	pub const BROADCOM_PROPRIETARY_KHR : Self = Self(12);
	pub const BROADCOM_PROPRIETARY : Self = Self(12);
	pub const MESA_LLVMPIPE : Self = Self(13);
	pub const MOLTENVK : Self = Self(14);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccelerationStructureTypeKHR(pub i32);
impl AccelerationStructureTypeKHR {
	pub const TOP_LEVEL_NV : Self = Self(0);
	pub const TOP_LEVEL_KHR : Self = Self(0);
	pub const BOTTOM_LEVEL_NV : Self = Self(1);
	pub const BOTTOM_LEVEL_KHR : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ResolveModeFlags(pub i32);
impl ResolveModeFlags {
	pub const NONE_KHR : Self = Self(0);
	pub const NONE : Self = Self(0);
	pub const SAMPLE_ZERO_KHR : Self = Self(0x00000001);
	pub const SAMPLE_ZERO : Self = Self(0x00000001);
	pub const AVERAGE_KHR : Self = Self(0x00000002);
	pub const AVERAGE : Self = Self(0x00000002);
	pub const MIN_KHR : Self = Self(0x00000004);
	pub const MIN : Self = Self(0x00000004);
	pub const MAX : Self = Self(0x00000008);
	pub const MAX_KHR : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ResolveModeFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExternalFenceHandleTypeFlags(pub i32);
impl ExternalFenceHandleTypeFlags {
	pub const OPAQUE_FD_KHR : Self = Self(0x00000001);
	pub const OPAQUE_FD : Self = Self(0x00000001);
	pub const OPAQUE_WIN32_KHR : Self = Self(0x00000002);
	pub const OPAQUE_WIN32 : Self = Self(0x00000002);
	pub const OPAQUE_WIN32_KMT_KHR : Self = Self(0x00000004);
	pub const OPAQUE_WIN32_KMT : Self = Self(0x00000004);
	pub const SYNC_FD_KHR : Self = Self(0x00000008);
	pub const SYNC_FD : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ExternalFenceHandleTypeFlags {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PresentModeKHR(pub i32);
impl PresentModeKHR {
	pub const IMMEDIATE_KHR : Self = Self(0);
	pub const MAILBOX_KHR : Self = Self(1);
	pub const FIFO_KHR : Self = Self(2);
	pub const FIFO_RELAXED_KHR : Self = Self(3);
	pub const SHARED_DEMAND_REFRESH_KHR : Self = Self(1000111000);
	pub const SHARED_CONTINUOUS_REFRESH_KHR : Self = Self(1000111001);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SwapchainCreateFlagsKHR(pub i32);
impl SwapchainCreateFlagsKHR {
	pub const SPLIT_INSTANCE_BIND_REGIONS_KHR : Self = Self(0x00000001);
	pub const PROTECTED_KHR : Self = Self(0x00000002);
	pub const MUTABLE_FORMAT_KHR : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SwapchainCreateFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceCounterUnitKHR(pub i32);
impl PerformanceCounterUnitKHR {
	pub const GENERIC_KHR : Self = Self(0);
	pub const PERCENTAGE_KHR : Self = Self(1);
	pub const NANOSECONDS_KHR : Self = Self(2);
	pub const BYTES_KHR : Self = Self(3);
	pub const BYTES_PER_SECOND_KHR : Self = Self(4);
	pub const KELVIN_KHR : Self = Self(5);
	pub const WATTS_KHR : Self = Self(6);
	pub const VOLTS_KHR : Self = Self(7);
	pub const AMPS_KHR : Self = Self(8);
	pub const HERTZ_KHR : Self = Self(9);
	pub const CYCLES_KHR : Self = Self(10);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceCounterStorageKHR(pub i32);
impl PerformanceCounterStorageKHR {
	pub const INT32_KHR : Self = Self(0);
	pub const INT64_KHR : Self = Self(1);
	pub const UINT32_KHR : Self = Self(2);
	pub const UINT64_KHR : Self = Self(3);
	pub const FLOAT32_KHR : Self = Self(4);
	pub const FLOAT64_KHR : Self = Self(5);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PerformanceCounterDescriptionFlagsKHR(pub i32);
impl PerformanceCounterDescriptionFlagsKHR {
	pub const PERFORMANCE_IMPACTING_KHR : Self = Self(0x00000001);
	pub const CONCURRENTLY_IMPACTED_KHR : Self = Self(0x00000002);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for PerformanceCounterDescriptionFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ShaderInfoTypeAMD(pub i32);
impl ShaderInfoTypeAMD {
	pub const STATISTICS_AMD : Self = Self(0);
	pub const BINARY_AMD : Self = Self(1);
	pub const DISASSEMBLY_AMD : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DebugUtilsMessageTypeFlagsEXT(pub i32);
impl DebugUtilsMessageTypeFlagsEXT {
	pub const GENERAL_EXT : Self = Self(0x00000001);
	pub const VALIDATION_EXT : Self = Self(0x00000002);
	pub const PERFORMANCE_EXT : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for DebugUtilsMessageTypeFlagsEXT {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExternalMemoryHandleTypeFlagsNV(pub i32);
impl ExternalMemoryHandleTypeFlagsNV {
	pub const OPAQUE_WIN32_NV : Self = Self(0x00000001);
	pub const OPAQUE_WIN32_KMT_NV : Self = Self(0x00000002);
	pub const D3D11_IMAGE_NV : Self = Self(0x00000004);
	pub const D3D11_IMAGE_KMT_NV : Self = Self(0x00000008);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ExternalMemoryHandleTypeFlagsNV {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SurfaceTransformFlagsKHR(pub i32);
impl SurfaceTransformFlagsKHR {
	pub const IDENTITY_KHR : Self = Self(0x00000001);
	pub const ROTATE_90_KHR : Self = Self(0x00000002);
	pub const ROTATE_180_KHR : Self = Self(0x00000004);
	pub const ROTATE_270_KHR : Self = Self(0x00000008);
	pub const HORIZONTAL_MIRROR_KHR : Self = Self(0x00000010);
	pub const HORIZONTAL_MIRROR_ROTATE_90_KHR : Self = Self(0x00000020);
	pub const HORIZONTAL_MIRROR_ROTATE_180_KHR : Self = Self(0x00000040);
	pub const HORIZONTAL_MIRROR_ROTATE_270_KHR : Self = Self(0x00000080);
	pub const INHERIT_KHR : Self = Self(0x00000100);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for SurfaceTransformFlagsKHR {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExternalMemoryFeatureFlagsNV(pub i32);
impl ExternalMemoryFeatureFlagsNV {
	pub const DEDICATED_ONLY_NV : Self = Self(0x00000001);
	pub const EXPORTABLE_NV : Self = Self(0x00000002);
	pub const IMPORTABLE_NV : Self = Self(0x00000004);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ExternalMemoryFeatureFlagsNV {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VertexInputRate(pub i32);
impl VertexInputRate {
	pub const VERTEX : Self = Self(0);
	pub const INSTANCE : Self = Self(1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ConditionalRenderingFlagsEXT(pub i32);
impl ConditionalRenderingFlagsEXT {
	pub const INVERTED_EXT : Self = Self(0x00000001);

	pub fn or(self, r : Self) -> Self { Self(self.0 | r.0) }
	pub fn has(self, r : Self) -> bool { (self.0 & r.0) == r.0 }
}

impl std::ops::BitOr for ConditionalRenderingFlagsEXT {
	type Output = Self;
	fn bitor(self, r : Self) -> Self { self.or(r) }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ConservativeRasterizationModeEXT(pub i32);
impl ConservativeRasterizationModeEXT {
	pub const DISABLED_EXT : Self = Self(0);
	pub const OVERESTIMATE_EXT : Self = Self(1);
	pub const UNDERESTIMATE_EXT : Self = Self(2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlendOp(pub i32);
impl BlendOp {
	pub const ADD : Self = Self(0);
	pub const SUBTRACT : Self = Self(1);
	pub const REVERSE_SUBTRACT : Self = Self(2);
	pub const MIN : Self = Self(3);
	pub const MAX : Self = Self(4);
	pub const ZERO_EXT : Self = Self(1000148000);
	pub const SRC_EXT : Self = Self(1000148001);
	pub const DST_EXT : Self = Self(1000148002);
	pub const SRC_OVER_EXT : Self = Self(1000148003);
	pub const DST_OVER_EXT : Self = Self(1000148004);
	pub const SRC_IN_EXT : Self = Self(1000148005);
	pub const DST_IN_EXT : Self = Self(1000148006);
	pub const SRC_OUT_EXT : Self = Self(1000148007);
	pub const DST_OUT_EXT : Self = Self(1000148008);
	pub const SRC_ATOP_EXT : Self = Self(1000148009);
	pub const DST_ATOP_EXT : Self = Self(1000148010);
	pub const XOR_EXT : Self = Self(1000148011);
	pub const MULTIPLY_EXT : Self = Self(1000148012);
	pub const SCREEN_EXT : Self = Self(1000148013);
	pub const OVERLAY_EXT : Self = Self(1000148014);
	pub const DARKEN_EXT : Self = Self(1000148015);
	pub const LIGHTEN_EXT : Self = Self(1000148016);
	pub const COLORDODGE_EXT : Self = Self(1000148017);
	pub const COLORBURN_EXT : Self = Self(1000148018);
	pub const HARDLIGHT_EXT : Self = Self(1000148019);
	pub const SOFTLIGHT_EXT : Self = Self(1000148020);
	pub const DIFFERENCE_EXT : Self = Self(1000148021);
	pub const EXCLUSION_EXT : Self = Self(1000148022);
	pub const INVERT_EXT : Self = Self(1000148023);
	pub const INVERT_RGB_EXT : Self = Self(1000148024);
	pub const LINEARDODGE_EXT : Self = Self(1000148025);
	pub const LINEARBURN_EXT : Self = Self(1000148026);
	pub const VIVIDLIGHT_EXT : Self = Self(1000148027);
	pub const LINEARLIGHT_EXT : Self = Self(1000148028);
	pub const PINLIGHT_EXT : Self = Self(1000148029);
	pub const HARDMIX_EXT : Self = Self(1000148030);
	pub const HSL_HUE_EXT : Self = Self(1000148031);
	pub const HSL_SATURATION_EXT : Self = Self(1000148032);
	pub const HSL_COLOR_EXT : Self = Self(1000148033);
	pub const HSL_LUMINOSITY_EXT : Self = Self(1000148034);
	pub const PLUS_EXT : Self = Self(1000148035);
	pub const PLUS_CLAMPED_EXT : Self = Self(1000148036);
	pub const PLUS_CLAMPED_ALPHA_EXT : Self = Self(1000148037);
	pub const PLUS_DARKER_EXT : Self = Self(1000148038);
	pub const MINUS_EXT : Self = Self(1000148039);
	pub const MINUS_CLAMPED_EXT : Self = Self(1000148040);
	pub const CONTRAST_EXT : Self = Self(1000148041);
	pub const INVERT_OVG_EXT : Self = Self(1000148042);
	pub const RED_EXT : Self = Self(1000148043);
	pub const GREEN_EXT : Self = Self(1000148044);
	pub const BLUE_EXT : Self = Self(1000148045);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DisplayEventTypeEXT(pub i32);
impl DisplayEventTypeEXT {
	pub const FIRST_PIXEL_OUT_EXT : Self = Self(0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceEventTypeEXT(pub i32);
impl DeviceEventTypeEXT {
	pub const DISPLAY_HOTPLUG_EXT : Self = Self(0);
}

pub const VK_MAX_DRIVER_NAME_SIZE          : usize = 256usize;
pub const VK_UUID_SIZE                     : usize = 16usize;
pub const VK_LUID_SIZE                     : usize = 8usize;
pub const VK_MAX_EXTENSION_NAME_SIZE       : usize = 256usize;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE : usize = 256usize;
pub const VK_MAX_MEMORY_HEAPS              : usize = 16usize;
pub const VK_TRUE                          : i32 = 1i32;
pub const VK_FALSE                         : i32 = 0i32;
pub const VK_WHOLE_SIZE                    : u64 = 18446744073709551615u64;
pub const VK_SUBPASS_EXTERNAL              : u32 = 4294967295u32;
pub const VK_REMAINING_MIP_LEVELS          : u32 = 4294967295u32;
pub const VK_MAX_DRIVER_INFO_SIZE          : usize = 256usize;
pub const VK_MAX_DESCRIPTION_SIZE          : usize = 256usize;
pub const VK_REMAINING_ARRAY_LAYERS        : u32 = 4294967295u32;
pub const VK_QUEUE_FAMILY_IGNORED          : u32 = 4294967295u32;
pub const VK_MAX_MEMORY_TYPES              : usize = 32usize;
pub const VK_ATTACHMENT_UNUSED             : u32 = 4294967295u32;
pub const VK_MAX_DEVICE_GROUP_SIZE         : usize = 32usize;
pub const VK_QUEUE_FAMILY_EXTERNAL         : u32 = 4294967294u32;
pub const VK_LOD_CLAMP_NONE                : f32 = 1000f32;
pub type MetalSurfaceCreateFlagsEXT = Flags;
pub type MetalLayer = c_void;
pub type MacOSSurfaceCreateFlagsMVK = Flags;
pub type WaylandSurfaceCreateFlagsKHR = Flags;
pub type Win32SurfaceCreateFlagsKHR = Flags;
pub type PhysicalDeviceHostQueryResetFeaturesEXT = PhysicalDeviceHostQueryResetFeatures;
pub type AndroidSurfaceCreateFlagsKHR = Flags;
pub type BufferDeviceAddressInfoEXT = BufferDeviceAddressInfo;
pub type PhysicalDeviceBufferAddressFeaturesEXT = PhysicalDeviceBufferDeviceAddressFeaturesEXT;
pub type PhysicalDeviceScalarBlockLayoutFeaturesEXT = PhysicalDeviceScalarBlockLayoutFeatures;
pub type XcbSurfaceCreateFlagsKHR = Flags;
pub type QueryPoolCreateInfoINTEL = QueryPoolPerformanceQueryCreateInfoINTEL;
pub type AccelerationStructureInstanceNV = AccelerationStructureInstanceKHR;
pub type AabbPositionsNV = AabbPositionsKHR;
pub type TransformMatrixNV = TransformMatrixKHR;
pub type WriteDescriptorSetAccelerationStructureNV = WriteDescriptorSetAccelerationStructureKHR;
pub type IOSSurfaceCreateFlagsMVK = Flags;
pub type GeometryInstanceFlagBitsNV = GeometryInstanceFlagsKHR;
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
pub type GeometryFlagBitsNV = GeometryFlagsKHR;
pub type AccelerationStructureMemoryRequirementsTypeNV = AccelerationStructureMemoryRequirementsTypeKHR;
pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;
pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;
pub type GeometryTypeNV = GeometryTypeKHR;
pub type ExportMemoryAllocateInfoKHR = ExportMemoryAllocateInfo;
pub type ExternalMemoryImageCreateInfoKHR = ExternalMemoryImageCreateInfo;
pub type PhysicalDeviceIDPropertiesKHR = PhysicalDeviceIDProperties;
pub type ExternalImageFormatPropertiesKHR = ExternalImageFormatProperties;
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;
pub type ExternalFencePropertiesKHR = ExternalFenceProperties;
pub type DisplayModeCreateFlagsKHR = Flags;
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
pub type DeviceGroupRenderPassBeginInfoKHR = DeviceGroupRenderPassBeginInfo;
pub type PhysicalDevicePointClippingPropertiesKHR = PhysicalDevicePointClippingProperties;
pub type PhysicalDeviceMemoryProperties2KHR = PhysicalDeviceMemoryProperties2;
pub type FormatProperties2KHR = FormatProperties2;
pub type PhysicalDeviceProperties2KHR = PhysicalDeviceProperties2;
pub type PhysicalDeviceImagelessFramebufferFeaturesKHR = PhysicalDeviceImagelessFramebufferFeatures;
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT = DescriptorSetVariableDescriptorCountLayoutSupport;
pub type DisplaySurfaceCreateFlagsKHR = Flags;
pub type DeviceGroupCommandBufferBeginInfoKHR = DeviceGroupCommandBufferBeginInfo;
pub type PhysicalDeviceVulkanMemoryModelFeaturesKHR = PhysicalDeviceVulkanMemoryModelFeatures;
pub type PipelineInputAssemblyStateCreateFlags = Flags;
pub type PeerMemoryFeatureFlagBitsKHR = PeerMemoryFeatureFlags;
pub type PipelineDepthStencilStateCreateFlags = Flags;
pub type PhysicalDeviceShaderDrawParameterFeatures = PhysicalDeviceShaderDrawParametersFeatures;
pub type PhysicalDeviceVariablePointerFeatures = PhysicalDeviceVariablePointersFeatures;
pub type BuildAccelerationStructureFlagBitsNV = BuildAccelerationStructureFlagsKHR;
pub type RenderPassMultiviewCreateInfoKHR = RenderPassMultiviewCreateInfo;
pub type MemoryDedicatedAllocateInfoKHR = MemoryDedicatedAllocateInfo;
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
pub type ExternalBufferPropertiesKHR = ExternalBufferProperties;
pub type PhysicalDeviceExternalBufferInfoKHR = PhysicalDeviceExternalBufferInfo;
pub type DebugUtilsMessengerCallbackDataFlagsEXT = Flags;
pub type MemoryAllocateFlagsInfoKHR = MemoryAllocateFlagsInfo;
pub type DescriptorUpdateTemplateCreateFlags = Flags;
pub type SemaphoreTypeCreateInfoKHR = SemaphoreTypeCreateInfo;
pub type PhysicalDeviceGroupPropertiesKHR = PhysicalDeviceGroupProperties;
pub type ImageViewUsageCreateInfoKHR = ImageViewUsageCreateInfo;
pub type PipelineMultisampleStateCreateFlags = Flags;
pub type HeadlessSurfaceCreateFlagsEXT = Flags;
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT = DescriptorSetVariableDescriptorCountAllocateInfo;
pub type FenceImportFlagsKHR = FenceImportFlags;
pub type PipelineColorBlendStateCreateFlags = Flags;
pub type DeviceGroupBindSparseInfoKHR = DeviceGroupBindSparseInfo;
pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
pub type ExternalMemoryBufferCreateInfoKHR = ExternalMemoryBufferCreateInfo;
pub type MemoryAllocateFlagBitsKHR = MemoryAllocateFlags;
pub type Bool32 = u32;
pub type CommandPoolTrimFlags = Flags;
pub type PhysicalDeviceFeatures2KHR = PhysicalDeviceFeatures2;
pub type MemoryMapFlags = Flags;
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
pub type QueryPoolCreateFlags = Flags;
pub type DeviceCreateFlags = Flags;
pub type ExternalSemaphorePropertiesKHR = ExternalSemaphoreProperties;
pub type PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR = PhysicalDeviceUniformBufferStandardLayoutFeatures;
pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
pub type DescriptorPoolResetFlags = Flags;
pub type ExternalMemoryHandleTypeFlagBitsKHR = ExternalMemoryHandleTypeFlags;
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
pub type SemaphoreCreateFlags = Flags;
pub type ImageStencilUsageCreateInfoEXT = ImageStencilUsageCreateInfo;
pub type EventCreateFlags = Flags;
pub type DeviceSize = u64;
pub type PipelineTessellationDomainOriginStateCreateInfoKHR = PipelineTessellationDomainOriginStateCreateInfo;
pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
pub type PhysicalDeviceExternalImageFormatInfoKHR = PhysicalDeviceExternalImageFormatInfo;
pub type PipelineCoverageModulationStateCreateFlagsNV = Flags;
pub type PhysicalDeviceShaderFloat16Int8FeaturesKHR = PhysicalDeviceShaderFloat16Int8Features;
pub type SamplerYcbcrConversionInfoKHR = SamplerYcbcrConversionInfo;
pub type PhysicalDeviceImageFormatInfo2KHR = PhysicalDeviceImageFormatInfo2;
pub type DescriptorUpdateTemplateCreateInfoKHR = DescriptorUpdateTemplateCreateInfo;
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
pub type PipelineRasterizationStateCreateFlags = Flags;
pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;
pub type DeviceAddress = u64;
pub type ExternalFenceFeatureFlagBitsKHR = ExternalFenceFeatureFlags;
pub type InstanceCreateFlags = Flags;
pub type QueueFamilyProperties2KHR = QueueFamilyProperties2;
pub type ImageFormatProperties2KHR = ImageFormatProperties2;
pub type SubpassDescriptionDepthStencilResolveKHR = SubpassDescriptionDepthStencilResolve;
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
pub type PhysicalDeviceShaderAtomicInt64FeaturesKHR = PhysicalDeviceShaderAtomicInt64Features;
pub type BufferViewCreateFlags = Flags;
pub type DeviceGroupSubmitInfoKHR = DeviceGroupSubmitInfo;
pub type PhysicalDeviceMultiviewFeaturesKHR = PhysicalDeviceMultiviewFeatures;
pub type DescriptorSetLayoutSupportKHR = DescriptorSetLayoutSupport;
pub type PipelineVertexInputStateCreateFlags = Flags;
pub type PipelineTessellationStateCreateFlags = Flags;
pub type ExportFenceCreateInfoKHR = ExportFenceCreateInfo;
pub type PipelineViewportStateCreateFlags = Flags;
pub type PipelineLayoutCreateFlags = Flags;
pub type ExternalFenceHandleTypeFlagBitsKHR = ExternalFenceHandleTypeFlags;
pub type SemaphoreImportFlagBitsKHR = SemaphoreImportFlags;
pub type ExternalMemoryFeatureFlagBitsKHR = ExternalMemoryFeatureFlags;
pub type SemaphoreWaitInfoKHR = SemaphoreWaitInfo;
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
pub type ExternalSemaphoreHandleTypeFlagBitsKHR = ExternalSemaphoreHandleTypeFlags;
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
pub type SparseImageFormatProperties2KHR = SparseImageFormatProperties2;
pub type ImageSparseMemoryRequirementsInfo2KHR = ImageSparseMemoryRequirementsInfo2;
pub type ExportSemaphoreCreateInfoKHR = ExportSemaphoreCreateInfo;
pub type PhysicalDeviceDepthStencilResolvePropertiesKHR = PhysicalDeviceDepthStencilResolveProperties;
pub type SemaphoreWaitFlagBitsKHR = SemaphoreWaitFlags;
pub type PhysicalDevice16BitStorageFeaturesKHR = PhysicalDevice16BitStorageFeatures;
pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;
pub type SemaphoreSignalInfoKHR = SemaphoreSignalInfo;
pub type DebugUtilsMessengerCreateFlagsEXT = Flags;
pub type SamplerReductionModeCreateInfoEXT = SamplerReductionModeCreateInfo;
pub type DescriptorUpdateTemplateEntryKHR = DescriptorUpdateTemplateEntry;
pub type FramebufferAttachmentsCreateInfoKHR = FramebufferAttachmentsCreateInfo;
pub type FramebufferAttachmentImageInfoKHR = FramebufferAttachmentImageInfo;
pub type RenderPassAttachmentBeginInfoKHR = RenderPassAttachmentBeginInfo;
pub type DescriptorBindingFlagBitsEXT = DescriptorBindingFlags;
pub type AttachmentDescription2KHR = AttachmentDescription2;
pub type Flags = u32;
pub type AttachmentReference2KHR = AttachmentReference2;
pub type XlibSurfaceCreateFlagsKHR = Flags;
pub type BufferDeviceAddressInfoKHR = BufferDeviceAddressInfo;
pub type SubpassDescription2KHR = SubpassDescription2;
pub type BindImagePlaneMemoryInfoKHR = BindImagePlaneMemoryInfo;
pub type SubpassDependency2KHR = SubpassDependency2;
pub type SubpassBeginInfoKHR = SubpassBeginInfo;
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
pub type PhysicalDeviceExternalFenceInfoKHR = PhysicalDeviceExternalFenceInfo;
pub type BindAccelerationStructureMemoryInfoNV = BindAccelerationStructureMemoryInfoKHR;
pub type BindImageMemoryDeviceGroupInfoKHR = BindImageMemoryDeviceGroupInfo;
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT = PhysicalDeviceDescriptorIndexingProperties;
pub type PointClippingBehaviorKHR = PointClippingBehavior;
pub type PipelineViewportSwizzleStateCreateFlagsNV = Flags;
pub type PhysicalDeviceSparseImageFormatInfo2KHR = PhysicalDeviceSparseImageFormatInfo2;
pub type FenceImportFlagBitsKHR = FenceImportFlags;
pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
pub type ValidationCacheCreateFlagsEXT = Flags;
pub type ResolveModeFlagBitsKHR = ResolveModeFlags;
pub type RenderPassInputAttachmentAspectCreateInfoKHR = RenderPassInputAttachmentAspectCreateInfo;
pub type SparseImageMemoryRequirements2KHR = SparseImageMemoryRequirements2;
pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;
pub type RenderPassCreateInfo2KHR = RenderPassCreateInfo2;
pub type ConformanceVersionKHR = ConformanceVersion;
pub type PhysicalDeviceVariablePointersFeaturesKHR = PhysicalDeviceVariablePointersFeatures;
pub type MemoryDedicatedRequirementsKHR = MemoryDedicatedRequirements;
pub type ImagePlaneMemoryRequirementsInfoKHR = ImagePlaneMemoryRequirementsInfo;
pub type PhysicalDeviceExternalSemaphoreInfoKHR = PhysicalDeviceExternalSemaphoreInfo;
pub type BufferMemoryRequirementsInfo2KHR = BufferMemoryRequirementsInfo2;
pub type PhysicalDeviceVariablePointerFeaturesKHR = PhysicalDeviceVariablePointersFeatures;
pub type ImageMemoryRequirementsInfo2KHR = ImageMemoryRequirementsInfo2;
pub type MemoryRequirements2KHR = MemoryRequirements2;
pub type ImageFormatListCreateInfoKHR = ImageFormatListCreateInfo;
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
pub type PipelineCoverageReductionStateCreateFlagsNV = Flags;
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT = PhysicalDeviceDescriptorIndexingFeatures;
pub type ChromaLocationKHR = ChromaLocation;
pub type SamplerYcbcrConversionCreateInfoKHR = SamplerYcbcrConversionCreateInfo;
pub type AttachmentDescriptionStencilLayoutKHR = AttachmentDescriptionStencilLayout;
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR = PhysicalDeviceSamplerYcbcrConversionFeatures;
pub type SamplerYcbcrConversionImageFormatPropertiesKHR = SamplerYcbcrConversionImageFormatProperties;
pub type BindBufferMemoryInfoKHR = BindBufferMemoryInfo;
pub type BindImageMemoryInfoKHR = BindImageMemoryInfo;
pub type SemaphoreTypeKHR = SemaphoreType;
pub type PhysicalDeviceMaintenance3PropertiesKHR = PhysicalDeviceMaintenance3Properties;
pub type BindBufferMemoryDeviceGroupInfoKHR = BindBufferMemoryDeviceGroupInfo;
pub type PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR = PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
pub type PhysicalDeviceDriverPropertiesKHR = PhysicalDeviceDriverProperties;
pub type PhysicalDeviceMultiviewPropertiesKHR = PhysicalDeviceMultiviewProperties;
pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;
pub type ResolveModeFlagsKHR = ResolveModeFlags;
pub type PhysicalDeviceFloatControlsPropertiesKHR = PhysicalDeviceFloatControlsProperties;
pub type SubpassEndInfoKHR = SubpassEndInfo;
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR = PhysicalDeviceTimelineSemaphoreFeatures;
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR = PhysicalDeviceTimelineSemaphoreProperties;
pub type TimelineSemaphoreSubmitInfoKHR = TimelineSemaphoreSubmitInfo;
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR = PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
pub type PhysicalDevice8BitStorageFeaturesKHR = PhysicalDevice8BitStorageFeatures;
pub type AttachmentReferenceStencilLayoutKHR = AttachmentReferenceStencilLayout;
pub type GeometryFlagsNV = GeometryFlagsKHR;
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR = PhysicalDeviceBufferDeviceAddressFeatures;
pub type BufferOpaqueCaptureAddressCreateInfoKHR = BufferOpaqueCaptureAddressCreateInfo;
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR = MemoryOpaqueCaptureAddressAllocateInfo;
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR = DeviceMemoryOpaqueCaptureAddressInfo;
pub type PipelineRasterizationStateStreamCreateFlagsEXT = Flags;
pub type PipelineDiscardRectangleStateCreateFlagsEXT = Flags;
pub type PipelineRasterizationConservativeStateCreateFlagsEXT = Flags;
pub type PipelineDynamicStateCreateFlags = Flags;
pub type PipelineRasterizationDepthClipStateCreateFlagsEXT = Flags;
pub type DeviceGroupDeviceCreateInfoKHR = DeviceGroupDeviceCreateInfo;
pub type SampleMask = u32;
pub type PhysicalDeviceFloat16Int8FeaturesKHR = PhysicalDeviceShaderFloat16Int8Features;
pub type SamplerReductionModeEXT = SamplerReductionMode;
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT = PhysicalDeviceSamplerFilterMinmaxProperties;
pub type DriverIdKHR = DriverId;
pub type PipelineCoverageToColorStateCreateFlagsNV = Flags;
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
pub type ExternalSemaphoreFeatureFlagBitsKHR = ExternalSemaphoreFeatureFlags;
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT = DescriptorSetLayoutBindingFlagsCreateInfo;
pub type AccelerationStructureNV = AccelerationStructureKHR;
pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MacOSSurfaceCreateInfoMVK {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : MacOSSurfaceCreateFlagsMVK,
	pub p_view : *const c_void,
}

impl MacOSSurfaceCreateInfoMVK {
	pub fn new() -> Self {
		Self {
			s_type : 1000123000,
			p_next : null(),
			flags : <_>::default(),
			p_view : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : MacOSSurfaceCreateFlagsMVK) -> Self {
		self.flags = val;
		self
	}
	pub fn view(mut self, val : *const c_void) -> Self {
		self.p_view = val;
		self
	}
}

impl std::default::Default for MacOSSurfaceCreateInfoMVK {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct WaylandSurfaceCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : WaylandSurfaceCreateFlagsKHR,
	pub display : *mut wl_display,
	pub surface : *mut wl_surface,
}

impl WaylandSurfaceCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000006000,
			p_next : null(),
			flags : <_>::default(),
			display : null_mut(),
			surface : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : WaylandSurfaceCreateFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn display(mut self, val : *mut wl_display) -> Self {
		self.display = val;
		self
	}
	pub fn surface(mut self, val : *mut wl_surface) -> Self {
		self.surface = val;
		self
	}
}

impl std::default::Default for WaylandSurfaceCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub hmonitor : HMONITOR,
}

impl SurfaceFullScreenExclusiveWin32InfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000255001,
			p_next : null(),
			hmonitor : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn hmonitor(mut self, val : HMONITOR) -> Self {
		self.hmonitor = val;
		self
	}
}

impl std::default::Default for SurfaceFullScreenExclusiveWin32InfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub full_screen_exclusive_supported : Bool32,
}

impl SurfaceCapabilitiesFullScreenExclusiveEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000255002,
			p_next : null_mut(),
			full_screen_exclusive_supported : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn full_screen_exclusive_supported(mut self, val : Bool32) -> Self {
		self.full_screen_exclusive_supported = val;
		self
	}
}

impl std::default::Default for SurfaceCapabilitiesFullScreenExclusiveEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FenceGetWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub fence : Fence,
	pub handle_type : ExternalFenceHandleTypeFlags,
}

impl FenceGetWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000114002,
			p_next : null(),
			fence : Fence(0),
			handle_type : ExternalFenceHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fence(mut self, val : Fence) -> Self {
		self.fence = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalFenceHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for FenceGetWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExportFenceWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_attributes : *const SECURITY_ATTRIBUTES,
	pub dw_access : u32,
	pub name : *const u16,
}

impl ExportFenceWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000114001,
			p_next : null(),
			p_attributes : null(),
			dw_access : <_>::default(),
			name : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn attributes(mut self, val : *const SECURITY_ATTRIBUTES) -> Self {
		self.p_attributes = val;
		self
	}
	pub fn dw_access(mut self, val : u32) -> Self {
		self.dw_access = val;
		self
	}
	pub fn name(mut self, val : *const u16) -> Self {
		self.name = val;
		self
	}
}

impl std::default::Default for ExportFenceWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportFenceWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub fence : Fence,
	pub flags : FenceImportFlags,
	pub handle_type : ExternalFenceHandleTypeFlags,
	pub handle : HANDLE,
	pub name : *const u16,
}

impl ImportFenceWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000114000,
			p_next : null(),
			fence : Fence(0),
			flags : <_>::default(),
			handle_type : ExternalFenceHandleTypeFlags::default(),
			handle : <_>::default(),
			name : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fence(mut self, val : Fence) -> Self {
		self.fence = val;
		self
	}
	pub fn flags(mut self, val : FenceImportFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalFenceHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
	pub fn handle(mut self, val : HANDLE) -> Self {
		self.handle = val;
		self
	}
	pub fn name(mut self, val : *const u16) -> Self {
		self.name = val;
		self
	}
}

impl std::default::Default for ImportFenceWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SemaphoreGetWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub semaphore : Semaphore,
	pub handle_type : ExternalSemaphoreHandleTypeFlags,
}

impl SemaphoreGetWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000078003,
			p_next : null(),
			semaphore : Semaphore(0),
			handle_type : ExternalSemaphoreHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn semaphore(mut self, val : Semaphore) -> Self {
		self.semaphore = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalSemaphoreHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for SemaphoreGetWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub semaphore : Semaphore,
	pub flags : SemaphoreImportFlags,
	pub handle_type : ExternalSemaphoreHandleTypeFlags,
	pub handle : HANDLE,
	pub name : *const u16,
}

impl ImportSemaphoreWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000078000,
			p_next : null(),
			semaphore : Semaphore(0),
			flags : <_>::default(),
			handle_type : ExternalSemaphoreHandleTypeFlags::default(),
			handle : <_>::default(),
			name : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn semaphore(mut self, val : Semaphore) -> Self {
		self.semaphore = val;
		self
	}
	pub fn flags(mut self, val : SemaphoreImportFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalSemaphoreHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
	pub fn handle(mut self, val : HANDLE) -> Self {
		self.handle = val;
		self
	}
	pub fn name(mut self, val : *const u16) -> Self {
		self.name = val;
		self
	}
}

impl std::default::Default for ImportSemaphoreWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportMemoryWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_type : ExternalMemoryHandleTypeFlags,
	pub handle : HANDLE,
	pub name : *const u16,
}

impl ImportMemoryWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000073000,
			p_next : null(),
			handle_type : ExternalMemoryHandleTypeFlags::default(),
			handle : <_>::default(),
			name : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
	pub fn handle(mut self, val : HANDLE) -> Self {
		self.handle = val;
		self
	}
	pub fn name(mut self, val : *const u16) -> Self {
		self.name = val;
		self
	}
}

impl std::default::Default for ImportMemoryWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Win32SurfaceCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : Win32SurfaceCreateFlagsKHR,
	pub hinstance : HINSTANCE,
	pub hwnd : HWND,
}

impl Win32SurfaceCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000009000,
			p_next : null(),
			flags : <_>::default(),
			hinstance : null(),
			hwnd : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : Win32SurfaceCreateFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn hinstance(mut self, val : HINSTANCE) -> Self {
		self.hinstance = val;
		self
	}
	pub fn hwnd(mut self, val : HWND) -> Self {
		self.hwnd = val;
		self
	}
}

impl std::default::Default for Win32SurfaceCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalFormatANDROID {
	s_type : i32,
	pub p_next : *mut c_void,
	pub external_format : u64,
}

impl ExternalFormatANDROID {
	pub fn new() -> Self {
		Self {
			s_type : 1000129005,
			p_next : null_mut(),
			external_format : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn external_format(mut self, val : u64) -> Self {
		self.external_format = val;
		self
	}
}

impl std::default::Default for ExternalFormatANDROID {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
	s_type : i32,
	pub p_next : *mut c_void,
	pub format : Format,
	pub external_format : u64,
	pub format_features : FormatFeatureFlags,
	pub sampler_ycbcr_conversion_components : ComponentMapping,
	pub suggested_ycbcr_model : SamplerYcbcrModelConversion,
	pub suggested_ycbcr_range : SamplerYcbcrRange,
	pub suggested_xchroma_offset : ChromaLocation,
	pub suggested_ychroma_offset : ChromaLocation,
}

impl AndroidHardwareBufferFormatPropertiesANDROID {
	pub fn new() -> Self {
		Self {
			s_type : 1000129002,
			p_next : null_mut(),
			format : Format::default(),
			external_format : <_>::default(),
			format_features : <_>::default(),
			sampler_ycbcr_conversion_components : ComponentMapping::new(),
			suggested_ycbcr_model : SamplerYcbcrModelConversion::default(),
			suggested_ycbcr_range : SamplerYcbcrRange::default(),
			suggested_xchroma_offset : ChromaLocation::default(),
			suggested_ychroma_offset : ChromaLocation::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn external_format(mut self, val : u64) -> Self {
		self.external_format = val;
		self
	}
	pub fn format_features(mut self, val : FormatFeatureFlags) -> Self {
		self.format_features = val;
		self
	}
	pub fn sampler_ycbcr_conversion_components(mut self, val : ComponentMapping) -> Self {
		self.sampler_ycbcr_conversion_components = val;
		self
	}
	pub fn suggested_ycbcr_model(mut self, val : SamplerYcbcrModelConversion) -> Self {
		self.suggested_ycbcr_model = val;
		self
	}
	pub fn suggested_ycbcr_range(mut self, val : SamplerYcbcrRange) -> Self {
		self.suggested_ycbcr_range = val;
		self
	}
	pub fn suggested_xchroma_offset(mut self, val : ChromaLocation) -> Self {
		self.suggested_xchroma_offset = val;
		self
	}
	pub fn suggested_ychroma_offset(mut self, val : ChromaLocation) -> Self {
		self.suggested_ychroma_offset = val;
		self
	}
}

impl std::default::Default for AndroidHardwareBufferFormatPropertiesANDROID {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevice4444FormatsFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub format_a_4_r_4_g_4_b_4 : Bool32,
	pub format_a_4_b_4_g_4_r_4 : Bool32,
}

impl PhysicalDevice4444FormatsFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000340000,
			p_next : null_mut(),
			format_a_4_r_4_g_4_b_4 : <_>::default(),
			format_a_4_b_4_g_4_r_4 : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn format_a_4_r_4_g_4_b_4(mut self, val : Bool32) -> Self {
		self.format_a_4_r_4_g_4_b_4 = val;
		self
	}
	pub fn format_a_4_b_4_g_4_r_4(mut self, val : Bool32) -> Self {
		self.format_a_4_b_4_g_4_r_4 = val;
		self
	}
}

impl std::default::Default for PhysicalDevice4444FormatsFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub subsampled_loads : Bool32,
	pub subsampled_coarse_reconstruction_early_access : Bool32,
	pub max_subsampled_array_layers : u32,
	pub max_descriptor_set_subsampled_samplers : u32,
}

impl PhysicalDeviceFragmentDensityMap2PropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000332001,
			p_next : null_mut(),
			subsampled_loads : <_>::default(),
			subsampled_coarse_reconstruction_early_access : <_>::default(),
			max_subsampled_array_layers : <_>::default(),
			max_descriptor_set_subsampled_samplers : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn subsampled_loads(mut self, val : Bool32) -> Self {
		self.subsampled_loads = val;
		self
	}
	pub fn subsampled_coarse_reconstruction_early_access(mut self, val : Bool32) -> Self {
		self.subsampled_coarse_reconstruction_early_access = val;
		self
	}
	pub fn max_subsampled_array_layers(mut self, val : u32) -> Self {
		self.max_subsampled_array_layers = val;
		self
	}
	pub fn max_descriptor_set_subsampled_samplers(mut self, val : u32) -> Self {
		self.max_descriptor_set_subsampled_samplers = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFragmentDensityMap2PropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub fragment_density_map_deferred : Bool32,
}

impl PhysicalDeviceFragmentDensityMap2FeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000332000,
			p_next : null_mut(),
			fragment_density_map_deferred : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fragment_density_map_deferred(mut self, val : Bool32) -> Self {
		self.fragment_density_map_deferred = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFragmentDensityMap2FeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PrivateDataSlotCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PrivateDataSlotCreateFlagsEXT,
}

impl PrivateDataSlotCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000295002,
			p_next : null(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PrivateDataSlotCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for PrivateDataSlotCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub custom_border_color : ClearColorValue,
	pub format : Format,
}

impl SamplerCustomBorderColorCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000287000,
			p_next : null(),
			custom_border_color : ClearColorValue::new(),
			format : Format::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn custom_border_color(mut self, val : ClearColorValue) -> Self {
		self.custom_border_color = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
}

impl std::default::Default for SamplerCustomBorderColorCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM {
	s_type : i32,
	pub p_next : *mut c_void,
	pub transform : SurfaceTransformFlagsKHR,
	pub render_area : Rect2D,
}

impl CommandBufferInheritanceRenderPassTransformInfoQCOM {
	pub fn new() -> Self {
		Self {
			s_type : 1000282000,
			p_next : null_mut(),
			transform : SurfaceTransformFlagsKHR::default(),
			render_area : Rect2D::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn transform(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.transform = val;
		self
	}
	pub fn render_area(mut self, val : Rect2D) -> Self {
		self.render_area = val;
		self
	}
}

impl std::default::Default for CommandBufferInheritanceRenderPassTransformInfoQCOM {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub storage_texel_buffer_offset_alignment_bytes : DeviceSize,
	pub storage_texel_buffer_offset_single_texel_alignment : Bool32,
	pub uniform_texel_buffer_offset_alignment_bytes : DeviceSize,
	pub uniform_texel_buffer_offset_single_texel_alignment : Bool32,
}

impl PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000281001,
			p_next : null_mut(),
			storage_texel_buffer_offset_alignment_bytes : <_>::default(),
			storage_texel_buffer_offset_single_texel_alignment : <_>::default(),
			uniform_texel_buffer_offset_alignment_bytes : <_>::default(),
			uniform_texel_buffer_offset_single_texel_alignment : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn storage_texel_buffer_offset_alignment_bytes(mut self, val : DeviceSize) -> Self {
		self.storage_texel_buffer_offset_alignment_bytes = val;
		self
	}
	pub fn storage_texel_buffer_offset_single_texel_alignment(mut self, val : Bool32) -> Self {
		self.storage_texel_buffer_offset_single_texel_alignment = val;
		self
	}
	pub fn uniform_texel_buffer_offset_alignment_bytes(mut self, val : DeviceSize) -> Self {
		self.uniform_texel_buffer_offset_alignment_bytes = val;
		self
	}
	pub fn uniform_texel_buffer_offset_single_texel_alignment(mut self, val : Bool32) -> Self {
		self.uniform_texel_buffer_offset_single_texel_alignment = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct XcbSurfaceCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : XcbSurfaceCreateFlagsKHR,
	pub connection : *mut xcb_connection_t,
	pub window : xcb_window_t,
}

impl XcbSurfaceCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000005000,
			p_next : null(),
			flags : <_>::default(),
			connection : null_mut(),
			window : 0,
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : XcbSurfaceCreateFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn connection(mut self, val : *mut xcb_connection_t) -> Self {
		self.connection = val;
		self
	}
	pub fn window(mut self, val : xcb_window_t) -> Self {
		self.window = val;
		self
	}
}

impl std::default::Default for XcbSurfaceCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub pipeline_bind_point : PipelineBindPoint,
	pub pipeline : Pipeline,
	pub indirect_commands_layout : IndirectCommandsLayoutNV,
	pub max_sequences_count : u32,
}

impl GeneratedCommandsMemoryRequirementsInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000277006,
			p_next : null(),
			pipeline_bind_point : PipelineBindPoint::default(),
			pipeline : Pipeline(0),
			indirect_commands_layout : IndirectCommandsLayoutNV(0),
			max_sequences_count : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn pipeline_bind_point(mut self, val : PipelineBindPoint) -> Self {
		self.pipeline_bind_point = val;
		self
	}
	pub fn pipeline(mut self, val : Pipeline) -> Self {
		self.pipeline = val;
		self
	}
	pub fn indirect_commands_layout(mut self, val : IndirectCommandsLayoutNV) -> Self {
		self.indirect_commands_layout = val;
		self
	}
	pub fn max_sequences_count(mut self, val : u32) -> Self {
		self.max_sequences_count = val;
		self
	}
}

impl std::default::Default for GeneratedCommandsMemoryRequirementsInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GeneratedCommandsInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub pipeline_bind_point : PipelineBindPoint,
	pub pipeline : Pipeline,
	pub indirect_commands_layout : IndirectCommandsLayoutNV,
	pub stream_count : u32,
	pub p_streams : *const IndirectCommandsStreamNV,
	pub sequences_count : u32,
	pub preprocess_buffer : Buffer,
	pub preprocess_offset : DeviceSize,
	pub preprocess_size : DeviceSize,
	pub sequences_count_buffer : Buffer,
	pub sequences_count_offset : DeviceSize,
	pub sequences_index_buffer : Buffer,
	pub sequences_index_offset : DeviceSize,
}

impl GeneratedCommandsInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000277005,
			p_next : null(),
			pipeline_bind_point : PipelineBindPoint::default(),
			pipeline : Pipeline(0),
			indirect_commands_layout : IndirectCommandsLayoutNV(0),
			stream_count : <_>::default(),
			p_streams : null(),
			sequences_count : <_>::default(),
			preprocess_buffer : Buffer(0),
			preprocess_offset : <_>::default(),
			preprocess_size : <_>::default(),
			sequences_count_buffer : Buffer(0),
			sequences_count_offset : <_>::default(),
			sequences_index_buffer : Buffer(0),
			sequences_index_offset : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn pipeline_bind_point(mut self, val : PipelineBindPoint) -> Self {
		self.pipeline_bind_point = val;
		self
	}
	pub fn pipeline(mut self, val : Pipeline) -> Self {
		self.pipeline = val;
		self
	}
	pub fn indirect_commands_layout(mut self, val : IndirectCommandsLayoutNV) -> Self {
		self.indirect_commands_layout = val;
		self
	}
	pub fn sequences_count(mut self, val : u32) -> Self {
		self.sequences_count = val;
		self
	}
	pub fn preprocess_buffer(mut self, val : Buffer) -> Self {
		self.preprocess_buffer = val;
		self
	}
	pub fn preprocess_offset(mut self, val : DeviceSize) -> Self {
		self.preprocess_offset = val;
		self
	}
	pub fn preprocess_size(mut self, val : DeviceSize) -> Self {
		self.preprocess_size = val;
		self
	}
	pub fn sequences_count_buffer(mut self, val : Buffer) -> Self {
		self.sequences_count_buffer = val;
		self
	}
	pub fn sequences_count_offset(mut self, val : DeviceSize) -> Self {
		self.sequences_count_offset = val;
		self
	}
	pub fn sequences_index_buffer(mut self, val : Buffer) -> Self {
		self.sequences_index_buffer = val;
		self
	}
	pub fn sequences_index_offset(mut self, val : DeviceSize) -> Self {
		self.sequences_index_offset = val;
		self
	}
	pub fn streams(mut self, val : &[IndirectCommandsStreamNV]) -> Self {
		self.stream_count = val.len() as _;
		self.p_streams = val.as_ptr();
		self
	}
	pub fn stream(mut self, val : &IndirectCommandsStreamNV) -> Self {
		self.stream_count = 1;
		self.p_streams = val;
		self
	}
}

impl std::default::Default for GeneratedCommandsInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct IndirectCommandsLayoutCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : IndirectCommandsLayoutUsageFlagsNV,
	pub pipeline_bind_point : PipelineBindPoint,
	pub token_count : u32,
	pub p_tokens : *const IndirectCommandsLayoutTokenNV,
	pub stream_count : u32,
	pub p_stream_strides : *const u32,
}

impl IndirectCommandsLayoutCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000277004,
			p_next : null(),
			flags : <_>::default(),
			pipeline_bind_point : PipelineBindPoint::default(),
			token_count : <_>::default(),
			p_tokens : null(),
			stream_count : <_>::default(),
			p_stream_strides : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : IndirectCommandsLayoutUsageFlagsNV) -> Self {
		self.flags = val;
		self
	}
	pub fn pipeline_bind_point(mut self, val : PipelineBindPoint) -> Self {
		self.pipeline_bind_point = val;
		self
	}
	pub fn tokens(mut self, val : &[IndirectCommandsLayoutTokenNV]) -> Self {
		self.token_count = val.len() as _;
		self.p_tokens = val.as_ptr();
		self
	}
	pub fn token(mut self, val : &IndirectCommandsLayoutTokenNV) -> Self {
		self.token_count = 1;
		self.p_tokens = val;
		self
	}
	pub fn stream_strides(mut self, val : &[u32]) -> Self {
		self.stream_count = val.len() as _;
		self.p_stream_strides = val.as_ptr();
		self
	}
	pub fn stream_stride(mut self, val : &u32) -> Self {
		self.stream_count = 1;
		self.p_stream_strides = val;
		self
	}
}

impl std::default::Default for IndirectCommandsLayoutCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct IndirectCommandsStreamNV {
	pub buffer : Buffer,
	pub offset : DeviceSize,
}

impl IndirectCommandsStreamNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
	pub fn offset(mut self, val : DeviceSize) -> Self {
		self.offset = val;
		self
	}
}

impl std::default::Default for IndirectCommandsStreamNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindVertexBufferIndirectCommandNV {
	pub buffer_address : DeviceAddress,
	pub size : u32,
	pub stride : u32,
}

impl BindVertexBufferIndirectCommandNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn buffer_address(mut self, val : DeviceAddress) -> Self {
		self.buffer_address = val;
		self
	}
	pub fn size(mut self, val : u32) -> Self {
		self.size = val;
		self
	}
	pub fn stride(mut self, val : u32) -> Self {
		self.stride = val;
		self
	}
}

impl std::default::Default for BindVertexBufferIndirectCommandNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindShaderGroupIndirectCommandNV {
	pub group_index : u32,
}

impl BindShaderGroupIndirectCommandNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
}

impl std::default::Default for BindShaderGroupIndirectCommandNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_demote_to_helper_invocation : Bool32,
}

impl PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000276000,
			p_next : null_mut(),
			shader_demote_to_helper_invocation : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_demote_to_helper_invocation(mut self, val : Bool32) -> Self {
		self.shader_demote_to_helper_invocation = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MetalSurfaceCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : MetalSurfaceCreateFlagsEXT,
	pub p_layer : *const CAMetalLayer,
}

impl MetalSurfaceCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000217000,
			p_next : null(),
			flags : <_>::default(),
			p_layer : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : MetalSurfaceCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn layer(mut self, val : *const CAMetalLayer) -> Self {
		self.p_layer = val;
		self
	}
}

impl std::default::Default for MetalSurfaceCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SetStateFlagsIndirectCommandNV {
	pub data : u32,
}

impl SetStateFlagsIndirectCommandNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
}

impl std::default::Default for SetStateFlagsIndirectCommandNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub rectangular_lines : Bool32,
	pub bresenham_lines : Bool32,
	pub smooth_lines : Bool32,
	pub stippled_rectangular_lines : Bool32,
	pub stippled_bresenham_lines : Bool32,
	pub stippled_smooth_lines : Bool32,
}

impl PhysicalDeviceLineRasterizationFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000259000,
			p_next : null_mut(),
			rectangular_lines : <_>::default(),
			bresenham_lines : <_>::default(),
			smooth_lines : <_>::default(),
			stippled_rectangular_lines : <_>::default(),
			stippled_bresenham_lines : <_>::default(),
			stippled_smooth_lines : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn rectangular_lines(mut self, val : Bool32) -> Self {
		self.rectangular_lines = val;
		self
	}
	pub fn bresenham_lines(mut self, val : Bool32) -> Self {
		self.bresenham_lines = val;
		self
	}
	pub fn smooth_lines(mut self, val : Bool32) -> Self {
		self.smooth_lines = val;
		self
	}
	pub fn stippled_rectangular_lines(mut self, val : Bool32) -> Self {
		self.stippled_rectangular_lines = val;
		self
	}
	pub fn stippled_bresenham_lines(mut self, val : Bool32) -> Self {
		self.stippled_bresenham_lines = val;
		self
	}
	pub fn stippled_smooth_lines(mut self, val : Bool32) -> Self {
		self.stippled_smooth_lines = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceLineRasterizationFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct HeadlessSurfaceCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : HeadlessSurfaceCreateFlagsEXT,
}

impl HeadlessSurfaceCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000256000,
			p_next : null(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : HeadlessSurfaceCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for HeadlessSurfaceCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub fragment_shader_sample_interlock : Bool32,
	pub fragment_shader_pixel_interlock : Bool32,
	pub fragment_shader_shading_rate_interlock : Bool32,
}

impl PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000251000,
			p_next : null_mut(),
			fragment_shader_sample_interlock : <_>::default(),
			fragment_shader_pixel_interlock : <_>::default(),
			fragment_shader_shading_rate_interlock : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fragment_shader_sample_interlock(mut self, val : Bool32) -> Self {
		self.fragment_shader_sample_interlock = val;
		self
	}
	pub fn fragment_shader_pixel_interlock(mut self, val : Bool32) -> Self {
		self.fragment_shader_pixel_interlock = val;
		self
	}
	pub fn fragment_shader_shading_rate_interlock(mut self, val : Bool32) -> Self {
		self.fragment_shader_shading_rate_interlock = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub cooperative_matrix_supported_stages : ShaderStageFlags,
}

impl PhysicalDeviceCooperativeMatrixPropertiesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000249002,
			p_next : null_mut(),
			cooperative_matrix_supported_stages : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn cooperative_matrix_supported_stages(mut self, val : ShaderStageFlags) -> Self {
		self.cooperative_matrix_supported_stages = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceCooperativeMatrixPropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceToolPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub name : [u8; VK_MAX_EXTENSION_NAME_SIZE],
	pub version : [u8; VK_MAX_EXTENSION_NAME_SIZE],
	pub purposes : ToolPurposeFlagsEXT,
	pub description : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub layer : [u8; VK_MAX_EXTENSION_NAME_SIZE],
}

impl PhysicalDeviceToolPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000245000,
			p_next : null_mut(),
			name : [0 as _ ;VK_MAX_EXTENSION_NAME_SIZE],
			version : [0 as _ ;VK_MAX_EXTENSION_NAME_SIZE],
			purposes : <_>::default(),
			description : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			layer : [0 as _ ;VK_MAX_EXTENSION_NAME_SIZE],
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn name(mut self, val : [u8; VK_MAX_EXTENSION_NAME_SIZE]) -> Self {
		self.name = val;
		self
	}
	pub fn version(mut self, val : [u8; VK_MAX_EXTENSION_NAME_SIZE]) -> Self {
		self.version = val;
		self
	}
	pub fn purposes(mut self, val : ToolPurposeFlagsEXT) -> Self {
		self.purposes = val;
		self
	}
	pub fn description(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.description = val;
		self
	}
	pub fn layer(mut self, val : [u8; VK_MAX_EXTENSION_NAME_SIZE]) -> Self {
		self.layer = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceToolPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferDeviceAddressCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub device_address : DeviceAddress,
}

impl BufferDeviceAddressCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000244002,
			p_next : null(),
			device_address : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_address(mut self, val : DeviceAddress) -> Self {
		self.device_address = val;
		self
	}
}

impl std::default::Default for BufferDeviceAddressCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceRobustness2PropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub robust_storage_buffer_access_size_alignment : DeviceSize,
	pub robust_uniform_buffer_access_size_alignment : DeviceSize,
}

impl PhysicalDeviceRobustness2PropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000286001,
			p_next : null_mut(),
			robust_storage_buffer_access_size_alignment : <_>::default(),
			robust_uniform_buffer_access_size_alignment : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn robust_storage_buffer_access_size_alignment(mut self, val : DeviceSize) -> Self {
		self.robust_storage_buffer_access_size_alignment = val;
		self
	}
	pub fn robust_uniform_buffer_access_size_alignment(mut self, val : DeviceSize) -> Self {
		self.robust_uniform_buffer_access_size_alignment = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceRobustness2PropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub dedicated_allocation_image_aliasing : Bool32,
}

impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000240000,
			p_next : null_mut(),
			dedicated_allocation_image_aliasing : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn dedicated_allocation_image_aliasing(mut self, val : Bool32) -> Self {
		self.dedicated_allocation_image_aliasing = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GraphicsShaderGroupCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub stage_count : u32,
	pub p_stages : *const PipelineShaderStageCreateInfo,
	pub p_vertex_input_state : *const PipelineVertexInputStateCreateInfo,
	pub p_tessellation_state : *const PipelineTessellationStateCreateInfo,
}

impl GraphicsShaderGroupCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000277001,
			p_next : null(),
			stage_count : <_>::default(),
			p_stages : null(),
			p_vertex_input_state : null(),
			p_tessellation_state : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn vertex_input_state(mut self, val : *const PipelineVertexInputStateCreateInfo) -> Self {
		self.p_vertex_input_state = val;
		self
	}
	pub fn tessellation_state(mut self, val : *const PipelineTessellationStateCreateInfo) -> Self {
		self.p_tessellation_state = val;
		self
	}
	pub fn stages(mut self, val : &[PipelineShaderStageCreateInfo]) -> Self {
		self.stage_count = val.len() as _;
		self.p_stages = val.as_ptr();
		self
	}
	pub fn stage(mut self, val : &PipelineShaderStageCreateInfo) -> Self {
		self.stage_count = 1;
		self.p_stages = val;
		self
	}
}

impl std::default::Default for GraphicsShaderGroupCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub line_sub_pixel_precision_bits : u32,
}

impl PhysicalDeviceLineRasterizationPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000259002,
			p_next : null_mut(),
			line_sub_pixel_precision_bits : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn line_sub_pixel_precision_bits(mut self, val : u32) -> Self {
		self.line_sub_pixel_precision_bits = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceLineRasterizationPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
	s_type : i32,
	pub p_next : *mut c_void,
	pub device_coherent_memory : Bool32,
}

impl PhysicalDeviceCoherentMemoryFeaturesAMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000229000,
			p_next : null_mut(),
			device_coherent_memory : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_coherent_memory(mut self, val : Bool32) -> Self {
		self.device_coherent_memory = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceCoherentMemoryFeaturesAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_core_features : ShaderCorePropertiesFlagsAMD,
	pub active_compute_unit_count : u32,
}

impl PhysicalDeviceShaderCoreProperties2AMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000227000,
			p_next : null_mut(),
			shader_core_features : <_>::default(),
			active_compute_unit_count : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_core_features(mut self, val : ShaderCorePropertiesFlagsAMD) -> Self {
		self.shader_core_features = val;
		self
	}
	pub fn active_compute_unit_count(mut self, val : u32) -> Self {
		self.active_compute_unit_count = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderCoreProperties2AMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub fragment_density_map_attachment : AttachmentReference,
}

impl RenderPassFragmentDensityMapCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000218002,
			p_next : null(),
			fragment_density_map_attachment : AttachmentReference::new(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fragment_density_map_attachment(mut self, val : AttachmentReference) -> Self {
		self.fragment_density_map_attachment = val;
		self
	}
}

impl std::default::Default for RenderPassFragmentDensityMapCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub min_fragment_density_texel_size : Extent2D,
	pub max_fragment_density_texel_size : Extent2D,
	pub fragment_density_invocations : Bool32,
}

impl PhysicalDeviceFragmentDensityMapPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000218001,
			p_next : null_mut(),
			min_fragment_density_texel_size : Extent2D::new(),
			max_fragment_density_texel_size : Extent2D::new(),
			fragment_density_invocations : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn min_fragment_density_texel_size(mut self, val : Extent2D) -> Self {
		self.min_fragment_density_texel_size = val;
		self
	}
	pub fn max_fragment_density_texel_size(mut self, val : Extent2D) -> Self {
		self.max_fragment_density_texel_size = val;
		self
	}
	pub fn fragment_density_invocations(mut self, val : Bool32) -> Self {
		self.fragment_density_invocations = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFragmentDensityMapPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub fragment_density_map : Bool32,
	pub fragment_density_map_dynamic : Bool32,
	pub fragment_density_map_non_subsampled_images : Bool32,
}

impl PhysicalDeviceFragmentDensityMapFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000218000,
			p_next : null_mut(),
			fragment_density_map : <_>::default(),
			fragment_density_map_dynamic : <_>::default(),
			fragment_density_map_non_subsampled_images : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fragment_density_map(mut self, val : Bool32) -> Self {
		self.fragment_density_map = val;
		self
	}
	pub fn fragment_density_map_dynamic(mut self, val : Bool32) -> Self {
		self.fragment_density_map_dynamic = val;
		self
	}
	pub fn fragment_density_map_non_subsampled_images(mut self, val : Bool32) -> Self {
		self.fragment_density_map_non_subsampled_images = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFragmentDensityMapFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevicePCIBusInfoPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub pci_domain : u32,
	pub pci_bus : u32,
	pub pci_device : u32,
	pub pci_function : u32,
}

impl PhysicalDevicePCIBusInfoPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000212000,
			p_next : null_mut(),
			pci_domain : <_>::default(),
			pci_bus : <_>::default(),
			pci_device : <_>::default(),
			pci_function : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn pci_domain(mut self, val : u32) -> Self {
		self.pci_domain = val;
		self
	}
	pub fn pci_bus(mut self, val : u32) -> Self {
		self.pci_bus = val;
		self
	}
	pub fn pci_device(mut self, val : u32) -> Self {
		self.pci_device = val;
		self
	}
	pub fn pci_function(mut self, val : u32) -> Self {
		self.pci_function = val;
		self
	}
}

impl std::default::Default for PhysicalDevicePCIBusInfoPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerformanceStreamMarkerInfoINTEL {
	s_type : i32,
	pub p_next : *const c_void,
	pub marker : u32,
}

impl PerformanceStreamMarkerInfoINTEL {
	pub fn new() -> Self {
		Self {
			s_type : 1000210003,
			p_next : null(),
			marker : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn marker(mut self, val : u32) -> Self {
		self.marker = val;
		self
	}
}

impl std::default::Default for PerformanceStreamMarkerInfoINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerformanceValueDataINTEL {
	pub value_32 : u32,
	pub value_64 : u64,
	pub value_float : f32,
	pub value_bool : Bool32,
	pub value_string : *const u8,
}

impl PerformanceValueDataINTEL {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn value_32(mut self, val : u32) -> Self {
		self.value_32 = val;
		self
	}
	pub fn value_64(mut self, val : u64) -> Self {
		self.value_64 = val;
		self
	}
	pub fn value_float(mut self, val : f32) -> Self {
		self.value_float = val;
		self
	}
	pub fn value_bool(mut self, val : Bool32) -> Self {
		self.value_bool = val;
		self
	}
	pub fn value_string(mut self, val : *const u8) -> Self {
		self.value_string = val;
		self
	}
}

impl std::default::Default for PerformanceValueDataINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CheckpointDataNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub stage : PipelineStageFlags,
	pub p_checkpoint_marker : *mut c_void,
}

impl CheckpointDataNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000206000,
			p_next : null_mut(),
			stage : PipelineStageFlags::default(),
			p_checkpoint_marker : null_mut(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn stage(mut self, val : PipelineStageFlags) -> Self {
		self.stage = val;
		self
	}
	pub fn checkpoint_marker(mut self, val : *mut c_void) -> Self {
		self.p_checkpoint_marker = val;
		self
	}
}

impl std::default::Default for CheckpointDataNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct QueueFamilyCheckpointPropertiesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub checkpoint_execution_stage_mask : PipelineStageFlags,
}

impl QueueFamilyCheckpointPropertiesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000206001,
			p_next : null_mut(),
			checkpoint_execution_stage_mask : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn checkpoint_execution_stage_mask(mut self, val : PipelineStageFlags) -> Self {
		self.checkpoint_execution_stage_mask = val;
		self
	}
}

impl std::default::Default for QueueFamilyCheckpointPropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub exclusive_scissor_count : u32,
	pub p_exclusive_scissors : *const Rect2D,
}

impl PipelineViewportExclusiveScissorStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000205000,
			p_next : null(),
			exclusive_scissor_count : <_>::default(),
			p_exclusive_scissors : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn exclusive_scissors(mut self, val : &[Rect2D]) -> Self {
		self.exclusive_scissor_count = val.len() as _;
		self.p_exclusive_scissors = val.as_ptr();
		self
	}
	pub fn exclusive_scissor(mut self, val : &Rect2D) -> Self {
		self.exclusive_scissor_count = 1;
		self.p_exclusive_scissors = val;
		self
	}
}

impl std::default::Default for PipelineViewportExclusiveScissorStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub fragment_shader_barycentric : Bool32,
}

impl PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000203000,
			p_next : null_mut(),
			fragment_shader_barycentric : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fragment_shader_barycentric(mut self, val : Bool32) -> Self {
		self.fragment_shader_barycentric = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub task_shader : Bool32,
	pub mesh_shader : Bool32,
}

impl PhysicalDeviceMeshShaderFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000202000,
			p_next : null_mut(),
			task_shader : <_>::default(),
			mesh_shader : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn task_shader(mut self, val : Bool32) -> Self {
		self.task_shader = val;
		self
	}
	pub fn mesh_shader(mut self, val : Bool32) -> Self {
		self.mesh_shader = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMeshShaderFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub compute_derivative_group_quads : Bool32,
	pub compute_derivative_group_linear : Bool32,
}

impl PhysicalDeviceComputeShaderDerivativesFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000201000,
			p_next : null_mut(),
			compute_derivative_group_quads : <_>::default(),
			compute_derivative_group_linear : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn compute_derivative_group_quads(mut self, val : Bool32) -> Self {
		self.compute_derivative_group_quads = val;
		self
	}
	pub fn compute_derivative_group_linear(mut self, val : Bool32) -> Self {
		self.compute_derivative_group_linear = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub vertex_attribute_instance_rate_divisor : Bool32,
	pub vertex_attribute_instance_rate_zero_divisor : Bool32,
}

impl PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000190002,
			p_next : null_mut(),
			vertex_attribute_instance_rate_divisor : <_>::default(),
			vertex_attribute_instance_rate_zero_divisor : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn vertex_attribute_instance_rate_divisor(mut self, val : Bool32) -> Self {
		self.vertex_attribute_instance_rate_divisor = val;
		self
	}
	pub fn vertex_attribute_instance_rate_zero_divisor(mut self, val : Bool32) -> Self {
		self.vertex_attribute_instance_rate_zero_divisor = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct VertexInputBindingDivisorDescriptionEXT {
	pub binding : u32,
	pub divisor : u32,
}

impl VertexInputBindingDivisorDescriptionEXT {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn binding(mut self, val : u32) -> Self {
		self.binding = val;
		self
	}
	pub fn divisor(mut self, val : u32) -> Self {
		self.divisor = val;
		self
	}
}

impl std::default::Default for VertexInputBindingDivisorDescriptionEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_engine_count : u32,
	pub shader_arrays_per_engine_count : u32,
	pub compute_units_per_shader_array : u32,
	pub simd_per_compute_unit : u32,
	pub wavefronts_per_simd : u32,
	pub wavefront_size : u32,
	pub sgprs_per_simd : u32,
	pub min_sgpr_allocation : u32,
	pub max_sgpr_allocation : u32,
	pub sgpr_allocation_granularity : u32,
	pub vgprs_per_simd : u32,
	pub min_vgpr_allocation : u32,
	pub max_vgpr_allocation : u32,
	pub vgpr_allocation_granularity : u32,
}

impl PhysicalDeviceShaderCorePropertiesAMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000185000,
			p_next : null_mut(),
			shader_engine_count : <_>::default(),
			shader_arrays_per_engine_count : <_>::default(),
			compute_units_per_shader_array : <_>::default(),
			simd_per_compute_unit : <_>::default(),
			wavefronts_per_simd : <_>::default(),
			wavefront_size : <_>::default(),
			sgprs_per_simd : <_>::default(),
			min_sgpr_allocation : <_>::default(),
			max_sgpr_allocation : <_>::default(),
			sgpr_allocation_granularity : <_>::default(),
			vgprs_per_simd : <_>::default(),
			min_vgpr_allocation : <_>::default(),
			max_vgpr_allocation : <_>::default(),
			vgpr_allocation_granularity : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_engine_count(mut self, val : u32) -> Self {
		self.shader_engine_count = val;
		self
	}
	pub fn shader_arrays_per_engine_count(mut self, val : u32) -> Self {
		self.shader_arrays_per_engine_count = val;
		self
	}
	pub fn compute_units_per_shader_array(mut self, val : u32) -> Self {
		self.compute_units_per_shader_array = val;
		self
	}
	pub fn simd_per_compute_unit(mut self, val : u32) -> Self {
		self.simd_per_compute_unit = val;
		self
	}
	pub fn wavefronts_per_simd(mut self, val : u32) -> Self {
		self.wavefronts_per_simd = val;
		self
	}
	pub fn wavefront_size(mut self, val : u32) -> Self {
		self.wavefront_size = val;
		self
	}
	pub fn sgprs_per_simd(mut self, val : u32) -> Self {
		self.sgprs_per_simd = val;
		self
	}
	pub fn min_sgpr_allocation(mut self, val : u32) -> Self {
		self.min_sgpr_allocation = val;
		self
	}
	pub fn max_sgpr_allocation(mut self, val : u32) -> Self {
		self.max_sgpr_allocation = val;
		self
	}
	pub fn sgpr_allocation_granularity(mut self, val : u32) -> Self {
		self.sgpr_allocation_granularity = val;
		self
	}
	pub fn vgprs_per_simd(mut self, val : u32) -> Self {
		self.vgprs_per_simd = val;
		self
	}
	pub fn min_vgpr_allocation(mut self, val : u32) -> Self {
		self.min_vgpr_allocation = val;
		self
	}
	pub fn max_vgpr_allocation(mut self, val : u32) -> Self {
		self.max_vgpr_allocation = val;
		self
	}
	pub fn vgpr_allocation_granularity(mut self, val : u32) -> Self {
		self.vgpr_allocation_granularity = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderCorePropertiesAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineCompilerControlCreateInfoAMD {
	s_type : i32,
	pub p_next : *const c_void,
	pub compiler_control_flags : PipelineCompilerControlFlagsAMD,
}

impl PipelineCompilerControlCreateInfoAMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000183000,
			p_next : null(),
			compiler_control_flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn compiler_control_flags(mut self, val : PipelineCompilerControlFlagsAMD) -> Self {
		self.compiler_control_flags = val;
		self
	}
}

impl std::default::Default for PipelineCompilerControlCreateInfoAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub filter_cubic : Bool32,
	pub filter_cubic_minmax : Bool32,
}

impl FilterCubicImageViewImageFormatPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000170001,
			p_next : null_mut(),
			filter_cubic : <_>::default(),
			filter_cubic_minmax : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn filter_cubic(mut self, val : Bool32) -> Self {
		self.filter_cubic = val;
		self
	}
	pub fn filter_cubic_minmax(mut self, val : Bool32) -> Self {
		self.filter_cubic_minmax = val;
		self
	}
}

impl std::default::Default for FilterCubicImageViewImageFormatPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AndroidHardwareBufferUsageANDROID {
	s_type : i32,
	pub p_next : *mut c_void,
	pub android_hardware_buffer_usage : u64,
}

impl AndroidHardwareBufferUsageANDROID {
	pub fn new() -> Self {
		Self {
			s_type : 1000129000,
			p_next : null_mut(),
			android_hardware_buffer_usage : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn android_hardware_buffer_usage(mut self, val : u64) -> Self {
		self.android_hardware_buffer_usage = val;
		self
	}
}

impl std::default::Default for AndroidHardwareBufferUsageANDROID {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub representative_fragment_test_enable : Bool32,
}

impl PipelineRepresentativeFragmentTestStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000166001,
			p_next : null(),
			representative_fragment_test_enable : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn representative_fragment_test_enable(mut self, val : Bool32) -> Self {
		self.representative_fragment_test_enable = val;
		self
	}
}

impl std::default::Default for PipelineRepresentativeFragmentTestStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub representative_fragment_test : Bool32,
}

impl PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000166000,
			p_next : null_mut(),
			representative_fragment_test : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn representative_fragment_test(mut self, val : Bool32) -> Self {
		self.representative_fragment_test = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct AccelerationStructureInstanceKHR {
	pub transform : TransformMatrixKHR,
	pub instance_custom_inde : u32,
	pub instance_shader_binding_table_record_offse : u32,
	pub acceleration_structure_reference : u64,
}

impl AccelerationStructureInstanceKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn transform(mut self, val : TransformMatrixKHR) -> Self {
		self.transform = val;
		self
	}
	pub fn instance_custom_inde(mut self, val : u32) -> Self {
		self.instance_custom_inde = val;
		self
	}
	pub fn instance_shader_binding_table_record_offse(mut self, val : u32) -> Self {
		self.instance_shader_binding_table_record_offse = val;
		self
	}
	pub fn acceleration_structure_reference(mut self, val : u64) -> Self {
		self.acceleration_structure_reference = val;
		self
	}
}

impl std::default::Default for AccelerationStructureInstanceKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AabbPositionsKHR {
	pub min_x : f32,
	pub min_y : f32,
	pub min_z : f32,
	pub max_x : f32,
	pub max_y : f32,
	pub max_z : f32,
}

impl AabbPositionsKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn min_x(mut self, val : f32) -> Self {
		self.min_x = val;
		self
	}
	pub fn min_y(mut self, val : f32) -> Self {
		self.min_y = val;
		self
	}
	pub fn min_z(mut self, val : f32) -> Self {
		self.min_z = val;
		self
	}
	pub fn max_x(mut self, val : f32) -> Self {
		self.max_x = val;
		self
	}
	pub fn max_y(mut self, val : f32) -> Self {
		self.max_y = val;
		self
	}
	pub fn max_z(mut self, val : f32) -> Self {
		self.max_z = val;
		self
	}
}

impl std::default::Default for AabbPositionsKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerformanceMarkerInfoINTEL {
	s_type : i32,
	pub p_next : *const c_void,
	pub marker : u64,
}

impl PerformanceMarkerInfoINTEL {
	pub fn new() -> Self {
		Self {
			s_type : 1000210002,
			p_next : null(),
			marker : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn marker(mut self, val : u64) -> Self {
		self.marker = val;
		self
	}
}

impl std::default::Default for PerformanceMarkerInfoINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceRayTracingPropertiesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_group_handle_size : u32,
	pub max_recursion_depth : u32,
	pub max_shader_group_stride : u32,
	pub shader_group_base_alignment : u32,
	pub max_geometry_count : u64,
	pub max_instance_count : u64,
	pub max_triangle_count : u64,
	pub max_descriptor_set_acceleration_structures : u32,
}

impl PhysicalDeviceRayTracingPropertiesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165009,
			p_next : null_mut(),
			shader_group_handle_size : <_>::default(),
			max_recursion_depth : <_>::default(),
			max_shader_group_stride : <_>::default(),
			shader_group_base_alignment : <_>::default(),
			max_geometry_count : <_>::default(),
			max_instance_count : <_>::default(),
			max_triangle_count : <_>::default(),
			max_descriptor_set_acceleration_structures : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_group_handle_size(mut self, val : u32) -> Self {
		self.shader_group_handle_size = val;
		self
	}
	pub fn max_recursion_depth(mut self, val : u32) -> Self {
		self.max_recursion_depth = val;
		self
	}
	pub fn max_shader_group_stride(mut self, val : u32) -> Self {
		self.max_shader_group_stride = val;
		self
	}
	pub fn shader_group_base_alignment(mut self, val : u32) -> Self {
		self.shader_group_base_alignment = val;
		self
	}
	pub fn max_geometry_count(mut self, val : u64) -> Self {
		self.max_geometry_count = val;
		self
	}
	pub fn max_instance_count(mut self, val : u64) -> Self {
		self.max_instance_count = val;
		self
	}
	pub fn max_triangle_count(mut self, val : u64) -> Self {
		self.max_triangle_count = val;
		self
	}
	pub fn max_descriptor_set_acceleration_structures(mut self, val : u32) -> Self {
		self.max_descriptor_set_acceleration_structures = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceRayTracingPropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub subgroup_size_control : Bool32,
	pub compute_full_subgroups : Bool32,
}

impl PhysicalDeviceSubgroupSizeControlFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000225002,
			p_next : null_mut(),
			subgroup_size_control : <_>::default(),
			compute_full_subgroups : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn subgroup_size_control(mut self, val : Bool32) -> Self {
		self.subgroup_size_control = val;
		self
	}
	pub fn compute_full_subgroups(mut self, val : Bool32) -> Self {
		self.compute_full_subgroups = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSubgroupSizeControlFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub acceleration_structure_count : u32,
	pub p_acceleration_structures : *const AccelerationStructureKHR,
}

impl WriteDescriptorSetAccelerationStructureKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000165007,
			p_next : null(),
			acceleration_structure_count : <_>::default(),
			p_acceleration_structures : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn acceleration_structures(mut self, val : &[AccelerationStructureKHR]) -> Self {
		self.acceleration_structure_count = val.len() as _;
		self.p_acceleration_structures = val.as_ptr();
		self
	}
	pub fn acceleration_structure(mut self, val : &AccelerationStructureKHR) -> Self {
		self.acceleration_structure_count = 1;
		self.p_acceleration_structures = val;
		self
	}
}

impl std::default::Default for WriteDescriptorSetAccelerationStructureKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub buffer_device_address : Bool32,
	pub buffer_device_address_capture_replay : Bool32,
	pub buffer_device_address_multi_device : Bool32,
}

impl PhysicalDeviceBufferDeviceAddressFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000244000,
			p_next : null_mut(),
			buffer_device_address : <_>::default(),
			buffer_device_address_capture_replay : <_>::default(),
			buffer_device_address_multi_device : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn buffer_device_address(mut self, val : Bool32) -> Self {
		self.buffer_device_address = val;
		self
	}
	pub fn buffer_device_address_capture_replay(mut self, val : Bool32) -> Self {
		self.buffer_device_address_capture_replay = val;
		self
	}
	pub fn buffer_device_address_multi_device(mut self, val : Bool32) -> Self {
		self.buffer_device_address_multi_device = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindAccelerationStructureMemoryInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub acceleration_structure : AccelerationStructureKHR,
	pub memory : DeviceMemory,
	pub memory_offset : DeviceSize,
	pub device_index_count : u32,
	pub p_device_indices : *const u32,
}

impl BindAccelerationStructureMemoryInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000165006,
			p_next : null(),
			acceleration_structure : AccelerationStructureKHR(0),
			memory : DeviceMemory(0),
			memory_offset : <_>::default(),
			device_index_count : <_>::default(),
			p_device_indices : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn acceleration_structure(mut self, val : AccelerationStructureKHR) -> Self {
		self.acceleration_structure = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
	pub fn memory_offset(mut self, val : DeviceSize) -> Self {
		self.memory_offset = val;
		self
	}
	pub fn device_indices(mut self, val : &[u32]) -> Self {
		self.device_index_count = val.len() as _;
		self.p_device_indices = val.as_ptr();
		self
	}
	pub fn device_indice(mut self, val : &u32) -> Self {
		self.device_index_count = 1;
		self.p_device_indices = val;
		self
	}
}

impl std::default::Default for BindAccelerationStructureMemoryInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GeometryNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub geometry_type : GeometryTypeKHR,
	pub geometry : GeometryDataNV,
	pub flags : GeometryFlagsKHR,
}

impl GeometryNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165003,
			p_next : null(),
			geometry_type : GeometryTypeKHR::default(),
			geometry : GeometryDataNV::new(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn geometry_type(mut self, val : GeometryTypeKHR) -> Self {
		self.geometry_type = val;
		self
	}
	pub fn geometry(mut self, val : GeometryDataNV) -> Self {
		self.geometry = val;
		self
	}
	pub fn flags(mut self, val : GeometryFlagsKHR) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for GeometryNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RayTracingPipelineCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineCreateFlags,
	pub stage_count : u32,
	pub p_stages : *const PipelineShaderStageCreateInfo,
	pub group_count : u32,
	pub p_groups : *const RayTracingShaderGroupCreateInfoNV,
	pub max_recursion_depth : u32,
	pub layout : PipelineLayout,
	pub base_pipeline_handle : Pipeline,
	pub base_pipeline_index : i32,
}

impl RayTracingPipelineCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165000,
			p_next : null(),
			flags : <_>::default(),
			stage_count : <_>::default(),
			p_stages : null(),
			group_count : <_>::default(),
			p_groups : null(),
			max_recursion_depth : <_>::default(),
			layout : PipelineLayout(0),
			base_pipeline_handle : Pipeline(0),
			base_pipeline_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn max_recursion_depth(mut self, val : u32) -> Self {
		self.max_recursion_depth = val;
		self
	}
	pub fn layout(mut self, val : PipelineLayout) -> Self {
		self.layout = val;
		self
	}
	pub fn base_pipeline_handle(mut self, val : Pipeline) -> Self {
		self.base_pipeline_handle = val;
		self
	}
	pub fn base_pipeline_index(mut self, val : i32) -> Self {
		self.base_pipeline_index = val;
		self
	}
	pub fn stages(mut self, val : &[PipelineShaderStageCreateInfo]) -> Self {
		self.stage_count = val.len() as _;
		self.p_stages = val.as_ptr();
		self
	}
	pub fn stage(mut self, val : &PipelineShaderStageCreateInfo) -> Self {
		self.stage_count = 1;
		self.p_stages = val;
		self
	}
	pub fn groups(mut self, val : &[RayTracingShaderGroupCreateInfoNV]) -> Self {
		self.group_count = val.len() as _;
		self.p_groups = val.as_ptr();
		self
	}
	pub fn group(mut self, val : &RayTracingShaderGroupCreateInfoNV) -> Self {
		self.group_count = 1;
		self.p_groups = val;
		self
	}
}

impl std::default::Default for RayTracingPipelineCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RayTracingShaderGroupCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub r#type : RayTracingShaderGroupTypeKHR,
	pub general_shader : u32,
	pub closest_hit_shader : u32,
	pub any_hit_shader : u32,
	pub intersection_shader : u32,
}

impl RayTracingShaderGroupCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165011,
			p_next : null(),
			r#type : RayTracingShaderGroupTypeKHR::default(),
			general_shader : <_>::default(),
			closest_hit_shader : <_>::default(),
			any_hit_shader : <_>::default(),
			intersection_shader : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn r#type(mut self, val : RayTracingShaderGroupTypeKHR) -> Self {
		self.r#type = val;
		self
	}
	pub fn general_shader(mut self, val : u32) -> Self {
		self.general_shader = val;
		self
	}
	pub fn closest_hit_shader(mut self, val : u32) -> Self {
		self.closest_hit_shader = val;
		self
	}
	pub fn any_hit_shader(mut self, val : u32) -> Self {
		self.any_hit_shader = val;
		self
	}
	pub fn intersection_shader(mut self, val : u32) -> Self {
		self.intersection_shader = val;
		self
	}
}

impl std::default::Default for RayTracingShaderGroupCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub sample_order_type : CoarseSampleOrderTypeNV,
	pub custom_sample_order_count : u32,
	pub p_custom_sample_orders : *const CoarseSampleOrderCustomNV,
}

impl PipelineViewportCoarseSampleOrderStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000164005,
			p_next : null(),
			sample_order_type : CoarseSampleOrderTypeNV::default(),
			custom_sample_order_count : <_>::default(),
			p_custom_sample_orders : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn sample_order_type(mut self, val : CoarseSampleOrderTypeNV) -> Self {
		self.sample_order_type = val;
		self
	}
	pub fn custom_sample_orders(mut self, val : &[CoarseSampleOrderCustomNV]) -> Self {
		self.custom_sample_order_count = val.len() as _;
		self.p_custom_sample_orders = val.as_ptr();
		self
	}
	pub fn custom_sample_order(mut self, val : &CoarseSampleOrderCustomNV) -> Self {
		self.custom_sample_order_count = 1;
		self.p_custom_sample_orders = val;
		self
	}
}

impl std::default::Default for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shading_rate_image : Bool32,
	pub shading_rate_coarse_sample_order : Bool32,
}

impl PhysicalDeviceShadingRateImageFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000164001,
			p_next : null_mut(),
			shading_rate_image : <_>::default(),
			shading_rate_coarse_sample_order : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shading_rate_image(mut self, val : Bool32) -> Self {
		self.shading_rate_image = val;
		self
	}
	pub fn shading_rate_coarse_sample_order(mut self, val : Bool32) -> Self {
		self.shading_rate_coarse_sample_order = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShadingRateImageFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
	s_type : i32,
	pub p_next : *const c_void,
	pub performance_counters_sampling : QueryPoolSamplingModeINTEL,
}

impl QueryPoolPerformanceQueryCreateInfoINTEL {
	pub fn new() -> Self {
		Self {
			s_type : 1000210000,
			p_next : null(),
			performance_counters_sampling : QueryPoolSamplingModeINTEL::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn performance_counters_sampling(mut self, val : QueryPoolSamplingModeINTEL) -> Self {
		self.performance_counters_sampling = val;
		self
	}
}

impl std::default::Default for QueryPoolPerformanceQueryCreateInfoINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub shading_rate_image_enable : Bool32,
	pub viewport_count : u32,
	pub p_shading_rate_palettes : *const ShadingRatePaletteNV,
}

impl PipelineViewportShadingRateImageStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000164000,
			p_next : null(),
			shading_rate_image_enable : <_>::default(),
			viewport_count : <_>::default(),
			p_shading_rate_palettes : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shading_rate_image_enable(mut self, val : Bool32) -> Self {
		self.shading_rate_image_enable = val;
		self
	}
	pub fn shading_rate_palettes(mut self, val : &[ShadingRatePaletteNV]) -> Self {
		self.viewport_count = val.len() as _;
		self.p_shading_rate_palettes = val.as_ptr();
		self
	}
	pub fn shading_rate_palette(mut self, val : &ShadingRatePaletteNV) -> Self {
		self.viewport_count = 1;
		self.p_shading_rate_palettes = val;
		self
	}
}

impl std::default::Default for PipelineViewportShadingRateImageStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
	s_type : i32,
	pub p_next : *const c_void,
	pub memory : DeviceMemory,
}

impl MemoryGetAndroidHardwareBufferInfoANDROID {
	pub fn new() -> Self {
		Self {
			s_type : 1000129004,
			p_next : null(),
			memory : DeviceMemory(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
}

impl std::default::Default for MemoryGetAndroidHardwareBufferInfoANDROID {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerformanceOverrideInfoINTEL {
	s_type : i32,
	pub p_next : *const c_void,
	pub r#type : PerformanceOverrideTypeINTEL,
	pub enable : Bool32,
	pub parameter : u64,
}

impl PerformanceOverrideInfoINTEL {
	pub fn new() -> Self {
		Self {
			s_type : 1000210004,
			p_next : null(),
			r#type : PerformanceOverrideTypeINTEL::default(),
			enable : <_>::default(),
			parameter : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn r#type(mut self, val : PerformanceOverrideTypeINTEL) -> Self {
		self.r#type = val;
		self
	}
	pub fn enable(mut self, val : Bool32) -> Self {
		self.enable = val;
		self
	}
	pub fn parameter(mut self, val : u64) -> Self {
		self.parameter = val;
		self
	}
}

impl std::default::Default for PerformanceOverrideInfoINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shading_rate_texel_size : Extent2D,
	pub shading_rate_palette_size : u32,
	pub shading_rate_max_coarse_samples : u32,
}

impl PhysicalDeviceShadingRateImagePropertiesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000164002,
			p_next : null_mut(),
			shading_rate_texel_size : Extent2D::new(),
			shading_rate_palette_size : <_>::default(),
			shading_rate_max_coarse_samples : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shading_rate_texel_size(mut self, val : Extent2D) -> Self {
		self.shading_rate_texel_size = val;
		self
	}
	pub fn shading_rate_palette_size(mut self, val : u32) -> Self {
		self.shading_rate_palette_size = val;
		self
	}
	pub fn shading_rate_max_coarse_samples(mut self, val : u32) -> Self {
		self.shading_rate_max_coarse_samples = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShadingRateImagePropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ShadingRatePaletteNV {
	pub shading_rate_palette_entry_count : u32,
	pub p_shading_rate_palette_entries : *const ShadingRatePaletteEntryNV,
}

impl ShadingRatePaletteNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn shading_rate_palette_entries(mut self, val : &[ShadingRatePaletteEntryNV]) -> Self {
		self.shading_rate_palette_entry_count = val.len() as _;
		self.p_shading_rate_palette_entries = val.as_ptr();
		self
	}
	pub fn shading_rate_palette_entry(mut self, val : &ShadingRatePaletteEntryNV) -> Self {
		self.shading_rate_palette_entry_count = 1;
		self.p_shading_rate_palette_entries = val;
		self
	}
}

impl std::default::Default for ShadingRatePaletteNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub validation_cache : ValidationCacheEXT,
}

impl ShaderModuleValidationCacheCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000160001,
			p_next : null(),
			validation_cache : ValidationCacheEXT(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn validation_cache(mut self, val : ValidationCacheEXT) -> Self {
		self.validation_cache = val;
		self
	}
}

impl std::default::Default for ShaderModuleValidationCacheCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageDrmFormatModifierPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub drm_format_modifier : u64,
}

impl ImageDrmFormatModifierPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000158005,
			p_next : null_mut(),
			drm_format_modifier : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn drm_format_modifier(mut self, val : u64) -> Self {
		self.drm_format_modifier = val;
		self
	}
}

impl std::default::Default for ImageDrmFormatModifierPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DevicePrivateDataCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub private_data_slot_request_count : u32,
}

impl DevicePrivateDataCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000295001,
			p_next : null(),
			private_data_slot_request_count : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn private_data_slot_request_count(mut self, val : u32) -> Self {
		self.private_data_slot_request_count = val;
		self
	}
}

impl std::default::Default for DevicePrivateDataCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub drm_format_modifier : u64,
	pub drm_format_modifier_plane_count : u32,
	pub p_plane_layouts : *const SubresourceLayout,
}

impl ImageDrmFormatModifierExplicitCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000158004,
			p_next : null(),
			drm_format_modifier : <_>::default(),
			drm_format_modifier_plane_count : <_>::default(),
			p_plane_layouts : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn drm_format_modifier(mut self, val : u64) -> Self {
		self.drm_format_modifier = val;
		self
	}
	pub fn plane_layouts(mut self, val : &[SubresourceLayout]) -> Self {
		self.drm_format_modifier_plane_count = val.len() as _;
		self.p_plane_layouts = val.as_ptr();
		self
	}
	pub fn plane_layout(mut self, val : &SubresourceLayout) -> Self {
		self.drm_format_modifier_plane_count = 1;
		self.p_plane_layouts = val;
		self
	}
}

impl std::default::Default for ImageDrmFormatModifierExplicitCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_integer_functions_2 : Bool32,
}

impl PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
	pub fn new() -> Self {
		Self {
			s_type : 1000209000,
			p_next : null_mut(),
			shader_integer_functions_2 : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_integer_functions_2(mut self, val : Bool32) -> Self {
		self.shader_integer_functions_2 = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub drm_format_modifier_count : u32,
	pub p_drm_format_modifiers : *const u64,
}

impl ImageDrmFormatModifierListCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000158003,
			p_next : null(),
			drm_format_modifier_count : <_>::default(),
			p_drm_format_modifiers : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn drm_format_modifiers(mut self, val : &[u64]) -> Self {
		self.drm_format_modifier_count = val.len() as _;
		self.p_drm_format_modifiers = val.as_ptr();
		self
	}
	pub fn drm_format_modifier(mut self, val : &u64) -> Self {
		self.drm_format_modifier_count = 1;
		self.p_drm_format_modifiers = val;
		self
	}
}

impl std::default::Default for ImageDrmFormatModifierListCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DrmFormatModifierPropertiesListEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub drm_format_modifier_count : u32,
	pub p_drm_format_modifier_properties : *mut DrmFormatModifierPropertiesEXT,
}

impl DrmFormatModifierPropertiesListEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000158000,
			p_next : null_mut(),
			drm_format_modifier_count : <_>::default(),
			p_drm_format_modifier_properties : null_mut(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn drm_format_modifier_properties(mut self, val : &mut [DrmFormatModifierPropertiesEXT]) -> Self {
		self.drm_format_modifier_count = val.len() as _;
		self.p_drm_format_modifier_properties = val.as_mut_ptr();
		self
	}
	pub fn drm_format_modifier_property(mut self, val : &mut DrmFormatModifierPropertiesEXT) -> Self {
		self.drm_format_modifier_count = 1;
		self.p_drm_format_modifier_properties = val;
		self
	}
}

impl std::default::Default for DrmFormatModifierPropertiesListEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_smbuiltins : Bool32,
}

impl PhysicalDeviceShaderSMBuiltinsFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000154000,
			p_next : null_mut(),
			shader_smbuiltins : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_smbuiltins(mut self, val : Bool32) -> Self {
		self.shader_smbuiltins = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderSMBuiltinsFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_smcount : u32,
	pub shader_warps_per_sm : u32,
}

impl PhysicalDeviceShaderSMBuiltinsPropertiesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000154001,
			p_next : null_mut(),
			shader_smcount : <_>::default(),
			shader_warps_per_sm : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_smcount(mut self, val : u32) -> Self {
		self.shader_smcount = val;
		self
	}
	pub fn shader_warps_per_sm(mut self, val : u32) -> Self {
		self.shader_warps_per_sm = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderSMBuiltinsPropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_graphics_shader_group_count : u32,
	pub max_indirect_sequence_count : u32,
	pub max_indirect_commands_token_count : u32,
	pub max_indirect_commands_stream_count : u32,
	pub max_indirect_commands_token_offset : u32,
	pub max_indirect_commands_stream_stride : u32,
	pub min_sequences_count_buffer_offset_alignment : u32,
	pub min_sequences_index_buffer_offset_alignment : u32,
	pub min_indirect_commands_buffer_offset_alignment : u32,
}

impl PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000277000,
			p_next : null_mut(),
			max_graphics_shader_group_count : <_>::default(),
			max_indirect_sequence_count : <_>::default(),
			max_indirect_commands_token_count : <_>::default(),
			max_indirect_commands_stream_count : <_>::default(),
			max_indirect_commands_token_offset : <_>::default(),
			max_indirect_commands_stream_stride : <_>::default(),
			min_sequences_count_buffer_offset_alignment : <_>::default(),
			min_sequences_index_buffer_offset_alignment : <_>::default(),
			min_indirect_commands_buffer_offset_alignment : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_graphics_shader_group_count(mut self, val : u32) -> Self {
		self.max_graphics_shader_group_count = val;
		self
	}
	pub fn max_indirect_sequence_count(mut self, val : u32) -> Self {
		self.max_indirect_sequence_count = val;
		self
	}
	pub fn max_indirect_commands_token_count(mut self, val : u32) -> Self {
		self.max_indirect_commands_token_count = val;
		self
	}
	pub fn max_indirect_commands_stream_count(mut self, val : u32) -> Self {
		self.max_indirect_commands_stream_count = val;
		self
	}
	pub fn max_indirect_commands_token_offset(mut self, val : u32) -> Self {
		self.max_indirect_commands_token_offset = val;
		self
	}
	pub fn max_indirect_commands_stream_stride(mut self, val : u32) -> Self {
		self.max_indirect_commands_stream_stride = val;
		self
	}
	pub fn min_sequences_count_buffer_offset_alignment(mut self, val : u32) -> Self {
		self.min_sequences_count_buffer_offset_alignment = val;
		self
	}
	pub fn min_sequences_index_buffer_offset_alignment(mut self, val : u32) -> Self {
		self.min_sequences_index_buffer_offset_alignment = val;
		self
	}
	pub fn min_indirect_commands_buffer_offset_alignment(mut self, val : u32) -> Self {
		self.min_indirect_commands_buffer_offset_alignment = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub src_premultiplied : Bool32,
	pub dst_premultiplied : Bool32,
	pub blend_overlap : BlendOverlapEXT,
}

impl PipelineColorBlendAdvancedStateCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000148002,
			p_next : null(),
			src_premultiplied : <_>::default(),
			dst_premultiplied : <_>::default(),
			blend_overlap : BlendOverlapEXT::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn src_premultiplied(mut self, val : Bool32) -> Self {
		self.src_premultiplied = val;
		self
	}
	pub fn dst_premultiplied(mut self, val : Bool32) -> Self {
		self.dst_premultiplied = val;
		self
	}
	pub fn blend_overlap(mut self, val : BlendOverlapEXT) -> Self {
		self.blend_overlap = val;
		self
	}
}

impl std::default::Default for PipelineColorBlendAdvancedStateCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubpassSampleLocationsEXT {
	pub subpass_index : u32,
	pub sample_locations_info : SampleLocationsInfoEXT,
}

impl SubpassSampleLocationsEXT {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn subpass_index(mut self, val : u32) -> Self {
		self.subpass_index = val;
		self
	}
	pub fn sample_locations_info(mut self, val : SampleLocationsInfoEXT) -> Self {
		self.sample_locations_info = val;
		self
	}
}

impl std::default::Default for SubpassSampleLocationsEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SampleLocationsInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub sample_locations_per_pixel : SampleCountFlags,
	pub sample_location_grid_size : Extent2D,
	pub sample_locations_count : u32,
	pub p_sample_locations : *const SampleLocationEXT,
}

impl SampleLocationsInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000143000,
			p_next : null(),
			sample_locations_per_pixel : SampleCountFlags::default(),
			sample_location_grid_size : Extent2D::new(),
			sample_locations_count : <_>::default(),
			p_sample_locations : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn sample_locations_per_pixel(mut self, val : SampleCountFlags) -> Self {
		self.sample_locations_per_pixel = val;
		self
	}
	pub fn sample_location_grid_size(mut self, val : Extent2D) -> Self {
		self.sample_location_grid_size = val;
		self
	}
	pub fn sample_locations(mut self, val : &[SampleLocationEXT]) -> Self {
		self.sample_locations_count = val.len() as _;
		self.p_sample_locations = val.as_ptr();
		self
	}
	pub fn sample_location(mut self, val : &SampleLocationEXT) -> Self {
		self.sample_locations_count = 1;
		self.p_sample_locations = val;
		self
	}
}

impl std::default::Default for SampleLocationsInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SampleLocationEXT {
	pub x : f32,
	pub y : f32,
}

impl SampleLocationEXT {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn x(mut self, val : f32) -> Self {
		self.x = val;
		self
	}
	pub fn y(mut self, val : f32) -> Self {
		self.y = val;
		self
	}
}

impl std::default::Default for SampleLocationEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub max_inline_uniform_block_bindings : u32,
}

impl DescriptorPoolInlineUniformBlockCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000138003,
			p_next : null(),
			max_inline_uniform_block_bindings : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_inline_uniform_block_bindings(mut self, val : u32) -> Self {
		self.max_inline_uniform_block_bindings = val;
		self
	}
}

impl std::default::Default for DescriptorPoolInlineUniformBlockCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DebugUtilsObjectNameInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub object_type : ObjectType,
	pub object_handle : u64,
	pub p_object_name : *const u8,
}

impl DebugUtilsObjectNameInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000128000,
			p_next : null(),
			object_type : ObjectType::default(),
			object_handle : <_>::default(),
			p_object_name : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn object_type(mut self, val : ObjectType) -> Self {
		self.object_type = val;
		self
	}
	pub fn object_handle(mut self, val : u64) -> Self {
		self.object_handle = val;
		self
	}
	pub fn object_name(mut self, val : *const u8) -> Self {
		self.p_object_name = val;
		self
	}
}

impl std::default::Default for DebugUtilsObjectNameInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AndroidHardwareBufferPropertiesANDROID {
	s_type : i32,
	pub p_next : *mut c_void,
	pub allocation_size : DeviceSize,
	pub memory_type_bits : u32,
}

impl AndroidHardwareBufferPropertiesANDROID {
	pub fn new() -> Self {
		Self {
			s_type : 1000129001,
			p_next : null_mut(),
			allocation_size : <_>::default(),
			memory_type_bits : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn allocation_size(mut self, val : DeviceSize) -> Self {
		self.allocation_size = val;
		self
	}
	pub fn memory_type_bits(mut self, val : u32) -> Self {
		self.memory_type_bits = val;
		self
	}
}

impl std::default::Default for AndroidHardwareBufferPropertiesANDROID {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct DebugUtilsLabelEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_label_name : *const u8,
	pub color : [f32; 4],
}

impl DebugUtilsLabelEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000128002,
			p_next : null(),
			p_label_name : null(),
			color : [0 as _ ;4],
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn label_name(mut self, val : *const u8) -> Self {
		self.p_label_name = val;
		self
	}
	pub fn color(mut self, val : [f32; 4]) -> Self {
		self.color = val;
		self
	}
}

impl std::default::Default for DebugUtilsLabelEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct XYColorEXT {
	pub x : f32,
	pub y : f32,
}

impl XYColorEXT {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn x(mut self, val : f32) -> Self {
		self.x = val;
		self
	}
	pub fn y(mut self, val : f32) -> Self {
		self.y = val;
		self
	}
}

impl std::default::Default for XYColorEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CoarseSampleOrderCustomNV {
	pub shading_rate : ShadingRatePaletteEntryNV,
	pub sample_count : u32,
	pub sample_location_count : u32,
	pub p_sample_locations : *const CoarseSampleLocationNV,
}

impl CoarseSampleOrderCustomNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn shading_rate(mut self, val : ShadingRatePaletteEntryNV) -> Self {
		self.shading_rate = val;
		self
	}
	pub fn sample_count(mut self, val : u32) -> Self {
		self.sample_count = val;
		self
	}
	pub fn sample_locations(mut self, val : &[CoarseSampleLocationNV]) -> Self {
		self.sample_location_count = val.len() as _;
		self.p_sample_locations = val.as_ptr();
		self
	}
	pub fn sample_location(mut self, val : &CoarseSampleLocationNV) -> Self {
		self.sample_location_count = 1;
		self.p_sample_locations = val;
		self
	}
}

impl std::default::Default for CoarseSampleOrderCustomNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineRasterizationDepthClipStateCreateFlagsEXT,
	pub depth_clip_enable : Bool32,
}

impl PipelineRasterizationDepthClipStateCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000102001,
			p_next : null(),
			flags : <_>::default(),
			depth_clip_enable : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineRasterizationDepthClipStateCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn depth_clip_enable(mut self, val : Bool32) -> Self {
		self.depth_clip_enable = val;
		self
	}
}

impl std::default::Default for PipelineRasterizationDepthClipStateCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub depth_clip_enable : Bool32,
}

impl PhysicalDeviceDepthClipEnableFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000102000,
			p_next : null_mut(),
			depth_clip_enable : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn depth_clip_enable(mut self, val : Bool32) -> Self {
		self.depth_clip_enable = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDepthClipEnableFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineViewportSwizzleStateCreateFlagsNV,
	pub viewport_count : u32,
	pub p_viewport_swizzles : *const ViewportSwizzleNV,
}

impl PipelineViewportSwizzleStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000098000,
			p_next : null(),
			flags : <_>::default(),
			viewport_count : <_>::default(),
			p_viewport_swizzles : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineViewportSwizzleStateCreateFlagsNV) -> Self {
		self.flags = val;
		self
	}
	pub fn viewport_swizzles(mut self, val : &[ViewportSwizzleNV]) -> Self {
		self.viewport_count = val.len() as _;
		self.p_viewport_swizzles = val.as_ptr();
		self
	}
	pub fn viewport_swizzle(mut self, val : &ViewportSwizzleNV) -> Self {
		self.viewport_count = 1;
		self.p_viewport_swizzles = val;
		self
	}
}

impl std::default::Default for PipelineViewportSwizzleStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
	s_type : i32,
	pub p_next : *mut c_void,
	pub per_view_position_all_components : Bool32,
}

impl PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
	pub fn new() -> Self {
		Self {
			s_type : 1000097000,
			p_next : null_mut(),
			per_view_position_all_components : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn per_view_position_all_components(mut self, val : Bool32) -> Self {
		self.per_view_position_all_components = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PresentTimesInfoGOOGLE {
	s_type : i32,
	pub p_next : *const c_void,
	pub swapchain_count : u32,
	pub p_times : *const PresentTimeGOOGLE,
}

impl PresentTimesInfoGOOGLE {
	pub fn new() -> Self {
		Self {
			s_type : 1000092000,
			p_next : null(),
			swapchain_count : <_>::default(),
			p_times : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn times(mut self, val : &[PresentTimeGOOGLE]) -> Self {
		self.swapchain_count = val.len() as _;
		self.p_times = val.as_ptr();
		self
	}
	pub fn time(mut self, val : &PresentTimeGOOGLE) -> Self {
		self.swapchain_count = 1;
		self.p_times = val;
		self
	}
}

impl std::default::Default for PresentTimesInfoGOOGLE {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExportMemoryWin32HandleInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_attributes : *const SECURITY_ATTRIBUTES,
	pub dw_access : u32,
}

impl ExportMemoryWin32HandleInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000057001,
			p_next : null(),
			p_attributes : null(),
			dw_access : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn attributes(mut self, val : *const SECURITY_ATTRIBUTES) -> Self {
		self.p_attributes = val;
		self
	}
	pub fn dw_access(mut self, val : u32) -> Self {
		self.dw_access = val;
		self
	}
}

impl std::default::Default for ExportMemoryWin32HandleInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PresentTimeGOOGLE {
	pub present_id : u32,
	pub desired_present_time : u64,
}

impl PresentTimeGOOGLE {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn present_id(mut self, val : u32) -> Self {
		self.present_id = val;
		self
	}
	pub fn desired_present_time(mut self, val : u64) -> Self {
		self.desired_present_time = val;
		self
	}
}

impl std::default::Default for PresentTimeGOOGLE {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RefreshCycleDurationGOOGLE {
	pub refresh_duration : u64,
}

impl RefreshCycleDurationGOOGLE {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
}

impl std::default::Default for RefreshCycleDurationGOOGLE {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub acquire_count : u32,
	pub p_acquire_syncs : *const DeviceMemory,
	pub p_acquire_keys : *const u64,
	pub p_acquire_timeouts : *const u32,
	pub release_count : u32,
	pub p_release_syncs : *const DeviceMemory,
	pub p_release_keys : *const u64,
}

impl Win32KeyedMutexAcquireReleaseInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000075000,
			p_next : null(),
			acquire_count : <_>::default(),
			p_acquire_syncs : null(),
			p_acquire_keys : null(),
			p_acquire_timeouts : null(),
			release_count : <_>::default(),
			p_release_syncs : null(),
			p_release_keys : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn acquire_keys(mut self, val : *const u64) -> Self {
		self.p_acquire_keys = val;
		self
	}
	pub fn acquire_timeouts(mut self, val : *const u32) -> Self {
		self.p_acquire_timeouts = val;
		self
	}
	pub fn acquire_syncs(mut self, val : &[DeviceMemory]) -> Self {
		self.acquire_count = val.len() as _;
		self.p_acquire_syncs = val.as_ptr();
		self
	}
	pub fn acquire_sync(mut self, val : &DeviceMemory) -> Self {
		self.acquire_count = 1;
		self.p_acquire_syncs = val;
		self
	}
	pub fn release_syncs(mut self, val : &[DeviceMemory]) -> Self {
		self.release_count = val.len() as _;
		self.p_release_syncs = val.as_ptr();
		self
	}
	pub fn release_sync(mut self, val : &DeviceMemory) -> Self {
		self.release_count = 1;
		self.p_release_syncs = val;
		self
	}
}

impl std::default::Default for Win32KeyedMutexAcquireReleaseInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SwapchainCounterCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub surface_counters : SurfaceCounterFlagsEXT,
}

impl SwapchainCounterCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000091003,
			p_next : null(),
			surface_counters : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn surface_counters(mut self, val : SurfaceCounterFlagsEXT) -> Self {
		self.surface_counters = val;
		self
	}
}

impl std::default::Default for SwapchainCounterCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceEventInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub device_event : DeviceEventTypeEXT,
}

impl DeviceEventInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000091001,
			p_next : null(),
			device_event : DeviceEventTypeEXT::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_event(mut self, val : DeviceEventTypeEXT) -> Self {
		self.device_event = val;
		self
	}
}

impl std::default::Default for DeviceEventInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub viewport_wscaling_enable : Bool32,
	pub viewport_count : u32,
	pub p_viewport_wscalings : *const ViewportWScalingNV,
}

impl PipelineViewportWScalingStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000087000,
			p_next : null(),
			viewport_wscaling_enable : <_>::default(),
			viewport_count : <_>::default(),
			p_viewport_wscalings : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn viewport_wscaling_enable(mut self, val : Bool32) -> Self {
		self.viewport_wscaling_enable = val;
		self
	}
	pub fn viewport_wscalings(mut self, val : &[ViewportWScalingNV]) -> Self {
		self.viewport_count = val.len() as _;
		self.p_viewport_wscalings = val.as_ptr();
		self
	}
	pub fn viewport_wscaling(mut self, val : &ViewportWScalingNV) -> Self {
		self.viewport_count = 1;
		self.p_viewport_wscalings = val;
		self
	}
}

impl std::default::Default for PipelineViewportWScalingStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub conditional_rendering_enable : Bool32,
}

impl CommandBufferInheritanceConditionalRenderingInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000081000,
			p_next : null(),
			conditional_rendering_enable : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn conditional_rendering_enable(mut self, val : Bool32) -> Self {
		self.conditional_rendering_enable = val;
		self
	}
}

impl std::default::Default for CommandBufferInheritanceConditionalRenderingInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub image_footprint : Bool32,
}

impl PhysicalDeviceShaderImageFootprintFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000204000,
			p_next : null_mut(),
			image_footprint : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image_footprint(mut self, val : Bool32) -> Self {
		self.image_footprint = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderImageFootprintFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceMeshShaderPropertiesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_draw_mesh_tasks_count : u32,
	pub max_task_work_group_invocations : u32,
	pub max_task_work_group_size : [u32; 3],
	pub max_task_total_memory_size : u32,
	pub max_task_output_count : u32,
	pub max_mesh_work_group_invocations : u32,
	pub max_mesh_work_group_size : [u32; 3],
	pub max_mesh_total_memory_size : u32,
	pub max_mesh_output_vertices : u32,
	pub max_mesh_output_primitives : u32,
	pub max_mesh_multiview_view_count : u32,
	pub mesh_output_per_vertex_granularity : u32,
	pub mesh_output_per_primitive_granularity : u32,
}

impl PhysicalDeviceMeshShaderPropertiesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000202001,
			p_next : null_mut(),
			max_draw_mesh_tasks_count : <_>::default(),
			max_task_work_group_invocations : <_>::default(),
			max_task_work_group_size : [0 as _ ;3],
			max_task_total_memory_size : <_>::default(),
			max_task_output_count : <_>::default(),
			max_mesh_work_group_invocations : <_>::default(),
			max_mesh_work_group_size : [0 as _ ;3],
			max_mesh_total_memory_size : <_>::default(),
			max_mesh_output_vertices : <_>::default(),
			max_mesh_output_primitives : <_>::default(),
			max_mesh_multiview_view_count : <_>::default(),
			mesh_output_per_vertex_granularity : <_>::default(),
			mesh_output_per_primitive_granularity : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_draw_mesh_tasks_count(mut self, val : u32) -> Self {
		self.max_draw_mesh_tasks_count = val;
		self
	}
	pub fn max_task_work_group_invocations(mut self, val : u32) -> Self {
		self.max_task_work_group_invocations = val;
		self
	}
	pub fn max_task_work_group_size(mut self, val : [u32; 3]) -> Self {
		self.max_task_work_group_size = val;
		self
	}
	pub fn max_task_total_memory_size(mut self, val : u32) -> Self {
		self.max_task_total_memory_size = val;
		self
	}
	pub fn max_task_output_count(mut self, val : u32) -> Self {
		self.max_task_output_count = val;
		self
	}
	pub fn max_mesh_work_group_invocations(mut self, val : u32) -> Self {
		self.max_mesh_work_group_invocations = val;
		self
	}
	pub fn max_mesh_work_group_size(mut self, val : [u32; 3]) -> Self {
		self.max_mesh_work_group_size = val;
		self
	}
	pub fn max_mesh_total_memory_size(mut self, val : u32) -> Self {
		self.max_mesh_total_memory_size = val;
		self
	}
	pub fn max_mesh_output_vertices(mut self, val : u32) -> Self {
		self.max_mesh_output_vertices = val;
		self
	}
	pub fn max_mesh_output_primitives(mut self, val : u32) -> Self {
		self.max_mesh_output_primitives = val;
		self
	}
	pub fn max_mesh_multiview_view_count(mut self, val : u32) -> Self {
		self.max_mesh_multiview_view_count = val;
		self
	}
	pub fn mesh_output_per_vertex_granularity(mut self, val : u32) -> Self {
		self.mesh_output_per_vertex_granularity = val;
		self
	}
	pub fn mesh_output_per_primitive_granularity(mut self, val : u32) -> Self {
		self.mesh_output_per_primitive_granularity = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMeshShaderPropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub conditional_rendering : Bool32,
	pub inherited_conditional_rendering : Bool32,
}

impl PhysicalDeviceConditionalRenderingFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000081001,
			p_next : null_mut(),
			conditional_rendering : <_>::default(),
			inherited_conditional_rendering : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn conditional_rendering(mut self, val : Bool32) -> Self {
		self.conditional_rendering = val;
		self
	}
	pub fn inherited_conditional_rendering(mut self, val : Bool32) -> Self {
		self.inherited_conditional_rendering = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceConditionalRenderingFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub decode_mode_shared_exponent : Bool32,
}

impl PhysicalDeviceASTCDecodeFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000067001,
			p_next : null_mut(),
			decode_mode_shared_exponent : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn decode_mode_shared_exponent(mut self, val : Bool32) -> Self {
		self.decode_mode_shared_exponent = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceASTCDecodeFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExportMemoryAllocateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_types : ExternalMemoryHandleTypeFlagsNV,
}

impl ExportMemoryAllocateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000056001,
			p_next : null(),
			handle_types : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_types(mut self, val : ExternalMemoryHandleTypeFlagsNV) -> Self {
		self.handle_types = val;
		self
	}
}

impl std::default::Default for ExportMemoryAllocateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalMemoryImageCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_types : ExternalMemoryHandleTypeFlagsNV,
}

impl ExternalMemoryImageCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000056000,
			p_next : null(),
			handle_types : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_types(mut self, val : ExternalMemoryHandleTypeFlagsNV) -> Self {
		self.handle_types = val;
		self
	}
}

impl std::default::Default for ExternalMemoryImageCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalImageFormatPropertiesNV {
	pub image_format_properties : ImageFormatProperties,
	pub external_memory_features : ExternalMemoryFeatureFlagsNV,
	pub export_from_imported_handle_types : ExternalMemoryHandleTypeFlagsNV,
	pub compatible_handle_types : ExternalMemoryHandleTypeFlagsNV,
}

impl ExternalImageFormatPropertiesNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn image_format_properties(mut self, val : ImageFormatProperties) -> Self {
		self.image_format_properties = val;
		self
	}
	pub fn external_memory_features(mut self, val : ExternalMemoryFeatureFlagsNV) -> Self {
		self.external_memory_features = val;
		self
	}
	pub fn export_from_imported_handle_types(mut self, val : ExternalMemoryHandleTypeFlagsNV) -> Self {
		self.export_from_imported_handle_types = val;
		self
	}
	pub fn compatible_handle_types(mut self, val : ExternalMemoryHandleTypeFlagsNV) -> Self {
		self.compatible_handle_types = val;
		self
	}
}

impl std::default::Default for ExternalImageFormatPropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DebugUtilsMessengerCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DebugUtilsMessengerCreateFlagsEXT,
	pub message_severity : DebugUtilsMessageSeverityFlagsEXT,
	pub message_type : DebugUtilsMessageTypeFlagsEXT,
	pub pfn_user_callback : Option<extern "C" fn()>,
	pub p_user_data : *mut c_void,
}

impl DebugUtilsMessengerCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000128004,
			p_next : null(),
			flags : <_>::default(),
			message_severity : <_>::default(),
			message_type : <_>::default(),
			pfn_user_callback : None,
			p_user_data : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DebugUtilsMessengerCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn message_severity(mut self, val : DebugUtilsMessageSeverityFlagsEXT) -> Self {
		self.message_severity = val;
		self
	}
	pub fn message_type(mut self, val : DebugUtilsMessageTypeFlagsEXT) -> Self {
		self.message_type = val;
		self
	}
	pub fn pfn_user_callback(mut self, val : Option<extern "C" fn()>) -> Self {
		self.pfn_user_callback = val;
		self
	}
	pub fn user_data(mut self, val : *mut c_void) -> Self {
		self.p_user_data = val;
		self
	}
}

impl std::default::Default for DebugUtilsMessengerCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct ShaderStatisticsInfoAMD {
	pub shader_stage_mask : ShaderStageFlags,
	pub resource_usage : ShaderResourceUsageAMD,
	pub num_physical_vgprs : u32,
	pub num_physical_sgprs : u32,
	pub num_available_vgprs : u32,
	pub num_available_sgprs : u32,
	pub compute_work_group_size : [u32; 3],
}

impl ShaderStatisticsInfoAMD {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn shader_stage_mask(mut self, val : ShaderStageFlags) -> Self {
		self.shader_stage_mask = val;
		self
	}
	pub fn resource_usage(mut self, val : ShaderResourceUsageAMD) -> Self {
		self.resource_usage = val;
		self
	}
	pub fn num_physical_vgprs(mut self, val : u32) -> Self {
		self.num_physical_vgprs = val;
		self
	}
	pub fn num_physical_sgprs(mut self, val : u32) -> Self {
		self.num_physical_sgprs = val;
		self
	}
	pub fn num_available_vgprs(mut self, val : u32) -> Self {
		self.num_available_vgprs = val;
		self
	}
	pub fn num_available_sgprs(mut self, val : u32) -> Self {
		self.num_available_sgprs = val;
		self
	}
	pub fn compute_work_group_size(mut self, val : [u32; 3]) -> Self {
		self.compute_work_group_size = val;
		self
	}
}

impl std::default::Default for ShaderStatisticsInfoAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct TextureLODGatherFormatPropertiesAMD {
	s_type : i32,
	pub p_next : *mut c_void,
	pub supports_texture_gather_lodbias_amd : Bool32,
}

impl TextureLODGatherFormatPropertiesAMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000041000,
			p_next : null_mut(),
			supports_texture_gather_lodbias_amd : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn supports_texture_gather_lodbias_amd(mut self, val : Bool32) -> Self {
		self.supports_texture_gather_lodbias_amd = val;
		self
	}
}

impl std::default::Default for TextureLODGatherFormatPropertiesAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageViewAddressPropertiesNVX {
	s_type : i32,
	pub p_next : *mut c_void,
	pub device_address : DeviceAddress,
	pub size : DeviceSize,
}

impl ImageViewAddressPropertiesNVX {
	pub fn new() -> Self {
		Self {
			s_type : 1000030001,
			p_next : null_mut(),
			device_address : <_>::default(),
			size : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_address(mut self, val : DeviceAddress) -> Self {
		self.device_address = val;
		self
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
}

impl std::default::Default for ImageViewAddressPropertiesNVX {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageViewHandleInfoNVX {
	s_type : i32,
	pub p_next : *const c_void,
	pub image_view : ImageView,
	pub descriptor_type : DescriptorType,
	pub sampler : Sampler,
}

impl ImageViewHandleInfoNVX {
	pub fn new() -> Self {
		Self {
			s_type : 1000030000,
			p_next : null(),
			image_view : ImageView(0),
			descriptor_type : DescriptorType::default(),
			sampler : Sampler(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image_view(mut self, val : ImageView) -> Self {
		self.image_view = val;
		self
	}
	pub fn descriptor_type(mut self, val : DescriptorType) -> Self {
		self.descriptor_type = val;
		self
	}
	pub fn sampler(mut self, val : Sampler) -> Self {
		self.sampler = val;
		self
	}
}

impl std::default::Default for ImageViewHandleInfoNVX {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineRasterizationStateStreamCreateFlagsEXT,
	pub rasterization_stream : u32,
}

impl PipelineRasterizationStateStreamCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000028002,
			p_next : null(),
			flags : <_>::default(),
			rasterization_stream : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineRasterizationStateStreamCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn rasterization_stream(mut self, val : u32) -> Self {
		self.rasterization_stream = val;
		self
	}
}

impl std::default::Default for PipelineRasterizationStateStreamCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_transform_feedback_streams : u32,
	pub max_transform_feedback_buffers : u32,
	pub max_transform_feedback_buffer_size : DeviceSize,
	pub max_transform_feedback_stream_data_size : u32,
	pub max_transform_feedback_buffer_data_size : u32,
	pub max_transform_feedback_buffer_data_stride : u32,
	pub transform_feedback_queries : Bool32,
	pub transform_feedback_streams_lines_triangles : Bool32,
	pub transform_feedback_rasterization_stream_select : Bool32,
	pub transform_feedback_draw : Bool32,
}

impl PhysicalDeviceTransformFeedbackPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000028001,
			p_next : null_mut(),
			max_transform_feedback_streams : <_>::default(),
			max_transform_feedback_buffers : <_>::default(),
			max_transform_feedback_buffer_size : <_>::default(),
			max_transform_feedback_stream_data_size : <_>::default(),
			max_transform_feedback_buffer_data_size : <_>::default(),
			max_transform_feedback_buffer_data_stride : <_>::default(),
			transform_feedback_queries : <_>::default(),
			transform_feedback_streams_lines_triangles : <_>::default(),
			transform_feedback_rasterization_stream_select : <_>::default(),
			transform_feedback_draw : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_transform_feedback_streams(mut self, val : u32) -> Self {
		self.max_transform_feedback_streams = val;
		self
	}
	pub fn max_transform_feedback_buffers(mut self, val : u32) -> Self {
		self.max_transform_feedback_buffers = val;
		self
	}
	pub fn max_transform_feedback_buffer_size(mut self, val : DeviceSize) -> Self {
		self.max_transform_feedback_buffer_size = val;
		self
	}
	pub fn max_transform_feedback_stream_data_size(mut self, val : u32) -> Self {
		self.max_transform_feedback_stream_data_size = val;
		self
	}
	pub fn max_transform_feedback_buffer_data_size(mut self, val : u32) -> Self {
		self.max_transform_feedback_buffer_data_size = val;
		self
	}
	pub fn max_transform_feedback_buffer_data_stride(mut self, val : u32) -> Self {
		self.max_transform_feedback_buffer_data_stride = val;
		self
	}
	pub fn transform_feedback_queries(mut self, val : Bool32) -> Self {
		self.transform_feedback_queries = val;
		self
	}
	pub fn transform_feedback_streams_lines_triangles(mut self, val : Bool32) -> Self {
		self.transform_feedback_streams_lines_triangles = val;
		self
	}
	pub fn transform_feedback_rasterization_stream_select(mut self, val : Bool32) -> Self {
		self.transform_feedback_rasterization_stream_select = val;
		self
	}
	pub fn transform_feedback_draw(mut self, val : Bool32) -> Self {
		self.transform_feedback_draw = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceTransformFeedbackPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub corner_sampled_image : Bool32,
}

impl PhysicalDeviceCornerSampledImageFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000050000,
			p_next : null_mut(),
			corner_sampled_image : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn corner_sampled_image(mut self, val : Bool32) -> Self {
		self.corner_sampled_image = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceCornerSampledImageFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DedicatedAllocationImageCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub dedicated_allocation : Bool32,
}

impl DedicatedAllocationImageCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000026000,
			p_next : null(),
			dedicated_allocation : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn dedicated_allocation(mut self, val : Bool32) -> Self {
		self.dedicated_allocation = val;
		self
	}
}

impl std::default::Default for DedicatedAllocationImageCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct DebugMarkerMarkerInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_marker_name : *const u8,
	pub color : [f32; 4],
}

impl DebugMarkerMarkerInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000022002,
			p_next : null(),
			p_marker_name : null(),
			color : [0 as _ ;4],
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn marker_name(mut self, val : *const u8) -> Self {
		self.p_marker_name = val;
		self
	}
	pub fn color(mut self, val : [f32; 4]) -> Self {
		self.color = val;
		self
	}
}

impl std::default::Default for DebugMarkerMarkerInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DebugMarkerObjectTagInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub object_type : DebugReportObjectTypeEXT,
	pub object : u64,
	pub tag_name : u64,
	pub tag_size : usize,
	pub p_tag : *const c_void,
}

impl DebugMarkerObjectTagInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000022001,
			p_next : null(),
			object_type : DebugReportObjectTypeEXT::default(),
			object : <_>::default(),
			tag_name : <_>::default(),
			tag_size : <_>::default(),
			p_tag : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn object_type(mut self, val : DebugReportObjectTypeEXT) -> Self {
		self.object_type = val;
		self
	}
	pub fn object(mut self, val : u64) -> Self {
		self.object = val;
		self
	}
	pub fn tag_name(mut self, val : u64) -> Self {
		self.tag_name = val;
		self
	}
	pub fn tag_size(mut self, val : usize) -> Self {
		self.tag_size = val;
		self
	}
	pub fn tag(mut self, val : *const c_void) -> Self {
		self.p_tag = val;
		self
	}
}

impl std::default::Default for DebugMarkerObjectTagInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub required_subgroup_size : u32,
}

impl PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000225001,
			p_next : null_mut(),
			required_subgroup_size : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn required_subgroup_size(mut self, val : u32) -> Self {
		self.required_subgroup_size = val;
		self
	}
}

impl std::default::Default for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PipelineExecutableStatisticKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub name : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub description : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub format : PipelineExecutableStatisticFormatKHR,
	pub value : PipelineExecutableStatisticValueKHR,
}

impl PipelineExecutableStatisticKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000269004,
			p_next : null_mut(),
			name : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			description : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			format : PipelineExecutableStatisticFormatKHR::default(),
			value : PipelineExecutableStatisticValueKHR::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn name(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.name = val;
		self
	}
	pub fn description(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.description = val;
		self
	}
	pub fn format(mut self, val : PipelineExecutableStatisticFormatKHR) -> Self {
		self.format = val;
		self
	}
	pub fn value(mut self, val : PipelineExecutableStatisticValueKHR) -> Self {
		self.value = val;
		self
	}
}

impl std::default::Default for PipelineExecutableStatisticKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineExecutableStatisticValueKHR {
	pub b_32 : Bool32,
	pub i_64 : i64,
	pub u_64 : u64,
	pub f_64 : f64,
}

impl PipelineExecutableStatisticValueKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn b_32(mut self, val : Bool32) -> Self {
		self.b_32 = val;
		self
	}
	pub fn i_64(mut self, val : i64) -> Self {
		self.i_64 = val;
		self
	}
	pub fn u_64(mut self, val : u64) -> Self {
		self.u_64 = val;
		self
	}
	pub fn f_64(mut self, val : f64) -> Self {
		self.f_64 = val;
		self
	}
}

impl std::default::Default for PipelineExecutableStatisticValueKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineExecutableInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub pipeline : Pipeline,
	pub executable_index : u32,
}

impl PipelineExecutableInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000269003,
			p_next : null(),
			pipeline : Pipeline(0),
			executable_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn pipeline(mut self, val : Pipeline) -> Self {
		self.pipeline = val;
		self
	}
	pub fn executable_index(mut self, val : u32) -> Self {
		self.executable_index = val;
		self
	}
}

impl std::default::Default for PipelineExecutableInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub cooperative_matrix : Bool32,
	pub cooperative_matrix_robust_buffer_access : Bool32,
}

impl PhysicalDeviceCooperativeMatrixFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000249000,
			p_next : null_mut(),
			cooperative_matrix : <_>::default(),
			cooperative_matrix_robust_buffer_access : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn cooperative_matrix(mut self, val : Bool32) -> Self {
		self.cooperative_matrix = val;
		self
	}
	pub fn cooperative_matrix_robust_buffer_access(mut self, val : Bool32) -> Self {
		self.cooperative_matrix_robust_buffer_access = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceCooperativeMatrixFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PipelineExecutablePropertiesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub stages : ShaderStageFlags,
	pub name : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub description : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub subgroup_size : u32,
}

impl PipelineExecutablePropertiesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000269002,
			p_next : null_mut(),
			stages : <_>::default(),
			name : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			description : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			subgroup_size : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn stages(mut self, val : ShaderStageFlags) -> Self {
		self.stages = val;
		self
	}
	pub fn name(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.name = val;
		self
	}
	pub fn description(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.description = val;
		self
	}
	pub fn subgroup_size(mut self, val : u32) -> Self {
		self.subgroup_size = val;
		self
	}
}

impl std::default::Default for PipelineExecutablePropertiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayPlaneCapabilities2KHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub capabilities : DisplayPlaneCapabilitiesKHR,
}

impl DisplayPlaneCapabilities2KHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000121004,
			p_next : null_mut(),
			capabilities : DisplayPlaneCapabilitiesKHR::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn capabilities(mut self, val : DisplayPlaneCapabilitiesKHR) -> Self {
		self.capabilities = val;
		self
	}
}

impl std::default::Default for DisplayPlaneCapabilities2KHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct XlibSurfaceCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : XlibSurfaceCreateFlagsKHR,
	pub dpy : *mut Display,
	pub window : u32,
}

impl XlibSurfaceCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000004000,
			p_next : null(),
			flags : <_>::default(),
			dpy : null_mut(),
			window : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : XlibSurfaceCreateFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn dpy(mut self, val : *mut Display) -> Self {
		self.dpy = val;
		self
	}
	pub fn window(mut self, val : u32) -> Self {
		self.window = val;
		self
	}
}

impl std::default::Default for XlibSurfaceCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub index_type_uint_8 : Bool32,
}

impl PhysicalDeviceIndexTypeUint8FeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000265000,
			p_next : null_mut(),
			index_type_uint_8 : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn index_type_uint_8(mut self, val : Bool32) -> Self {
		self.index_type_uint_8 = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceIndexTypeUint8FeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub coverage_reduction_mode : Bool32,
}

impl PhysicalDeviceCoverageReductionModeFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000250000,
			p_next : null_mut(),
			coverage_reduction_mode : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn coverage_reduction_mode(mut self, val : Bool32) -> Self {
		self.coverage_reduction_mode = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceCoverageReductionModeFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayModeProperties2KHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub display_mode_properties : DisplayModePropertiesKHR,
}

impl DisplayModeProperties2KHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000121002,
			p_next : null_mut(),
			display_mode_properties : DisplayModePropertiesKHR::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn display_mode_properties(mut self, val : DisplayModePropertiesKHR) -> Self {
		self.display_mode_properties = val;
		self
	}
}

impl std::default::Default for DisplayModeProperties2KHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayProperties2KHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub display_properties : DisplayPropertiesKHR,
}

impl DisplayProperties2KHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000121000,
			p_next : null_mut(),
			display_properties : DisplayPropertiesKHR::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn display_properties(mut self, val : DisplayPropertiesKHR) -> Self {
		self.display_properties = val;
		self
	}
}

impl std::default::Default for DisplayProperties2KHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AndroidSurfaceCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : AndroidSurfaceCreateFlagsKHR,
	pub window : *mut ANativeWindow,
}

impl AndroidSurfaceCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000008000,
			p_next : null(),
			flags : <_>::default(),
			window : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : AndroidSurfaceCreateFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn window(mut self, val : *mut ANativeWindow) -> Self {
		self.window = val;
		self
	}
}

impl std::default::Default for AndroidSurfaceCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub image_view_type : ImageViewType,
}

impl PhysicalDeviceImageViewImageFormatInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000170000,
			p_next : null_mut(),
			image_view_type : ImageViewType::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image_view_type(mut self, val : ImageViewType) -> Self {
		self.image_view_type = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceImageViewImageFormatInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerformanceQuerySubmitInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub counter_pass_index : u32,
}

impl PerformanceQuerySubmitInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000116003,
			p_next : null(),
			counter_pass_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn counter_pass_index(mut self, val : u32) -> Self {
		self.counter_pass_index = val;
		self
	}
}

impl std::default::Default for PerformanceQuerySubmitInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSubgroupSizeControlPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub min_subgroup_size : u32,
	pub max_subgroup_size : u32,
	pub max_compute_workgroup_subgroups : u32,
	pub required_subgroup_size_stages : ShaderStageFlags,
}

impl PhysicalDeviceSubgroupSizeControlPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000225000,
			p_next : null_mut(),
			min_subgroup_size : <_>::default(),
			max_subgroup_size : <_>::default(),
			max_compute_workgroup_subgroups : <_>::default(),
			required_subgroup_size_stages : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn min_subgroup_size(mut self, val : u32) -> Self {
		self.min_subgroup_size = val;
		self
	}
	pub fn max_subgroup_size(mut self, val : u32) -> Self {
		self.max_subgroup_size = val;
		self
	}
	pub fn max_compute_workgroup_subgroups(mut self, val : u32) -> Self {
		self.max_compute_workgroup_subgroups = val;
		self
	}
	pub fn required_subgroup_size_stages(mut self, val : ShaderStageFlags) -> Self {
		self.required_subgroup_size_stages = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSubgroupSizeControlPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerformanceCounterResultKHR {
	pub int_32 : i32,
	pub int_64 : i64,
	pub uint_32 : u32,
	pub uint_64 : u64,
	pub float_32 : f32,
	pub float_64 : f64,
}

impl PerformanceCounterResultKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn int_32(mut self, val : i32) -> Self {
		self.int_32 = val;
		self
	}
	pub fn int_64(mut self, val : i64) -> Self {
		self.int_64 = val;
		self
	}
	pub fn uint_32(mut self, val : u32) -> Self {
		self.uint_32 = val;
		self
	}
	pub fn uint_64(mut self, val : u64) -> Self {
		self.uint_64 = val;
		self
	}
	pub fn float_32(mut self, val : f32) -> Self {
		self.float_32 = val;
		self
	}
	pub fn float_64(mut self, val : f64) -> Self {
		self.float_64 = val;
		self
	}
}

impl std::default::Default for PerformanceCounterResultKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub custom_border_colors : Bool32,
	pub custom_border_color_without_format : Bool32,
}

impl PhysicalDeviceCustomBorderColorFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000287002,
			p_next : null_mut(),
			custom_border_colors : <_>::default(),
			custom_border_color_without_format : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn custom_border_colors(mut self, val : Bool32) -> Self {
		self.custom_border_colors = val;
		self
	}
	pub fn custom_border_color_without_format(mut self, val : Bool32) -> Self {
		self.custom_border_color_without_format = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceCustomBorderColorFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ValidationFlagsEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub disabled_validation_check_count : u32,
	pub p_disabled_validation_checks : *const ValidationCheckEXT,
}

impl ValidationFlagsEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000061000,
			p_next : null(),
			disabled_validation_check_count : <_>::default(),
			p_disabled_validation_checks : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn disabled_validation_checks(mut self, val : &[ValidationCheckEXT]) -> Self {
		self.disabled_validation_check_count = val.len() as _;
		self.p_disabled_validation_checks = val.as_ptr();
		self
	}
	pub fn disabled_validation_check(mut self, val : &ValidationCheckEXT) -> Self {
		self.disabled_validation_check_count = 1;
		self.p_disabled_validation_checks = val;
		self
	}
}

impl std::default::Default for ValidationFlagsEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct QueryPoolPerformanceCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub queue_family_index : u32,
	pub counter_index_count : u32,
	pub p_counter_indices : *const u32,
}

impl QueryPoolPerformanceCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000116002,
			p_next : null(),
			queue_family_index : <_>::default(),
			counter_index_count : <_>::default(),
			p_counter_indices : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn queue_family_index(mut self, val : u32) -> Self {
		self.queue_family_index = val;
		self
	}
	pub fn counter_indices(mut self, val : &[u32]) -> Self {
		self.counter_index_count = val.len() as _;
		self.p_counter_indices = val.as_ptr();
		self
	}
	pub fn counter_indice(mut self, val : &u32) -> Self {
		self.counter_index_count = 1;
		self.p_counter_indices = val;
		self
	}
}

impl std::default::Default for QueryPoolPerformanceCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PerformanceCounterDescriptionKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PerformanceCounterDescriptionFlagsKHR,
	pub name : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub category : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub description : [u8; VK_MAX_DESCRIPTION_SIZE],
}

impl PerformanceCounterDescriptionKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000116006,
			p_next : null(),
			flags : <_>::default(),
			name : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			category : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			description : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PerformanceCounterDescriptionFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn name(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.name = val;
		self
	}
	pub fn category(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.category = val;
		self
	}
	pub fn description(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.description = val;
		self
	}
}

impl std::default::Default for PerformanceCounterDescriptionKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PerformanceCounterKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub unit : PerformanceCounterUnitKHR,
	pub scope : PerformanceCounterScopeKHR,
	pub storage : PerformanceCounterStorageKHR,
	pub uuid : [u8; VK_UUID_SIZE],
}

impl PerformanceCounterKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000116005,
			p_next : null(),
			unit : PerformanceCounterUnitKHR::default(),
			scope : PerformanceCounterScopeKHR::default(),
			storage : PerformanceCounterStorageKHR::default(),
			uuid : [0 as _ ;VK_UUID_SIZE],
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn unit(mut self, val : PerformanceCounterUnitKHR) -> Self {
		self.unit = val;
		self
	}
	pub fn scope(mut self, val : PerformanceCounterScopeKHR) -> Self {
		self.scope = val;
		self
	}
	pub fn storage(mut self, val : PerformanceCounterStorageKHR) -> Self {
		self.storage = val;
		self
	}
	pub fn uuid(mut self, val : [u8; VK_UUID_SIZE]) -> Self {
		self.uuid = val;
		self
	}
}

impl std::default::Default for PerformanceCounterKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ValidationCacheCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : ValidationCacheCreateFlagsEXT,
	pub initial_data_size : usize,
	pub p_initial_data : *const c_void,
}

impl ValidationCacheCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000160000,
			p_next : null(),
			flags : <_>::default(),
			initial_data_size : <_>::default(),
			p_initial_data : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : ValidationCacheCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn initial_data_size(mut self, val : usize) -> Self {
		self.initial_data_size = val;
		self
	}
	pub fn initial_data(mut self, val : *const c_void) -> Self {
		self.p_initial_data = val;
		self
	}
}

impl std::default::Default for ValidationCacheCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryRequirements2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub memory_requirements : MemoryRequirements,
}

impl MemoryRequirements2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000146003,
			p_next : null_mut(),
			memory_requirements : MemoryRequirements::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory_requirements(mut self, val : MemoryRequirements) -> Self {
		self.memory_requirements = val;
		self
	}
}

impl std::default::Default for MemoryRequirements2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageSparseMemoryRequirementsInfo2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub image : Image,
}

impl ImageSparseMemoryRequirementsInfo2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000146002,
			p_next : null(),
			image : Image(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
}

impl std::default::Default for ImageSparseMemoryRequirementsInfo2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub min_imported_host_pointer_alignment : DeviceSize,
}

impl PhysicalDeviceExternalMemoryHostPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000178002,
			p_next : null_mut(),
			min_imported_host_pointer_alignment : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn min_imported_host_pointer_alignment(mut self, val : DeviceSize) -> Self {
		self.min_imported_host_pointer_alignment = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceExternalMemoryHostPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub transform_feedback : Bool32,
	pub geometry_streams : Bool32,
}

impl PhysicalDeviceTransformFeedbackFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000028000,
			p_next : null_mut(),
			transform_feedback : <_>::default(),
			geometry_streams : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn transform_feedback(mut self, val : Bool32) -> Self {
		self.transform_feedback = val;
		self
	}
	pub fn geometry_streams(mut self, val : Bool32) -> Self {
		self.geometry_streams = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceTransformFeedbackFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct StencilOpState {
	pub fail_op : StencilOp,
	pub pass_op : StencilOp,
	pub depth_fail_op : StencilOp,
	pub compare_op : CompareOp,
	pub compare_mask : u32,
	pub write_mask : u32,
	pub reference : u32,
}

impl StencilOpState {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn fail_op(mut self, val : StencilOp) -> Self {
		self.fail_op = val;
		self
	}
	pub fn pass_op(mut self, val : StencilOp) -> Self {
		self.pass_op = val;
		self
	}
	pub fn depth_fail_op(mut self, val : StencilOp) -> Self {
		self.depth_fail_op = val;
		self
	}
	pub fn compare_op(mut self, val : CompareOp) -> Self {
		self.compare_op = val;
		self
	}
	pub fn compare_mask(mut self, val : u32) -> Self {
		self.compare_mask = val;
		self
	}
	pub fn write_mask(mut self, val : u32) -> Self {
		self.write_mask = val;
		self
	}
	pub fn reference(mut self, val : u32) -> Self {
		self.reference = val;
		self
	}
}

impl std::default::Default for StencilOpState {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalMemoryProperties {
	pub external_memory_features : ExternalMemoryFeatureFlags,
	pub export_from_imported_handle_types : ExternalMemoryHandleTypeFlags,
	pub compatible_handle_types : ExternalMemoryHandleTypeFlags,
}

impl ExternalMemoryProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn external_memory_features(mut self, val : ExternalMemoryFeatureFlags) -> Self {
		self.external_memory_features = val;
		self
	}
	pub fn export_from_imported_handle_types(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.export_from_imported_handle_types = val;
		self
	}
	pub fn compatible_handle_types(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.compatible_handle_types = val;
		self
	}
}

impl std::default::Default for ExternalMemoryProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DrawMeshTasksIndirectCommandNV {
	pub task_count : u32,
	pub first_task : u32,
}

impl DrawMeshTasksIndirectCommandNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn task_count(mut self, val : u32) -> Self {
		self.task_count = val;
		self
	}
	pub fn first_task(mut self, val : u32) -> Self {
		self.first_task = val;
		self
	}
}

impl std::default::Default for DrawMeshTasksIndirectCommandNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceGroupProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub physical_device_count : u32,
	pub physical_devices : [PhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE],
	pub subset_allocation : Bool32,
}

impl PhysicalDeviceGroupProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000070000,
			p_next : null_mut(),
			physical_device_count : <_>::default(),
			physical_devices : [PhysicalDevice(0); VK_MAX_DEVICE_GROUP_SIZE],
			subset_allocation : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn physical_device_count(mut self, val : u32) -> Self {
		self.physical_device_count = val;
		self
	}
	pub fn physical_devices(mut self, val : [PhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE]) -> Self {
		self.physical_devices = val;
		self
	}
	pub fn subset_allocation(mut self, val : Bool32) -> Self {
		self.subset_allocation = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceGroupProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PastPresentationTimingGOOGLE {
	pub present_id : u32,
	pub desired_present_time : u64,
	pub actual_present_time : u64,
	pub earliest_present_time : u64,
	pub present_margin : u64,
}

impl PastPresentationTimingGOOGLE {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn present_id(mut self, val : u32) -> Self {
		self.present_id = val;
		self
	}
	pub fn desired_present_time(mut self, val : u64) -> Self {
		self.desired_present_time = val;
		self
	}
	pub fn actual_present_time(mut self, val : u64) -> Self {
		self.actual_present_time = val;
		self
	}
	pub fn earliest_present_time(mut self, val : u64) -> Self {
		self.earliest_present_time = val;
		self
	}
	pub fn present_margin(mut self, val : u64) -> Self {
		self.present_margin = val;
		self
	}
}

impl std::default::Default for PastPresentationTimingGOOGLE {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ShaderResourceUsageAMD {
	pub num_used_vgprs : u32,
	pub num_used_sgprs : u32,
	pub lds_size_per_local_work_group : u32,
	pub lds_usage_size_in_bytes : usize,
	pub scratch_mem_usage_in_bytes : usize,
}

impl ShaderResourceUsageAMD {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn num_used_vgprs(mut self, val : u32) -> Self {
		self.num_used_vgprs = val;
		self
	}
	pub fn num_used_sgprs(mut self, val : u32) -> Self {
		self.num_used_sgprs = val;
		self
	}
	pub fn lds_size_per_local_work_group(mut self, val : u32) -> Self {
		self.lds_size_per_local_work_group = val;
		self
	}
	pub fn lds_usage_size_in_bytes(mut self, val : usize) -> Self {
		self.lds_usage_size_in_bytes = val;
		self
	}
	pub fn scratch_mem_usage_in_bytes(mut self, val : usize) -> Self {
		self.scratch_mem_usage_in_bytes = val;
		self
	}
}

impl std::default::Default for ShaderResourceUsageAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceLimits {
	pub max_image_dimension_1_d : u32,
	pub max_image_dimension_2_d : u32,
	pub max_image_dimension_3_d : u32,
	pub max_image_dimension_cube : u32,
	pub max_image_array_layers : u32,
	pub max_texel_buffer_elements : u32,
	pub max_uniform_buffer_range : u32,
	pub max_storage_buffer_range : u32,
	pub max_push_constants_size : u32,
	pub max_memory_allocation_count : u32,
	pub max_sampler_allocation_count : u32,
	pub buffer_image_granularity : DeviceSize,
	pub sparse_address_space_size : DeviceSize,
	pub max_bound_descriptor_sets : u32,
	pub max_per_stage_descriptor_samplers : u32,
	pub max_per_stage_descriptor_uniform_buffers : u32,
	pub max_per_stage_descriptor_storage_buffers : u32,
	pub max_per_stage_descriptor_sampled_images : u32,
	pub max_per_stage_descriptor_storage_images : u32,
	pub max_per_stage_descriptor_input_attachments : u32,
	pub max_per_stage_resources : u32,
	pub max_descriptor_set_samplers : u32,
	pub max_descriptor_set_uniform_buffers : u32,
	pub max_descriptor_set_uniform_buffers_dynamic : u32,
	pub max_descriptor_set_storage_buffers : u32,
	pub max_descriptor_set_storage_buffers_dynamic : u32,
	pub max_descriptor_set_sampled_images : u32,
	pub max_descriptor_set_storage_images : u32,
	pub max_descriptor_set_input_attachments : u32,
	pub max_vertex_input_attributes : u32,
	pub max_vertex_input_bindings : u32,
	pub max_vertex_input_attribute_offset : u32,
	pub max_vertex_input_binding_stride : u32,
	pub max_vertex_output_components : u32,
	pub max_tessellation_generation_level : u32,
	pub max_tessellation_patch_size : u32,
	pub max_tessellation_control_per_vertex_input_components : u32,
	pub max_tessellation_control_per_vertex_output_components : u32,
	pub max_tessellation_control_per_patch_output_components : u32,
	pub max_tessellation_control_total_output_components : u32,
	pub max_tessellation_evaluation_input_components : u32,
	pub max_tessellation_evaluation_output_components : u32,
	pub max_geometry_shader_invocations : u32,
	pub max_geometry_input_components : u32,
	pub max_geometry_output_components : u32,
	pub max_geometry_output_vertices : u32,
	pub max_geometry_total_output_components : u32,
	pub max_fragment_input_components : u32,
	pub max_fragment_output_attachments : u32,
	pub max_fragment_dual_src_attachments : u32,
	pub max_fragment_combined_output_resources : u32,
	pub max_compute_shared_memory_size : u32,
	pub max_compute_work_group_count : [u32; 3],
	pub max_compute_work_group_invocations : u32,
	pub max_compute_work_group_size : [u32; 3],
	pub sub_pixel_precision_bits : u32,
	pub sub_texel_precision_bits : u32,
	pub mipmap_precision_bits : u32,
	pub max_draw_indexed_index_value : u32,
	pub max_draw_indirect_count : u32,
	pub max_sampler_lod_bias : f32,
	pub max_sampler_anisotropy : f32,
	pub max_viewports : u32,
	pub max_viewport_dimensions : [u32; 2],
	pub viewport_bounds_range : [f32; 2],
	pub viewport_sub_pixel_bits : u32,
	pub min_memory_map_alignment : usize,
	pub min_texel_buffer_offset_alignment : DeviceSize,
	pub min_uniform_buffer_offset_alignment : DeviceSize,
	pub min_storage_buffer_offset_alignment : DeviceSize,
	pub min_texel_offset : i32,
	pub max_texel_offset : u32,
	pub min_texel_gather_offset : i32,
	pub max_texel_gather_offset : u32,
	pub min_interpolation_offset : f32,
	pub max_interpolation_offset : f32,
	pub sub_pixel_interpolation_offset_bits : u32,
	pub max_framebuffer_width : u32,
	pub max_framebuffer_height : u32,
	pub max_framebuffer_layers : u32,
	pub framebuffer_color_sample_counts : SampleCountFlags,
	pub framebuffer_depth_sample_counts : SampleCountFlags,
	pub framebuffer_stencil_sample_counts : SampleCountFlags,
	pub framebuffer_no_attachments_sample_counts : SampleCountFlags,
	pub max_color_attachments : u32,
	pub sampled_image_color_sample_counts : SampleCountFlags,
	pub sampled_image_integer_sample_counts : SampleCountFlags,
	pub sampled_image_depth_sample_counts : SampleCountFlags,
	pub sampled_image_stencil_sample_counts : SampleCountFlags,
	pub storage_image_sample_counts : SampleCountFlags,
	pub max_sample_mask_words : u32,
	pub timestamp_compute_and_graphics : Bool32,
	pub timestamp_period : f32,
	pub max_clip_distances : u32,
	pub max_cull_distances : u32,
	pub max_combined_clip_and_cull_distances : u32,
	pub discrete_queue_priorities : u32,
	pub point_size_range : [f32; 2],
	pub line_width_range : [f32; 2],
	pub point_size_granularity : f32,
	pub line_width_granularity : f32,
	pub strict_lines : Bool32,
	pub standard_sample_locations : Bool32,
	pub optimal_buffer_copy_offset_alignment : DeviceSize,
	pub optimal_buffer_copy_row_pitch_alignment : DeviceSize,
	pub non_coherent_atom_size : DeviceSize,
}

impl PhysicalDeviceLimits {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn max_image_dimension_1_d(mut self, val : u32) -> Self {
		self.max_image_dimension_1_d = val;
		self
	}
	pub fn max_image_dimension_2_d(mut self, val : u32) -> Self {
		self.max_image_dimension_2_d = val;
		self
	}
	pub fn max_image_dimension_3_d(mut self, val : u32) -> Self {
		self.max_image_dimension_3_d = val;
		self
	}
	pub fn max_image_dimension_cube(mut self, val : u32) -> Self {
		self.max_image_dimension_cube = val;
		self
	}
	pub fn max_image_array_layers(mut self, val : u32) -> Self {
		self.max_image_array_layers = val;
		self
	}
	pub fn max_texel_buffer_elements(mut self, val : u32) -> Self {
		self.max_texel_buffer_elements = val;
		self
	}
	pub fn max_uniform_buffer_range(mut self, val : u32) -> Self {
		self.max_uniform_buffer_range = val;
		self
	}
	pub fn max_storage_buffer_range(mut self, val : u32) -> Self {
		self.max_storage_buffer_range = val;
		self
	}
	pub fn max_push_constants_size(mut self, val : u32) -> Self {
		self.max_push_constants_size = val;
		self
	}
	pub fn max_memory_allocation_count(mut self, val : u32) -> Self {
		self.max_memory_allocation_count = val;
		self
	}
	pub fn max_sampler_allocation_count(mut self, val : u32) -> Self {
		self.max_sampler_allocation_count = val;
		self
	}
	pub fn buffer_image_granularity(mut self, val : DeviceSize) -> Self {
		self.buffer_image_granularity = val;
		self
	}
	pub fn sparse_address_space_size(mut self, val : DeviceSize) -> Self {
		self.sparse_address_space_size = val;
		self
	}
	pub fn max_bound_descriptor_sets(mut self, val : u32) -> Self {
		self.max_bound_descriptor_sets = val;
		self
	}
	pub fn max_per_stage_descriptor_samplers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_samplers = val;
		self
	}
	pub fn max_per_stage_descriptor_uniform_buffers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_uniform_buffers = val;
		self
	}
	pub fn max_per_stage_descriptor_storage_buffers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_storage_buffers = val;
		self
	}
	pub fn max_per_stage_descriptor_sampled_images(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_sampled_images = val;
		self
	}
	pub fn max_per_stage_descriptor_storage_images(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_storage_images = val;
		self
	}
	pub fn max_per_stage_descriptor_input_attachments(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_input_attachments = val;
		self
	}
	pub fn max_per_stage_resources(mut self, val : u32) -> Self {
		self.max_per_stage_resources = val;
		self
	}
	pub fn max_descriptor_set_samplers(mut self, val : u32) -> Self {
		self.max_descriptor_set_samplers = val;
		self
	}
	pub fn max_descriptor_set_uniform_buffers(mut self, val : u32) -> Self {
		self.max_descriptor_set_uniform_buffers = val;
		self
	}
	pub fn max_descriptor_set_uniform_buffers_dynamic(mut self, val : u32) -> Self {
		self.max_descriptor_set_uniform_buffers_dynamic = val;
		self
	}
	pub fn max_descriptor_set_storage_buffers(mut self, val : u32) -> Self {
		self.max_descriptor_set_storage_buffers = val;
		self
	}
	pub fn max_descriptor_set_storage_buffers_dynamic(mut self, val : u32) -> Self {
		self.max_descriptor_set_storage_buffers_dynamic = val;
		self
	}
	pub fn max_descriptor_set_sampled_images(mut self, val : u32) -> Self {
		self.max_descriptor_set_sampled_images = val;
		self
	}
	pub fn max_descriptor_set_storage_images(mut self, val : u32) -> Self {
		self.max_descriptor_set_storage_images = val;
		self
	}
	pub fn max_descriptor_set_input_attachments(mut self, val : u32) -> Self {
		self.max_descriptor_set_input_attachments = val;
		self
	}
	pub fn max_vertex_input_attributes(mut self, val : u32) -> Self {
		self.max_vertex_input_attributes = val;
		self
	}
	pub fn max_vertex_input_bindings(mut self, val : u32) -> Self {
		self.max_vertex_input_bindings = val;
		self
	}
	pub fn max_vertex_input_attribute_offset(mut self, val : u32) -> Self {
		self.max_vertex_input_attribute_offset = val;
		self
	}
	pub fn max_vertex_input_binding_stride(mut self, val : u32) -> Self {
		self.max_vertex_input_binding_stride = val;
		self
	}
	pub fn max_vertex_output_components(mut self, val : u32) -> Self {
		self.max_vertex_output_components = val;
		self
	}
	pub fn max_tessellation_generation_level(mut self, val : u32) -> Self {
		self.max_tessellation_generation_level = val;
		self
	}
	pub fn max_tessellation_patch_size(mut self, val : u32) -> Self {
		self.max_tessellation_patch_size = val;
		self
	}
	pub fn max_tessellation_control_per_vertex_input_components(mut self, val : u32) -> Self {
		self.max_tessellation_control_per_vertex_input_components = val;
		self
	}
	pub fn max_tessellation_control_per_vertex_output_components(mut self, val : u32) -> Self {
		self.max_tessellation_control_per_vertex_output_components = val;
		self
	}
	pub fn max_tessellation_control_per_patch_output_components(mut self, val : u32) -> Self {
		self.max_tessellation_control_per_patch_output_components = val;
		self
	}
	pub fn max_tessellation_control_total_output_components(mut self, val : u32) -> Self {
		self.max_tessellation_control_total_output_components = val;
		self
	}
	pub fn max_tessellation_evaluation_input_components(mut self, val : u32) -> Self {
		self.max_tessellation_evaluation_input_components = val;
		self
	}
	pub fn max_tessellation_evaluation_output_components(mut self, val : u32) -> Self {
		self.max_tessellation_evaluation_output_components = val;
		self
	}
	pub fn max_geometry_shader_invocations(mut self, val : u32) -> Self {
		self.max_geometry_shader_invocations = val;
		self
	}
	pub fn max_geometry_input_components(mut self, val : u32) -> Self {
		self.max_geometry_input_components = val;
		self
	}
	pub fn max_geometry_output_components(mut self, val : u32) -> Self {
		self.max_geometry_output_components = val;
		self
	}
	pub fn max_geometry_output_vertices(mut self, val : u32) -> Self {
		self.max_geometry_output_vertices = val;
		self
	}
	pub fn max_geometry_total_output_components(mut self, val : u32) -> Self {
		self.max_geometry_total_output_components = val;
		self
	}
	pub fn max_fragment_input_components(mut self, val : u32) -> Self {
		self.max_fragment_input_components = val;
		self
	}
	pub fn max_fragment_output_attachments(mut self, val : u32) -> Self {
		self.max_fragment_output_attachments = val;
		self
	}
	pub fn max_fragment_dual_src_attachments(mut self, val : u32) -> Self {
		self.max_fragment_dual_src_attachments = val;
		self
	}
	pub fn max_fragment_combined_output_resources(mut self, val : u32) -> Self {
		self.max_fragment_combined_output_resources = val;
		self
	}
	pub fn max_compute_shared_memory_size(mut self, val : u32) -> Self {
		self.max_compute_shared_memory_size = val;
		self
	}
	pub fn max_compute_work_group_count(mut self, val : [u32; 3]) -> Self {
		self.max_compute_work_group_count = val;
		self
	}
	pub fn max_compute_work_group_invocations(mut self, val : u32) -> Self {
		self.max_compute_work_group_invocations = val;
		self
	}
	pub fn max_compute_work_group_size(mut self, val : [u32; 3]) -> Self {
		self.max_compute_work_group_size = val;
		self
	}
	pub fn sub_pixel_precision_bits(mut self, val : u32) -> Self {
		self.sub_pixel_precision_bits = val;
		self
	}
	pub fn sub_texel_precision_bits(mut self, val : u32) -> Self {
		self.sub_texel_precision_bits = val;
		self
	}
	pub fn mipmap_precision_bits(mut self, val : u32) -> Self {
		self.mipmap_precision_bits = val;
		self
	}
	pub fn max_draw_indexed_index_value(mut self, val : u32) -> Self {
		self.max_draw_indexed_index_value = val;
		self
	}
	pub fn max_draw_indirect_count(mut self, val : u32) -> Self {
		self.max_draw_indirect_count = val;
		self
	}
	pub fn max_sampler_lod_bias(mut self, val : f32) -> Self {
		self.max_sampler_lod_bias = val;
		self
	}
	pub fn max_sampler_anisotropy(mut self, val : f32) -> Self {
		self.max_sampler_anisotropy = val;
		self
	}
	pub fn max_viewports(mut self, val : u32) -> Self {
		self.max_viewports = val;
		self
	}
	pub fn max_viewport_dimensions(mut self, val : [u32; 2]) -> Self {
		self.max_viewport_dimensions = val;
		self
	}
	pub fn viewport_bounds_range(mut self, val : [f32; 2]) -> Self {
		self.viewport_bounds_range = val;
		self
	}
	pub fn viewport_sub_pixel_bits(mut self, val : u32) -> Self {
		self.viewport_sub_pixel_bits = val;
		self
	}
	pub fn min_memory_map_alignment(mut self, val : usize) -> Self {
		self.min_memory_map_alignment = val;
		self
	}
	pub fn min_texel_buffer_offset_alignment(mut self, val : DeviceSize) -> Self {
		self.min_texel_buffer_offset_alignment = val;
		self
	}
	pub fn min_uniform_buffer_offset_alignment(mut self, val : DeviceSize) -> Self {
		self.min_uniform_buffer_offset_alignment = val;
		self
	}
	pub fn min_storage_buffer_offset_alignment(mut self, val : DeviceSize) -> Self {
		self.min_storage_buffer_offset_alignment = val;
		self
	}
	pub fn min_texel_offset(mut self, val : i32) -> Self {
		self.min_texel_offset = val;
		self
	}
	pub fn max_texel_offset(mut self, val : u32) -> Self {
		self.max_texel_offset = val;
		self
	}
	pub fn min_texel_gather_offset(mut self, val : i32) -> Self {
		self.min_texel_gather_offset = val;
		self
	}
	pub fn max_texel_gather_offset(mut self, val : u32) -> Self {
		self.max_texel_gather_offset = val;
		self
	}
	pub fn min_interpolation_offset(mut self, val : f32) -> Self {
		self.min_interpolation_offset = val;
		self
	}
	pub fn max_interpolation_offset(mut self, val : f32) -> Self {
		self.max_interpolation_offset = val;
		self
	}
	pub fn sub_pixel_interpolation_offset_bits(mut self, val : u32) -> Self {
		self.sub_pixel_interpolation_offset_bits = val;
		self
	}
	pub fn max_framebuffer_width(mut self, val : u32) -> Self {
		self.max_framebuffer_width = val;
		self
	}
	pub fn max_framebuffer_height(mut self, val : u32) -> Self {
		self.max_framebuffer_height = val;
		self
	}
	pub fn max_framebuffer_layers(mut self, val : u32) -> Self {
		self.max_framebuffer_layers = val;
		self
	}
	pub fn framebuffer_color_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.framebuffer_color_sample_counts = val;
		self
	}
	pub fn framebuffer_depth_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.framebuffer_depth_sample_counts = val;
		self
	}
	pub fn framebuffer_stencil_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.framebuffer_stencil_sample_counts = val;
		self
	}
	pub fn framebuffer_no_attachments_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.framebuffer_no_attachments_sample_counts = val;
		self
	}
	pub fn max_color_attachments(mut self, val : u32) -> Self {
		self.max_color_attachments = val;
		self
	}
	pub fn sampled_image_color_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.sampled_image_color_sample_counts = val;
		self
	}
	pub fn sampled_image_integer_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.sampled_image_integer_sample_counts = val;
		self
	}
	pub fn sampled_image_depth_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.sampled_image_depth_sample_counts = val;
		self
	}
	pub fn sampled_image_stencil_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.sampled_image_stencil_sample_counts = val;
		self
	}
	pub fn storage_image_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.storage_image_sample_counts = val;
		self
	}
	pub fn max_sample_mask_words(mut self, val : u32) -> Self {
		self.max_sample_mask_words = val;
		self
	}
	pub fn timestamp_compute_and_graphics(mut self, val : Bool32) -> Self {
		self.timestamp_compute_and_graphics = val;
		self
	}
	pub fn timestamp_period(mut self, val : f32) -> Self {
		self.timestamp_period = val;
		self
	}
	pub fn max_clip_distances(mut self, val : u32) -> Self {
		self.max_clip_distances = val;
		self
	}
	pub fn max_cull_distances(mut self, val : u32) -> Self {
		self.max_cull_distances = val;
		self
	}
	pub fn max_combined_clip_and_cull_distances(mut self, val : u32) -> Self {
		self.max_combined_clip_and_cull_distances = val;
		self
	}
	pub fn discrete_queue_priorities(mut self, val : u32) -> Self {
		self.discrete_queue_priorities = val;
		self
	}
	pub fn point_size_range(mut self, val : [f32; 2]) -> Self {
		self.point_size_range = val;
		self
	}
	pub fn line_width_range(mut self, val : [f32; 2]) -> Self {
		self.line_width_range = val;
		self
	}
	pub fn point_size_granularity(mut self, val : f32) -> Self {
		self.point_size_granularity = val;
		self
	}
	pub fn line_width_granularity(mut self, val : f32) -> Self {
		self.line_width_granularity = val;
		self
	}
	pub fn strict_lines(mut self, val : Bool32) -> Self {
		self.strict_lines = val;
		self
	}
	pub fn standard_sample_locations(mut self, val : Bool32) -> Self {
		self.standard_sample_locations = val;
		self
	}
	pub fn optimal_buffer_copy_offset_alignment(mut self, val : DeviceSize) -> Self {
		self.optimal_buffer_copy_offset_alignment = val;
		self
	}
	pub fn optimal_buffer_copy_row_pitch_alignment(mut self, val : DeviceSize) -> Self {
		self.optimal_buffer_copy_row_pitch_alignment = val;
		self
	}
	pub fn non_coherent_atom_size(mut self, val : DeviceSize) -> Self {
		self.non_coherent_atom_size = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceLimits {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub performance_counter_query_pools : Bool32,
	pub performance_counter_multiple_query_pools : Bool32,
}

impl PhysicalDevicePerformanceQueryFeaturesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000116000,
			p_next : null_mut(),
			performance_counter_query_pools : <_>::default(),
			performance_counter_multiple_query_pools : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn performance_counter_query_pools(mut self, val : Bool32) -> Self {
		self.performance_counter_query_pools = val;
		self
	}
	pub fn performance_counter_multiple_query_pools(mut self, val : Bool32) -> Self {
		self.performance_counter_multiple_query_pools = val;
		self
	}
}

impl std::default::Default for PhysicalDevicePerformanceQueryFeaturesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineCreationFeedbackEXT {
	pub flags : PipelineCreationFeedbackFlagsEXT,
	pub duration : u64,
}

impl PipelineCreationFeedbackEXT {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn flags(mut self, val : PipelineCreationFeedbackFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn duration(mut self, val : u64) -> Self {
		self.duration = val;
		self
	}
}

impl std::default::Default for PipelineCreationFeedbackEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevice16BitStorageFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub storage_buffer_16_bit_access : Bool32,
	pub uniform_and_storage_buffer_16_bit_access : Bool32,
	pub storage_push_constant_16 : Bool32,
	pub storage_input_output_16 : Bool32,
}

impl PhysicalDevice16BitStorageFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000083000,
			p_next : null_mut(),
			storage_buffer_16_bit_access : <_>::default(),
			uniform_and_storage_buffer_16_bit_access : <_>::default(),
			storage_push_constant_16 : <_>::default(),
			storage_input_output_16 : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn storage_buffer_16_bit_access(mut self, val : Bool32) -> Self {
		self.storage_buffer_16_bit_access = val;
		self
	}
	pub fn uniform_and_storage_buffer_16_bit_access(mut self, val : Bool32) -> Self {
		self.uniform_and_storage_buffer_16_bit_access = val;
		self
	}
	pub fn storage_push_constant_16(mut self, val : Bool32) -> Self {
		self.storage_push_constant_16 = val;
		self
	}
	pub fn storage_input_output_16(mut self, val : Bool32) -> Self {
		self.storage_input_output_16 = val;
		self
	}
}

impl std::default::Default for PhysicalDevice16BitStorageFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineViewportStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineViewportStateCreateFlags,
	pub viewport_count : u32,
	pub p_viewports : *const Viewport,
	pub scissor_count : u32,
	pub p_scissors : *const Rect2D,
}

impl PipelineViewportStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 22,
			p_next : null(),
			flags : <_>::default(),
			viewport_count : <_>::default(),
			p_viewports : null(),
			scissor_count : <_>::default(),
			p_scissors : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineViewportStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn viewports(mut self, val : &[Viewport]) -> Self {
		self.viewport_count = val.len() as _;
		self.p_viewports = val.as_ptr();
		self
	}
	pub fn viewport(mut self, val : &Viewport) -> Self {
		self.viewport_count = 1;
		self.p_viewports = val;
		self
	}
	pub fn scissors(mut self, val : &[Rect2D]) -> Self {
		self.scissor_count = val.len() as _;
		self.p_scissors = val.as_ptr();
		self
	}
	pub fn scissor(mut self, val : &Rect2D) -> Self {
		self.scissor_count = 1;
		self.p_scissors = val;
		self
	}
}

impl std::default::Default for PipelineViewportStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MultisamplePropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_sample_location_grid_size : Extent2D,
}

impl MultisamplePropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000143004,
			p_next : null_mut(),
			max_sample_location_grid_size : Extent2D::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_sample_location_grid_size(mut self, val : Extent2D) -> Self {
		self.max_sample_location_grid_size = val;
		self
	}
}

impl std::default::Default for MultisamplePropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalFenceProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub export_from_imported_handle_types : ExternalFenceHandleTypeFlags,
	pub compatible_handle_types : ExternalFenceHandleTypeFlags,
	pub external_fence_features : ExternalFenceFeatureFlags,
}

impl ExternalFenceProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000112001,
			p_next : null_mut(),
			export_from_imported_handle_types : <_>::default(),
			compatible_handle_types : <_>::default(),
			external_fence_features : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn export_from_imported_handle_types(mut self, val : ExternalFenceHandleTypeFlags) -> Self {
		self.export_from_imported_handle_types = val;
		self
	}
	pub fn compatible_handle_types(mut self, val : ExternalFenceHandleTypeFlags) -> Self {
		self.compatible_handle_types = val;
		self
	}
	pub fn external_fence_features(mut self, val : ExternalFenceFeatureFlags) -> Self {
		self.external_fence_features = val;
		self
	}
}

impl std::default::Default for ExternalFenceProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct WriteDescriptorSetInlineUniformBlockEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub data_size : u32,
	pub p_data : *const c_void,
}

impl WriteDescriptorSetInlineUniformBlockEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000138002,
			p_next : null(),
			data_size : <_>::default(),
			p_data : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn data_size(mut self, val : u32) -> Self {
		self.data_size = val;
		self
	}
	pub fn data(mut self, val : *const c_void) -> Self {
		self.p_data = val;
		self
	}
}

impl std::default::Default for WriteDescriptorSetInlineUniformBlockEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassBeginInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub render_pass : RenderPass,
	pub framebuffer : Framebuffer,
	pub render_area : Rect2D,
	pub clear_value_count : u32,
	pub p_clear_values : *const ClearValue,
}

impl RenderPassBeginInfo {
	pub fn new() -> Self {
		Self {
			s_type : 43,
			p_next : null(),
			render_pass : RenderPass(0),
			framebuffer : Framebuffer(0),
			render_area : Rect2D::new(),
			clear_value_count : <_>::default(),
			p_clear_values : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn render_pass(mut self, val : RenderPass) -> Self {
		self.render_pass = val;
		self
	}
	pub fn framebuffer(mut self, val : Framebuffer) -> Self {
		self.framebuffer = val;
		self
	}
	pub fn render_area(mut self, val : Rect2D) -> Self {
		self.render_area = val;
		self
	}
	pub fn clear_values(mut self, val : &[ClearValue]) -> Self {
		self.clear_value_count = val.len() as _;
		self.p_clear_values = val.as_ptr();
		self
	}
	pub fn clear_value(mut self, val : &ClearValue) -> Self {
		self.clear_value_count = 1;
		self.p_clear_values = val;
		self
	}
}

impl std::default::Default for RenderPassBeginInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindBufferMemoryDeviceGroupInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub device_index_count : u32,
	pub p_device_indices : *const u32,
}

impl BindBufferMemoryDeviceGroupInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000060013,
			p_next : null(),
			device_index_count : <_>::default(),
			p_device_indices : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_indices(mut self, val : &[u32]) -> Self {
		self.device_index_count = val.len() as _;
		self.p_device_indices = val.as_ptr();
		self
	}
	pub fn device_indice(mut self, val : &u32) -> Self {
		self.device_index_count = 1;
		self.p_device_indices = val;
		self
	}
}

impl std::default::Default for BindBufferMemoryDeviceGroupInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExportMemoryWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_attributes : *const SECURITY_ATTRIBUTES,
	pub dw_access : u32,
	pub name : *const u16,
}

impl ExportMemoryWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000073001,
			p_next : null(),
			p_attributes : null(),
			dw_access : <_>::default(),
			name : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn attributes(mut self, val : *const SECURITY_ATTRIBUTES) -> Self {
		self.p_attributes = val;
		self
	}
	pub fn dw_access(mut self, val : u32) -> Self {
		self.dw_access = val;
		self
	}
	pub fn name(mut self, val : *const u16) -> Self {
		self.name = val;
		self
	}
}

impl std::default::Default for ExportMemoryWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportMemoryFdInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_type : ExternalMemoryHandleTypeFlags,
	pub fd : i32,
}

impl ImportMemoryFdInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000074000,
			p_next : null(),
			handle_type : ExternalMemoryHandleTypeFlags::default(),
			fd : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
	pub fn fd(mut self, val : i32) -> Self {
		self.fd = val;
		self
	}
}

impl std::default::Default for ImportMemoryFdInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageCopy {
	pub src_subresource : ImageSubresourceLayers,
	pub src_offset : Offset3D,
	pub dst_subresource : ImageSubresourceLayers,
	pub dst_offset : Offset3D,
	pub extent : Extent3D,
}

impl ImageCopy {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn src_subresource(mut self, val : ImageSubresourceLayers) -> Self {
		self.src_subresource = val;
		self
	}
	pub fn src_offset(mut self, val : Offset3D) -> Self {
		self.src_offset = val;
		self
	}
	pub fn dst_subresource(mut self, val : ImageSubresourceLayers) -> Self {
		self.dst_subresource = val;
		self
	}
	pub fn dst_offset(mut self, val : Offset3D) -> Self {
		self.dst_offset = val;
		self
	}
	pub fn extent(mut self, val : Extent3D) -> Self {
		self.extent = val;
		self
	}
}

impl std::default::Default for ImageCopy {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct ImageBlit {
	pub src_subresource : ImageSubresourceLayers,
	pub src_offsets : [Offset3D; 2],
	pub dst_subresource : ImageSubresourceLayers,
	pub dst_offsets : [Offset3D; 2],
}

impl ImageBlit {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn src_subresource(mut self, val : ImageSubresourceLayers) -> Self {
		self.src_subresource = val;
		self
	}
	pub fn src_offsets(mut self, val : [Offset3D; 2]) -> Self {
		self.src_offsets = val;
		self
	}
	pub fn dst_subresource(mut self, val : ImageSubresourceLayers) -> Self {
		self.dst_subresource = val;
		self
	}
	pub fn dst_offsets(mut self, val : [Offset3D; 2]) -> Self {
		self.dst_offsets = val;
		self
	}
}

impl std::default::Default for ImageBlit {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_vertex_attrib_divisor : u32,
}

impl PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000190000,
			p_next : null_mut(),
			max_vertex_attrib_divisor : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_vertex_attrib_divisor(mut self, val : u32) -> Self {
		self.max_vertex_attrib_divisor = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorSetAllocateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub descriptor_pool : DescriptorPool,
	pub descriptor_set_count : u32,
	pub p_set_layouts : *const DescriptorSetLayout,
}

impl DescriptorSetAllocateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 34,
			p_next : null(),
			descriptor_pool : DescriptorPool(0),
			descriptor_set_count : <_>::default(),
			p_set_layouts : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn descriptor_pool(mut self, val : DescriptorPool) -> Self {
		self.descriptor_pool = val;
		self
	}
	pub fn set_layouts(mut self, val : &[DescriptorSetLayout]) -> Self {
		self.descriptor_set_count = val.len() as _;
		self.p_set_layouts = val.as_ptr();
		self
	}
	pub fn set_layout(mut self, val : &DescriptorSetLayout) -> Self {
		self.descriptor_set_count = 1;
		self.p_set_layouts = val;
		self
	}
}

impl std::default::Default for DescriptorSetAllocateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SemaphoreGetFdInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub semaphore : Semaphore,
	pub handle_type : ExternalSemaphoreHandleTypeFlags,
}

impl SemaphoreGetFdInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000079001,
			p_next : null(),
			semaphore : Semaphore(0),
			handle_type : ExternalSemaphoreHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn semaphore(mut self, val : Semaphore) -> Self {
		self.semaphore = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalSemaphoreHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for SemaphoreGetFdInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct D3D12FenceSubmitInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub wait_semaphore_values_count : u32,
	pub p_wait_semaphore_values : *const u64,
	pub signal_semaphore_values_count : u32,
	pub p_signal_semaphore_values : *const u64,
}

impl D3D12FenceSubmitInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000078002,
			p_next : null(),
			wait_semaphore_values_count : <_>::default(),
			p_wait_semaphore_values : null(),
			signal_semaphore_values_count : <_>::default(),
			p_signal_semaphore_values : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn wait_semaphore_values(mut self, val : &[u64]) -> Self {
		self.wait_semaphore_values_count = val.len() as _;
		self.p_wait_semaphore_values = val.as_ptr();
		self
	}
	pub fn wait_semaphore_value(mut self, val : &u64) -> Self {
		self.wait_semaphore_values_count = 1;
		self.p_wait_semaphore_values = val;
		self
	}
	pub fn signal_semaphore_values(mut self, val : &[u64]) -> Self {
		self.signal_semaphore_values_count = val.len() as _;
		self.p_signal_semaphore_values = val.as_ptr();
		self
	}
	pub fn signal_semaphore_value(mut self, val : &u64) -> Self {
		self.signal_semaphore_values_count = 1;
		self.p_signal_semaphore_values = val;
		self
	}
}

impl std::default::Default for D3D12FenceSubmitInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ClearRect {
	pub rect : Rect2D,
	pub base_array_layer : u32,
	pub layer_count : u32,
}

impl ClearRect {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn rect(mut self, val : Rect2D) -> Self {
		self.rect = val;
		self
	}
	pub fn base_array_layer(mut self, val : u32) -> Self {
		self.base_array_layer = val;
		self
	}
	pub fn layer_count(mut self, val : u32) -> Self {
		self.layer_count = val;
		self
	}
}

impl std::default::Default for ClearRect {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferImageCopy {
	pub buffer_offset : DeviceSize,
	pub buffer_row_length : u32,
	pub buffer_image_height : u32,
	pub image_subresource : ImageSubresourceLayers,
	pub image_offset : Offset3D,
	pub image_extent : Extent3D,
}

impl BufferImageCopy {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn buffer_offset(mut self, val : DeviceSize) -> Self {
		self.buffer_offset = val;
		self
	}
	pub fn buffer_row_length(mut self, val : u32) -> Self {
		self.buffer_row_length = val;
		self
	}
	pub fn buffer_image_height(mut self, val : u32) -> Self {
		self.buffer_image_height = val;
		self
	}
	pub fn image_subresource(mut self, val : ImageSubresourceLayers) -> Self {
		self.image_subresource = val;
		self
	}
	pub fn image_offset(mut self, val : Offset3D) -> Self {
		self.image_offset = val;
		self
	}
	pub fn image_extent(mut self, val : Extent3D) -> Self {
		self.image_extent = val;
		self
	}
}

impl std::default::Default for BufferImageCopy {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageMemoryRequirementsInfo2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub image : Image,
}

impl ImageMemoryRequirementsInfo2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000146001,
			p_next : null(),
			image : Image(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
}

impl std::default::Default for ImageMemoryRequirementsInfo2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub vulkan_memory_model : Bool32,
	pub vulkan_memory_model_device_scope : Bool32,
	pub vulkan_memory_model_availability_visibility_chains : Bool32,
}

impl PhysicalDeviceVulkanMemoryModelFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000211000,
			p_next : null_mut(),
			vulkan_memory_model : <_>::default(),
			vulkan_memory_model_device_scope : <_>::default(),
			vulkan_memory_model_availability_visibility_chains : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn vulkan_memory_model(mut self, val : Bool32) -> Self {
		self.vulkan_memory_model = val;
		self
	}
	pub fn vulkan_memory_model_device_scope(mut self, val : Bool32) -> Self {
		self.vulkan_memory_model_device_scope = val;
		self
	}
	pub fn vulkan_memory_model_availability_visibility_chains(mut self, val : Bool32) -> Self {
		self.vulkan_memory_model_availability_visibility_chains = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceVulkanMemoryModelFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceInlineUniformBlockPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_inline_uniform_block_size : u32,
	pub max_per_stage_descriptor_inline_uniform_blocks : u32,
	pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks : u32,
	pub max_descriptor_set_inline_uniform_blocks : u32,
	pub max_descriptor_set_update_after_bind_inline_uniform_blocks : u32,
}

impl PhysicalDeviceInlineUniformBlockPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000138001,
			p_next : null_mut(),
			max_inline_uniform_block_size : <_>::default(),
			max_per_stage_descriptor_inline_uniform_blocks : <_>::default(),
			max_per_stage_descriptor_update_after_bind_inline_uniform_blocks : <_>::default(),
			max_descriptor_set_inline_uniform_blocks : <_>::default(),
			max_descriptor_set_update_after_bind_inline_uniform_blocks : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_inline_uniform_block_size(mut self, val : u32) -> Self {
		self.max_inline_uniform_block_size = val;
		self
	}
	pub fn max_per_stage_descriptor_inline_uniform_blocks(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_inline_uniform_blocks = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks = val;
		self
	}
	pub fn max_descriptor_set_inline_uniform_blocks(mut self, val : u32) -> Self {
		self.max_descriptor_set_inline_uniform_blocks = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_inline_uniform_blocks = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceInlineUniformBlockPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindImageMemoryInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub image : Image,
	pub memory : DeviceMemory,
	pub memory_offset : DeviceSize,
}

impl BindImageMemoryInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000157001,
			p_next : null(),
			image : Image(0),
			memory : DeviceMemory(0),
			memory_offset : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
	pub fn memory_offset(mut self, val : DeviceSize) -> Self {
		self.memory_offset = val;
		self
	}
}

impl std::default::Default for BindImageMemoryInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct ClearAttachment {
	pub aspect_mask : ImageAspectFlags,
	pub color_attachment : u32,
	pub clear_value : ClearValue,
}

impl ClearAttachment {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn aspect_mask(mut self, val : ImageAspectFlags) -> Self {
		self.aspect_mask = val;
		self
	}
	pub fn color_attachment(mut self, val : u32) -> Self {
		self.color_attachment = val;
		self
	}
	pub fn clear_value(mut self, val : ClearValue) -> Self {
		self.clear_value = val;
		self
	}
}

impl std::default::Default for ClearAttachment {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CalibratedTimestampInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub time_domain : TimeDomainEXT,
}

impl CalibratedTimestampInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000184000,
			p_next : null(),
			time_domain : TimeDomainEXT::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn time_domain(mut self, val : TimeDomainEXT) -> Self {
		self.time_domain = val;
		self
	}
}

impl std::default::Default for CalibratedTimestampInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageSubresourceLayers {
	pub aspect_mask : ImageAspectFlags,
	pub mip_level : u32,
	pub base_array_layer : u32,
	pub layer_count : u32,
}

impl ImageSubresourceLayers {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn aspect_mask(mut self, val : ImageAspectFlags) -> Self {
		self.aspect_mask = val;
		self
	}
	pub fn mip_level(mut self, val : u32) -> Self {
		self.mip_level = val;
		self
	}
	pub fn base_array_layer(mut self, val : u32) -> Self {
		self.base_array_layer = val;
		self
	}
	pub fn layer_count(mut self, val : u32) -> Self {
		self.layer_count = val;
		self
	}
}

impl std::default::Default for ImageSubresourceLayers {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindIndexBufferIndirectCommandNV {
	pub buffer_address : DeviceAddress,
	pub size : u32,
	pub index_type : IndexType,
}

impl BindIndexBufferIndirectCommandNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn buffer_address(mut self, val : DeviceAddress) -> Self {
		self.buffer_address = val;
		self
	}
	pub fn size(mut self, val : u32) -> Self {
		self.size = val;
		self
	}
	pub fn index_type(mut self, val : IndexType) -> Self {
		self.index_type = val;
		self
	}
}

impl std::default::Default for BindIndexBufferIndirectCommandNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferCopy {
	pub src_offset : DeviceSize,
	pub dst_offset : DeviceSize,
	pub size : DeviceSize,
}

impl BufferCopy {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn src_offset(mut self, val : DeviceSize) -> Self {
		self.src_offset = val;
		self
	}
	pub fn dst_offset(mut self, val : DeviceSize) -> Self {
		self.dst_offset = val;
		self
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
}

impl std::default::Default for BufferCopy {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub binding_count : u32,
	pub p_binding_flags : *const DescriptorBindingFlags,
}

impl DescriptorSetLayoutBindingFlagsCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000161000,
			p_next : null(),
			binding_count : <_>::default(),
			p_binding_flags : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn binding_flags(mut self, val : &[DescriptorBindingFlags]) -> Self {
		self.binding_count = val.len() as _;
		self.p_binding_flags = val.as_ptr();
		self
	}
	pub fn binding_flag(mut self, val : &DescriptorBindingFlags) -> Self {
		self.binding_count = 1;
		self.p_binding_flags = val;
		self
	}
}

impl std::default::Default for DescriptorSetLayoutBindingFlagsCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryHostPointerPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub memory_type_bits : u32,
}

impl MemoryHostPointerPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000178001,
			p_next : null_mut(),
			memory_type_bits : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory_type_bits(mut self, val : u32) -> Self {
		self.memory_type_bits = val;
		self
	}
}

impl std::default::Default for MemoryHostPointerPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_push_descriptors : u32,
}

impl PhysicalDevicePushDescriptorPropertiesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000080000,
			p_next : null_mut(),
			max_push_descriptors : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_push_descriptors(mut self, val : u32) -> Self {
		self.max_push_descriptors = val;
		self
	}
}

impl std::default::Default for PhysicalDevicePushDescriptorPropertiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct HdrMetadataEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub display_primary_red : XYColorEXT,
	pub display_primary_green : XYColorEXT,
	pub display_primary_blue : XYColorEXT,
	pub white_point : XYColorEXT,
	pub max_luminance : f32,
	pub min_luminance : f32,
	pub max_content_light_level : f32,
	pub max_frame_average_light_level : f32,
}

impl HdrMetadataEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000105000,
			p_next : null(),
			display_primary_red : XYColorEXT::new(),
			display_primary_green : XYColorEXT::new(),
			display_primary_blue : XYColorEXT::new(),
			white_point : XYColorEXT::new(),
			max_luminance : <_>::default(),
			min_luminance : <_>::default(),
			max_content_light_level : <_>::default(),
			max_frame_average_light_level : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn display_primary_red(mut self, val : XYColorEXT) -> Self {
		self.display_primary_red = val;
		self
	}
	pub fn display_primary_green(mut self, val : XYColorEXT) -> Self {
		self.display_primary_green = val;
		self
	}
	pub fn display_primary_blue(mut self, val : XYColorEXT) -> Self {
		self.display_primary_blue = val;
		self
	}
	pub fn white_point(mut self, val : XYColorEXT) -> Self {
		self.white_point = val;
		self
	}
	pub fn max_luminance(mut self, val : f32) -> Self {
		self.max_luminance = val;
		self
	}
	pub fn min_luminance(mut self, val : f32) -> Self {
		self.min_luminance = val;
		self
	}
	pub fn max_content_light_level(mut self, val : f32) -> Self {
		self.max_content_light_level = val;
		self
	}
	pub fn max_frame_average_light_level(mut self, val : f32) -> Self {
		self.max_frame_average_light_level = val;
		self
	}
}

impl std::default::Default for HdrMetadataEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ViewportSwizzleNV {
	pub x : ViewportCoordinateSwizzleNV,
	pub y : ViewportCoordinateSwizzleNV,
	pub z : ViewportCoordinateSwizzleNV,
	pub w : ViewportCoordinateSwizzleNV,
}

impl ViewportSwizzleNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn x(mut self, val : ViewportCoordinateSwizzleNV) -> Self {
		self.x = val;
		self
	}
	pub fn y(mut self, val : ViewportCoordinateSwizzleNV) -> Self {
		self.y = val;
		self
	}
	pub fn z(mut self, val : ViewportCoordinateSwizzleNV) -> Self {
		self.z = val;
		self
	}
	pub fn w(mut self, val : ViewportCoordinateSwizzleNV) -> Self {
		self.w = val;
		self
	}
}

impl std::default::Default for ViewportSwizzleNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AttachmentDescription {
	pub flags : AttachmentDescriptionFlags,
	pub format : Format,
	pub samples : SampleCountFlags,
	pub load_op : AttachmentLoadOp,
	pub store_op : AttachmentStoreOp,
	pub stencil_load_op : AttachmentLoadOp,
	pub stencil_store_op : AttachmentStoreOp,
	pub initial_layout : ImageLayout,
	pub final_layout : ImageLayout,
}

impl AttachmentDescription {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn flags(mut self, val : AttachmentDescriptionFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn samples(mut self, val : SampleCountFlags) -> Self {
		self.samples = val;
		self
	}
	pub fn load_op(mut self, val : AttachmentLoadOp) -> Self {
		self.load_op = val;
		self
	}
	pub fn store_op(mut self, val : AttachmentStoreOp) -> Self {
		self.store_op = val;
		self
	}
	pub fn stencil_load_op(mut self, val : AttachmentLoadOp) -> Self {
		self.stencil_load_op = val;
		self
	}
	pub fn stencil_store_op(mut self, val : AttachmentStoreOp) -> Self {
		self.stencil_store_op = val;
		self
	}
	pub fn initial_layout(mut self, val : ImageLayout) -> Self {
		self.initial_layout = val;
		self
	}
	pub fn final_layout(mut self, val : ImageLayout) -> Self {
		self.final_layout = val;
		self
	}
}

impl std::default::Default for AttachmentDescription {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineRasterizationConservativeStateCreateFlagsEXT,
	pub conservative_rasterization_mode : ConservativeRasterizationModeEXT,
	pub extra_primitive_overestimation_size : f32,
}

impl PipelineRasterizationConservativeStateCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000101001,
			p_next : null(),
			flags : <_>::default(),
			conservative_rasterization_mode : ConservativeRasterizationModeEXT::default(),
			extra_primitive_overestimation_size : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineRasterizationConservativeStateCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn conservative_rasterization_mode(mut self, val : ConservativeRasterizationModeEXT) -> Self {
		self.conservative_rasterization_mode = val;
		self
	}
	pub fn extra_primitive_overestimation_size(mut self, val : f32) -> Self {
		self.extra_primitive_overestimation_size = val;
		self
	}
}

impl std::default::Default for PipelineRasterizationConservativeStateCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CommandBufferAllocateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub command_pool : CommandPool,
	pub level : CommandBufferLevel,
	pub command_buffer_count : u32,
}

impl CommandBufferAllocateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 40,
			p_next : null(),
			command_pool : CommandPool(0),
			level : CommandBufferLevel::default(),
			command_buffer_count : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn command_pool(mut self, val : CommandPool) -> Self {
		self.command_pool = val;
		self
	}
	pub fn level(mut self, val : CommandBufferLevel) -> Self {
		self.level = val;
		self
	}
	pub fn command_buffer_count(mut self, val : u32) -> Self {
		self.command_buffer_count = val;
		self
	}
}

impl std::default::Default for CommandBufferAllocateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubpassDependency {
	pub src_subpass : u32,
	pub dst_subpass : u32,
	pub src_stage_mask : PipelineStageFlags,
	pub dst_stage_mask : PipelineStageFlags,
	pub src_access_mask : AccessFlags,
	pub dst_access_mask : AccessFlags,
	pub dependency_flags : DependencyFlags,
}

impl SubpassDependency {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn src_subpass(mut self, val : u32) -> Self {
		self.src_subpass = val;
		self
	}
	pub fn dst_subpass(mut self, val : u32) -> Self {
		self.dst_subpass = val;
		self
	}
	pub fn src_stage_mask(mut self, val : PipelineStageFlags) -> Self {
		self.src_stage_mask = val;
		self
	}
	pub fn dst_stage_mask(mut self, val : PipelineStageFlags) -> Self {
		self.dst_stage_mask = val;
		self
	}
	pub fn src_access_mask(mut self, val : AccessFlags) -> Self {
		self.src_access_mask = val;
		self
	}
	pub fn dst_access_mask(mut self, val : AccessFlags) -> Self {
		self.dst_access_mask = val;
		self
	}
	pub fn dependency_flags(mut self, val : DependencyFlags) -> Self {
		self.dependency_flags = val;
		self
	}
}

impl std::default::Default for SubpassDependency {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorSetLayoutCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DescriptorSetLayoutCreateFlags,
	pub binding_count : u32,
	pub p_bindings : *const DescriptorSetLayoutBinding,
}

impl DescriptorSetLayoutCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 32,
			p_next : null(),
			flags : <_>::default(),
			binding_count : <_>::default(),
			p_bindings : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DescriptorSetLayoutCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn bindings(mut self, val : &[DescriptorSetLayoutBinding]) -> Self {
		self.binding_count = val.len() as _;
		self.p_bindings = val.as_ptr();
		self
	}
	pub fn binding(mut self, val : &DescriptorSetLayoutBinding) -> Self {
		self.binding_count = 1;
		self.p_bindings = val;
		self
	}
}

impl std::default::Default for DescriptorSetLayoutCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineCoverageModulationStateCreateFlagsNV,
	pub coverage_modulation_mode : CoverageModulationModeNV,
	pub coverage_modulation_table_enable : Bool32,
	pub coverage_modulation_table_count : u32,
	pub p_coverage_modulation_table : *const f32,
}

impl PipelineCoverageModulationStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000152000,
			p_next : null(),
			flags : <_>::default(),
			coverage_modulation_mode : CoverageModulationModeNV::default(),
			coverage_modulation_table_enable : <_>::default(),
			coverage_modulation_table_count : <_>::default(),
			p_coverage_modulation_table : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineCoverageModulationStateCreateFlagsNV) -> Self {
		self.flags = val;
		self
	}
	pub fn coverage_modulation_mode(mut self, val : CoverageModulationModeNV) -> Self {
		self.coverage_modulation_mode = val;
		self
	}
	pub fn coverage_modulation_table_enable(mut self, val : Bool32) -> Self {
		self.coverage_modulation_table_enable = val;
		self
	}
	pub fn coverage_modulation_table(mut self, val : &[f32]) -> Self {
		self.coverage_modulation_table_count = val.len() as _;
		self.p_coverage_modulation_table = val.as_ptr();
		self
	}
	pub fn coverage_modulation_tabl(mut self, val : &f32) -> Self {
		self.coverage_modulation_table_count = 1;
		self.p_coverage_modulation_table = val;
		self
	}
}

impl std::default::Default for PipelineCoverageModulationStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorSetLayoutBinding {
	pub binding : u32,
	pub descriptor_type : DescriptorType,
	pub descriptor_count : u32,
	pub stage_flags : ShaderStageFlags,
	pub p_immutable_samplers : *const Sampler,
}

impl DescriptorSetLayoutBinding {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn binding(mut self, val : u32) -> Self {
		self.binding = val;
		self
	}
	pub fn descriptor_type(mut self, val : DescriptorType) -> Self {
		self.descriptor_type = val;
		self
	}
	pub fn descriptor_count(mut self, val : u32) -> Self {
		self.descriptor_count = val;
		self
	}
	pub fn stage_flags(mut self, val : ShaderStageFlags) -> Self {
		self.stage_flags = val;
		self
	}
	pub fn immutable_samplers(mut self, val : *const Sampler) -> Self {
		self.p_immutable_samplers = val;
		self
	}
}

impl std::default::Default for DescriptorSetLayoutBinding {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ViewportWScalingNV {
	pub xcoeff : f32,
	pub ycoeff : f32,
}

impl ViewportWScalingNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn xcoeff(mut self, val : f32) -> Self {
		self.xcoeff = val;
		self
	}
	pub fn ycoeff(mut self, val : f32) -> Self {
		self.ycoeff = val;
		self
	}
}

impl std::default::Default for ViewportWScalingNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubpassDependency2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub src_subpass : u32,
	pub dst_subpass : u32,
	pub src_stage_mask : PipelineStageFlags,
	pub dst_stage_mask : PipelineStageFlags,
	pub src_access_mask : AccessFlags,
	pub dst_access_mask : AccessFlags,
	pub dependency_flags : DependencyFlags,
	pub view_offset : i32,
}

impl SubpassDependency2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000109003,
			p_next : null(),
			src_subpass : <_>::default(),
			dst_subpass : <_>::default(),
			src_stage_mask : <_>::default(),
			dst_stage_mask : <_>::default(),
			src_access_mask : <_>::default(),
			dst_access_mask : <_>::default(),
			dependency_flags : <_>::default(),
			view_offset : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn src_subpass(mut self, val : u32) -> Self {
		self.src_subpass = val;
		self
	}
	pub fn dst_subpass(mut self, val : u32) -> Self {
		self.dst_subpass = val;
		self
	}
	pub fn src_stage_mask(mut self, val : PipelineStageFlags) -> Self {
		self.src_stage_mask = val;
		self
	}
	pub fn dst_stage_mask(mut self, val : PipelineStageFlags) -> Self {
		self.dst_stage_mask = val;
		self
	}
	pub fn src_access_mask(mut self, val : AccessFlags) -> Self {
		self.src_access_mask = val;
		self
	}
	pub fn dst_access_mask(mut self, val : AccessFlags) -> Self {
		self.dst_access_mask = val;
		self
	}
	pub fn dependency_flags(mut self, val : DependencyFlags) -> Self {
		self.dependency_flags = val;
		self
	}
	pub fn view_offset(mut self, val : i32) -> Self {
		self.view_offset = val;
		self
	}
}

impl std::default::Default for SubpassDependency2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceProtectedCapabilitiesKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub supports_protected : Bool32,
}

impl SurfaceProtectedCapabilitiesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000239000,
			p_next : null(),
			supports_protected : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn supports_protected(mut self, val : Bool32) -> Self {
		self.supports_protected = val;
		self
	}
}

impl std::default::Default for SurfaceProtectedCapabilitiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindImageMemoryDeviceGroupInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub device_index_count : u32,
	pub p_device_indices : *const u32,
	pub split_instance_bind_region_count : u32,
	pub p_split_instance_bind_regions : *const Rect2D,
}

impl BindImageMemoryDeviceGroupInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000060014,
			p_next : null(),
			device_index_count : <_>::default(),
			p_device_indices : null(),
			split_instance_bind_region_count : <_>::default(),
			p_split_instance_bind_regions : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_indices(mut self, val : &[u32]) -> Self {
		self.device_index_count = val.len() as _;
		self.p_device_indices = val.as_ptr();
		self
	}
	pub fn device_indice(mut self, val : &u32) -> Self {
		self.device_index_count = 1;
		self.p_device_indices = val;
		self
	}
	pub fn split_instance_bind_regions(mut self, val : &[Rect2D]) -> Self {
		self.split_instance_bind_region_count = val.len() as _;
		self.p_split_instance_bind_regions = val.as_ptr();
		self
	}
	pub fn split_instance_bind_region(mut self, val : &Rect2D) -> Self {
		self.split_instance_bind_region_count = 1;
		self.p_split_instance_bind_regions = val;
		self
	}
}

impl std::default::Default for BindImageMemoryDeviceGroupInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CoarseSampleLocationNV {
	pub pixel_x : u32,
	pub pixel_y : u32,
	pub sample : u32,
}

impl CoarseSampleLocationNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn pixel_x(mut self, val : u32) -> Self {
		self.pixel_x = val;
		self
	}
	pub fn pixel_y(mut self, val : u32) -> Self {
		self.pixel_y = val;
		self
	}
	pub fn sample(mut self, val : u32) -> Self {
		self.sample = val;
		self
	}
}

impl std::default::Default for CoarseSampleLocationNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorBufferInfo {
	pub buffer : Buffer,
	pub offset : DeviceSize,
	pub range : DeviceSize,
}

impl DescriptorBufferInfo {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
	pub fn offset(mut self, val : DeviceSize) -> Self {
		self.offset = val;
		self
	}
	pub fn range(mut self, val : DeviceSize) -> Self {
		self.range = val;
		self
	}
}

impl std::default::Default for DescriptorBufferInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceGroupCommandBufferBeginInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub device_mask : u32,
}

impl DeviceGroupCommandBufferBeginInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000060004,
			p_next : null(),
			device_mask : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_mask(mut self, val : u32) -> Self {
		self.device_mask = val;
		self
	}
}

impl std::default::Default for DeviceGroupCommandBufferBeginInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub pipeline : Pipeline,
}

impl PipelineInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000269001,
			p_next : null(),
			pipeline : Pipeline(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn pipeline(mut self, val : Pipeline) -> Self {
		self.pipeline = val;
		self
	}
}

impl std::default::Default for PipelineInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubpassDescription2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : SubpassDescriptionFlags,
	pub pipeline_bind_point : PipelineBindPoint,
	pub view_mask : u32,
	pub input_attachment_count : u32,
	pub p_input_attachments : *const AttachmentReference2,
	pub color_attachment_count : u32,
	pub p_color_attachments : *const AttachmentReference2,
	pub p_resolve_attachments : *const AttachmentReference2,
	pub p_depth_stencil_attachment : *const AttachmentReference2,
	pub preserve_attachment_count : u32,
	pub p_preserve_attachments : *const u32,
}

impl SubpassDescription2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000109002,
			p_next : null(),
			flags : <_>::default(),
			pipeline_bind_point : PipelineBindPoint::default(),
			view_mask : <_>::default(),
			input_attachment_count : <_>::default(),
			p_input_attachments : null(),
			color_attachment_count : <_>::default(),
			p_color_attachments : null(),
			p_resolve_attachments : null(),
			p_depth_stencil_attachment : null(),
			preserve_attachment_count : <_>::default(),
			p_preserve_attachments : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : SubpassDescriptionFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn pipeline_bind_point(mut self, val : PipelineBindPoint) -> Self {
		self.pipeline_bind_point = val;
		self
	}
	pub fn view_mask(mut self, val : u32) -> Self {
		self.view_mask = val;
		self
	}
	pub fn resolve_attachments(mut self, val : *const AttachmentReference2) -> Self {
		self.p_resolve_attachments = val;
		self
	}
	pub fn depth_stencil_attachment(mut self, val : *const AttachmentReference2) -> Self {
		self.p_depth_stencil_attachment = val;
		self
	}
	pub fn input_attachments(mut self, val : &[AttachmentReference2]) -> Self {
		self.input_attachment_count = val.len() as _;
		self.p_input_attachments = val.as_ptr();
		self
	}
	pub fn input_attachment(mut self, val : &AttachmentReference2) -> Self {
		self.input_attachment_count = 1;
		self.p_input_attachments = val;
		self
	}
	pub fn color_attachments(mut self, val : &[AttachmentReference2]) -> Self {
		self.color_attachment_count = val.len() as _;
		self.p_color_attachments = val.as_ptr();
		self
	}
	pub fn color_attachment(mut self, val : &AttachmentReference2) -> Self {
		self.color_attachment_count = 1;
		self.p_color_attachments = val;
		self
	}
	pub fn preserve_attachments(mut self, val : &[u32]) -> Self {
		self.preserve_attachment_count = val.len() as _;
		self.p_preserve_attachments = val.as_ptr();
		self
	}
	pub fn preserve_attachment(mut self, val : &u32) -> Self {
		self.preserve_attachment_count = 1;
		self.p_preserve_attachments = val;
		self
	}
}

impl std::default::Default for SubpassDescription2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub filter_minmax_single_component_formats : Bool32,
	pub filter_minmax_image_component_mapping : Bool32,
}

impl PhysicalDeviceSamplerFilterMinmaxProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000130000,
			p_next : null_mut(),
			filter_minmax_single_component_formats : <_>::default(),
			filter_minmax_image_component_mapping : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn filter_minmax_single_component_formats(mut self, val : Bool32) -> Self {
		self.filter_minmax_single_component_formats = val;
		self
	}
	pub fn filter_minmax_image_component_mapping(mut self, val : Bool32) -> Self {
		self.filter_minmax_image_component_mapping = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSamplerFilterMinmaxProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceGroupDeviceCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub physical_device_count : u32,
	pub p_physical_devices : *const PhysicalDevice,
}

impl DeviceGroupDeviceCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000070001,
			p_next : null(),
			physical_device_count : <_>::default(),
			p_physical_devices : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn physical_devices(mut self, val : &[PhysicalDevice]) -> Self {
		self.physical_device_count = val.len() as _;
		self.p_physical_devices = val.as_ptr();
		self
	}
	pub fn physical_device(mut self, val : &PhysicalDevice) -> Self {
		self.physical_device_count = 1;
		self.p_physical_devices = val;
		self
	}
}

impl std::default::Default for DeviceGroupDeviceCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CopyDescriptorSet {
	s_type : i32,
	pub p_next : *const c_void,
	pub src_set : DescriptorSet,
	pub src_binding : u32,
	pub src_array_element : u32,
	pub dst_set : DescriptorSet,
	pub dst_binding : u32,
	pub dst_array_element : u32,
	pub descriptor_count : u32,
}

impl CopyDescriptorSet {
	pub fn new() -> Self {
		Self {
			s_type : 36,
			p_next : null(),
			src_set : DescriptorSet(0),
			src_binding : <_>::default(),
			src_array_element : <_>::default(),
			dst_set : DescriptorSet(0),
			dst_binding : <_>::default(),
			dst_array_element : <_>::default(),
			descriptor_count : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn src_set(mut self, val : DescriptorSet) -> Self {
		self.src_set = val;
		self
	}
	pub fn src_binding(mut self, val : u32) -> Self {
		self.src_binding = val;
		self
	}
	pub fn src_array_element(mut self, val : u32) -> Self {
		self.src_array_element = val;
		self
	}
	pub fn dst_set(mut self, val : DescriptorSet) -> Self {
		self.dst_set = val;
		self
	}
	pub fn dst_binding(mut self, val : u32) -> Self {
		self.dst_binding = val;
		self
	}
	pub fn dst_array_element(mut self, val : u32) -> Self {
		self.dst_array_element = val;
		self
	}
	pub fn descriptor_count(mut self, val : u32) -> Self {
		self.descriptor_count = val;
		self
	}
}

impl std::default::Default for CopyDescriptorSet {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct DeviceGroupPresentCapabilitiesKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub present_mask : [u32; VK_MAX_DEVICE_GROUP_SIZE],
	pub modes : DeviceGroupPresentModeFlagsKHR,
}

impl DeviceGroupPresentCapabilitiesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000060007,
			p_next : null(),
			present_mask : [0 as _ ;VK_MAX_DEVICE_GROUP_SIZE],
			modes : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn present_mask(mut self, val : [u32; VK_MAX_DEVICE_GROUP_SIZE]) -> Self {
		self.present_mask = val;
		self
	}
	pub fn modes(mut self, val : DeviceGroupPresentModeFlagsKHR) -> Self {
		self.modes = val;
		self
	}
}

impl std::default::Default for DeviceGroupPresentCapabilitiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryFdPropertiesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub memory_type_bits : u32,
}

impl MemoryFdPropertiesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000074001,
			p_next : null_mut(),
			memory_type_bits : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory_type_bits(mut self, val : u32) -> Self {
		self.memory_type_bits = val;
		self
	}
}

impl std::default::Default for MemoryFdPropertiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PushConstantRange {
	pub stage_flags : ShaderStageFlags,
	pub offset : u32,
	pub size : u32,
}

impl PushConstantRange {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn stage_flags(mut self, val : ShaderStageFlags) -> Self {
		self.stage_flags = val;
		self
	}
	pub fn offset(mut self, val : u32) -> Self {
		self.offset = val;
		self
	}
	pub fn size(mut self, val : u32) -> Self {
		self.size = val;
		self
	}
}

impl std::default::Default for PushConstantRange {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct EventCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : EventCreateFlags,
}

impl EventCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 10,
			p_next : null(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : EventCreateFlags) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for EventCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CommandBufferInheritanceInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub render_pass : RenderPass,
	pub subpass : u32,
	pub framebuffer : Framebuffer,
	pub occlusion_query_enable : Bool32,
	pub query_flags : QueryControlFlags,
	pub pipeline_statistics : QueryPipelineStatisticFlags,
}

impl CommandBufferInheritanceInfo {
	pub fn new() -> Self {
		Self {
			s_type : 41,
			p_next : null(),
			render_pass : RenderPass(0),
			subpass : <_>::default(),
			framebuffer : Framebuffer(0),
			occlusion_query_enable : <_>::default(),
			query_flags : <_>::default(),
			pipeline_statistics : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn render_pass(mut self, val : RenderPass) -> Self {
		self.render_pass = val;
		self
	}
	pub fn subpass(mut self, val : u32) -> Self {
		self.subpass = val;
		self
	}
	pub fn framebuffer(mut self, val : Framebuffer) -> Self {
		self.framebuffer = val;
		self
	}
	pub fn occlusion_query_enable(mut self, val : Bool32) -> Self {
		self.occlusion_query_enable = val;
		self
	}
	pub fn query_flags(mut self, val : QueryControlFlags) -> Self {
		self.query_flags = val;
		self
	}
	pub fn pipeline_statistics(mut self, val : QueryPipelineStatisticFlags) -> Self {
		self.pipeline_statistics = val;
		self
	}
}

impl std::default::Default for CommandBufferInheritanceInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub global_priority : QueueGlobalPriorityEXT,
}

impl DeviceQueueGlobalPriorityCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000174000,
			p_next : null(),
			global_priority : QueueGlobalPriorityEXT::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn global_priority(mut self, val : QueueGlobalPriorityEXT) -> Self {
		self.global_priority = val;
		self
	}
}

impl std::default::Default for DeviceQueueGlobalPriorityCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DebugUtilsMessengerCallbackDataEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DebugUtilsMessengerCallbackDataFlagsEXT,
	pub p_message_id_name : *const u8,
	pub message_id_number : i32,
	pub p_message : *const u8,
	pub queue_label_count : u32,
	pub p_queue_labels : *const DebugUtilsLabelEXT,
	pub cmd_buf_label_count : u32,
	pub p_cmd_buf_labels : *const DebugUtilsLabelEXT,
	pub object_count : u32,
	pub p_objects : *const DebugUtilsObjectNameInfoEXT,
}

impl DebugUtilsMessengerCallbackDataEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000128003,
			p_next : null(),
			flags : <_>::default(),
			p_message_id_name : null(),
			message_id_number : <_>::default(),
			p_message : null(),
			queue_label_count : <_>::default(),
			p_queue_labels : null(),
			cmd_buf_label_count : <_>::default(),
			p_cmd_buf_labels : null(),
			object_count : <_>::default(),
			p_objects : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DebugUtilsMessengerCallbackDataFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn message_id_name(mut self, val : *const u8) -> Self {
		self.p_message_id_name = val;
		self
	}
	pub fn message_id_number(mut self, val : i32) -> Self {
		self.message_id_number = val;
		self
	}
	pub fn message(mut self, val : *const u8) -> Self {
		self.p_message = val;
		self
	}
	pub fn queue_labels(mut self, val : &[DebugUtilsLabelEXT]) -> Self {
		self.queue_label_count = val.len() as _;
		self.p_queue_labels = val.as_ptr();
		self
	}
	pub fn queue_label(mut self, val : &DebugUtilsLabelEXT) -> Self {
		self.queue_label_count = 1;
		self.p_queue_labels = val;
		self
	}
	pub fn cmd_buf_labels(mut self, val : &[DebugUtilsLabelEXT]) -> Self {
		self.cmd_buf_label_count = val.len() as _;
		self.p_cmd_buf_labels = val.as_ptr();
		self
	}
	pub fn cmd_buf_label(mut self, val : &DebugUtilsLabelEXT) -> Self {
		self.cmd_buf_label_count = 1;
		self.p_cmd_buf_labels = val;
		self
	}
	pub fn objects(mut self, val : &[DebugUtilsObjectNameInfoEXT]) -> Self {
		self.object_count = val.len() as _;
		self.p_objects = val.as_ptr();
		self
	}
	pub fn object(mut self, val : &DebugUtilsObjectNameInfoEXT) -> Self {
		self.object_count = 1;
		self.p_objects = val;
		self
	}
}

impl std::default::Default for DebugUtilsMessengerCallbackDataEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PipelineColorBlendStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineColorBlendStateCreateFlags,
	pub logic_op_enable : Bool32,
	pub logic_op : LogicOp,
	pub attachment_count : u32,
	pub p_attachments : *const PipelineColorBlendAttachmentState,
	pub blend_constants : [f32; 4],
}

impl PipelineColorBlendStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 26,
			p_next : null(),
			flags : <_>::default(),
			logic_op_enable : <_>::default(),
			logic_op : LogicOp::default(),
			attachment_count : <_>::default(),
			p_attachments : null(),
			blend_constants : [0 as _ ;4],
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineColorBlendStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn logic_op_enable(mut self, val : Bool32) -> Self {
		self.logic_op_enable = val;
		self
	}
	pub fn logic_op(mut self, val : LogicOp) -> Self {
		self.logic_op = val;
		self
	}
	pub fn attachments(mut self, val : &[PipelineColorBlendAttachmentState]) -> Self {
		self.attachment_count = val.len() as _;
		self.p_attachments = val.as_ptr();
		self
	}
	pub fn attachment(mut self, val : &PipelineColorBlendAttachmentState) -> Self {
		self.attachment_count = 1;
		self.p_attachments = val;
		self
	}
}

impl std::default::Default for PipelineColorBlendStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindBufferMemoryInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub buffer : Buffer,
	pub memory : DeviceMemory,
	pub memory_offset : DeviceSize,
}

impl BindBufferMemoryInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000157000,
			p_next : null(),
			buffer : Buffer(0),
			memory : DeviceMemory(0),
			memory_offset : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
	pub fn memory_offset(mut self, val : DeviceSize) -> Self {
		self.memory_offset = val;
		self
	}
}

impl std::default::Default for BindBufferMemoryInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Offset2D {
	pub x : i32,
	pub y : i32,
}

impl Offset2D {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn x(mut self, val : i32) -> Self {
		self.x = val;
		self
	}
	pub fn y(mut self, val : i32) -> Self {
		self.y = val;
		self
	}
}

impl std::default::Default for Offset2D {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageSubresourceRange {
	pub aspect_mask : ImageAspectFlags,
	pub base_mip_level : u32,
	pub level_count : u32,
	pub base_array_layer : u32,
	pub layer_count : u32,
}

impl ImageSubresourceRange {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn aspect_mask(mut self, val : ImageAspectFlags) -> Self {
		self.aspect_mask = val;
		self
	}
	pub fn base_mip_level(mut self, val : u32) -> Self {
		self.base_mip_level = val;
		self
	}
	pub fn level_count(mut self, val : u32) -> Self {
		self.level_count = val;
		self
	}
	pub fn base_array_layer(mut self, val : u32) -> Self {
		self.base_array_layer = val;
		self
	}
	pub fn layer_count(mut self, val : u32) -> Self {
		self.layer_count = val;
		self
	}
}

impl std::default::Default for ImageSubresourceRange {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SamplerReductionModeCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub reduction_mode : SamplerReductionMode,
}

impl SamplerReductionModeCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000130001,
			p_next : null(),
			reduction_mode : SamplerReductionMode::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn reduction_mode(mut self, val : SamplerReductionMode) -> Self {
		self.reduction_mode = val;
		self
	}
}

impl std::default::Default for SamplerReductionModeCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearValue {
	pub color : ClearColorValue,
	pub depth_stencil : ClearDepthStencilValue,
}

impl ClearValue {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn color(mut self, val : ClearColorValue) -> Self {
		self.color = val;
		self
	}
	pub fn depth_stencil(mut self, val : ClearDepthStencilValue) -> Self {
		self.depth_stencil = val;
		self
	}
}

impl std::default::Default for ClearValue {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub acquire_count : u32,
	pub p_acquire_syncs : *const DeviceMemory,
	pub p_acquire_keys : *const u64,
	pub p_acquire_timeout_milliseconds : *const u32,
	pub release_count : u32,
	pub p_release_syncs : *const DeviceMemory,
	pub p_release_keys : *const u64,
}

impl Win32KeyedMutexAcquireReleaseInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000058000,
			p_next : null(),
			acquire_count : <_>::default(),
			p_acquire_syncs : null(),
			p_acquire_keys : null(),
			p_acquire_timeout_milliseconds : null(),
			release_count : <_>::default(),
			p_release_syncs : null(),
			p_release_keys : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn acquire_keys(mut self, val : *const u64) -> Self {
		self.p_acquire_keys = val;
		self
	}
	pub fn acquire_timeout_milliseconds(mut self, val : *const u32) -> Self {
		self.p_acquire_timeout_milliseconds = val;
		self
	}
	pub fn acquire_syncs(mut self, val : &[DeviceMemory]) -> Self {
		self.acquire_count = val.len() as _;
		self.p_acquire_syncs = val.as_ptr();
		self
	}
	pub fn acquire_sync(mut self, val : &DeviceMemory) -> Self {
		self.acquire_count = 1;
		self.p_acquire_syncs = val;
		self
	}
	pub fn release_syncs(mut self, val : &[DeviceMemory]) -> Self {
		self.release_count = val.len() as _;
		self.p_release_syncs = val.as_ptr();
		self
	}
	pub fn release_sync(mut self, val : &DeviceMemory) -> Self {
		self.release_count = 1;
		self.p_release_syncs = val;
		self
	}
}

impl std::default::Default for Win32KeyedMutexAcquireReleaseInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryType {
	pub property_flags : MemoryPropertyFlags,
	pub heap_index : u32,
}

impl MemoryType {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn property_flags(mut self, val : MemoryPropertyFlags) -> Self {
		self.property_flags = val;
		self
	}
	pub fn heap_index(mut self, val : u32) -> Self {
		self.heap_index = val;
		self
	}
}

impl std::default::Default for MemoryType {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceMaintenance3Properties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_per_set_descriptors : u32,
	pub max_memory_allocation_size : DeviceSize,
}

impl PhysicalDeviceMaintenance3Properties {
	pub fn new() -> Self {
		Self {
			s_type : 1000168000,
			p_next : null_mut(),
			max_per_set_descriptors : <_>::default(),
			max_memory_allocation_size : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_per_set_descriptors(mut self, val : u32) -> Self {
		self.max_per_set_descriptors = val;
		self
	}
	pub fn max_memory_allocation_size(mut self, val : DeviceSize) -> Self {
		self.max_memory_allocation_size = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMaintenance3Properties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
	s_type : i32,
	pub p_next : *mut c_void,
	pub local_dimming_support : Bool32,
}

impl DisplayNativeHdrSurfaceCapabilitiesAMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000213000,
			p_next : null_mut(),
			local_dimming_support : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn local_dimming_support(mut self, val : Bool32) -> Self {
		self.local_dimming_support = val;
		self
	}
}

impl std::default::Default for DisplayNativeHdrSurfaceCapabilitiesAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineVertexInputStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineVertexInputStateCreateFlags,
	pub vertex_binding_description_count : u32,
	pub p_vertex_binding_descriptions : *const VertexInputBindingDescription,
	pub vertex_attribute_description_count : u32,
	pub p_vertex_attribute_descriptions : *const VertexInputAttributeDescription,
}

impl PipelineVertexInputStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 19,
			p_next : null(),
			flags : <_>::default(),
			vertex_binding_description_count : <_>::default(),
			p_vertex_binding_descriptions : null(),
			vertex_attribute_description_count : <_>::default(),
			p_vertex_attribute_descriptions : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineVertexInputStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn vertex_binding_descriptions(mut self, val : &[VertexInputBindingDescription]) -> Self {
		self.vertex_binding_description_count = val.len() as _;
		self.p_vertex_binding_descriptions = val.as_ptr();
		self
	}
	pub fn vertex_binding_description(mut self, val : &VertexInputBindingDescription) -> Self {
		self.vertex_binding_description_count = 1;
		self.p_vertex_binding_descriptions = val;
		self
	}
	pub fn vertex_attribute_descriptions(mut self, val : &[VertexInputAttributeDescription]) -> Self {
		self.vertex_attribute_description_count = val.len() as _;
		self.p_vertex_attribute_descriptions = val.as_ptr();
		self
	}
	pub fn vertex_attribute_description(mut self, val : &VertexInputAttributeDescription) -> Self {
		self.vertex_attribute_description_count = 1;
		self.p_vertex_attribute_descriptions = val;
		self
	}
}

impl std::default::Default for PipelineVertexInputStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub texture_compression_astc__hdr : Bool32,
}

impl PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000066000,
			p_next : null_mut(),
			texture_compression_astc__hdr : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn texture_compression_astc__hdr(mut self, val : Bool32) -> Self {
		self.texture_compression_astc__hdr = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DeviceCreateFlags,
	pub queue_create_info_count : u32,
	pub p_queue_create_infos : *const DeviceQueueCreateInfo,
	pub enabled_layer_count : u32,
	pub pp_enabled_layer_names : *const *const u8,
	pub enabled_extension_count : u32,
	pub pp_enabled_extension_names : *const *const u8,
	pub p_enabled_features : *const PhysicalDeviceFeatures,
}

impl DeviceCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 3,
			p_next : null(),
			flags : <_>::default(),
			queue_create_info_count : <_>::default(),
			p_queue_create_infos : null(),
			enabled_layer_count : <_>::default(),
			pp_enabled_layer_names : null(),
			enabled_extension_count : <_>::default(),
			pp_enabled_extension_names : null(),
			p_enabled_features : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DeviceCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn queue_create_infos(mut self, val : &[DeviceQueueCreateInfo]) -> Self {
		self.queue_create_info_count = val.len() as _;
		self.p_queue_create_infos = val.as_ptr();
		self
	}
	pub fn queue_create_info(mut self, val : &DeviceQueueCreateInfo) -> Self {
		self.queue_create_info_count = 1;
		self.p_queue_create_infos = val;
		self
	}
	pub fn enabled_layer_names(mut self, val : &[*const u8]) -> Self {
		self.enabled_layer_count = val.len() as _;
		self.pp_enabled_layer_names = val.as_ptr();
		self
	}
	pub fn enabled_layer_name(mut self, val : &*const u8) -> Self {
		self.enabled_layer_count = 1;
		self.pp_enabled_layer_names = val;
		self
	}
	pub fn enabled_extension_names(mut self, val : &[*const u8]) -> Self {
		self.enabled_extension_count = val.len() as _;
		self.pp_enabled_extension_names = val.as_ptr();
		self
	}
	pub fn enabled_extension_name(mut self, val : &*const u8) -> Self {
		self.enabled_extension_count = 1;
		self.pp_enabled_extension_names = val;
		self
	}
}

impl std::default::Default for DeviceCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AccelerationStructureCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub compacted_size : DeviceSize,
	pub info : AccelerationStructureInfoNV,
}

impl AccelerationStructureCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165001,
			p_next : null(),
			compacted_size : <_>::default(),
			info : AccelerationStructureInfoNV::new(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn compacted_size(mut self, val : DeviceSize) -> Self {
		self.compacted_size = val;
		self
	}
	pub fn info(mut self, val : AccelerationStructureInfoNV) -> Self {
		self.info = val;
		self
	}
}

impl std::default::Default for AccelerationStructureCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct LayerProperties {
	pub layer_name : [u8; VK_MAX_EXTENSION_NAME_SIZE],
	pub spec_version : u32,
	pub implementation_version : u32,
	pub description : [u8; VK_MAX_DESCRIPTION_SIZE],
}

impl LayerProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn layer_name(mut self, val : [u8; VK_MAX_EXTENSION_NAME_SIZE]) -> Self {
		self.layer_name = val;
		self
	}
	pub fn spec_version(mut self, val : u32) -> Self {
		self.spec_version = val;
		self
	}
	pub fn implementation_version(mut self, val : u32) -> Self {
		self.implementation_version = val;
		self
	}
	pub fn description(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.description = val;
		self
	}
}

impl std::default::Default for LayerProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FramebufferAttachmentImageInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : ImageCreateFlags,
	pub usage : ImageUsageFlags,
	pub width : u32,
	pub height : u32,
	pub layer_count : u32,
	pub view_format_count : u32,
	pub p_view_formats : *const Format,
}

impl FramebufferAttachmentImageInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000108002,
			p_next : null(),
			flags : <_>::default(),
			usage : <_>::default(),
			width : <_>::default(),
			height : <_>::default(),
			layer_count : <_>::default(),
			view_format_count : <_>::default(),
			p_view_formats : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : ImageCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn usage(mut self, val : ImageUsageFlags) -> Self {
		self.usage = val;
		self
	}
	pub fn width(mut self, val : u32) -> Self {
		self.width = val;
		self
	}
	pub fn height(mut self, val : u32) -> Self {
		self.height = val;
		self
	}
	pub fn layer_count(mut self, val : u32) -> Self {
		self.layer_count = val;
		self
	}
	pub fn view_formats(mut self, val : &[Format]) -> Self {
		self.view_format_count = val.len() as _;
		self.p_view_formats = val.as_ptr();
		self
	}
	pub fn view_format(mut self, val : &Format) -> Self {
		self.view_format_count = 1;
		self.p_view_formats = val;
		self
	}
}

impl std::default::Default for FramebufferAttachmentImageInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ConditionalRenderingBeginInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub buffer : Buffer,
	pub offset : DeviceSize,
	pub flags : ConditionalRenderingFlagsEXT,
}

impl ConditionalRenderingBeginInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000081002,
			p_next : null(),
			buffer : Buffer(0),
			offset : <_>::default(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
	pub fn offset(mut self, val : DeviceSize) -> Self {
		self.offset = val;
		self
	}
	pub fn flags(mut self, val : ConditionalRenderingFlagsEXT) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for ConditionalRenderingBeginInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageResolve {
	pub src_subresource : ImageSubresourceLayers,
	pub src_offset : Offset3D,
	pub dst_subresource : ImageSubresourceLayers,
	pub dst_offset : Offset3D,
	pub extent : Extent3D,
}

impl ImageResolve {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn src_subresource(mut self, val : ImageSubresourceLayers) -> Self {
		self.src_subresource = val;
		self
	}
	pub fn src_offset(mut self, val : Offset3D) -> Self {
		self.src_offset = val;
		self
	}
	pub fn dst_subresource(mut self, val : ImageSubresourceLayers) -> Self {
		self.dst_subresource = val;
		self
	}
	pub fn dst_offset(mut self, val : Offset3D) -> Self {
		self.dst_offset = val;
		self
	}
	pub fn extent(mut self, val : Extent3D) -> Self {
		self.extent = val;
		self
	}
}

impl std::default::Default for ImageResolve {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct ExtensionProperties {
	pub extension_name : [u8; VK_MAX_EXTENSION_NAME_SIZE],
	pub spec_version : u32,
}

impl ExtensionProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn extension_name(mut self, val : [u8; VK_MAX_EXTENSION_NAME_SIZE]) -> Self {
		self.extension_name = val;
		self
	}
	pub fn spec_version(mut self, val : u32) -> Self {
		self.spec_version = val;
		self
	}
}

impl std::default::Default for ExtensionProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DeviceDiagnosticsConfigFlagsNV,
}

impl DeviceDiagnosticsConfigCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000300001,
			p_next : null(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DeviceDiagnosticsConfigFlagsNV) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for DeviceDiagnosticsConfigCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSparseProperties {
	pub residency_standard_2_dblock_shape : Bool32,
	pub residency_standard_2_dmultisample_block_shape : Bool32,
	pub residency_standard_3_dblock_shape : Bool32,
	pub residency_aligned_mip_size : Bool32,
	pub residency_non_resident_strict : Bool32,
}

impl PhysicalDeviceSparseProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn residency_standard_2_dblock_shape(mut self, val : Bool32) -> Self {
		self.residency_standard_2_dblock_shape = val;
		self
	}
	pub fn residency_standard_2_dmultisample_block_shape(mut self, val : Bool32) -> Self {
		self.residency_standard_2_dmultisample_block_shape = val;
		self
	}
	pub fn residency_standard_3_dblock_shape(mut self, val : Bool32) -> Self {
		self.residency_standard_3_dblock_shape = val;
		self
	}
	pub fn residency_aligned_mip_size(mut self, val : Bool32) -> Self {
		self.residency_aligned_mip_size = val;
		self
	}
	pub fn residency_non_resident_strict(mut self, val : Bool32) -> Self {
		self.residency_non_resident_strict = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSparseProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceMemoryProperties2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub memory_properties : PhysicalDeviceMemoryProperties,
}

impl PhysicalDeviceMemoryProperties2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059006,
			p_next : null_mut(),
			memory_properties : PhysicalDeviceMemoryProperties::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory_properties(mut self, val : PhysicalDeviceMemoryProperties) -> Self {
		self.memory_properties = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMemoryProperties2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AllocationCallbacks {
	pub p_user_data : *mut c_void,
	pub pfn_allocation : Option<extern "C" fn()>,
	pub pfn_reallocation : Option<extern "C" fn()>,
	pub pfn_free : Option<extern "C" fn()>,
	pub pfn_internal_allocation : Option<extern "C" fn()>,
	pub pfn_internal_free : Option<extern "C" fn()>,
}

impl AllocationCallbacks {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn user_data(mut self, val : *mut c_void) -> Self {
		self.p_user_data = val;
		self
	}
	pub fn pfn_allocation(mut self, val : Option<extern "C" fn()>) -> Self {
		self.pfn_allocation = val;
		self
	}
	pub fn pfn_reallocation(mut self, val : Option<extern "C" fn()>) -> Self {
		self.pfn_reallocation = val;
		self
	}
	pub fn pfn_free(mut self, val : Option<extern "C" fn()>) -> Self {
		self.pfn_free = val;
		self
	}
	pub fn pfn_internal_allocation(mut self, val : Option<extern "C" fn()>) -> Self {
		self.pfn_internal_allocation = val;
		self
	}
	pub fn pfn_internal_free(mut self, val : Option<extern "C" fn()>) -> Self {
		self.pfn_internal_free = val;
		self
	}
}

impl std::default::Default for AllocationCallbacks {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryBarrier {
	s_type : i32,
	pub p_next : *const c_void,
	pub src_access_mask : AccessFlags,
	pub dst_access_mask : AccessFlags,
}

impl MemoryBarrier {
	pub fn new() -> Self {
		Self {
			s_type : 46,
			p_next : null(),
			src_access_mask : <_>::default(),
			dst_access_mask : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn src_access_mask(mut self, val : AccessFlags) -> Self {
		self.src_access_mask = val;
		self
	}
	pub fn dst_access_mask(mut self, val : AccessFlags) -> Self {
		self.dst_access_mask = val;
		self
	}
}

impl std::default::Default for MemoryBarrier {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub vertex_binding_divisor_count : u32,
	pub p_vertex_binding_divisors : *const VertexInputBindingDivisorDescriptionEXT,
}

impl PipelineVertexInputDivisorStateCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000190001,
			p_next : null(),
			vertex_binding_divisor_count : <_>::default(),
			p_vertex_binding_divisors : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn vertex_binding_divisors(mut self, val : &[VertexInputBindingDivisorDescriptionEXT]) -> Self {
		self.vertex_binding_divisor_count = val.len() as _;
		self.p_vertex_binding_divisors = val.as_ptr();
		self
	}
	pub fn vertex_binding_divisor(mut self, val : &VertexInputBindingDivisorDescriptionEXT) -> Self {
		self.vertex_binding_divisor_count = 1;
		self.p_vertex_binding_divisors = val;
		self
	}
}

impl std::default::Default for PipelineVertexInputDivisorStateCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Rect2D {
	pub offset : Offset2D,
	pub extent : Extent2D,
}

impl Rect2D {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn offset(mut self, val : Offset2D) -> Self {
		self.offset = val;
		self
	}
	pub fn extent(mut self, val : Extent2D) -> Self {
		self.extent = val;
		self
	}
}

impl std::default::Default for Rect2D {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFloatControlsProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub denorm_behavior_independence : ShaderFloatControlsIndependence,
	pub rounding_mode_independence : ShaderFloatControlsIndependence,
	pub shader_signed_zero_inf_nan_preserve_float_16 : Bool32,
	pub shader_signed_zero_inf_nan_preserve_float_32 : Bool32,
	pub shader_signed_zero_inf_nan_preserve_float_64 : Bool32,
	pub shader_denorm_preserve_float_16 : Bool32,
	pub shader_denorm_preserve_float_32 : Bool32,
	pub shader_denorm_preserve_float_64 : Bool32,
	pub shader_denorm_flush_to_zero_float_16 : Bool32,
	pub shader_denorm_flush_to_zero_float_32 : Bool32,
	pub shader_denorm_flush_to_zero_float_64 : Bool32,
	pub shader_rounding_mode_rtefloat_16 : Bool32,
	pub shader_rounding_mode_rtefloat_32 : Bool32,
	pub shader_rounding_mode_rtefloat_64 : Bool32,
	pub shader_rounding_mode_rtzfloat_16 : Bool32,
	pub shader_rounding_mode_rtzfloat_32 : Bool32,
	pub shader_rounding_mode_rtzfloat_64 : Bool32,
}

impl PhysicalDeviceFloatControlsProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000197000,
			p_next : null_mut(),
			denorm_behavior_independence : ShaderFloatControlsIndependence::default(),
			rounding_mode_independence : ShaderFloatControlsIndependence::default(),
			shader_signed_zero_inf_nan_preserve_float_16 : <_>::default(),
			shader_signed_zero_inf_nan_preserve_float_32 : <_>::default(),
			shader_signed_zero_inf_nan_preserve_float_64 : <_>::default(),
			shader_denorm_preserve_float_16 : <_>::default(),
			shader_denorm_preserve_float_32 : <_>::default(),
			shader_denorm_preserve_float_64 : <_>::default(),
			shader_denorm_flush_to_zero_float_16 : <_>::default(),
			shader_denorm_flush_to_zero_float_32 : <_>::default(),
			shader_denorm_flush_to_zero_float_64 : <_>::default(),
			shader_rounding_mode_rtefloat_16 : <_>::default(),
			shader_rounding_mode_rtefloat_32 : <_>::default(),
			shader_rounding_mode_rtefloat_64 : <_>::default(),
			shader_rounding_mode_rtzfloat_16 : <_>::default(),
			shader_rounding_mode_rtzfloat_32 : <_>::default(),
			shader_rounding_mode_rtzfloat_64 : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn denorm_behavior_independence(mut self, val : ShaderFloatControlsIndependence) -> Self {
		self.denorm_behavior_independence = val;
		self
	}
	pub fn rounding_mode_independence(mut self, val : ShaderFloatControlsIndependence) -> Self {
		self.rounding_mode_independence = val;
		self
	}
	pub fn shader_signed_zero_inf_nan_preserve_float_16(mut self, val : Bool32) -> Self {
		self.shader_signed_zero_inf_nan_preserve_float_16 = val;
		self
	}
	pub fn shader_signed_zero_inf_nan_preserve_float_32(mut self, val : Bool32) -> Self {
		self.shader_signed_zero_inf_nan_preserve_float_32 = val;
		self
	}
	pub fn shader_signed_zero_inf_nan_preserve_float_64(mut self, val : Bool32) -> Self {
		self.shader_signed_zero_inf_nan_preserve_float_64 = val;
		self
	}
	pub fn shader_denorm_preserve_float_16(mut self, val : Bool32) -> Self {
		self.shader_denorm_preserve_float_16 = val;
		self
	}
	pub fn shader_denorm_preserve_float_32(mut self, val : Bool32) -> Self {
		self.shader_denorm_preserve_float_32 = val;
		self
	}
	pub fn shader_denorm_preserve_float_64(mut self, val : Bool32) -> Self {
		self.shader_denorm_preserve_float_64 = val;
		self
	}
	pub fn shader_denorm_flush_to_zero_float_16(mut self, val : Bool32) -> Self {
		self.shader_denorm_flush_to_zero_float_16 = val;
		self
	}
	pub fn shader_denorm_flush_to_zero_float_32(mut self, val : Bool32) -> Self {
		self.shader_denorm_flush_to_zero_float_32 = val;
		self
	}
	pub fn shader_denorm_flush_to_zero_float_64(mut self, val : Bool32) -> Self {
		self.shader_denorm_flush_to_zero_float_64 = val;
		self
	}
	pub fn shader_rounding_mode_rtefloat_16(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtefloat_16 = val;
		self
	}
	pub fn shader_rounding_mode_rtefloat_32(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtefloat_32 = val;
		self
	}
	pub fn shader_rounding_mode_rtefloat_64(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtefloat_64 = val;
		self
	}
	pub fn shader_rounding_mode_rtzfloat_16(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtzfloat_16 = val;
		self
	}
	pub fn shader_rounding_mode_rtzfloat_32(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtzfloat_32 = val;
		self
	}
	pub fn shader_rounding_mode_rtzfloat_64(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtzfloat_64 = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFloatControlsProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DrawIndexedIndirectCommand {
	pub index_count : u32,
	pub instance_count : u32,
	pub first_index : u32,
	pub vertex_offset : i32,
	pub first_instance : u32,
}

impl DrawIndexedIndirectCommand {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn index_count(mut self, val : u32) -> Self {
		self.index_count = val;
		self
	}
	pub fn instance_count(mut self, val : u32) -> Self {
		self.instance_count = val;
		self
	}
	pub fn first_index(mut self, val : u32) -> Self {
		self.first_index = val;
		self
	}
	pub fn vertex_offset(mut self, val : i32) -> Self {
		self.vertex_offset = val;
		self
	}
	pub fn first_instance(mut self, val : u32) -> Self {
		self.first_instance = val;
		self
	}
}

impl std::default::Default for DrawIndexedIndirectCommand {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryDedicatedAllocateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub image : Image,
	pub buffer : Buffer,
}

impl MemoryDedicatedAllocateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000127001,
			p_next : null(),
			image : Image(0),
			buffer : Buffer(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
}

impl std::default::Default for MemoryDedicatedAllocateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub full_screen_exclusive : FullScreenExclusiveEXT,
}

impl SurfaceFullScreenExclusiveInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000255000,
			p_next : null_mut(),
			full_screen_exclusive : FullScreenExclusiveEXT::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn full_screen_exclusive(mut self, val : FullScreenExclusiveEXT) -> Self {
		self.full_screen_exclusive = val;
		self
	}
}

impl std::default::Default for SurfaceFullScreenExclusiveInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AcquireProfilingLockInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : AcquireProfilingLockFlagsKHR,
	pub timeout : u64,
}

impl AcquireProfilingLockInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000116004,
			p_next : null(),
			flags : <_>::default(),
			timeout : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : AcquireProfilingLockFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn timeout(mut self, val : u64) -> Self {
		self.timeout = val;
		self
	}
}

impl std::default::Default for AcquireProfilingLockInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineDepthStencilStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineDepthStencilStateCreateFlags,
	pub depth_test_enable : Bool32,
	pub depth_write_enable : Bool32,
	pub depth_compare_op : CompareOp,
	pub depth_bounds_test_enable : Bool32,
	pub stencil_test_enable : Bool32,
	pub front : StencilOpState,
	pub back : StencilOpState,
	pub min_depth_bounds : f32,
	pub max_depth_bounds : f32,
}

impl PipelineDepthStencilStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 25,
			p_next : null(),
			flags : <_>::default(),
			depth_test_enable : <_>::default(),
			depth_write_enable : <_>::default(),
			depth_compare_op : CompareOp::default(),
			depth_bounds_test_enable : <_>::default(),
			stencil_test_enable : <_>::default(),
			front : StencilOpState::new(),
			back : StencilOpState::new(),
			min_depth_bounds : <_>::default(),
			max_depth_bounds : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineDepthStencilStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn depth_test_enable(mut self, val : Bool32) -> Self {
		self.depth_test_enable = val;
		self
	}
	pub fn depth_write_enable(mut self, val : Bool32) -> Self {
		self.depth_write_enable = val;
		self
	}
	pub fn depth_compare_op(mut self, val : CompareOp) -> Self {
		self.depth_compare_op = val;
		self
	}
	pub fn depth_bounds_test_enable(mut self, val : Bool32) -> Self {
		self.depth_bounds_test_enable = val;
		self
	}
	pub fn stencil_test_enable(mut self, val : Bool32) -> Self {
		self.stencil_test_enable = val;
		self
	}
	pub fn front(mut self, val : StencilOpState) -> Self {
		self.front = val;
		self
	}
	pub fn back(mut self, val : StencilOpState) -> Self {
		self.back = val;
		self
	}
	pub fn min_depth_bounds(mut self, val : f32) -> Self {
		self.min_depth_bounds = val;
		self
	}
	pub fn max_depth_bounds(mut self, val : f32) -> Self {
		self.max_depth_bounds = val;
		self
	}
}

impl std::default::Default for PipelineDepthStencilStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub texel_buffer_alignment : Bool32,
}

impl PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000281000,
			p_next : null_mut(),
			texel_buffer_alignment : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn texel_buffer_alignment(mut self, val : Bool32) -> Self {
		self.texel_buffer_alignment = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SpecializationInfo {
	pub map_entry_count : u32,
	pub p_map_entries : *const SpecializationMapEntry,
	pub data_size : usize,
	pub p_data : *const c_void,
}

impl SpecializationInfo {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn data_size(mut self, val : usize) -> Self {
		self.data_size = val;
		self
	}
	pub fn data(mut self, val : *const c_void) -> Self {
		self.p_data = val;
		self
	}
	pub fn map_entries(mut self, val : &[SpecializationMapEntry]) -> Self {
		self.map_entry_count = val.len() as _;
		self.p_map_entries = val.as_ptr();
		self
	}
	pub fn map_entry(mut self, val : &SpecializationMapEntry) -> Self {
		self.map_entry_count = 1;
		self.p_map_entries = val;
		self
	}
}

impl std::default::Default for SpecializationInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct IOSSurfaceCreateInfoMVK {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : IOSSurfaceCreateFlagsMVK,
	pub p_view : *const c_void,
}

impl IOSSurfaceCreateInfoMVK {
	pub fn new() -> Self {
		Self {
			s_type : 1000122000,
			p_next : null(),
			flags : <_>::default(),
			p_view : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : IOSSurfaceCreateFlagsMVK) -> Self {
		self.flags = val;
		self
	}
	pub fn view(mut self, val : *const c_void) -> Self {
		self.p_view = val;
		self
	}
}

impl std::default::Default for IOSSurfaceCreateInfoMVK {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportMemoryWin32HandleInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_type : ExternalMemoryHandleTypeFlagsNV,
	pub handle : HANDLE,
}

impl ImportMemoryWin32HandleInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000057000,
			p_next : null(),
			handle_type : <_>::default(),
			handle : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalMemoryHandleTypeFlagsNV) -> Self {
		self.handle_type = val;
		self
	}
	pub fn handle(mut self, val : HANDLE) -> Self {
		self.handle = val;
		self
	}
}

impl std::default::Default for ImportMemoryWin32HandleInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevicePrivateDataFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub private_data : Bool32,
}

impl PhysicalDevicePrivateDataFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000295000,
			p_next : null_mut(),
			private_data : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn private_data(mut self, val : Bool32) -> Self {
		self.private_data = val;
		self
	}
}

impl std::default::Default for PhysicalDevicePrivateDataFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Viewport {
	pub x : f32,
	pub y : f32,
	pub width : f32,
	pub height : f32,
	pub min_depth : f32,
	pub max_depth : f32,
}

impl Viewport {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn x(mut self, val : f32) -> Self {
		self.x = val;
		self
	}
	pub fn y(mut self, val : f32) -> Self {
		self.y = val;
		self
	}
	pub fn width(mut self, val : f32) -> Self {
		self.width = val;
		self
	}
	pub fn height(mut self, val : f32) -> Self {
		self.height = val;
		self
	}
	pub fn min_depth(mut self, val : f32) -> Self {
		self.min_depth = val;
		self
	}
	pub fn max_depth(mut self, val : f32) -> Self {
		self.max_depth = val;
		self
	}
}

impl std::default::Default for Viewport {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferMemoryBarrier {
	s_type : i32,
	pub p_next : *const c_void,
	pub src_access_mask : AccessFlags,
	pub dst_access_mask : AccessFlags,
	pub src_queue_family_index : u32,
	pub dst_queue_family_index : u32,
	pub buffer : Buffer,
	pub offset : DeviceSize,
	pub size : DeviceSize,
}

impl BufferMemoryBarrier {
	pub fn new() -> Self {
		Self {
			s_type : 44,
			p_next : null(),
			src_access_mask : <_>::default(),
			dst_access_mask : <_>::default(),
			src_queue_family_index : <_>::default(),
			dst_queue_family_index : <_>::default(),
			buffer : Buffer(0),
			offset : <_>::default(),
			size : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn src_access_mask(mut self, val : AccessFlags) -> Self {
		self.src_access_mask = val;
		self
	}
	pub fn dst_access_mask(mut self, val : AccessFlags) -> Self {
		self.dst_access_mask = val;
		self
	}
	pub fn src_queue_family_index(mut self, val : u32) -> Self {
		self.src_queue_family_index = val;
		self
	}
	pub fn dst_queue_family_index(mut self, val : u32) -> Self {
		self.dst_queue_family_index = val;
		self
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
	pub fn offset(mut self, val : DeviceSize) -> Self {
		self.offset = val;
		self
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
}

impl std::default::Default for BufferMemoryBarrier {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_buffer_float_32_atomics : Bool32,
	pub shader_buffer_float_32_atomic_add : Bool32,
	pub shader_buffer_float_64_atomics : Bool32,
	pub shader_buffer_float_64_atomic_add : Bool32,
	pub shader_shared_float_32_atomics : Bool32,
	pub shader_shared_float_32_atomic_add : Bool32,
	pub shader_shared_float_64_atomics : Bool32,
	pub shader_shared_float_64_atomic_add : Bool32,
	pub shader_image_float_32_atomics : Bool32,
	pub shader_image_float_32_atomic_add : Bool32,
	pub sparse_image_float_32_atomics : Bool32,
	pub sparse_image_float_32_atomic_add : Bool32,
}

impl PhysicalDeviceShaderAtomicFloatFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000260000,
			p_next : null_mut(),
			shader_buffer_float_32_atomics : <_>::default(),
			shader_buffer_float_32_atomic_add : <_>::default(),
			shader_buffer_float_64_atomics : <_>::default(),
			shader_buffer_float_64_atomic_add : <_>::default(),
			shader_shared_float_32_atomics : <_>::default(),
			shader_shared_float_32_atomic_add : <_>::default(),
			shader_shared_float_64_atomics : <_>::default(),
			shader_shared_float_64_atomic_add : <_>::default(),
			shader_image_float_32_atomics : <_>::default(),
			shader_image_float_32_atomic_add : <_>::default(),
			sparse_image_float_32_atomics : <_>::default(),
			sparse_image_float_32_atomic_add : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_buffer_float_32_atomics(mut self, val : Bool32) -> Self {
		self.shader_buffer_float_32_atomics = val;
		self
	}
	pub fn shader_buffer_float_32_atomic_add(mut self, val : Bool32) -> Self {
		self.shader_buffer_float_32_atomic_add = val;
		self
	}
	pub fn shader_buffer_float_64_atomics(mut self, val : Bool32) -> Self {
		self.shader_buffer_float_64_atomics = val;
		self
	}
	pub fn shader_buffer_float_64_atomic_add(mut self, val : Bool32) -> Self {
		self.shader_buffer_float_64_atomic_add = val;
		self
	}
	pub fn shader_shared_float_32_atomics(mut self, val : Bool32) -> Self {
		self.shader_shared_float_32_atomics = val;
		self
	}
	pub fn shader_shared_float_32_atomic_add(mut self, val : Bool32) -> Self {
		self.shader_shared_float_32_atomic_add = val;
		self
	}
	pub fn shader_shared_float_64_atomics(mut self, val : Bool32) -> Self {
		self.shader_shared_float_64_atomics = val;
		self
	}
	pub fn shader_shared_float_64_atomic_add(mut self, val : Bool32) -> Self {
		self.shader_shared_float_64_atomic_add = val;
		self
	}
	pub fn shader_image_float_32_atomics(mut self, val : Bool32) -> Self {
		self.shader_image_float_32_atomics = val;
		self
	}
	pub fn shader_image_float_32_atomic_add(mut self, val : Bool32) -> Self {
		self.shader_image_float_32_atomic_add = val;
		self
	}
	pub fn sparse_image_float_32_atomics(mut self, val : Bool32) -> Self {
		self.sparse_image_float_32_atomics = val;
		self
	}
	pub fn sparse_image_float_32_atomic_add(mut self, val : Bool32) -> Self {
		self.sparse_image_float_32_atomic_add = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderAtomicFloatFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceMemoryProperties {
	pub memory_type_count : u32,
	pub memory_types : [MemoryType; VK_MAX_MEMORY_TYPES],
	pub memory_heap_count : u32,
	pub memory_heaps : [MemoryHeap; VK_MAX_MEMORY_HEAPS],
}

impl PhysicalDeviceMemoryProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn memory_type_count(mut self, val : u32) -> Self {
		self.memory_type_count = val;
		self
	}
	pub fn memory_types(mut self, val : [MemoryType; VK_MAX_MEMORY_TYPES]) -> Self {
		self.memory_types = val;
		self
	}
	pub fn memory_heap_count(mut self, val : u32) -> Self {
		self.memory_heap_count = val;
		self
	}
	pub fn memory_heaps(mut self, val : [MemoryHeap; VK_MAX_MEMORY_HEAPS]) -> Self {
		self.memory_heaps = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMemoryProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceCapabilities2KHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub surface_capabilities : SurfaceCapabilitiesKHR,
}

impl SurfaceCapabilities2KHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000119001,
			p_next : null_mut(),
			surface_capabilities : SurfaceCapabilitiesKHR::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn surface_capabilities(mut self, val : SurfaceCapabilitiesKHR) -> Self {
		self.surface_capabilities = val;
		self
	}
}

impl std::default::Default for SurfaceCapabilities2KHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceProperties {
	pub api_version : u32,
	pub driver_version : u32,
	pub vendor_id : u32,
	pub device_id : u32,
	pub device_type : PhysicalDeviceType,
	pub device_name : [u8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
	pub pipeline_cache_uuid : [u8; VK_UUID_SIZE],
	pub limits : PhysicalDeviceLimits,
	pub sparse_properties : PhysicalDeviceSparseProperties,
}

impl PhysicalDeviceProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn api_version(mut self, val : u32) -> Self {
		self.api_version = val;
		self
	}
	pub fn driver_version(mut self, val : u32) -> Self {
		self.driver_version = val;
		self
	}
	pub fn vendor_id(mut self, val : u32) -> Self {
		self.vendor_id = val;
		self
	}
	pub fn device_id(mut self, val : u32) -> Self {
		self.device_id = val;
		self
	}
	pub fn device_type(mut self, val : PhysicalDeviceType) -> Self {
		self.device_type = val;
		self
	}
	pub fn device_name(mut self, val : [u8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]) -> Self {
		self.device_name = val;
		self
	}
	pub fn pipeline_cache_uuid(mut self, val : [u8; VK_UUID_SIZE]) -> Self {
		self.pipeline_cache_uuid = val;
		self
	}
	pub fn limits(mut self, val : PhysicalDeviceLimits) -> Self {
		self.limits = val;
		self
	}
	pub fn sparse_properties(mut self, val : PhysicalDeviceSparseProperties) -> Self {
		self.sparse_properties = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
	s_type : i32,
	pub p_next : *const c_void,
	pub local_dimming_enable : Bool32,
}

impl SwapchainDisplayNativeHdrCreateInfoAMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000213001,
			p_next : null(),
			local_dimming_enable : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn local_dimming_enable(mut self, val : Bool32) -> Self {
		self.local_dimming_enable = val;
		self
	}
}

impl std::default::Default for SwapchainDisplayNativeHdrCreateInfoAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceGroupBindSparseInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub resource_device_index : u32,
	pub memory_device_index : u32,
}

impl DeviceGroupBindSparseInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000060006,
			p_next : null(),
			resource_device_index : <_>::default(),
			memory_device_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn resource_device_index(mut self, val : u32) -> Self {
		self.resource_device_index = val;
		self
	}
	pub fn memory_device_index(mut self, val : u32) -> Self {
		self.memory_device_index = val;
		self
	}
}

impl std::default::Default for DeviceGroupBindSparseInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassTransformBeginInfoQCOM {
	s_type : i32,
	pub p_next : *mut c_void,
	pub transform : SurfaceTransformFlagsKHR,
}

impl RenderPassTransformBeginInfoQCOM {
	pub fn new() -> Self {
		Self {
			s_type : 1000282001,
			p_next : null_mut(),
			transform : SurfaceTransformFlagsKHR::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn transform(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.transform = val;
		self
	}
}

impl std::default::Default for RenderPassTransformBeginInfoQCOM {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalBufferProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub external_memory_properties : ExternalMemoryProperties,
}

impl ExternalBufferProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000071003,
			p_next : null_mut(),
			external_memory_properties : ExternalMemoryProperties::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn external_memory_properties(mut self, val : ExternalMemoryProperties) -> Self {
		self.external_memory_properties = val;
		self
	}
}

impl std::default::Default for ExternalBufferProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub device_generated_commands : Bool32,
}

impl PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000277007,
			p_next : null_mut(),
			device_generated_commands : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_generated_commands(mut self, val : Bool32) -> Self {
		self.device_generated_commands = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageSwapchainCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub swapchain : SwapchainKHR,
}

impl ImageSwapchainCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000060008,
			p_next : null(),
			swapchain : SwapchainKHR(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn swapchain(mut self, val : SwapchainKHR) -> Self {
		self.swapchain = val;
		self
	}
}

impl std::default::Default for ImageSwapchainCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportMemoryHostPointerInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_type : ExternalMemoryHandleTypeFlags,
	pub p_host_pointer : *mut c_void,
}

impl ImportMemoryHostPointerInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000178000,
			p_next : null(),
			handle_type : ExternalMemoryHandleTypeFlags::default(),
			p_host_pointer : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
	pub fn host_pointer(mut self, val : *mut c_void) -> Self {
		self.p_host_pointer = val;
		self
	}
}

impl std::default::Default for ImportMemoryHostPointerInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceGroupSubmitInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub wait_semaphore_count : u32,
	pub p_wait_semaphore_device_indices : *const u32,
	pub command_buffer_count : u32,
	pub p_command_buffer_device_masks : *const u32,
	pub signal_semaphore_count : u32,
	pub p_signal_semaphore_device_indices : *const u32,
}

impl DeviceGroupSubmitInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000060005,
			p_next : null(),
			wait_semaphore_count : <_>::default(),
			p_wait_semaphore_device_indices : null(),
			command_buffer_count : <_>::default(),
			p_command_buffer_device_masks : null(),
			signal_semaphore_count : <_>::default(),
			p_signal_semaphore_device_indices : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn wait_semaphore_device_indices(mut self, val : &[u32]) -> Self {
		self.wait_semaphore_count = val.len() as _;
		self.p_wait_semaphore_device_indices = val.as_ptr();
		self
	}
	pub fn wait_semaphore_device_indice(mut self, val : &u32) -> Self {
		self.wait_semaphore_count = 1;
		self.p_wait_semaphore_device_indices = val;
		self
	}
	pub fn command_buffer_device_masks(mut self, val : &[u32]) -> Self {
		self.command_buffer_count = val.len() as _;
		self.p_command_buffer_device_masks = val.as_ptr();
		self
	}
	pub fn command_buffer_device_mask(mut self, val : &u32) -> Self {
		self.command_buffer_count = 1;
		self.p_command_buffer_device_masks = val;
		self
	}
	pub fn signal_semaphore_device_indices(mut self, val : &[u32]) -> Self {
		self.signal_semaphore_count = val.len() as _;
		self.p_signal_semaphore_device_indices = val.as_ptr();
		self
	}
	pub fn signal_semaphore_device_indice(mut self, val : &u32) -> Self {
		self.signal_semaphore_count = 1;
		self.p_signal_semaphore_device_indices = val;
		self
	}
}

impl std::default::Default for DeviceGroupSubmitInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_discard_rectangles : u32,
}

impl PhysicalDeviceDiscardRectanglePropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000099000,
			p_next : null_mut(),
			max_discard_rectangles : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_discard_rectangles(mut self, val : u32) -> Self {
		self.max_discard_rectangles = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDiscardRectanglePropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevice8BitStorageFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub storage_buffer_8_bit_access : Bool32,
	pub uniform_and_storage_buffer_8_bit_access : Bool32,
	pub storage_push_constant_8 : Bool32,
}

impl PhysicalDevice8BitStorageFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000177000,
			p_next : null_mut(),
			storage_buffer_8_bit_access : <_>::default(),
			uniform_and_storage_buffer_8_bit_access : <_>::default(),
			storage_push_constant_8 : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn storage_buffer_8_bit_access(mut self, val : Bool32) -> Self {
		self.storage_buffer_8_bit_access = val;
		self
	}
	pub fn uniform_and_storage_buffer_8_bit_access(mut self, val : Bool32) -> Self {
		self.uniform_and_storage_buffer_8_bit_access = val;
		self
	}
	pub fn storage_push_constant_8(mut self, val : Bool32) -> Self {
		self.storage_push_constant_8 = val;
		self
	}
}

impl std::default::Default for PhysicalDevice8BitStorageFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryHeap {
	pub size : DeviceSize,
	pub flags : MemoryHeapFlags,
}

impl MemoryHeap {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
	pub fn flags(mut self, val : MemoryHeapFlags) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for MemoryHeap {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceExternalImageFormatInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_type : ExternalMemoryHandleTypeFlags,
}

impl PhysicalDeviceExternalImageFormatInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000071000,
			p_next : null(),
			handle_type : ExternalMemoryHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceExternalImageFormatInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageFormatProperties {
	pub max_extent : Extent3D,
	pub max_mip_levels : u32,
	pub max_array_layers : u32,
	pub sample_counts : SampleCountFlags,
	pub max_resource_size : DeviceSize,
}

impl ImageFormatProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn max_extent(mut self, val : Extent3D) -> Self {
		self.max_extent = val;
		self
	}
	pub fn max_mip_levels(mut self, val : u32) -> Self {
		self.max_mip_levels = val;
		self
	}
	pub fn max_array_layers(mut self, val : u32) -> Self {
		self.max_array_layers = val;
		self
	}
	pub fn sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.sample_counts = val;
		self
	}
	pub fn max_resource_size(mut self, val : DeviceSize) -> Self {
		self.max_resource_size = val;
		self
	}
}

impl std::default::Default for ImageFormatProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct QueueFamilyProperties {
	pub queue_flags : QueueFlags,
	pub queue_count : u32,
	pub timestamp_valid_bits : u32,
	pub min_image_transfer_granularity : Extent3D,
}

impl QueueFamilyProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn queue_flags(mut self, val : QueueFlags) -> Self {
		self.queue_flags = val;
		self
	}
	pub fn queue_count(mut self, val : u32) -> Self {
		self.queue_count = val;
		self
	}
	pub fn timestamp_valid_bits(mut self, val : u32) -> Self {
		self.timestamp_valid_bits = val;
		self
	}
	pub fn min_image_transfer_granularity(mut self, val : Extent3D) -> Self {
		self.min_image_transfer_granularity = val;
		self
	}
}

impl std::default::Default for QueueFamilyProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubpassDescriptionDepthStencilResolve {
	s_type : i32,
	pub p_next : *const c_void,
	pub depth_resolve_mode : ResolveModeFlags,
	pub stencil_resolve_mode : ResolveModeFlags,
	pub p_depth_stencil_resolve_attachment : *const AttachmentReference2,
}

impl SubpassDescriptionDepthStencilResolve {
	pub fn new() -> Self {
		Self {
			s_type : 1000199001,
			p_next : null(),
			depth_resolve_mode : ResolveModeFlags::default(),
			stencil_resolve_mode : ResolveModeFlags::default(),
			p_depth_stencil_resolve_attachment : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn depth_resolve_mode(mut self, val : ResolveModeFlags) -> Self {
		self.depth_resolve_mode = val;
		self
	}
	pub fn stencil_resolve_mode(mut self, val : ResolveModeFlags) -> Self {
		self.stencil_resolve_mode = val;
		self
	}
	pub fn depth_stencil_resolve_attachment(mut self, val : *const AttachmentReference2) -> Self {
		self.p_depth_stencil_resolve_attachment = val;
		self
	}
}

impl std::default::Default for SubpassDescriptionDepthStencilResolve {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AttachmentSampleLocationsEXT {
	pub attachment_index : u32,
	pub sample_locations_info : SampleLocationsInfoEXT,
}

impl AttachmentSampleLocationsEXT {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn attachment_index(mut self, val : u32) -> Self {
		self.attachment_index = val;
		self
	}
	pub fn sample_locations_info(mut self, val : SampleLocationsInfoEXT) -> Self {
		self.sample_locations_info = val;
		self
	}
}

impl std::default::Default for AttachmentSampleLocationsEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FormatProperties {
	pub linear_tiling_features : FormatFeatureFlags,
	pub optimal_tiling_features : FormatFeatureFlags,
	pub buffer_features : FormatFeatureFlags,
}

impl FormatProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn linear_tiling_features(mut self, val : FormatFeatureFlags) -> Self {
		self.linear_tiling_features = val;
		self
	}
	pub fn optimal_tiling_features(mut self, val : FormatFeatureFlags) -> Self {
		self.optimal_tiling_features = val;
		self
	}
	pub fn buffer_features(mut self, val : FormatFeatureFlags) -> Self {
		self.buffer_features = val;
		self
	}
}

impl std::default::Default for FormatProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub allow_command_buffer_query_copies : Bool32,
}

impl PhysicalDevicePerformanceQueryPropertiesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000116001,
			p_next : null_mut(),
			allow_command_buffer_query_copies : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn allow_command_buffer_query_copies(mut self, val : Bool32) -> Self {
		self.allow_command_buffer_query_copies = val;
		self
	}
}

impl std::default::Default for PhysicalDevicePerformanceQueryPropertiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : BufferCreateFlags,
	pub size : DeviceSize,
	pub usage : BufferUsageFlags,
	pub sharing_mode : SharingMode,
	pub queue_family_index_count : u32,
	pub p_queue_family_indices : *const u32,
}

impl BufferCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 12,
			p_next : null(),
			flags : <_>::default(),
			size : <_>::default(),
			usage : <_>::default(),
			sharing_mode : SharingMode::default(),
			queue_family_index_count : <_>::default(),
			p_queue_family_indices : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : BufferCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
	pub fn usage(mut self, val : BufferUsageFlags) -> Self {
		self.usage = val;
		self
	}
	pub fn sharing_mode(mut self, val : SharingMode) -> Self {
		self.sharing_mode = val;
		self
	}
	pub fn queue_family_indices(mut self, val : &[u32]) -> Self {
		self.queue_family_index_count = val.len() as _;
		self.p_queue_family_indices = val.as_ptr();
		self
	}
	pub fn queue_family_indice(mut self, val : &u32) -> Self {
		self.queue_family_index_count = 1;
		self.p_queue_family_indices = val;
		self
	}
}

impl std::default::Default for BufferCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ApplicationInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_application_name : *const u8,
	pub application_version : u32,
	pub p_engine_name : *const u8,
	pub engine_version : u32,
	pub api_version : u32,
}

impl ApplicationInfo {
	pub fn new() -> Self {
		Self {
			s_type : 0,
			p_next : null(),
			p_application_name : null(),
			application_version : <_>::default(),
			p_engine_name : null(),
			engine_version : <_>::default(),
			api_version : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn application_name(mut self, val : *const u8) -> Self {
		self.p_application_name = val;
		self
	}
	pub fn application_version(mut self, val : u32) -> Self {
		self.application_version = val;
		self
	}
	pub fn engine_name(mut self, val : *const u8) -> Self {
		self.p_engine_name = val;
		self
	}
	pub fn engine_version(mut self, val : u32) -> Self {
		self.engine_version = val;
		self
	}
	pub fn api_version(mut self, val : u32) -> Self {
		self.api_version = val;
		self
	}
}

impl std::default::Default for ApplicationInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct VertexInputAttributeDescription {
	pub location : u32,
	pub binding : u32,
	pub format : Format,
	pub offset : u32,
}

impl VertexInputAttributeDescription {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn location(mut self, val : u32) -> Self {
		self.location = val;
		self
	}
	pub fn binding(mut self, val : u32) -> Self {
		self.binding = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn offset(mut self, val : u32) -> Self {
		self.offset = val;
		self
	}
}

impl std::default::Default for VertexInputAttributeDescription {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GeometryDataNV {
	pub triangles : GeometryTrianglesNV,
	pub aabbs : GeometryAABBNV,
}

impl GeometryDataNV {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn triangles(mut self, val : GeometryTrianglesNV) -> Self {
		self.triangles = val;
		self
	}
	pub fn aabbs(mut self, val : GeometryAABBNV) -> Self {
		self.aabbs = val;
		self
	}
}

impl std::default::Default for GeometryDataNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceQueueCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DeviceQueueCreateFlags,
	pub queue_family_index : u32,
	pub queue_count : u32,
	pub p_queue_priorities : *const f32,
}

impl DeviceQueueCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 2,
			p_next : null(),
			flags : <_>::default(),
			queue_family_index : <_>::default(),
			queue_count : <_>::default(),
			p_queue_priorities : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DeviceQueueCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn queue_family_index(mut self, val : u32) -> Self {
		self.queue_family_index = val;
		self
	}
	pub fn queue_priorities(mut self, val : &[f32]) -> Self {
		self.queue_count = val.len() as _;
		self.p_queue_priorities = val.as_ptr();
		self
	}
	pub fn queue_priority(mut self, val : &f32) -> Self {
		self.queue_count = 1;
		self.p_queue_priorities = val;
		self
	}
}

impl std::default::Default for DeviceQueueCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubresourceLayout {
	pub offset : DeviceSize,
	pub size : DeviceSize,
	pub row_pitch : DeviceSize,
	pub array_pitch : DeviceSize,
	pub depth_pitch : DeviceSize,
}

impl SubresourceLayout {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn offset(mut self, val : DeviceSize) -> Self {
		self.offset = val;
		self
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
	pub fn row_pitch(mut self, val : DeviceSize) -> Self {
		self.row_pitch = val;
		self
	}
	pub fn array_pitch(mut self, val : DeviceSize) -> Self {
		self.array_pitch = val;
		self
	}
	pub fn depth_pitch(mut self, val : DeviceSize) -> Self {
		self.depth_pitch = val;
		self
	}
}

impl std::default::Default for SubresourceLayout {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineCoverageToColorStateCreateFlagsNV,
	pub coverage_to_color_enable : Bool32,
	pub coverage_to_color_location : u32,
}

impl PipelineCoverageToColorStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000149000,
			p_next : null(),
			flags : <_>::default(),
			coverage_to_color_enable : <_>::default(),
			coverage_to_color_location : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineCoverageToColorStateCreateFlagsNV) -> Self {
		self.flags = val;
		self
	}
	pub fn coverage_to_color_enable(mut self, val : Bool32) -> Self {
		self.coverage_to_color_enable = val;
		self
	}
	pub fn coverage_to_color_location(mut self, val : u32) -> Self {
		self.coverage_to_color_location = val;
		self
	}
}

impl std::default::Default for PipelineCoverageToColorStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryDedicatedRequirements {
	s_type : i32,
	pub p_next : *mut c_void,
	pub prefers_dedicated_allocation : Bool32,
	pub requires_dedicated_allocation : Bool32,
}

impl MemoryDedicatedRequirements {
	pub fn new() -> Self {
		Self {
			s_type : 1000127000,
			p_next : null_mut(),
			prefers_dedicated_allocation : <_>::default(),
			requires_dedicated_allocation : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn prefers_dedicated_allocation(mut self, val : Bool32) -> Self {
		self.prefers_dedicated_allocation = val;
		self
	}
	pub fn requires_dedicated_allocation(mut self, val : Bool32) -> Self {
		self.requires_dedicated_allocation = val;
		self
	}
}

impl std::default::Default for MemoryDedicatedRequirements {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Extent2D(pub u32, pub u32);

impl Extent2D {
	pub fn new() -> Self {
		Self(0, 0)
	}
}

impl std::default::Default for Extent2D {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorPoolCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DescriptorPoolCreateFlags,
	pub max_sets : u32,
	pub pool_size_count : u32,
	pub p_pool_sizes : *const DescriptorPoolSize,
}

impl DescriptorPoolCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 33,
			p_next : null(),
			flags : <_>::default(),
			max_sets : <_>::default(),
			pool_size_count : <_>::default(),
			p_pool_sizes : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DescriptorPoolCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn max_sets(mut self, val : u32) -> Self {
		self.max_sets = val;
		self
	}
	pub fn pool_sizes(mut self, val : &[DescriptorPoolSize]) -> Self {
		self.pool_size_count = val.len() as _;
		self.p_pool_sizes = val.as_ptr();
		self
	}
	pub fn pool_size(mut self, val : &DescriptorPoolSize) -> Self {
		self.pool_size_count = 1;
		self.p_pool_sizes = val;
		self
	}
}

impl std::default::Default for DescriptorPoolCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct IndirectCommandsLayoutTokenNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub token_type : IndirectCommandsTokenTypeNV,
	pub stream : u32,
	pub offset : u32,
	pub vertex_binding_unit : u32,
	pub vertex_dynamic_stride : Bool32,
	pub pushconstant_pipeline_layout : PipelineLayout,
	pub pushconstant_shader_stage_flags : ShaderStageFlags,
	pub pushconstant_offset : u32,
	pub pushconstant_size : u32,
	pub indirect_state_flags : IndirectStateFlagsNV,
	pub index_type_count : u32,
	pub p_index_types : *const IndexType,
	pub p_index_type_values : *const u32,
}

impl IndirectCommandsLayoutTokenNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000277003,
			p_next : null(),
			token_type : IndirectCommandsTokenTypeNV::default(),
			stream : <_>::default(),
			offset : <_>::default(),
			vertex_binding_unit : <_>::default(),
			vertex_dynamic_stride : <_>::default(),
			pushconstant_pipeline_layout : PipelineLayout(0),
			pushconstant_shader_stage_flags : <_>::default(),
			pushconstant_offset : <_>::default(),
			pushconstant_size : <_>::default(),
			indirect_state_flags : <_>::default(),
			index_type_count : <_>::default(),
			p_index_types : null(),
			p_index_type_values : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn token_type(mut self, val : IndirectCommandsTokenTypeNV) -> Self {
		self.token_type = val;
		self
	}
	pub fn stream(mut self, val : u32) -> Self {
		self.stream = val;
		self
	}
	pub fn offset(mut self, val : u32) -> Self {
		self.offset = val;
		self
	}
	pub fn vertex_binding_unit(mut self, val : u32) -> Self {
		self.vertex_binding_unit = val;
		self
	}
	pub fn vertex_dynamic_stride(mut self, val : Bool32) -> Self {
		self.vertex_dynamic_stride = val;
		self
	}
	pub fn pushconstant_pipeline_layout(mut self, val : PipelineLayout) -> Self {
		self.pushconstant_pipeline_layout = val;
		self
	}
	pub fn pushconstant_shader_stage_flags(mut self, val : ShaderStageFlags) -> Self {
		self.pushconstant_shader_stage_flags = val;
		self
	}
	pub fn pushconstant_offset(mut self, val : u32) -> Self {
		self.pushconstant_offset = val;
		self
	}
	pub fn pushconstant_size(mut self, val : u32) -> Self {
		self.pushconstant_size = val;
		self
	}
	pub fn indirect_state_flags(mut self, val : IndirectStateFlagsNV) -> Self {
		self.indirect_state_flags = val;
		self
	}
	pub fn index_types(mut self, val : &[IndexType]) -> Self {
		self.index_type_count = val.len() as _;
		self.p_index_types = val.as_ptr();
		self
	}
	pub fn index_type(mut self, val : &IndexType) -> Self {
		self.index_type_count = 1;
		self.p_index_types = val;
		self
	}
}

impl std::default::Default for IndirectCommandsLayoutTokenNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
	s_type : i32,
	pub p_next : *const c_void,
	pub overallocation_behavior : MemoryOverallocationBehaviorAMD,
}

impl DeviceMemoryOverallocationCreateInfoAMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000189000,
			p_next : null(),
			overallocation_behavior : MemoryOverallocationBehaviorAMD::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn overallocation_behavior(mut self, val : MemoryOverallocationBehaviorAMD) -> Self {
		self.overallocation_behavior = val;
		self
	}
}

impl std::default::Default for DeviceMemoryOverallocationCreateInfoAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DrmFormatModifierPropertiesEXT {
	pub drm_format_modifier : u64,
	pub drm_format_modifier_plane_count : u32,
	pub drm_format_modifier_tiling_features : FormatFeatureFlags,
}

impl DrmFormatModifierPropertiesEXT {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn drm_format_modifier(mut self, val : u64) -> Self {
		self.drm_format_modifier = val;
		self
	}
	pub fn drm_format_modifier_plane_count(mut self, val : u32) -> Self {
		self.drm_format_modifier_plane_count = val;
		self
	}
	pub fn drm_format_modifier_tiling_features(mut self, val : FormatFeatureFlags) -> Self {
		self.drm_format_modifier_tiling_features = val;
		self
	}
}

impl std::default::Default for DrmFormatModifierPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorSetLayoutSupport {
	s_type : i32,
	pub p_next : *mut c_void,
	pub supported : Bool32,
}

impl DescriptorSetLayoutSupport {
	pub fn new() -> Self {
		Self {
			s_type : 1000168001,
			p_next : null_mut(),
			supported : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn supported(mut self, val : Bool32) -> Self {
		self.supported = val;
		self
	}
}

impl std::default::Default for DescriptorSetLayoutSupport {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceVulkan11Features {
	s_type : i32,
	pub p_next : *mut c_void,
	pub storage_buffer_16_bit_access : Bool32,
	pub uniform_and_storage_buffer_16_bit_access : Bool32,
	pub storage_push_constant_16 : Bool32,
	pub storage_input_output_16 : Bool32,
	pub multiview : Bool32,
	pub multiview_geometry_shader : Bool32,
	pub multiview_tessellation_shader : Bool32,
	pub variable_pointers_storage_buffer : Bool32,
	pub variable_pointers : Bool32,
	pub protected_memory : Bool32,
	pub sampler_ycbcr_conversion : Bool32,
	pub shader_draw_parameters : Bool32,
}

impl PhysicalDeviceVulkan11Features {
	pub fn new() -> Self {
		Self {
			s_type : 49,
			p_next : null_mut(),
			storage_buffer_16_bit_access : <_>::default(),
			uniform_and_storage_buffer_16_bit_access : <_>::default(),
			storage_push_constant_16 : <_>::default(),
			storage_input_output_16 : <_>::default(),
			multiview : <_>::default(),
			multiview_geometry_shader : <_>::default(),
			multiview_tessellation_shader : <_>::default(),
			variable_pointers_storage_buffer : <_>::default(),
			variable_pointers : <_>::default(),
			protected_memory : <_>::default(),
			sampler_ycbcr_conversion : <_>::default(),
			shader_draw_parameters : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn storage_buffer_16_bit_access(mut self, val : Bool32) -> Self {
		self.storage_buffer_16_bit_access = val;
		self
	}
	pub fn uniform_and_storage_buffer_16_bit_access(mut self, val : Bool32) -> Self {
		self.uniform_and_storage_buffer_16_bit_access = val;
		self
	}
	pub fn storage_push_constant_16(mut self, val : Bool32) -> Self {
		self.storage_push_constant_16 = val;
		self
	}
	pub fn storage_input_output_16(mut self, val : Bool32) -> Self {
		self.storage_input_output_16 = val;
		self
	}
	pub fn multiview(mut self, val : Bool32) -> Self {
		self.multiview = val;
		self
	}
	pub fn multiview_geometry_shader(mut self, val : Bool32) -> Self {
		self.multiview_geometry_shader = val;
		self
	}
	pub fn multiview_tessellation_shader(mut self, val : Bool32) -> Self {
		self.multiview_tessellation_shader = val;
		self
	}
	pub fn variable_pointers_storage_buffer(mut self, val : Bool32) -> Self {
		self.variable_pointers_storage_buffer = val;
		self
	}
	pub fn variable_pointers(mut self, val : Bool32) -> Self {
		self.variable_pointers = val;
		self
	}
	pub fn protected_memory(mut self, val : Bool32) -> Self {
		self.protected_memory = val;
		self
	}
	pub fn sampler_ycbcr_conversion(mut self, val : Bool32) -> Self {
		self.sampler_ycbcr_conversion = val;
		self
	}
	pub fn shader_draw_parameters(mut self, val : Bool32) -> Self {
		self.shader_draw_parameters = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceVulkan11Features {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseImageFormatProperties {
	pub aspect_mask : ImageAspectFlags,
	pub image_granularity : Extent3D,
	pub flags : SparseImageFormatFlags,
}

impl SparseImageFormatProperties {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn aspect_mask(mut self, val : ImageAspectFlags) -> Self {
		self.aspect_mask = val;
		self
	}
	pub fn image_granularity(mut self, val : Extent3D) -> Self {
		self.image_granularity = val;
		self
	}
	pub fn flags(mut self, val : SparseImageFormatFlags) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for SparseImageFormatProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub opaque_capture_address : u64,
}

impl MemoryOpaqueCaptureAddressAllocateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000257003,
			p_next : null(),
			opaque_capture_address : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn opaque_capture_address(mut self, val : u64) -> Self {
		self.opaque_capture_address = val;
		self
	}
}

impl std::default::Default for MemoryOpaqueCaptureAddressAllocateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseImageMemoryRequirements2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub memory_requirements : SparseImageMemoryRequirements,
}

impl SparseImageMemoryRequirements2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000146004,
			p_next : null_mut(),
			memory_requirements : SparseImageMemoryRequirements::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory_requirements(mut self, val : SparseImageMemoryRequirements) -> Self {
		self.memory_requirements = val;
		self
	}
}

impl std::default::Default for SparseImageMemoryRequirements2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Extent3D {
	pub width : u32,
	pub height : u32,
	pub depth : u32,
}

impl Extent3D {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn width(mut self, val : u32) -> Self {
		self.width = val;
		self
	}
	pub fn height(mut self, val : u32) -> Self {
		self.height = val;
		self
	}
	pub fn depth(mut self, val : u32) -> Self {
		self.depth = val;
		self
	}
}

impl std::default::Default for Extent3D {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FramebufferMixedSamplesCombinationNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub coverage_reduction_mode : CoverageReductionModeNV,
	pub rasterization_samples : SampleCountFlags,
	pub depth_stencil_samples : SampleCountFlags,
	pub color_samples : SampleCountFlags,
}

impl FramebufferMixedSamplesCombinationNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000250002,
			p_next : null_mut(),
			coverage_reduction_mode : CoverageReductionModeNV::default(),
			rasterization_samples : SampleCountFlags::default(),
			depth_stencil_samples : <_>::default(),
			color_samples : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn coverage_reduction_mode(mut self, val : CoverageReductionModeNV) -> Self {
		self.coverage_reduction_mode = val;
		self
	}
	pub fn rasterization_samples(mut self, val : SampleCountFlags) -> Self {
		self.rasterization_samples = val;
		self
	}
	pub fn depth_stencil_samples(mut self, val : SampleCountFlags) -> Self {
		self.depth_stencil_samples = val;
		self
	}
	pub fn color_samples(mut self, val : SampleCountFlags) -> Self {
		self.color_samples = val;
		self
	}
}

impl std::default::Default for FramebufferMixedSamplesCombinationNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseImageOpaqueMemoryBindInfo {
	pub image : Image,
	pub bind_count : u32,
	pub p_binds : *const SparseMemoryBind,
}

impl SparseImageOpaqueMemoryBindInfo {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
	pub fn binds(mut self, val : &[SparseMemoryBind]) -> Self {
		self.bind_count = val.len() as _;
		self.p_binds = val.as_ptr();
		self
	}
	pub fn bind(mut self, val : &SparseMemoryBind) -> Self {
		self.bind_count = 1;
		self.p_binds = val;
		self
	}
}

impl std::default::Default for SparseImageOpaqueMemoryBindInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceCapabilities2EXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub min_image_count : u32,
	pub max_image_count : u32,
	pub current_extent : Extent2D,
	pub min_image_extent : Extent2D,
	pub max_image_extent : Extent2D,
	pub max_image_array_layers : u32,
	pub supported_transforms : SurfaceTransformFlagsKHR,
	pub current_transform : SurfaceTransformFlagsKHR,
	pub supported_composite_alpha : CompositeAlphaFlagsKHR,
	pub supported_usage_flags : ImageUsageFlags,
	pub supported_surface_counters : SurfaceCounterFlagsEXT,
}

impl SurfaceCapabilities2EXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000090000,
			p_next : null_mut(),
			min_image_count : <_>::default(),
			max_image_count : <_>::default(),
			current_extent : Extent2D::new(),
			min_image_extent : Extent2D::new(),
			max_image_extent : Extent2D::new(),
			max_image_array_layers : <_>::default(),
			supported_transforms : <_>::default(),
			current_transform : SurfaceTransformFlagsKHR::default(),
			supported_composite_alpha : <_>::default(),
			supported_usage_flags : <_>::default(),
			supported_surface_counters : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn min_image_count(mut self, val : u32) -> Self {
		self.min_image_count = val;
		self
	}
	pub fn max_image_count(mut self, val : u32) -> Self {
		self.max_image_count = val;
		self
	}
	pub fn current_extent(mut self, val : Extent2D) -> Self {
		self.current_extent = val;
		self
	}
	pub fn min_image_extent(mut self, val : Extent2D) -> Self {
		self.min_image_extent = val;
		self
	}
	pub fn max_image_extent(mut self, val : Extent2D) -> Self {
		self.max_image_extent = val;
		self
	}
	pub fn max_image_array_layers(mut self, val : u32) -> Self {
		self.max_image_array_layers = val;
		self
	}
	pub fn supported_transforms(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.supported_transforms = val;
		self
	}
	pub fn current_transform(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.current_transform = val;
		self
	}
	pub fn supported_composite_alpha(mut self, val : CompositeAlphaFlagsKHR) -> Self {
		self.supported_composite_alpha = val;
		self
	}
	pub fn supported_usage_flags(mut self, val : ImageUsageFlags) -> Self {
		self.supported_usage_flags = val;
		self
	}
	pub fn supported_surface_counters(mut self, val : SurfaceCounterFlagsEXT) -> Self {
		self.supported_surface_counters = val;
		self
	}
}

impl std::default::Default for SurfaceCapabilities2EXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageMemoryBarrier {
	s_type : i32,
	pub p_next : *const c_void,
	pub src_access_mask : AccessFlags,
	pub dst_access_mask : AccessFlags,
	pub old_layout : ImageLayout,
	pub new_layout : ImageLayout,
	pub src_queue_family_index : u32,
	pub dst_queue_family_index : u32,
	pub image : Image,
	pub subresource_range : ImageSubresourceRange,
}

impl ImageMemoryBarrier {
	pub fn new() -> Self {
		Self {
			s_type : 45,
			p_next : null(),
			src_access_mask : <_>::default(),
			dst_access_mask : <_>::default(),
			old_layout : ImageLayout::default(),
			new_layout : ImageLayout::default(),
			src_queue_family_index : <_>::default(),
			dst_queue_family_index : <_>::default(),
			image : Image(0),
			subresource_range : ImageSubresourceRange::new(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn src_access_mask(mut self, val : AccessFlags) -> Self {
		self.src_access_mask = val;
		self
	}
	pub fn dst_access_mask(mut self, val : AccessFlags) -> Self {
		self.dst_access_mask = val;
		self
	}
	pub fn old_layout(mut self, val : ImageLayout) -> Self {
		self.old_layout = val;
		self
	}
	pub fn new_layout(mut self, val : ImageLayout) -> Self {
		self.new_layout = val;
		self
	}
	pub fn src_queue_family_index(mut self, val : u32) -> Self {
		self.src_queue_family_index = val;
		self
	}
	pub fn dst_queue_family_index(mut self, val : u32) -> Self {
		self.dst_queue_family_index = val;
		self
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
	pub fn subresource_range(mut self, val : ImageSubresourceRange) -> Self {
		self.subresource_range = val;
		self
	}
}

impl std::default::Default for ImageMemoryBarrier {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceImageFormatInfo2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub format : Format,
	pub r#type : ImageType,
	pub tiling : ImageTiling,
	pub usage : ImageUsageFlags,
	pub flags : ImageCreateFlags,
}

impl PhysicalDeviceImageFormatInfo2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059004,
			p_next : null(),
			format : Format::default(),
			r#type : ImageType::default(),
			tiling : ImageTiling::default(),
			usage : <_>::default(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn r#type(mut self, val : ImageType) -> Self {
		self.r#type = val;
		self
	}
	pub fn tiling(mut self, val : ImageTiling) -> Self {
		self.tiling = val;
		self
	}
	pub fn usage(mut self, val : ImageUsageFlags) -> Self {
		self.usage = val;
		self
	}
	pub fn flags(mut self, val : ImageCreateFlags) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceImageFormatInfo2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub ycbcr_image_arrays : Bool32,
}

impl PhysicalDeviceYcbcrImageArraysFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000252000,
			p_next : null_mut(),
			ycbcr_image_arrays : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn ycbcr_image_arrays(mut self, val : Bool32) -> Self {
		self.ycbcr_image_arrays = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
	s_type : i32,
	pub p_next : *const c_void,
	pub rasterization_order : RasterizationOrderAMD,
}

impl PipelineRasterizationStateRasterizationOrderAMD {
	pub fn new() -> Self {
		Self {
			s_type : 1000018000,
			p_next : null(),
			rasterization_order : RasterizationOrderAMD::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn rasterization_order(mut self, val : RasterizationOrderAMD) -> Self {
		self.rasterization_order = val;
		self
	}
}

impl std::default::Default for PipelineRasterizationStateRasterizationOrderAMD {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SamplerYcbcrConversionInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub conversion : SamplerYcbcrConversion,
}

impl SamplerYcbcrConversionInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000156001,
			p_next : null(),
			conversion : SamplerYcbcrConversion(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn conversion(mut self, val : SamplerYcbcrConversion) -> Self {
		self.conversion = val;
		self
	}
}

impl std::default::Default for SamplerYcbcrConversionInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportFenceFdInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub fence : Fence,
	pub flags : FenceImportFlags,
	pub handle_type : ExternalFenceHandleTypeFlags,
	pub fd : i32,
}

impl ImportFenceFdInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000115000,
			p_next : null(),
			fence : Fence(0),
			flags : <_>::default(),
			handle_type : ExternalFenceHandleTypeFlags::default(),
			fd : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fence(mut self, val : Fence) -> Self {
		self.fence = val;
		self
	}
	pub fn flags(mut self, val : FenceImportFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalFenceHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
	pub fn fd(mut self, val : i32) -> Self {
		self.fd = val;
		self
	}
}

impl std::default::Default for ImportFenceFdInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearDepthStencilValue {
	pub depth : f32,
	pub stencil : u32,
}

impl ClearDepthStencilValue {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn depth(mut self, val : f32) -> Self {
		self.depth = val;
		self
	}
	pub fn stencil(mut self, val : u32) -> Self {
		self.stencil = val;
		self
	}
}

impl std::default::Default for ClearDepthStencilValue {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FenceCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : FenceCreateFlags,
}

impl FenceCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 8,
			p_next : null(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : FenceCreateFlags) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for FenceCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PresentInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub wait_semaphore_count : u32,
	pub p_wait_semaphores : *const Semaphore,
	pub swapchain_count : u32,
	pub p_swapchains : *const SwapchainKHR,
	pub p_image_indices : *const u32,
	pub p_results : *mut VkResult,
}

impl PresentInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000001001,
			p_next : null(),
			wait_semaphore_count : <_>::default(),
			p_wait_semaphores : null(),
			swapchain_count : <_>::default(),
			p_swapchains : null(),
			p_image_indices : null(),
			p_results : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image_indices(mut self, val : *const u32) -> Self {
		self.p_image_indices = val;
		self
	}
	pub fn results(mut self, val : *mut VkResult) -> Self {
		self.p_results = val;
		self
	}
	pub fn wait_semaphores(mut self, val : &[Semaphore]) -> Self {
		self.wait_semaphore_count = val.len() as _;
		self.p_wait_semaphores = val.as_ptr();
		self
	}
	pub fn wait_semaphore(mut self, val : &Semaphore) -> Self {
		self.wait_semaphore_count = 1;
		self.p_wait_semaphores = val;
		self
	}
	pub fn swapchains(mut self, val : &[SwapchainKHR]) -> Self {
		self.swapchain_count = val.len() as _;
		self.p_swapchains = val.as_ptr();
		self
	}
	pub fn swapchain(mut self, val : &SwapchainKHR) -> Self {
		self.swapchain_count = 1;
		self.p_swapchains = val;
		self
	}
}

impl std::default::Default for PresentInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : RenderPassCreateFlags,
	pub attachment_count : u32,
	pub p_attachments : *const AttachmentDescription,
	pub subpass_count : u32,
	pub p_subpasses : *const SubpassDescription,
	pub dependency_count : u32,
	pub p_dependencies : *const SubpassDependency,
}

impl RenderPassCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 38,
			p_next : null(),
			flags : <_>::default(),
			attachment_count : <_>::default(),
			p_attachments : null(),
			subpass_count : <_>::default(),
			p_subpasses : null(),
			dependency_count : <_>::default(),
			p_dependencies : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : RenderPassCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn attachments(mut self, val : &[AttachmentDescription]) -> Self {
		self.attachment_count = val.len() as _;
		self.p_attachments = val.as_ptr();
		self
	}
	pub fn attachment(mut self, val : &AttachmentDescription) -> Self {
		self.attachment_count = 1;
		self.p_attachments = val;
		self
	}
	pub fn subpasses(mut self, val : &[SubpassDescription]) -> Self {
		self.subpass_count = val.len() as _;
		self.p_subpasses = val.as_ptr();
		self
	}
	pub fn subpasse(mut self, val : &SubpassDescription) -> Self {
		self.subpass_count = 1;
		self.p_subpasses = val;
		self
	}
	pub fn dependencies(mut self, val : &[SubpassDependency]) -> Self {
		self.dependency_count = val.len() as _;
		self.p_dependencies = val.as_ptr();
		self
	}
	pub fn dependency(mut self, val : &SubpassDependency) -> Self {
		self.dependency_count = 1;
		self.p_dependencies = val;
		self
	}
}

impl std::default::Default for RenderPassCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceGroupRenderPassBeginInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub device_mask : u32,
	pub device_render_area_count : u32,
	pub p_device_render_areas : *const Rect2D,
}

impl DeviceGroupRenderPassBeginInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000060003,
			p_next : null(),
			device_mask : <_>::default(),
			device_render_area_count : <_>::default(),
			p_device_render_areas : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_mask(mut self, val : u32) -> Self {
		self.device_mask = val;
		self
	}
	pub fn device_render_areas(mut self, val : &[Rect2D]) -> Self {
		self.device_render_area_count = val.len() as _;
		self.p_device_render_areas = val.as_ptr();
		self
	}
	pub fn device_render_area(mut self, val : &Rect2D) -> Self {
		self.device_render_area_count = 1;
		self.p_device_render_areas = val;
		self
	}
}

impl std::default::Default for DeviceGroupRenderPassBeginInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseImageMemoryRequirements {
	pub format_properties : SparseImageFormatProperties,
	pub image_mip_tail_first_lod : u32,
	pub image_mip_tail_size : DeviceSize,
	pub image_mip_tail_offset : DeviceSize,
	pub image_mip_tail_stride : DeviceSize,
}

impl SparseImageMemoryRequirements {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn format_properties(mut self, val : SparseImageFormatProperties) -> Self {
		self.format_properties = val;
		self
	}
	pub fn image_mip_tail_first_lod(mut self, val : u32) -> Self {
		self.image_mip_tail_first_lod = val;
		self
	}
	pub fn image_mip_tail_size(mut self, val : DeviceSize) -> Self {
		self.image_mip_tail_size = val;
		self
	}
	pub fn image_mip_tail_offset(mut self, val : DeviceSize) -> Self {
		self.image_mip_tail_offset = val;
		self
	}
	pub fn image_mip_tail_stride(mut self, val : DeviceSize) -> Self {
		self.image_mip_tail_stride = val;
		self
	}
}

impl std::default::Default for SparseImageMemoryRequirements {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryGetWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub memory : DeviceMemory,
	pub handle_type : ExternalMemoryHandleTypeFlags,
}

impl MemoryGetWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000073003,
			p_next : null(),
			memory : DeviceMemory(0),
			handle_type : ExternalMemoryHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for MemoryGetWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MappedMemoryRange {
	s_type : i32,
	pub p_next : *const c_void,
	pub memory : DeviceMemory,
	pub offset : DeviceSize,
	pub size : DeviceSize,
}

impl MappedMemoryRange {
	pub fn new() -> Self {
		Self {
			s_type : 6,
			p_next : null(),
			memory : DeviceMemory(0),
			offset : <_>::default(),
			size : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
	pub fn offset(mut self, val : DeviceSize) -> Self {
		self.offset = val;
		self
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
}

impl std::default::Default for MappedMemoryRange {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GraphicsPipelineCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineCreateFlags,
	pub stage_count : u32,
	pub p_stages : *const PipelineShaderStageCreateInfo,
	pub p_vertex_input_state : *const PipelineVertexInputStateCreateInfo,
	pub p_input_assembly_state : *const PipelineInputAssemblyStateCreateInfo,
	pub p_tessellation_state : *const PipelineTessellationStateCreateInfo,
	pub p_viewport_state : *const PipelineViewportStateCreateInfo,
	pub p_rasterization_state : *const PipelineRasterizationStateCreateInfo,
	pub p_multisample_state : *const PipelineMultisampleStateCreateInfo,
	pub p_depth_stencil_state : *const PipelineDepthStencilStateCreateInfo,
	pub p_color_blend_state : *const PipelineColorBlendStateCreateInfo,
	pub p_dynamic_state : *const PipelineDynamicStateCreateInfo,
	pub layout : PipelineLayout,
	pub render_pass : RenderPass,
	pub subpass : u32,
	pub base_pipeline_handle : Pipeline,
	pub base_pipeline_index : i32,
}

impl GraphicsPipelineCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 28,
			p_next : null(),
			flags : <_>::default(),
			stage_count : <_>::default(),
			p_stages : null(),
			p_vertex_input_state : null(),
			p_input_assembly_state : null(),
			p_tessellation_state : null(),
			p_viewport_state : null(),
			p_rasterization_state : null(),
			p_multisample_state : null(),
			p_depth_stencil_state : null(),
			p_color_blend_state : null(),
			p_dynamic_state : null(),
			layout : PipelineLayout(0),
			render_pass : RenderPass(0),
			subpass : <_>::default(),
			base_pipeline_handle : Pipeline(0),
			base_pipeline_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn vertex_input_state(mut self, val : *const PipelineVertexInputStateCreateInfo) -> Self {
		self.p_vertex_input_state = val;
		self
	}
	pub fn input_assembly_state(mut self, val : *const PipelineInputAssemblyStateCreateInfo) -> Self {
		self.p_input_assembly_state = val;
		self
	}
	pub fn tessellation_state(mut self, val : *const PipelineTessellationStateCreateInfo) -> Self {
		self.p_tessellation_state = val;
		self
	}
	pub fn viewport_state(mut self, val : *const PipelineViewportStateCreateInfo) -> Self {
		self.p_viewport_state = val;
		self
	}
	pub fn rasterization_state(mut self, val : *const PipelineRasterizationStateCreateInfo) -> Self {
		self.p_rasterization_state = val;
		self
	}
	pub fn multisample_state(mut self, val : *const PipelineMultisampleStateCreateInfo) -> Self {
		self.p_multisample_state = val;
		self
	}
	pub fn depth_stencil_state(mut self, val : *const PipelineDepthStencilStateCreateInfo) -> Self {
		self.p_depth_stencil_state = val;
		self
	}
	pub fn color_blend_state(mut self, val : *const PipelineColorBlendStateCreateInfo) -> Self {
		self.p_color_blend_state = val;
		self
	}
	pub fn dynamic_state(mut self, val : *const PipelineDynamicStateCreateInfo) -> Self {
		self.p_dynamic_state = val;
		self
	}
	pub fn layout(mut self, val : PipelineLayout) -> Self {
		self.layout = val;
		self
	}
	pub fn render_pass(mut self, val : RenderPass) -> Self {
		self.render_pass = val;
		self
	}
	pub fn subpass(mut self, val : u32) -> Self {
		self.subpass = val;
		self
	}
	pub fn base_pipeline_handle(mut self, val : Pipeline) -> Self {
		self.base_pipeline_handle = val;
		self
	}
	pub fn base_pipeline_index(mut self, val : i32) -> Self {
		self.base_pipeline_index = val;
		self
	}
	pub fn stages(mut self, val : &[PipelineShaderStageCreateInfo]) -> Self {
		self.stage_count = val.len() as _;
		self.p_stages = val.as_ptr();
		self
	}
	pub fn stage(mut self, val : &PipelineShaderStageCreateInfo) -> Self {
		self.stage_count = 1;
		self.p_stages = val;
		self
	}
}

impl std::default::Default for GraphicsPipelineCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DedicatedAllocationBufferCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub dedicated_allocation : Bool32,
}

impl DedicatedAllocationBufferCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000026001,
			p_next : null(),
			dedicated_allocation : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn dedicated_allocation(mut self, val : Bool32) -> Self {
		self.dedicated_allocation = val;
		self
	}
}

impl std::default::Default for DedicatedAllocationBufferCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub buffer_device_address : Bool32,
	pub buffer_device_address_capture_replay : Bool32,
	pub buffer_device_address_multi_device : Bool32,
}

impl PhysicalDeviceBufferDeviceAddressFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000257000,
			p_next : null_mut(),
			buffer_device_address : <_>::default(),
			buffer_device_address_capture_replay : <_>::default(),
			buffer_device_address_multi_device : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn buffer_device_address(mut self, val : Bool32) -> Self {
		self.buffer_device_address = val;
		self
	}
	pub fn buffer_device_address_capture_replay(mut self, val : Bool32) -> Self {
		self.buffer_device_address_capture_replay = val;
		self
	}
	pub fn buffer_device_address_multi_device(mut self, val : Bool32) -> Self {
		self.buffer_device_address_multi_device = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceBufferDeviceAddressFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryAllocateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub allocation_size : DeviceSize,
	pub memory_type_index : u32,
}

impl MemoryAllocateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 5,
			p_next : null(),
			allocation_size : <_>::default(),
			memory_type_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn allocation_size(mut self, val : DeviceSize) -> Self {
		self.allocation_size = val;
		self
	}
	pub fn memory_type_index(mut self, val : u32) -> Self {
		self.memory_type_index = val;
		self
	}
}

impl std::default::Default for MemoryAllocateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryRequirements {
	pub size : DeviceSize,
	pub alignment : DeviceSize,
	pub memory_type_bits : u32,
}

impl MemoryRequirements {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
	pub fn alignment(mut self, val : DeviceSize) -> Self {
		self.alignment = val;
		self
	}
	pub fn memory_type_bits(mut self, val : u32) -> Self {
		self.memory_type_bits = val;
		self
	}
}

impl std::default::Default for MemoryRequirements {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FramebufferCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : FramebufferCreateFlags,
	pub render_pass : RenderPass,
	pub attachment_count : u32,
	pub p_attachments : *const ImageView,
	pub width : u32,
	pub height : u32,
	pub layers : u32,
}

impl FramebufferCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 37,
			p_next : null(),
			flags : <_>::default(),
			render_pass : RenderPass(0),
			attachment_count : <_>::default(),
			p_attachments : null(),
			width : <_>::default(),
			height : <_>::default(),
			layers : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : FramebufferCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn render_pass(mut self, val : RenderPass) -> Self {
		self.render_pass = val;
		self
	}
	pub fn width(mut self, val : u32) -> Self {
		self.width = val;
		self
	}
	pub fn height(mut self, val : u32) -> Self {
		self.height = val;
		self
	}
	pub fn layers(mut self, val : u32) -> Self {
		self.layers = val;
		self
	}
	pub fn attachments(mut self, val : &[ImageView]) -> Self {
		self.attachment_count = val.len() as _;
		self.p_attachments = val.as_ptr();
		self
	}
	pub fn attachment(mut self, val : &ImageView) -> Self {
		self.attachment_count = 1;
		self.p_attachments = val;
		self
	}
}

impl std::default::Default for FramebufferCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_variable_descriptor_count : u32,
}

impl DescriptorSetVariableDescriptorCountLayoutSupport {
	pub fn new() -> Self {
		Self {
			s_type : 1000161004,
			p_next : null_mut(),
			max_variable_descriptor_count : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_variable_descriptor_count(mut self, val : u32) -> Self {
		self.max_variable_descriptor_count = val;
		self
	}
}

impl std::default::Default for DescriptorSetVariableDescriptorCountLayoutSupport {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GeometryTrianglesNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub vertex_data : Buffer,
	pub vertex_offset : DeviceSize,
	pub vertex_count : u32,
	pub vertex_stride : DeviceSize,
	pub vertex_format : Format,
	pub index_data : Buffer,
	pub index_offset : DeviceSize,
	pub index_count : u32,
	pub index_type : IndexType,
	pub transform_data : Buffer,
	pub transform_offset : DeviceSize,
}

impl GeometryTrianglesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165004,
			p_next : null(),
			vertex_data : Buffer(0),
			vertex_offset : <_>::default(),
			vertex_count : <_>::default(),
			vertex_stride : <_>::default(),
			vertex_format : Format::default(),
			index_data : Buffer(0),
			index_offset : <_>::default(),
			index_count : <_>::default(),
			index_type : IndexType::default(),
			transform_data : Buffer(0),
			transform_offset : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn vertex_data(mut self, val : Buffer) -> Self {
		self.vertex_data = val;
		self
	}
	pub fn vertex_offset(mut self, val : DeviceSize) -> Self {
		self.vertex_offset = val;
		self
	}
	pub fn vertex_count(mut self, val : u32) -> Self {
		self.vertex_count = val;
		self
	}
	pub fn vertex_stride(mut self, val : DeviceSize) -> Self {
		self.vertex_stride = val;
		self
	}
	pub fn vertex_format(mut self, val : Format) -> Self {
		self.vertex_format = val;
		self
	}
	pub fn index_data(mut self, val : Buffer) -> Self {
		self.index_data = val;
		self
	}
	pub fn index_offset(mut self, val : DeviceSize) -> Self {
		self.index_offset = val;
		self
	}
	pub fn index_count(mut self, val : u32) -> Self {
		self.index_count = val;
		self
	}
	pub fn index_type(mut self, val : IndexType) -> Self {
		self.index_type = val;
		self
	}
	pub fn transform_data(mut self, val : Buffer) -> Self {
		self.transform_data = val;
		self
	}
	pub fn transform_offset(mut self, val : DeviceSize) -> Self {
		self.transform_offset = val;
		self
	}
}

impl std::default::Default for GeometryTrianglesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RectLayerKHR {
	pub offset : Offset2D,
	pub extent : Extent2D,
	pub layer : u32,
}

impl RectLayerKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn offset(mut self, val : Offset2D) -> Self {
		self.offset = val;
		self
	}
	pub fn extent(mut self, val : Extent2D) -> Self {
		self.extent = val;
		self
	}
	pub fn layer(mut self, val : u32) -> Self {
		self.layer = val;
		self
	}
}

impl std::default::Default for RectLayerKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineCoverageReductionStateCreateFlagsNV,
	pub coverage_reduction_mode : CoverageReductionModeNV,
}

impl PipelineCoverageReductionStateCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000250001,
			p_next : null(),
			flags : <_>::default(),
			coverage_reduction_mode : CoverageReductionModeNV::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineCoverageReductionStateCreateFlagsNV) -> Self {
		self.flags = val;
		self
	}
	pub fn coverage_reduction_mode(mut self, val : CoverageReductionModeNV) -> Self {
		self.coverage_reduction_mode = val;
		self
	}
}

impl std::default::Default for PipelineCoverageReductionStateCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct InstanceCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : InstanceCreateFlags,
	pub p_application_info : *const ApplicationInfo,
	pub enabled_layer_count : u32,
	pub pp_enabled_layer_names : *const *const u8,
	pub enabled_extension_count : u32,
	pub pp_enabled_extension_names : *const *const u8,
}

impl InstanceCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1,
			p_next : null(),
			flags : <_>::default(),
			p_application_info : null(),
			enabled_layer_count : <_>::default(),
			pp_enabled_layer_names : null(),
			enabled_extension_count : <_>::default(),
			pp_enabled_extension_names : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : InstanceCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn application_info(mut self, val : *const ApplicationInfo) -> Self {
		self.p_application_info = val;
		self
	}
	pub fn enabled_layer_names(mut self, val : &[*const u8]) -> Self {
		self.enabled_layer_count = val.len() as _;
		self.pp_enabled_layer_names = val.as_ptr();
		self
	}
	pub fn enabled_layer_name(mut self, val : &*const u8) -> Self {
		self.enabled_layer_count = 1;
		self.pp_enabled_layer_names = val;
		self
	}
	pub fn enabled_extension_names(mut self, val : &[*const u8]) -> Self {
		self.enabled_extension_count = val.len() as _;
		self.pp_enabled_extension_names = val.as_ptr();
		self
	}
	pub fn enabled_extension_name(mut self, val : &*const u8) -> Self {
		self.enabled_extension_count = 1;
		self.pp_enabled_extension_names = val;
		self
	}
}

impl std::default::Default for InstanceCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseImageFormatProperties2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub properties : SparseImageFormatProperties,
}

impl SparseImageFormatProperties2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059007,
			p_next : null_mut(),
			properties : SparseImageFormatProperties::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn properties(mut self, val : SparseImageFormatProperties) -> Self {
		self.properties = val;
		self
	}
}

impl std::default::Default for SparseImageFormatProperties2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub memory_priority : Bool32,
}

impl PhysicalDeviceMemoryPriorityFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000238000,
			p_next : null_mut(),
			memory_priority : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory_priority(mut self, val : Bool32) -> Self {
		self.memory_priority = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMemoryPriorityFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PresentRegionKHR {
	pub rectangle_count : u32,
	pub p_rectangles : *const RectLayerKHR,
}

impl PresentRegionKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn rectangles(mut self, val : &[RectLayerKHR]) -> Self {
		self.rectangle_count = val.len() as _;
		self.p_rectangles = val.as_ptr();
		self
	}
	pub fn rectangle(mut self, val : &RectLayerKHR) -> Self {
		self.rectangle_count = 1;
		self.p_rectangles = val;
		self
	}
}

impl std::default::Default for PresentRegionKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubmitInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub wait_semaphore_count : u32,
	pub p_wait_semaphores : *const Semaphore,
	pub p_wait_dst_stage_mask : *const PipelineStageFlags,
	pub command_buffer_count : u32,
	pub p_command_buffers : *const CommandBuffer,
	pub signal_semaphore_count : u32,
	pub p_signal_semaphores : *const Semaphore,
}

impl SubmitInfo {
	pub fn new() -> Self {
		Self {
			s_type : 4,
			p_next : null(),
			wait_semaphore_count : <_>::default(),
			p_wait_semaphores : null(),
			p_wait_dst_stage_mask : null(),
			command_buffer_count : <_>::default(),
			p_command_buffers : null(),
			signal_semaphore_count : <_>::default(),
			p_signal_semaphores : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn wait_dst_stage_mask(mut self, val : *const PipelineStageFlags) -> Self {
		self.p_wait_dst_stage_mask = val;
		self
	}
	pub fn wait_semaphores(mut self, val : &[Semaphore]) -> Self {
		self.wait_semaphore_count = val.len() as _;
		self.p_wait_semaphores = val.as_ptr();
		self
	}
	pub fn wait_semaphore(mut self, val : &Semaphore) -> Self {
		self.wait_semaphore_count = 1;
		self.p_wait_semaphores = val;
		self
	}
	pub fn command_buffers(mut self, val : &[CommandBuffer]) -> Self {
		self.command_buffer_count = val.len() as _;
		self.p_command_buffers = val.as_ptr();
		self
	}
	pub fn command_buffer(mut self, val : &CommandBuffer) -> Self {
		self.command_buffer_count = 1;
		self.p_command_buffers = val;
		self
	}
	pub fn signal_semaphores(mut self, val : &[Semaphore]) -> Self {
		self.signal_semaphore_count = val.len() as _;
		self.p_signal_semaphores = val.as_ptr();
		self
	}
	pub fn signal_semaphore(mut self, val : &Semaphore) -> Self {
		self.signal_semaphore_count = 1;
		self.p_signal_semaphores = val;
		self
	}
}

impl std::default::Default for SubmitInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct TransformMatrixKHR {
	pub matrix : [f32; 3],
}

impl TransformMatrixKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
}

impl std::default::Default for TransformMatrixKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub format : Format,
	pub r#type : ImageType,
	pub samples : SampleCountFlags,
	pub usage : ImageUsageFlags,
	pub tiling : ImageTiling,
}

impl PhysicalDeviceSparseImageFormatInfo2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059008,
			p_next : null(),
			format : Format::default(),
			r#type : ImageType::default(),
			samples : SampleCountFlags::default(),
			usage : <_>::default(),
			tiling : ImageTiling::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn r#type(mut self, val : ImageType) -> Self {
		self.r#type = val;
		self
	}
	pub fn samples(mut self, val : SampleCountFlags) -> Self {
		self.samples = val;
		self
	}
	pub fn usage(mut self, val : ImageUsageFlags) -> Self {
		self.usage = val;
		self
	}
	pub fn tiling(mut self, val : ImageTiling) -> Self {
		self.tiling = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSparseImageFormatInfo2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SemaphoreSignalInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub semaphore : Semaphore,
	pub value : u64,
}

impl SemaphoreSignalInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000207005,
			p_next : null(),
			semaphore : Semaphore(0),
			value : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn semaphore(mut self, val : Semaphore) -> Self {
		self.semaphore = val;
		self
	}
	pub fn value(mut self, val : u64) -> Self {
		self.value = val;
		self
	}
}

impl std::default::Default for SemaphoreSignalInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseImageMemoryBind {
	pub subresource : ImageSubresource,
	pub offset : Offset3D,
	pub extent : Extent3D,
	pub memory : DeviceMemory,
	pub memory_offset : DeviceSize,
	pub flags : SparseMemoryBindFlags,
}

impl SparseImageMemoryBind {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn subresource(mut self, val : ImageSubresource) -> Self {
		self.subresource = val;
		self
	}
	pub fn offset(mut self, val : Offset3D) -> Self {
		self.offset = val;
		self
	}
	pub fn extent(mut self, val : Extent3D) -> Self {
		self.extent = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
	pub fn memory_offset(mut self, val : DeviceSize) -> Self {
		self.memory_offset = val;
		self
	}
	pub fn flags(mut self, val : SparseMemoryBindFlags) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for SparseImageMemoryBind {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineMultisampleStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineMultisampleStateCreateFlags,
	pub rasterization_samples : SampleCountFlags,
	pub sample_shading_enable : Bool32,
	pub min_sample_shading : f32,
	pub p_sample_mask : *const SampleMask,
	pub alpha_to_coverage_enable : Bool32,
	pub alpha_to_one_enable : Bool32,
}

impl PipelineMultisampleStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 24,
			p_next : null(),
			flags : <_>::default(),
			rasterization_samples : SampleCountFlags::default(),
			sample_shading_enable : <_>::default(),
			min_sample_shading : <_>::default(),
			p_sample_mask : null(),
			alpha_to_coverage_enable : <_>::default(),
			alpha_to_one_enable : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineMultisampleStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn rasterization_samples(mut self, val : SampleCountFlags) -> Self {
		self.rasterization_samples = val;
		self
	}
	pub fn sample_shading_enable(mut self, val : Bool32) -> Self {
		self.sample_shading_enable = val;
		self
	}
	pub fn min_sample_shading(mut self, val : f32) -> Self {
		self.min_sample_shading = val;
		self
	}
	pub fn sample_mask(mut self, val : *const SampleMask) -> Self {
		self.p_sample_mask = val;
		self
	}
	pub fn alpha_to_coverage_enable(mut self, val : Bool32) -> Self {
		self.alpha_to_coverage_enable = val;
		self
	}
	pub fn alpha_to_one_enable(mut self, val : Bool32) -> Self {
		self.alpha_to_one_enable = val;
		self
	}
}

impl std::default::Default for PipelineMultisampleStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSubgroupProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub subgroup_size : u32,
	pub supported_stages : ShaderStageFlags,
	pub supported_operations : SubgroupFeatureFlags,
	pub quad_operations_in_all_stages : Bool32,
}

impl PhysicalDeviceSubgroupProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000094000,
			p_next : null_mut(),
			subgroup_size : <_>::default(),
			supported_stages : <_>::default(),
			supported_operations : <_>::default(),
			quad_operations_in_all_stages : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn subgroup_size(mut self, val : u32) -> Self {
		self.subgroup_size = val;
		self
	}
	pub fn supported_stages(mut self, val : ShaderStageFlags) -> Self {
		self.supported_stages = val;
		self
	}
	pub fn supported_operations(mut self, val : SubgroupFeatureFlags) -> Self {
		self.supported_operations = val;
		self
	}
	pub fn quad_operations_in_all_stages(mut self, val : Bool32) -> Self {
		self.quad_operations_in_all_stages = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSubgroupProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseImageMemoryBindInfo {
	pub image : Image,
	pub bind_count : u32,
	pub p_binds : *const SparseImageMemoryBind,
}

impl SparseImageMemoryBindInfo {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
	pub fn binds(mut self, val : &[SparseImageMemoryBind]) -> Self {
		self.bind_count = val.len() as _;
		self.p_binds = val.as_ptr();
		self
	}
	pub fn bind(mut self, val : &SparseImageMemoryBind) -> Self {
		self.bind_count = 1;
		self.p_binds = val;
		self
	}
}

impl std::default::Default for SparseImageMemoryBindInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineDynamicStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineDynamicStateCreateFlags,
	pub dynamic_state_count : u32,
	pub p_dynamic_states : *const DynamicState,
}

impl PipelineDynamicStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 27,
			p_next : null(),
			flags : <_>::default(),
			dynamic_state_count : <_>::default(),
			p_dynamic_states : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineDynamicStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn dynamic_states(mut self, val : &[DynamicState]) -> Self {
		self.dynamic_state_count = val.len() as _;
		self.p_dynamic_states = val.as_ptr();
		self
	}
	pub fn dynamic_state(mut self, val : &DynamicState) -> Self {
		self.dynamic_state_count = 1;
		self.p_dynamic_states = val;
		self
	}
}

impl std::default::Default for PipelineDynamicStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineShaderStageCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineShaderStageCreateFlags,
	pub stage : ShaderStageFlags,
	pub module : ShaderModule,
	pub p_name : *const u8,
	pub p_specialization_info : *const SpecializationInfo,
}

impl PipelineShaderStageCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 18,
			p_next : null(),
			flags : <_>::default(),
			stage : ShaderStageFlags::default(),
			module : ShaderModule(0),
			p_name : null(),
			p_specialization_info : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineShaderStageCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn stage(mut self, val : ShaderStageFlags) -> Self {
		self.stage = val;
		self
	}
	pub fn module(mut self, val : ShaderModule) -> Self {
		self.module = val;
		self
	}
	pub fn name(mut self, val : *const u8) -> Self {
		self.p_name = val;
		self
	}
	pub fn specialization_info(mut self, val : *const SpecializationInfo) -> Self {
		self.p_specialization_info = val;
		self
	}
}

impl std::default::Default for PipelineShaderStageCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayPowerInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub power_state : DisplayPowerStateEXT,
}

impl DisplayPowerInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000091000,
			p_next : null(),
			power_state : DisplayPowerStateEXT::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn power_state(mut self, val : DisplayPowerStateEXT) -> Self {
		self.power_state = val;
		self
	}
}

impl std::default::Default for DisplayPowerInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorImageInfo {
	pub sampler : Sampler,
	pub image_view : ImageView,
	pub image_layout : ImageLayout,
}

impl DescriptorImageInfo {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn sampler(mut self, val : Sampler) -> Self {
		self.sampler = val;
		self
	}
	pub fn image_view(mut self, val : ImageView) -> Self {
		self.image_view = val;
		self
	}
	pub fn image_layout(mut self, val : ImageLayout) -> Self {
		self.image_layout = val;
		self
	}
}

impl std::default::Default for DescriptorImageInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Offset3D {
	pub x : i32,
	pub y : i32,
	pub z : i32,
}

impl Offset3D {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn x(mut self, val : i32) -> Self {
		self.x = val;
		self
	}
	pub fn y(mut self, val : i32) -> Self {
		self.y = val;
		self
	}
	pub fn z(mut self, val : i32) -> Self {
		self.z = val;
		self
	}
}

impl std::default::Default for Offset3D {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineDiscardRectangleStateCreateFlagsEXT,
	pub discard_rectangle_mode : DiscardRectangleModeEXT,
	pub discard_rectangle_count : u32,
	pub p_discard_rectangles : *const Rect2D,
}

impl PipelineDiscardRectangleStateCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000099001,
			p_next : null(),
			flags : <_>::default(),
			discard_rectangle_mode : DiscardRectangleModeEXT::default(),
			discard_rectangle_count : <_>::default(),
			p_discard_rectangles : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineDiscardRectangleStateCreateFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn discard_rectangle_mode(mut self, val : DiscardRectangleModeEXT) -> Self {
		self.discard_rectangle_mode = val;
		self
	}
	pub fn discard_rectangles(mut self, val : &[Rect2D]) -> Self {
		self.discard_rectangle_count = val.len() as _;
		self.p_discard_rectangles = val.as_ptr();
		self
	}
	pub fn discard_rectangle(mut self, val : &Rect2D) -> Self {
		self.discard_rectangle_count = 1;
		self.p_discard_rectangles = val;
		self
	}
}

impl std::default::Default for PipelineDiscardRectangleStateCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImagePlaneMemoryRequirementsInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub plane_aspect : ImageAspectFlags,
}

impl ImagePlaneMemoryRequirementsInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000156003,
			p_next : null(),
			plane_aspect : ImageAspectFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn plane_aspect(mut self, val : ImageAspectFlags) -> Self {
		self.plane_aspect = val;
		self
	}
}

impl std::default::Default for ImagePlaneMemoryRequirementsInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AttachmentDescription2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : AttachmentDescriptionFlags,
	pub format : Format,
	pub samples : SampleCountFlags,
	pub load_op : AttachmentLoadOp,
	pub store_op : AttachmentStoreOp,
	pub stencil_load_op : AttachmentLoadOp,
	pub stencil_store_op : AttachmentStoreOp,
	pub initial_layout : ImageLayout,
	pub final_layout : ImageLayout,
}

impl AttachmentDescription2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000109000,
			p_next : null(),
			flags : <_>::default(),
			format : Format::default(),
			samples : SampleCountFlags::default(),
			load_op : AttachmentLoadOp::default(),
			store_op : AttachmentStoreOp::default(),
			stencil_load_op : AttachmentLoadOp::default(),
			stencil_store_op : AttachmentStoreOp::default(),
			initial_layout : ImageLayout::default(),
			final_layout : ImageLayout::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : AttachmentDescriptionFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn samples(mut self, val : SampleCountFlags) -> Self {
		self.samples = val;
		self
	}
	pub fn load_op(mut self, val : AttachmentLoadOp) -> Self {
		self.load_op = val;
		self
	}
	pub fn store_op(mut self, val : AttachmentStoreOp) -> Self {
		self.store_op = val;
		self
	}
	pub fn stencil_load_op(mut self, val : AttachmentLoadOp) -> Self {
		self.stencil_load_op = val;
		self
	}
	pub fn stencil_store_op(mut self, val : AttachmentStoreOp) -> Self {
		self.stencil_store_op = val;
		self
	}
	pub fn initial_layout(mut self, val : ImageLayout) -> Self {
		self.initial_layout = val;
		self
	}
	pub fn final_layout(mut self, val : ImageLayout) -> Self {
		self.final_layout = val;
		self
	}
}

impl std::default::Default for AttachmentDescription2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryPriorityAllocateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub priority : f32,
}

impl MemoryPriorityAllocateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000238001,
			p_next : null(),
			priority : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn priority(mut self, val : f32) -> Self {
		self.priority = val;
		self
	}
}

impl std::default::Default for MemoryPriorityAllocateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindSparseInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub wait_semaphore_count : u32,
	pub p_wait_semaphores : *const Semaphore,
	pub buffer_bind_count : u32,
	pub p_buffer_binds : *const SparseBufferMemoryBindInfo,
	pub image_opaque_bind_count : u32,
	pub p_image_opaque_binds : *const SparseImageOpaqueMemoryBindInfo,
	pub image_bind_count : u32,
	pub p_image_binds : *const SparseImageMemoryBindInfo,
	pub signal_semaphore_count : u32,
	pub p_signal_semaphores : *const Semaphore,
}

impl BindSparseInfo {
	pub fn new() -> Self {
		Self {
			s_type : 7,
			p_next : null(),
			wait_semaphore_count : <_>::default(),
			p_wait_semaphores : null(),
			buffer_bind_count : <_>::default(),
			p_buffer_binds : null(),
			image_opaque_bind_count : <_>::default(),
			p_image_opaque_binds : null(),
			image_bind_count : <_>::default(),
			p_image_binds : null(),
			signal_semaphore_count : <_>::default(),
			p_signal_semaphores : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn wait_semaphores(mut self, val : &[Semaphore]) -> Self {
		self.wait_semaphore_count = val.len() as _;
		self.p_wait_semaphores = val.as_ptr();
		self
	}
	pub fn wait_semaphore(mut self, val : &Semaphore) -> Self {
		self.wait_semaphore_count = 1;
		self.p_wait_semaphores = val;
		self
	}
	pub fn buffer_binds(mut self, val : &[SparseBufferMemoryBindInfo]) -> Self {
		self.buffer_bind_count = val.len() as _;
		self.p_buffer_binds = val.as_ptr();
		self
	}
	pub fn buffer_bind(mut self, val : &SparseBufferMemoryBindInfo) -> Self {
		self.buffer_bind_count = 1;
		self.p_buffer_binds = val;
		self
	}
	pub fn image_opaque_binds(mut self, val : &[SparseImageOpaqueMemoryBindInfo]) -> Self {
		self.image_opaque_bind_count = val.len() as _;
		self.p_image_opaque_binds = val.as_ptr();
		self
	}
	pub fn image_opaque_bind(mut self, val : &SparseImageOpaqueMemoryBindInfo) -> Self {
		self.image_opaque_bind_count = 1;
		self.p_image_opaque_binds = val;
		self
	}
	pub fn image_binds(mut self, val : &[SparseImageMemoryBindInfo]) -> Self {
		self.image_bind_count = val.len() as _;
		self.p_image_binds = val.as_ptr();
		self
	}
	pub fn image_bind(mut self, val : &SparseImageMemoryBindInfo) -> Self {
		self.image_bind_count = 1;
		self.p_image_binds = val;
		self
	}
	pub fn signal_semaphores(mut self, val : &[Semaphore]) -> Self {
		self.signal_semaphore_count = val.len() as _;
		self.p_signal_semaphores = val.as_ptr();
		self
	}
	pub fn signal_semaphore(mut self, val : &Semaphore) -> Self {
		self.signal_semaphore_count = 1;
		self.p_signal_semaphores = val;
		self
	}
}

impl std::default::Default for BindSparseInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub group_count : u32,
	pub p_groups : *const GraphicsShaderGroupCreateInfoNV,
	pub pipeline_count : u32,
	pub p_pipelines : *const Pipeline,
}

impl GraphicsPipelineShaderGroupsCreateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000277002,
			p_next : null(),
			group_count : <_>::default(),
			p_groups : null(),
			pipeline_count : <_>::default(),
			p_pipelines : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn groups(mut self, val : &[GraphicsShaderGroupCreateInfoNV]) -> Self {
		self.group_count = val.len() as _;
		self.p_groups = val.as_ptr();
		self
	}
	pub fn group(mut self, val : &GraphicsShaderGroupCreateInfoNV) -> Self {
		self.group_count = 1;
		self.p_groups = val;
		self
	}
	pub fn pipelines(mut self, val : &[Pipeline]) -> Self {
		self.pipeline_count = val.len() as _;
		self.p_pipelines = val.as_ptr();
		self
	}
	pub fn pipeline(mut self, val : &Pipeline) -> Self {
		self.pipeline_count = 1;
		self.p_pipelines = val;
		self
	}
}

impl std::default::Default for GraphicsPipelineShaderGroupsCreateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AttachmentReference2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub attachment : u32,
	pub layout : ImageLayout,
	pub aspect_mask : ImageAspectFlags,
}

impl AttachmentReference2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000109001,
			p_next : null(),
			attachment : <_>::default(),
			layout : ImageLayout::default(),
			aspect_mask : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn attachment(mut self, val : u32) -> Self {
		self.attachment = val;
		self
	}
	pub fn layout(mut self, val : ImageLayout) -> Self {
		self.layout = val;
		self
	}
	pub fn aspect_mask(mut self, val : ImageAspectFlags) -> Self {
		self.aspect_mask = val;
		self
	}
}

impl std::default::Default for AttachmentReference2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceGroupPresentInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub swapchain_count : u32,
	pub p_device_masks : *const u32,
	pub mode : DeviceGroupPresentModeFlagsKHR,
}

impl DeviceGroupPresentInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000060011,
			p_next : null(),
			swapchain_count : <_>::default(),
			p_device_masks : null(),
			mode : DeviceGroupPresentModeFlagsKHR::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_masks(mut self, val : &[u32]) -> Self {
		self.swapchain_count = val.len() as _;
		self.p_device_masks = val.as_ptr();
		self
	}
	pub fn device_mask(mut self, val : &u32) -> Self {
		self.swapchain_count = 1;
		self.p_device_masks = val;
		self
	}
}

impl std::default::Default for DeviceGroupPresentInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub exclusive_scissor : Bool32,
}

impl PhysicalDeviceExclusiveScissorFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000205002,
			p_next : null_mut(),
			exclusive_scissor : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn exclusive_scissor(mut self, val : Bool32) -> Self {
		self.exclusive_scissor = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceExclusiveScissorFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayModePropertiesKHR {
	pub display_mode : DisplayModeKHR,
	pub parameters : DisplayModeParametersKHR,
}

impl DisplayModePropertiesKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn display_mode(mut self, val : DisplayModeKHR) -> Self {
		self.display_mode = val;
		self
	}
	pub fn parameters(mut self, val : DisplayModeParametersKHR) -> Self {
		self.parameters = val;
		self
	}
}

impl std::default::Default for DisplayModePropertiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayPresentInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub src_rect : Rect2D,
	pub dst_rect : Rect2D,
	pub persistent : Bool32,
}

impl DisplayPresentInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000003000,
			p_next : null(),
			src_rect : Rect2D::new(),
			dst_rect : Rect2D::new(),
			persistent : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn src_rect(mut self, val : Rect2D) -> Self {
		self.src_rect = val;
		self
	}
	pub fn dst_rect(mut self, val : Rect2D) -> Self {
		self.dst_rect = val;
		self
	}
	pub fn persistent(mut self, val : Bool32) -> Self {
		self.persistent = val;
		self
	}
}

impl std::default::Default for DisplayPresentInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceFormat2KHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub surface_format : SurfaceFormatKHR,
}

impl SurfaceFormat2KHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000119002,
			p_next : null_mut(),
			surface_format : SurfaceFormatKHR::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn surface_format(mut self, val : SurfaceFormatKHR) -> Self {
		self.surface_format = val;
		self
	}
}

impl std::default::Default for SurfaceFormat2KHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorUpdateTemplateEntry {
	pub dst_binding : u32,
	pub dst_array_element : u32,
	pub descriptor_count : u32,
	pub descriptor_type : DescriptorType,
	pub offset : usize,
	pub stride : usize,
}

impl DescriptorUpdateTemplateEntry {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn dst_binding(mut self, val : u32) -> Self {
		self.dst_binding = val;
		self
	}
	pub fn dst_array_element(mut self, val : u32) -> Self {
		self.dst_array_element = val;
		self
	}
	pub fn descriptor_count(mut self, val : u32) -> Self {
		self.descriptor_count = val;
		self
	}
	pub fn descriptor_type(mut self, val : DescriptorType) -> Self {
		self.descriptor_type = val;
		self
	}
	pub fn offset(mut self, val : usize) -> Self {
		self.offset = val;
		self
	}
	pub fn stride(mut self, val : usize) -> Self {
		self.stride = val;
		self
	}
}

impl std::default::Default for DescriptorUpdateTemplateEntry {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct VertexInputBindingDescription {
	pub binding : u32,
	pub stride : u32,
	pub input_rate : VertexInputRate,
}

impl VertexInputBindingDescription {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn binding(mut self, val : u32) -> Self {
		self.binding = val;
		self
	}
	pub fn stride(mut self, val : u32) -> Self {
		self.stride = val;
		self
	}
	pub fn input_rate(mut self, val : VertexInputRate) -> Self {
		self.input_rate = val;
		self
	}
}

impl std::default::Default for VertexInputBindingDescription {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub surface : SurfaceKHR,
}

impl PhysicalDeviceSurfaceInfo2KHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000119000,
			p_next : null(),
			surface : SurfaceKHR(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn surface(mut self, val : SurfaceKHR) -> Self {
		self.surface = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSurfaceInfo2KHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferViewCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : BufferViewCreateFlags,
	pub buffer : Buffer,
	pub format : Format,
	pub offset : DeviceSize,
	pub range : DeviceSize,
}

impl BufferViewCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 13,
			p_next : null(),
			flags : <_>::default(),
			buffer : Buffer(0),
			format : Format::default(),
			offset : <_>::default(),
			range : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : BufferViewCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn offset(mut self, val : DeviceSize) -> Self {
		self.offset = val;
		self
	}
	pub fn range(mut self, val : DeviceSize) -> Self {
		self.range = val;
		self
	}
}

impl std::default::Default for BufferViewCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SemaphoreCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : SemaphoreCreateFlags,
}

impl SemaphoreCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 9,
			p_next : null(),
			flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : SemaphoreCreateFlags) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for SemaphoreCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct WriteDescriptorSet {
	s_type : i32,
	pub p_next : *const c_void,
	pub dst_set : DescriptorSet,
	pub dst_binding : u32,
	pub dst_array_element : u32,
	pub descriptor_count : u32,
	pub descriptor_type : DescriptorType,
	pub p_image_info : *const DescriptorImageInfo,
	pub p_buffer_info : *const DescriptorBufferInfo,
	pub p_texel_buffer_view : *const BufferView,
}

impl WriteDescriptorSet {
	pub fn new() -> Self {
		Self {
			s_type : 35,
			p_next : null(),
			dst_set : DescriptorSet(0),
			dst_binding : <_>::default(),
			dst_array_element : <_>::default(),
			descriptor_count : <_>::default(),
			descriptor_type : DescriptorType::default(),
			p_image_info : null(),
			p_buffer_info : null(),
			p_texel_buffer_view : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn dst_set(mut self, val : DescriptorSet) -> Self {
		self.dst_set = val;
		self
	}
	pub fn dst_binding(mut self, val : u32) -> Self {
		self.dst_binding = val;
		self
	}
	pub fn dst_array_element(mut self, val : u32) -> Self {
		self.dst_array_element = val;
		self
	}
	pub fn descriptor_count(mut self, val : u32) -> Self {
		self.descriptor_count = val;
		self
	}
	pub fn descriptor_type(mut self, val : DescriptorType) -> Self {
		self.descriptor_type = val;
		self
	}
	pub fn image_info(mut self, val : *const DescriptorImageInfo) -> Self {
		self.p_image_info = val;
		self
	}
	pub fn buffer_info(mut self, val : *const DescriptorBufferInfo) -> Self {
		self.p_buffer_info = val;
		self
	}
	pub fn texel_buffer_view(mut self, val : *const BufferView) -> Self {
		self.p_texel_buffer_view = val;
		self
	}
}

impl std::default::Default for WriteDescriptorSet {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct TimelineSemaphoreSubmitInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub wait_semaphore_value_count : u32,
	pub p_wait_semaphore_values : *const u64,
	pub signal_semaphore_value_count : u32,
	pub p_signal_semaphore_values : *const u64,
}

impl TimelineSemaphoreSubmitInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000207003,
			p_next : null(),
			wait_semaphore_value_count : <_>::default(),
			p_wait_semaphore_values : null(),
			signal_semaphore_value_count : <_>::default(),
			p_signal_semaphore_values : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn wait_semaphore_values(mut self, val : &[u64]) -> Self {
		self.wait_semaphore_value_count = val.len() as _;
		self.p_wait_semaphore_values = val.as_ptr();
		self
	}
	pub fn wait_semaphore_value(mut self, val : &u64) -> Self {
		self.wait_semaphore_value_count = 1;
		self.p_wait_semaphore_values = val;
		self
	}
	pub fn signal_semaphore_values(mut self, val : &[u64]) -> Self {
		self.signal_semaphore_value_count = val.len() as _;
		self.p_signal_semaphore_values = val.as_ptr();
		self
	}
	pub fn signal_semaphore_value(mut self, val : &u64) -> Self {
		self.signal_semaphore_value_count = 1;
		self.p_signal_semaphore_values = val;
		self
	}
}

impl std::default::Default for TimelineSemaphoreSubmitInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearColorValue {
	pub float_32 : [f32; 4],
	pub int_32 : [i32; 4],
	pub uint_32 : [u32; 4],
}

impl ClearColorValue {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn float_32(mut self, val : [f32; 4]) -> Self {
		self.float_32 = val;
		self
	}
	pub fn int_32(mut self, val : [i32; 4]) -> Self {
		self.int_32 = val;
		self
	}
	pub fn uint_32(mut self, val : [u32; 4]) -> Self {
		self.uint_32 = val;
		self
	}
}

impl std::default::Default for ClearColorValue {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub advanced_blend_max_color_attachments : u32,
	pub advanced_blend_independent_blend : Bool32,
	pub advanced_blend_non_premultiplied_src_color : Bool32,
	pub advanced_blend_non_premultiplied_dst_color : Bool32,
	pub advanced_blend_correlated_overlap : Bool32,
	pub advanced_blend_all_operations : Bool32,
}

impl PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000148001,
			p_next : null_mut(),
			advanced_blend_max_color_attachments : <_>::default(),
			advanced_blend_independent_blend : <_>::default(),
			advanced_blend_non_premultiplied_src_color : <_>::default(),
			advanced_blend_non_premultiplied_dst_color : <_>::default(),
			advanced_blend_correlated_overlap : <_>::default(),
			advanced_blend_all_operations : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn advanced_blend_max_color_attachments(mut self, val : u32) -> Self {
		self.advanced_blend_max_color_attachments = val;
		self
	}
	pub fn advanced_blend_independent_blend(mut self, val : Bool32) -> Self {
		self.advanced_blend_independent_blend = val;
		self
	}
	pub fn advanced_blend_non_premultiplied_src_color(mut self, val : Bool32) -> Self {
		self.advanced_blend_non_premultiplied_src_color = val;
		self
	}
	pub fn advanced_blend_non_premultiplied_dst_color(mut self, val : Bool32) -> Self {
		self.advanced_blend_non_premultiplied_dst_color = val;
		self
	}
	pub fn advanced_blend_correlated_overlap(mut self, val : Bool32) -> Self {
		self.advanced_blend_correlated_overlap = val;
		self
	}
	pub fn advanced_blend_all_operations(mut self, val : Bool32) -> Self {
		self.advanced_blend_all_operations = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageFormatListCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub view_format_count : u32,
	pub p_view_formats : *const Format,
}

impl ImageFormatListCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000147000,
			p_next : null(),
			view_format_count : <_>::default(),
			p_view_formats : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn view_formats(mut self, val : &[Format]) -> Self {
		self.view_format_count = val.len() as _;
		self.p_view_formats = val.as_ptr();
		self
	}
	pub fn view_format(mut self, val : &Format) -> Self {
		self.view_format_count = 1;
		self.p_view_formats = val;
		self
	}
}

impl std::default::Default for ImageFormatListCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct InitializePerformanceApiInfoINTEL {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_user_data : *mut c_void,
}

impl InitializePerformanceApiInfoINTEL {
	pub fn new() -> Self {
		Self {
			s_type : 1000210001,
			p_next : null(),
			p_user_data : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn user_data(mut self, val : *mut c_void) -> Self {
		self.p_user_data = val;
		self
	}
}

impl std::default::Default for InitializePerformanceApiInfoINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CommandPoolCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : CommandPoolCreateFlags,
	pub queue_family_index : u32,
}

impl CommandPoolCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 39,
			p_next : null(),
			flags : <_>::default(),
			queue_family_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : CommandPoolCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn queue_family_index(mut self, val : u32) -> Self {
		self.queue_family_index = val;
		self
	}
}

impl std::default::Default for CommandPoolCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub advanced_blend_coherent_operations : Bool32,
}

impl PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000148000,
			p_next : null_mut(),
			advanced_blend_coherent_operations : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn advanced_blend_coherent_operations(mut self, val : Bool32) -> Self {
		self.advanced_blend_coherent_operations = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFeatures {
	pub robust_buffer_access : Bool32,
	pub full_draw_index_uint_32 : Bool32,
	pub image_cube_array : Bool32,
	pub independent_blend : Bool32,
	pub geometry_shader : Bool32,
	pub tessellation_shader : Bool32,
	pub sample_rate_shading : Bool32,
	pub dual_src_blend : Bool32,
	pub logic_op : Bool32,
	pub multi_draw_indirect : Bool32,
	pub draw_indirect_first_instance : Bool32,
	pub depth_clamp : Bool32,
	pub depth_bias_clamp : Bool32,
	pub fill_mode_non_solid : Bool32,
	pub depth_bounds : Bool32,
	pub wide_lines : Bool32,
	pub large_points : Bool32,
	pub alpha_to_one : Bool32,
	pub multi_viewport : Bool32,
	pub sampler_anisotropy : Bool32,
	pub texture_compression_etc_2 : Bool32,
	pub texture_compression_astc__ldr : Bool32,
	pub texture_compression_bc : Bool32,
	pub occlusion_query_precise : Bool32,
	pub pipeline_statistics_query : Bool32,
	pub vertex_pipeline_stores_and_atomics : Bool32,
	pub fragment_stores_and_atomics : Bool32,
	pub shader_tessellation_and_geometry_point_size : Bool32,
	pub shader_image_gather_extended : Bool32,
	pub shader_storage_image_extended_formats : Bool32,
	pub shader_storage_image_multisample : Bool32,
	pub shader_storage_image_read_without_format : Bool32,
	pub shader_storage_image_write_without_format : Bool32,
	pub shader_uniform_buffer_array_dynamic_indexing : Bool32,
	pub shader_sampled_image_array_dynamic_indexing : Bool32,
	pub shader_storage_buffer_array_dynamic_indexing : Bool32,
	pub shader_storage_image_array_dynamic_indexing : Bool32,
	pub shader_clip_distance : Bool32,
	pub shader_cull_distance : Bool32,
	pub shader_float_64 : Bool32,
	pub shader_int_64 : Bool32,
	pub shader_int_16 : Bool32,
	pub shader_resource_residency : Bool32,
	pub shader_resource_min_lod : Bool32,
	pub sparse_binding : Bool32,
	pub sparse_residency_buffer : Bool32,
	pub sparse_residency_image_2_d : Bool32,
	pub sparse_residency_image_3_d : Bool32,
	pub sparse_residency_2_samples : Bool32,
	pub sparse_residency_4_samples : Bool32,
	pub sparse_residency_8_samples : Bool32,
	pub sparse_residency_16_samples : Bool32,
	pub sparse_residency_aliased : Bool32,
	pub variable_multisample_rate : Bool32,
	pub inherited_queries : Bool32,
}

impl PhysicalDeviceFeatures {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn robust_buffer_access(mut self, val : Bool32) -> Self {
		self.robust_buffer_access = val;
		self
	}
	pub fn full_draw_index_uint_32(mut self, val : Bool32) -> Self {
		self.full_draw_index_uint_32 = val;
		self
	}
	pub fn image_cube_array(mut self, val : Bool32) -> Self {
		self.image_cube_array = val;
		self
	}
	pub fn independent_blend(mut self, val : Bool32) -> Self {
		self.independent_blend = val;
		self
	}
	pub fn geometry_shader(mut self, val : Bool32) -> Self {
		self.geometry_shader = val;
		self
	}
	pub fn tessellation_shader(mut self, val : Bool32) -> Self {
		self.tessellation_shader = val;
		self
	}
	pub fn sample_rate_shading(mut self, val : Bool32) -> Self {
		self.sample_rate_shading = val;
		self
	}
	pub fn dual_src_blend(mut self, val : Bool32) -> Self {
		self.dual_src_blend = val;
		self
	}
	pub fn logic_op(mut self, val : Bool32) -> Self {
		self.logic_op = val;
		self
	}
	pub fn multi_draw_indirect(mut self, val : Bool32) -> Self {
		self.multi_draw_indirect = val;
		self
	}
	pub fn draw_indirect_first_instance(mut self, val : Bool32) -> Self {
		self.draw_indirect_first_instance = val;
		self
	}
	pub fn depth_clamp(mut self, val : Bool32) -> Self {
		self.depth_clamp = val;
		self
	}
	pub fn depth_bias_clamp(mut self, val : Bool32) -> Self {
		self.depth_bias_clamp = val;
		self
	}
	pub fn fill_mode_non_solid(mut self, val : Bool32) -> Self {
		self.fill_mode_non_solid = val;
		self
	}
	pub fn depth_bounds(mut self, val : Bool32) -> Self {
		self.depth_bounds = val;
		self
	}
	pub fn wide_lines(mut self, val : Bool32) -> Self {
		self.wide_lines = val;
		self
	}
	pub fn large_points(mut self, val : Bool32) -> Self {
		self.large_points = val;
		self
	}
	pub fn alpha_to_one(mut self, val : Bool32) -> Self {
		self.alpha_to_one = val;
		self
	}
	pub fn multi_viewport(mut self, val : Bool32) -> Self {
		self.multi_viewport = val;
		self
	}
	pub fn sampler_anisotropy(mut self, val : Bool32) -> Self {
		self.sampler_anisotropy = val;
		self
	}
	pub fn texture_compression_etc_2(mut self, val : Bool32) -> Self {
		self.texture_compression_etc_2 = val;
		self
	}
	pub fn texture_compression_astc__ldr(mut self, val : Bool32) -> Self {
		self.texture_compression_astc__ldr = val;
		self
	}
	pub fn texture_compression_bc(mut self, val : Bool32) -> Self {
		self.texture_compression_bc = val;
		self
	}
	pub fn occlusion_query_precise(mut self, val : Bool32) -> Self {
		self.occlusion_query_precise = val;
		self
	}
	pub fn pipeline_statistics_query(mut self, val : Bool32) -> Self {
		self.pipeline_statistics_query = val;
		self
	}
	pub fn vertex_pipeline_stores_and_atomics(mut self, val : Bool32) -> Self {
		self.vertex_pipeline_stores_and_atomics = val;
		self
	}
	pub fn fragment_stores_and_atomics(mut self, val : Bool32) -> Self {
		self.fragment_stores_and_atomics = val;
		self
	}
	pub fn shader_tessellation_and_geometry_point_size(mut self, val : Bool32) -> Self {
		self.shader_tessellation_and_geometry_point_size = val;
		self
	}
	pub fn shader_image_gather_extended(mut self, val : Bool32) -> Self {
		self.shader_image_gather_extended = val;
		self
	}
	pub fn shader_storage_image_extended_formats(mut self, val : Bool32) -> Self {
		self.shader_storage_image_extended_formats = val;
		self
	}
	pub fn shader_storage_image_multisample(mut self, val : Bool32) -> Self {
		self.shader_storage_image_multisample = val;
		self
	}
	pub fn shader_storage_image_read_without_format(mut self, val : Bool32) -> Self {
		self.shader_storage_image_read_without_format = val;
		self
	}
	pub fn shader_storage_image_write_without_format(mut self, val : Bool32) -> Self {
		self.shader_storage_image_write_without_format = val;
		self
	}
	pub fn shader_uniform_buffer_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_uniform_buffer_array_dynamic_indexing = val;
		self
	}
	pub fn shader_sampled_image_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_sampled_image_array_dynamic_indexing = val;
		self
	}
	pub fn shader_storage_buffer_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_buffer_array_dynamic_indexing = val;
		self
	}
	pub fn shader_storage_image_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_image_array_dynamic_indexing = val;
		self
	}
	pub fn shader_clip_distance(mut self, val : Bool32) -> Self {
		self.shader_clip_distance = val;
		self
	}
	pub fn shader_cull_distance(mut self, val : Bool32) -> Self {
		self.shader_cull_distance = val;
		self
	}
	pub fn shader_float_64(mut self, val : Bool32) -> Self {
		self.shader_float_64 = val;
		self
	}
	pub fn shader_int_64(mut self, val : Bool32) -> Self {
		self.shader_int_64 = val;
		self
	}
	pub fn shader_int_16(mut self, val : Bool32) -> Self {
		self.shader_int_16 = val;
		self
	}
	pub fn shader_resource_residency(mut self, val : Bool32) -> Self {
		self.shader_resource_residency = val;
		self
	}
	pub fn shader_resource_min_lod(mut self, val : Bool32) -> Self {
		self.shader_resource_min_lod = val;
		self
	}
	pub fn sparse_binding(mut self, val : Bool32) -> Self {
		self.sparse_binding = val;
		self
	}
	pub fn sparse_residency_buffer(mut self, val : Bool32) -> Self {
		self.sparse_residency_buffer = val;
		self
	}
	pub fn sparse_residency_image_2_d(mut self, val : Bool32) -> Self {
		self.sparse_residency_image_2_d = val;
		self
	}
	pub fn sparse_residency_image_3_d(mut self, val : Bool32) -> Self {
		self.sparse_residency_image_3_d = val;
		self
	}
	pub fn sparse_residency_2_samples(mut self, val : Bool32) -> Self {
		self.sparse_residency_2_samples = val;
		self
	}
	pub fn sparse_residency_4_samples(mut self, val : Bool32) -> Self {
		self.sparse_residency_4_samples = val;
		self
	}
	pub fn sparse_residency_8_samples(mut self, val : Bool32) -> Self {
		self.sparse_residency_8_samples = val;
		self
	}
	pub fn sparse_residency_16_samples(mut self, val : Bool32) -> Self {
		self.sparse_residency_16_samples = val;
		self
	}
	pub fn sparse_residency_aliased(mut self, val : Bool32) -> Self {
		self.sparse_residency_aliased = val;
		self
	}
	pub fn variable_multisample_rate(mut self, val : Bool32) -> Self {
		self.variable_multisample_rate = val;
		self
	}
	pub fn inherited_queries(mut self, val : Bool32) -> Self {
		self.inherited_queries = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SpecializationMapEntry {
	pub constant_id : u32,
	pub offset : u32,
	pub size : usize,
}

impl SpecializationMapEntry {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn constant_id(mut self, val : u32) -> Self {
		self.constant_id = val;
		self
	}
	pub fn offset(mut self, val : u32) -> Self {
		self.offset = val;
		self
	}
	pub fn size(mut self, val : usize) -> Self {
		self.size = val;
		self
	}
}

impl std::default::Default for SpecializationMapEntry {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExportSemaphoreCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_types : ExternalSemaphoreHandleTypeFlags,
}

impl ExportSemaphoreCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000077000,
			p_next : null(),
			handle_types : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_types(mut self, val : ExternalSemaphoreHandleTypeFlags) -> Self {
		self.handle_types = val;
		self
	}
}

impl std::default::Default for ExportSemaphoreCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineRasterizationStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineRasterizationStateCreateFlags,
	pub depth_clamp_enable : Bool32,
	pub rasterizer_discard_enable : Bool32,
	pub polygon_mode : PolygonMode,
	pub cull_mode : CullModeFlags,
	pub front_face : FrontFace,
	pub depth_bias_enable : Bool32,
	pub depth_bias_constant_factor : f32,
	pub depth_bias_clamp : f32,
	pub depth_bias_slope_factor : f32,
	pub line_width : f32,
}

impl PipelineRasterizationStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 23,
			p_next : null(),
			flags : <_>::default(),
			depth_clamp_enable : <_>::default(),
			rasterizer_discard_enable : <_>::default(),
			polygon_mode : PolygonMode::default(),
			cull_mode : <_>::default(),
			front_face : FrontFace::default(),
			depth_bias_enable : <_>::default(),
			depth_bias_constant_factor : <_>::default(),
			depth_bias_clamp : <_>::default(),
			depth_bias_slope_factor : <_>::default(),
			line_width : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineRasterizationStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn depth_clamp_enable(mut self, val : Bool32) -> Self {
		self.depth_clamp_enable = val;
		self
	}
	pub fn rasterizer_discard_enable(mut self, val : Bool32) -> Self {
		self.rasterizer_discard_enable = val;
		self
	}
	pub fn polygon_mode(mut self, val : PolygonMode) -> Self {
		self.polygon_mode = val;
		self
	}
	pub fn cull_mode(mut self, val : CullModeFlags) -> Self {
		self.cull_mode = val;
		self
	}
	pub fn front_face(mut self, val : FrontFace) -> Self {
		self.front_face = val;
		self
	}
	pub fn depth_bias_enable(mut self, val : Bool32) -> Self {
		self.depth_bias_enable = val;
		self
	}
	pub fn depth_bias_constant_factor(mut self, val : f32) -> Self {
		self.depth_bias_constant_factor = val;
		self
	}
	pub fn depth_bias_clamp(mut self, val : f32) -> Self {
		self.depth_bias_clamp = val;
		self
	}
	pub fn depth_bias_slope_factor(mut self, val : f32) -> Self {
		self.depth_bias_slope_factor = val;
		self
	}
	pub fn line_width(mut self, val : f32) -> Self {
		self.line_width = val;
		self
	}
}

impl std::default::Default for PipelineRasterizationStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseBufferMemoryBindInfo {
	pub buffer : Buffer,
	pub bind_count : u32,
	pub p_binds : *const SparseMemoryBind,
}

impl SparseBufferMemoryBindInfo {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
	pub fn binds(mut self, val : &[SparseMemoryBind]) -> Self {
		self.bind_count = val.len() as _;
		self.p_binds = val.as_ptr();
		self
	}
	pub fn bind(mut self, val : &SparseMemoryBind) -> Self {
		self.bind_count = 1;
		self.p_binds = val;
		self
	}
}

impl std::default::Default for SparseBufferMemoryBindInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ComponentMapping {
	pub r : ComponentSwizzle,
	pub g : ComponentSwizzle,
	pub b : ComponentSwizzle,
	pub a : ComponentSwizzle,
}

impl ComponentMapping {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn r(mut self, val : ComponentSwizzle) -> Self {
		self.r = val;
		self
	}
	pub fn g(mut self, val : ComponentSwizzle) -> Self {
		self.g = val;
		self
	}
	pub fn b(mut self, val : ComponentSwizzle) -> Self {
		self.b = val;
		self
	}
	pub fn a(mut self, val : ComponentSwizzle) -> Self {
		self.a = val;
		self
	}
}

impl std::default::Default for ComponentMapping {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageViewCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : ImageViewCreateFlags,
	pub image : Image,
	pub view_type : ImageViewType,
	pub format : Format,
	pub components : ComponentMapping,
	pub subresource_range : ImageSubresourceRange,
}

impl ImageViewCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 15,
			p_next : null(),
			flags : <_>::default(),
			image : Image(0),
			view_type : ImageViewType::default(),
			format : Format::default(),
			components : ComponentMapping::new(),
			subresource_range : ImageSubresourceRange::new(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : ImageViewCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
	pub fn view_type(mut self, val : ImageViewType) -> Self {
		self.view_type = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn components(mut self, val : ComponentMapping) -> Self {
		self.components = val;
		self
	}
	pub fn subresource_range(mut self, val : ImageSubresourceRange) -> Self {
		self.subresource_range = val;
		self
	}
}

impl std::default::Default for ImageViewCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ShaderModuleCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : ShaderModuleCreateFlags,
	pub code_size : usize,
	pub p_code : *const u32,
}

impl ShaderModuleCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 16,
			p_next : null(),
			flags : <_>::default(),
			code_size : <_>::default(),
			p_code : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : ShaderModuleCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn code_size(mut self, val : usize) -> Self {
		self.code_size = val;
		self
	}
	pub fn code(mut self, val : *const u32) -> Self {
		self.p_code = val;
		self
	}
}

impl std::default::Default for ShaderModuleCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferMemoryRequirementsInfo2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub buffer : Buffer,
}

impl BufferMemoryRequirementsInfo2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000146000,
			p_next : null(),
			buffer : Buffer(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
}

impl std::default::Default for BufferMemoryRequirementsInfo2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_update_after_bind_descriptors_in_all_pools : u32,
	pub shader_uniform_buffer_array_non_uniform_indexing_native : Bool32,
	pub shader_sampled_image_array_non_uniform_indexing_native : Bool32,
	pub shader_storage_buffer_array_non_uniform_indexing_native : Bool32,
	pub shader_storage_image_array_non_uniform_indexing_native : Bool32,
	pub shader_input_attachment_array_non_uniform_indexing_native : Bool32,
	pub robust_buffer_access_update_after_bind : Bool32,
	pub quad_divergent_implicit_lod : Bool32,
	pub max_per_stage_descriptor_update_after_bind_samplers : u32,
	pub max_per_stage_descriptor_update_after_bind_uniform_buffers : u32,
	pub max_per_stage_descriptor_update_after_bind_storage_buffers : u32,
	pub max_per_stage_descriptor_update_after_bind_sampled_images : u32,
	pub max_per_stage_descriptor_update_after_bind_storage_images : u32,
	pub max_per_stage_descriptor_update_after_bind_input_attachments : u32,
	pub max_per_stage_update_after_bind_resources : u32,
	pub max_descriptor_set_update_after_bind_samplers : u32,
	pub max_descriptor_set_update_after_bind_uniform_buffers : u32,
	pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic : u32,
	pub max_descriptor_set_update_after_bind_storage_buffers : u32,
	pub max_descriptor_set_update_after_bind_storage_buffers_dynamic : u32,
	pub max_descriptor_set_update_after_bind_sampled_images : u32,
	pub max_descriptor_set_update_after_bind_storage_images : u32,
	pub max_descriptor_set_update_after_bind_input_attachments : u32,
}

impl PhysicalDeviceDescriptorIndexingProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000161002,
			p_next : null_mut(),
			max_update_after_bind_descriptors_in_all_pools : <_>::default(),
			shader_uniform_buffer_array_non_uniform_indexing_native : <_>::default(),
			shader_sampled_image_array_non_uniform_indexing_native : <_>::default(),
			shader_storage_buffer_array_non_uniform_indexing_native : <_>::default(),
			shader_storage_image_array_non_uniform_indexing_native : <_>::default(),
			shader_input_attachment_array_non_uniform_indexing_native : <_>::default(),
			robust_buffer_access_update_after_bind : <_>::default(),
			quad_divergent_implicit_lod : <_>::default(),
			max_per_stage_descriptor_update_after_bind_samplers : <_>::default(),
			max_per_stage_descriptor_update_after_bind_uniform_buffers : <_>::default(),
			max_per_stage_descriptor_update_after_bind_storage_buffers : <_>::default(),
			max_per_stage_descriptor_update_after_bind_sampled_images : <_>::default(),
			max_per_stage_descriptor_update_after_bind_storage_images : <_>::default(),
			max_per_stage_descriptor_update_after_bind_input_attachments : <_>::default(),
			max_per_stage_update_after_bind_resources : <_>::default(),
			max_descriptor_set_update_after_bind_samplers : <_>::default(),
			max_descriptor_set_update_after_bind_uniform_buffers : <_>::default(),
			max_descriptor_set_update_after_bind_uniform_buffers_dynamic : <_>::default(),
			max_descriptor_set_update_after_bind_storage_buffers : <_>::default(),
			max_descriptor_set_update_after_bind_storage_buffers_dynamic : <_>::default(),
			max_descriptor_set_update_after_bind_sampled_images : <_>::default(),
			max_descriptor_set_update_after_bind_storage_images : <_>::default(),
			max_descriptor_set_update_after_bind_input_attachments : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_update_after_bind_descriptors_in_all_pools(mut self, val : u32) -> Self {
		self.max_update_after_bind_descriptors_in_all_pools = val;
		self
	}
	pub fn shader_uniform_buffer_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_uniform_buffer_array_non_uniform_indexing_native = val;
		self
	}
	pub fn shader_sampled_image_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_sampled_image_array_non_uniform_indexing_native = val;
		self
	}
	pub fn shader_storage_buffer_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_storage_buffer_array_non_uniform_indexing_native = val;
		self
	}
	pub fn shader_storage_image_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_storage_image_array_non_uniform_indexing_native = val;
		self
	}
	pub fn shader_input_attachment_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_input_attachment_array_non_uniform_indexing_native = val;
		self
	}
	pub fn robust_buffer_access_update_after_bind(mut self, val : Bool32) -> Self {
		self.robust_buffer_access_update_after_bind = val;
		self
	}
	pub fn quad_divergent_implicit_lod(mut self, val : Bool32) -> Self {
		self.quad_divergent_implicit_lod = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_samplers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_samplers = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_uniform_buffers = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_storage_buffers = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_sampled_images(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_sampled_images = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_storage_images(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_storage_images = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_input_attachments(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_input_attachments = val;
		self
	}
	pub fn max_per_stage_update_after_bind_resources(mut self, val : u32) -> Self {
		self.max_per_stage_update_after_bind_resources = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_samplers(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_samplers = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_uniform_buffers(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_uniform_buffers = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_storage_buffers(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_storage_buffers = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_storage_buffers_dynamic = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_sampled_images(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_sampled_images = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_storage_images(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_storage_images = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_input_attachments(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_input_attachments = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDescriptorIndexingProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayPlanePropertiesKHR {
	pub current_display : DisplayKHR,
	pub current_stack_index : u32,
}

impl DisplayPlanePropertiesKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn current_display(mut self, val : DisplayKHR) -> Self {
		self.current_display = val;
		self
	}
	pub fn current_stack_index(mut self, val : u32) -> Self {
		self.current_stack_index = val;
		self
	}
}

impl std::default::Default for DisplayPlanePropertiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceFeatures2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub features : PhysicalDeviceFeatures,
}

impl PhysicalDeviceFeatures2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059000,
			p_next : null_mut(),
			features : PhysicalDeviceFeatures::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn features(mut self, val : PhysicalDeviceFeatures) -> Self {
		self.features = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceFeatures2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceProperties2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub properties : PhysicalDeviceProperties,
}

impl PhysicalDeviceProperties2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059001,
			p_next : null_mut(),
			properties : PhysicalDeviceProperties::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn properties(mut self, val : PhysicalDeviceProperties) -> Self {
		self.properties = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceProperties2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineCreationFeedbackCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_pipeline_creation_feedback : *mut PipelineCreationFeedbackEXT,
	pub pipeline_stage_creation_feedback_count : u32,
	pub p_pipeline_stage_creation_feedbacks : *mut PipelineCreationFeedbackEXT,
}

impl PipelineCreationFeedbackCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000192000,
			p_next : null(),
			p_pipeline_creation_feedback : null_mut(),
			pipeline_stage_creation_feedback_count : <_>::default(),
			p_pipeline_stage_creation_feedbacks : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn pipeline_creation_feedback(mut self, val : *mut PipelineCreationFeedbackEXT) -> Self {
		self.p_pipeline_creation_feedback = val;
		self
	}
	pub fn pipeline_stage_creation_feedbacks(mut self, val : &mut [PipelineCreationFeedbackEXT]) -> Self {
		self.pipeline_stage_creation_feedback_count = val.len() as _;
		self.p_pipeline_stage_creation_feedbacks = val.as_mut_ptr();
		self
	}
	pub fn pipeline_stage_creation_feedback(mut self, val : &mut PipelineCreationFeedbackEXT) -> Self {
		self.pipeline_stage_creation_feedback_count = 1;
		self.p_pipeline_stage_creation_feedbacks = val;
		self
	}
}

impl std::default::Default for PipelineCreationFeedbackCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageViewASTCDecodeModeEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub decode_mode : Format,
}

impl ImageViewASTCDecodeModeEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000067000,
			p_next : null(),
			decode_mode : Format::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn decode_mode(mut self, val : Format) -> Self {
		self.decode_mode = val;
		self
	}
}

impl std::default::Default for ImageViewASTCDecodeModeEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FormatProperties2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub format_properties : FormatProperties,
}

impl FormatProperties2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059002,
			p_next : null_mut(),
			format_properties : FormatProperties::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn format_properties(mut self, val : FormatProperties) -> Self {
		self.format_properties = val;
		self
	}
}

impl std::default::Default for FormatProperties2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalSemaphoreProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub export_from_imported_handle_types : ExternalSemaphoreHandleTypeFlags,
	pub compatible_handle_types : ExternalSemaphoreHandleTypeFlags,
	pub external_semaphore_features : ExternalSemaphoreFeatureFlags,
}

impl ExternalSemaphoreProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000076001,
			p_next : null_mut(),
			export_from_imported_handle_types : <_>::default(),
			compatible_handle_types : <_>::default(),
			external_semaphore_features : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn export_from_imported_handle_types(mut self, val : ExternalSemaphoreHandleTypeFlags) -> Self {
		self.export_from_imported_handle_types = val;
		self
	}
	pub fn compatible_handle_types(mut self, val : ExternalSemaphoreHandleTypeFlags) -> Self {
		self.compatible_handle_types = val;
		self
	}
	pub fn external_semaphore_features(mut self, val : ExternalSemaphoreFeatureFlags) -> Self {
		self.external_semaphore_features = val;
		self
	}
}

impl std::default::Default for ExternalSemaphoreProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageFormatProperties2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub image_format_properties : ImageFormatProperties,
}

impl ImageFormatProperties2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059003,
			p_next : null_mut(),
			image_format_properties : ImageFormatProperties::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image_format_properties(mut self, val : ImageFormatProperties) -> Self {
		self.image_format_properties = val;
		self
	}
}

impl std::default::Default for ImageFormatProperties2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryAllocateFlagsInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : MemoryAllocateFlags,
	pub device_mask : u32,
}

impl MemoryAllocateFlagsInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000060000,
			p_next : null(),
			flags : <_>::default(),
			device_mask : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : MemoryAllocateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn device_mask(mut self, val : u32) -> Self {
		self.device_mask = val;
		self
	}
}

impl std::default::Default for MemoryAllocateFlagsInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SamplerCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : SamplerCreateFlags,
	pub mag_filter : Filter,
	pub min_filter : Filter,
	pub mipmap_mode : SamplerMipmapMode,
	pub address_mode_u : SamplerAddressMode,
	pub address_mode_v : SamplerAddressMode,
	pub address_mode_w : SamplerAddressMode,
	pub mip_lod_bias : f32,
	pub anisotropy_enable : Bool32,
	pub max_anisotropy : f32,
	pub compare_enable : Bool32,
	pub compare_op : CompareOp,
	pub min_lod : f32,
	pub max_lod : f32,
	pub border_color : BorderColor,
	pub unnormalized_coordinates : Bool32,
}

impl SamplerCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 31,
			p_next : null(),
			flags : <_>::default(),
			mag_filter : Filter::default(),
			min_filter : Filter::default(),
			mipmap_mode : SamplerMipmapMode::default(),
			address_mode_u : SamplerAddressMode::default(),
			address_mode_v : SamplerAddressMode::default(),
			address_mode_w : SamplerAddressMode::default(),
			mip_lod_bias : <_>::default(),
			anisotropy_enable : <_>::default(),
			max_anisotropy : <_>::default(),
			compare_enable : <_>::default(),
			compare_op : CompareOp::default(),
			min_lod : <_>::default(),
			max_lod : <_>::default(),
			border_color : BorderColor::default(),
			unnormalized_coordinates : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : SamplerCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn mag_filter(mut self, val : Filter) -> Self {
		self.mag_filter = val;
		self
	}
	pub fn min_filter(mut self, val : Filter) -> Self {
		self.min_filter = val;
		self
	}
	pub fn mipmap_mode(mut self, val : SamplerMipmapMode) -> Self {
		self.mipmap_mode = val;
		self
	}
	pub fn address_mode_u(mut self, val : SamplerAddressMode) -> Self {
		self.address_mode_u = val;
		self
	}
	pub fn address_mode_v(mut self, val : SamplerAddressMode) -> Self {
		self.address_mode_v = val;
		self
	}
	pub fn address_mode_w(mut self, val : SamplerAddressMode) -> Self {
		self.address_mode_w = val;
		self
	}
	pub fn mip_lod_bias(mut self, val : f32) -> Self {
		self.mip_lod_bias = val;
		self
	}
	pub fn anisotropy_enable(mut self, val : Bool32) -> Self {
		self.anisotropy_enable = val;
		self
	}
	pub fn max_anisotropy(mut self, val : f32) -> Self {
		self.max_anisotropy = val;
		self
	}
	pub fn compare_enable(mut self, val : Bool32) -> Self {
		self.compare_enable = val;
		self
	}
	pub fn compare_op(mut self, val : CompareOp) -> Self {
		self.compare_op = val;
		self
	}
	pub fn min_lod(mut self, val : f32) -> Self {
		self.min_lod = val;
		self
	}
	pub fn max_lod(mut self, val : f32) -> Self {
		self.max_lod = val;
		self
	}
	pub fn border_color(mut self, val : BorderColor) -> Self {
		self.border_color = val;
		self
	}
	pub fn unnormalized_coordinates(mut self, val : Bool32) -> Self {
		self.unnormalized_coordinates = val;
		self
	}
}

impl std::default::Default for SamplerCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceDriverProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub driver_id : DriverId,
	pub driver_name : [u8; VK_MAX_DRIVER_NAME_SIZE],
	pub driver_info : [u8; VK_MAX_DRIVER_INFO_SIZE],
	pub conformance_version : ConformanceVersion,
}

impl PhysicalDeviceDriverProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000196000,
			p_next : null_mut(),
			driver_id : DriverId::default(),
			driver_name : [0 as _ ;VK_MAX_DRIVER_NAME_SIZE],
			driver_info : [0 as _ ;VK_MAX_DRIVER_INFO_SIZE],
			conformance_version : ConformanceVersion::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn driver_id(mut self, val : DriverId) -> Self {
		self.driver_id = val;
		self
	}
	pub fn driver_name(mut self, val : [u8; VK_MAX_DRIVER_NAME_SIZE]) -> Self {
		self.driver_name = val;
		self
	}
	pub fn driver_info(mut self, val : [u8; VK_MAX_DRIVER_INFO_SIZE]) -> Self {
		self.driver_info = val;
		self
	}
	pub fn conformance_version(mut self, val : ConformanceVersion) -> Self {
		self.conformance_version = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDriverProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub drm_format_modifier : u64,
	pub sharing_mode : SharingMode,
	pub queue_family_index_count : u32,
	pub p_queue_family_indices : *const u32,
}

impl PhysicalDeviceImageDrmFormatModifierInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000158002,
			p_next : null(),
			drm_format_modifier : <_>::default(),
			sharing_mode : SharingMode::default(),
			queue_family_index_count : <_>::default(),
			p_queue_family_indices : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn drm_format_modifier(mut self, val : u64) -> Self {
		self.drm_format_modifier = val;
		self
	}
	pub fn sharing_mode(mut self, val : SharingMode) -> Self {
		self.sharing_mode = val;
		self
	}
	pub fn queue_family_indices(mut self, val : &[u32]) -> Self {
		self.queue_family_index_count = val.len() as _;
		self.p_queue_family_indices = val.as_ptr();
		self
	}
	pub fn queue_family_indice(mut self, val : &u32) -> Self {
		self.queue_family_index_count = 1;
		self.p_queue_family_indices = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceImageDrmFormatModifierInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceFormatKHR {
	pub format : Format,
	pub color_space : ColorSpaceKHR,
}

impl SurfaceFormatKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn color_space(mut self, val : ColorSpaceKHR) -> Self {
		self.color_space = val;
		self
	}
}

impl std::default::Default for SurfaceFormatKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageSubresource {
	pub aspect_mask : ImageAspectFlags,
	pub mip_level : u32,
	pub array_layer : u32,
}

impl ImageSubresource {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn aspect_mask(mut self, val : ImageAspectFlags) -> Self {
		self.aspect_mask = val;
		self
	}
	pub fn mip_level(mut self, val : u32) -> Self {
		self.mip_level = val;
		self
	}
	pub fn array_layer(mut self, val : u32) -> Self {
		self.array_layer = val;
		self
	}
}

impl std::default::Default for ImageSubresource {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevicePointClippingProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub point_clipping_behavior : PointClippingBehavior,
}

impl PhysicalDevicePointClippingProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000117000,
			p_next : null_mut(),
			point_clipping_behavior : PointClippingBehavior::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn point_clipping_behavior(mut self, val : PointClippingBehavior) -> Self {
		self.point_clipping_behavior = val;
		self
	}
}

impl std::default::Default for PhysicalDevicePointClippingProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerformanceValueINTEL {
	pub r#type : PerformanceValueTypeINTEL,
	pub data : PerformanceValueDataINTEL,
}

impl PerformanceValueINTEL {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn r#type(mut self, val : PerformanceValueTypeINTEL) -> Self {
		self.r#type = val;
		self
	}
	pub fn data(mut self, val : PerformanceValueDataINTEL) -> Self {
		self.data = val;
		self
	}
}

impl std::default::Default for PerformanceValueINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub aspect_reference_count : u32,
	pub p_aspect_references : *const InputAttachmentAspectReference,
}

impl RenderPassInputAttachmentAspectCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000117001,
			p_next : null(),
			aspect_reference_count : <_>::default(),
			p_aspect_references : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn aspect_references(mut self, val : &[InputAttachmentAspectReference]) -> Self {
		self.aspect_reference_count = val.len() as _;
		self.p_aspect_references = val.as_ptr();
		self
	}
	pub fn aspect_reference(mut self, val : &InputAttachmentAspectReference) -> Self {
		self.aspect_reference_count = 1;
		self.p_aspect_references = val;
		self
	}
}

impl std::default::Default for RenderPassInputAttachmentAspectCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineRasterizationLineStateCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub line_rasterization_mode : LineRasterizationModeEXT,
	pub stippled_line_enable : Bool32,
	pub line_stipple_factor : u32,
	pub line_stipple_pattern : u16,
}

impl PipelineRasterizationLineStateCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000259001,
			p_next : null(),
			line_rasterization_mode : LineRasterizationModeEXT::default(),
			stippled_line_enable : <_>::default(),
			line_stipple_factor : <_>::default(),
			line_stipple_pattern : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn line_rasterization_mode(mut self, val : LineRasterizationModeEXT) -> Self {
		self.line_rasterization_mode = val;
		self
	}
	pub fn stippled_line_enable(mut self, val : Bool32) -> Self {
		self.stippled_line_enable = val;
		self
	}
	pub fn line_stipple_factor(mut self, val : u32) -> Self {
		self.line_stipple_factor = val;
		self
	}
	pub fn line_stipple_pattern(mut self, val : u16) -> Self {
		self.line_stipple_pattern = val;
		self
	}
}

impl std::default::Default for PipelineRasterizationLineStateCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct InputAttachmentAspectReference {
	pub subpass : u32,
	pub input_attachment_index : u32,
	pub aspect_mask : ImageAspectFlags,
}

impl InputAttachmentAspectReference {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn subpass(mut self, val : u32) -> Self {
		self.subpass = val;
		self
	}
	pub fn input_attachment_index(mut self, val : u32) -> Self {
		self.input_attachment_index = val;
		self
	}
	pub fn aspect_mask(mut self, val : ImageAspectFlags) -> Self {
		self.aspect_mask = val;
		self
	}
}

impl std::default::Default for InputAttachmentAspectReference {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExportMemoryAllocateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_types : ExternalMemoryHandleTypeFlags,
}

impl ExportMemoryAllocateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000072002,
			p_next : null(),
			handle_types : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_types(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_types = val;
		self
	}
}

impl std::default::Default for ExportMemoryAllocateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageViewUsageCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub usage : ImageUsageFlags,
}

impl ImageViewUsageCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000117002,
			p_next : null(),
			usage : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn usage(mut self, val : ImageUsageFlags) -> Self {
		self.usage = val;
		self
	}
}

impl std::default::Default for ImageViewUsageCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub domain_origin : TessellationDomainOrigin,
}

impl PipelineTessellationDomainOriginStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000117003,
			p_next : null(),
			domain_origin : TessellationDomainOrigin::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn domain_origin(mut self, val : TessellationDomainOrigin) -> Self {
		self.domain_origin = val;
		self
	}
}

impl std::default::Default for PipelineTessellationDomainOriginStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassMultiviewCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub subpass_count : u32,
	pub p_view_masks : *const u32,
	pub dependency_count : u32,
	pub p_view_offsets : *const i32,
	pub correlation_mask_count : u32,
	pub p_correlation_masks : *const u32,
}

impl RenderPassMultiviewCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000053000,
			p_next : null(),
			subpass_count : <_>::default(),
			p_view_masks : null(),
			dependency_count : <_>::default(),
			p_view_offsets : null(),
			correlation_mask_count : <_>::default(),
			p_correlation_masks : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn view_masks(mut self, val : &[u32]) -> Self {
		self.subpass_count = val.len() as _;
		self.p_view_masks = val.as_ptr();
		self
	}
	pub fn view_mask(mut self, val : &u32) -> Self {
		self.subpass_count = 1;
		self.p_view_masks = val;
		self
	}
	pub fn view_offsets(mut self, val : &[i32]) -> Self {
		self.dependency_count = val.len() as _;
		self.p_view_offsets = val.as_ptr();
		self
	}
	pub fn view_offset(mut self, val : &i32) -> Self {
		self.dependency_count = 1;
		self.p_view_offsets = val;
		self
	}
	pub fn correlation_masks(mut self, val : &[u32]) -> Self {
		self.correlation_mask_count = val.len() as _;
		self.p_correlation_masks = val.as_ptr();
		self
	}
	pub fn correlation_mask(mut self, val : &u32) -> Self {
		self.correlation_mask_count = 1;
		self.p_correlation_masks = val;
		self
	}
}

impl std::default::Default for RenderPassMultiviewCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
	s_type : i32,
	pub p_next : *const c_void,
	pub buffer : *mut AHardwareBuffer,
}

impl ImportAndroidHardwareBufferInfoANDROID {
	pub fn new() -> Self {
		Self {
			s_type : 1000129003,
			p_next : null(),
			buffer : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn buffer(mut self, val : *mut AHardwareBuffer) -> Self {
		self.buffer = val;
		self
	}
}

impl std::default::Default for ImportAndroidHardwareBufferInfoANDROID {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub heap_budget : [DeviceSize; VK_MAX_MEMORY_HEAPS],
	pub heap_usage : [DeviceSize; VK_MAX_MEMORY_HEAPS],
}

impl PhysicalDeviceMemoryBudgetPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000237000,
			p_next : null_mut(),
			heap_budget : [0 as _ ;VK_MAX_MEMORY_HEAPS],
			heap_usage : [0 as _ ;VK_MAX_MEMORY_HEAPS],
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn heap_budget(mut self, val : [DeviceSize; VK_MAX_MEMORY_HEAPS]) -> Self {
		self.heap_budget = val;
		self
	}
	pub fn heap_usage(mut self, val : [DeviceSize; VK_MAX_MEMORY_HEAPS]) -> Self {
		self.heap_usage = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMemoryBudgetPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayEventInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub display_event : DisplayEventTypeEXT,
}

impl DisplayEventInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000091002,
			p_next : null(),
			display_event : DisplayEventTypeEXT::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn display_event(mut self, val : DisplayEventTypeEXT) -> Self {
		self.display_event = val;
		self
	}
}

impl std::default::Default for DisplayEventInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceMultiviewFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub multiview : Bool32,
	pub multiview_geometry_shader : Bool32,
	pub multiview_tessellation_shader : Bool32,
}

impl PhysicalDeviceMultiviewFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000053001,
			p_next : null_mut(),
			multiview : <_>::default(),
			multiview_geometry_shader : <_>::default(),
			multiview_tessellation_shader : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn multiview(mut self, val : Bool32) -> Self {
		self.multiview = val;
		self
	}
	pub fn multiview_geometry_shader(mut self, val : Bool32) -> Self {
		self.multiview_geometry_shader = val;
		self
	}
	pub fn multiview_tessellation_shader(mut self, val : Bool32) -> Self {
		self.multiview_tessellation_shader = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMultiviewFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DebugUtilsObjectTagInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub object_type : ObjectType,
	pub object_handle : u64,
	pub tag_name : u64,
	pub tag_size : usize,
	pub p_tag : *const c_void,
}

impl DebugUtilsObjectTagInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000128001,
			p_next : null(),
			object_type : ObjectType::default(),
			object_handle : <_>::default(),
			tag_name : <_>::default(),
			tag_size : <_>::default(),
			p_tag : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn object_type(mut self, val : ObjectType) -> Self {
		self.object_type = val;
		self
	}
	pub fn object_handle(mut self, val : u64) -> Self {
		self.object_handle = val;
		self
	}
	pub fn tag_name(mut self, val : u64) -> Self {
		self.tag_name = val;
		self
	}
	pub fn tag_size(mut self, val : usize) -> Self {
		self.tag_size = val;
		self
	}
	pub fn tag(mut self, val : *const c_void) -> Self {
		self.p_tag = val;
		self
	}
}

impl std::default::Default for DebugUtilsObjectTagInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubpassEndInfo {
	s_type : i32,
	pub p_next : *const c_void,
}

impl SubpassEndInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000109006,
			p_next : null(),
		}
	}
}

impl std::default::Default for SubpassEndInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceMultiviewProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_multiview_view_count : u32,
	pub max_multiview_instance_index : u32,
}

impl PhysicalDeviceMultiviewProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000053002,
			p_next : null_mut(),
			max_multiview_view_count : <_>::default(),
			max_multiview_instance_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_multiview_view_count(mut self, val : u32) -> Self {
		self.max_multiview_view_count = val;
		self
	}
	pub fn max_multiview_instance_index(mut self, val : u32) -> Self {
		self.max_multiview_instance_index = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceMultiviewProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub p_attributes : *const SECURITY_ATTRIBUTES,
	pub dw_access : u32,
	pub name : *const u16,
}

impl ExportSemaphoreWin32HandleInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000078001,
			p_next : null(),
			p_attributes : null(),
			dw_access : <_>::default(),
			name : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn attributes(mut self, val : *const SECURITY_ATTRIBUTES) -> Self {
		self.p_attributes = val;
		self
	}
	pub fn dw_access(mut self, val : u32) -> Self {
		self.dw_access = val;
		self
	}
	pub fn name(mut self, val : *const u16) -> Self {
		self.name = val;
		self
	}
}

impl std::default::Default for ExportSemaphoreWin32HandleInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ValidationFeaturesEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub enabled_validation_feature_count : u32,
	pub p_enabled_validation_features : *const ValidationFeatureEnableEXT,
	pub disabled_validation_feature_count : u32,
	pub p_disabled_validation_features : *const ValidationFeatureDisableEXT,
}

impl ValidationFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000247000,
			p_next : null(),
			enabled_validation_feature_count : <_>::default(),
			p_enabled_validation_features : null(),
			disabled_validation_feature_count : <_>::default(),
			p_disabled_validation_features : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn enabled_validation_features(mut self, val : &[ValidationFeatureEnableEXT]) -> Self {
		self.enabled_validation_feature_count = val.len() as _;
		self.p_enabled_validation_features = val.as_ptr();
		self
	}
	pub fn enabled_validation_feature(mut self, val : &ValidationFeatureEnableEXT) -> Self {
		self.enabled_validation_feature_count = 1;
		self.p_enabled_validation_features = val;
		self
	}
	pub fn disabled_validation_features(mut self, val : &[ValidationFeatureDisableEXT]) -> Self {
		self.disabled_validation_feature_count = val.len() as _;
		self.p_disabled_validation_features = val.as_ptr();
		self
	}
	pub fn disabled_validation_feature(mut self, val : &ValidationFeatureDisableEXT) -> Self {
		self.disabled_validation_feature_count = 1;
		self.p_disabled_validation_features = val;
		self
	}
}

impl std::default::Default for ValidationFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
	s_type : i32,
	pub p_next : *const c_void,
	pub r#type : PerformanceConfigurationTypeINTEL,
}

impl PerformanceConfigurationAcquireInfoINTEL {
	pub fn new() -> Self {
		Self {
			s_type : 1000210005,
			p_next : null(),
			r#type : PerformanceConfigurationTypeINTEL::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn r#type(mut self, val : PerformanceConfigurationTypeINTEL) -> Self {
		self.r#type = val;
		self
	}
}

impl std::default::Default for PerformanceConfigurationAcquireInfoINTEL {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceVariablePointersFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub variable_pointers_storage_buffer : Bool32,
	pub variable_pointers : Bool32,
}

impl PhysicalDeviceVariablePointersFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000120000,
			p_next : null_mut(),
			variable_pointers_storage_buffer : <_>::default(),
			variable_pointers : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn variable_pointers_storage_buffer(mut self, val : Bool32) -> Self {
		self.variable_pointers_storage_buffer = val;
		self
	}
	pub fn variable_pointers(mut self, val : Bool32) -> Self {
		self.variable_pointers = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceVariablePointersFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub attachment_initial_sample_locations_count : u32,
	pub p_attachment_initial_sample_locations : *const AttachmentSampleLocationsEXT,
	pub post_subpass_sample_locations_count : u32,
	pub p_post_subpass_sample_locations : *const SubpassSampleLocationsEXT,
}

impl RenderPassSampleLocationsBeginInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000143001,
			p_next : null(),
			attachment_initial_sample_locations_count : <_>::default(),
			p_attachment_initial_sample_locations : null(),
			post_subpass_sample_locations_count : <_>::default(),
			p_post_subpass_sample_locations : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn attachment_initial_sample_locations(mut self, val : &[AttachmentSampleLocationsEXT]) -> Self {
		self.attachment_initial_sample_locations_count = val.len() as _;
		self.p_attachment_initial_sample_locations = val.as_ptr();
		self
	}
	pub fn attachment_initial_sample_location(mut self, val : &AttachmentSampleLocationsEXT) -> Self {
		self.attachment_initial_sample_locations_count = 1;
		self.p_attachment_initial_sample_locations = val;
		self
	}
	pub fn post_subpass_sample_locations(mut self, val : &[SubpassSampleLocationsEXT]) -> Self {
		self.post_subpass_sample_locations_count = val.len() as _;
		self.p_post_subpass_sample_locations = val.as_ptr();
		self
	}
	pub fn post_subpass_sample_location(mut self, val : &SubpassSampleLocationsEXT) -> Self {
		self.post_subpass_sample_locations_count = 1;
		self.p_post_subpass_sample_locations = val;
		self
	}
}

impl std::default::Default for RenderPassSampleLocationsBeginInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceQueueInfo2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DeviceQueueCreateFlags,
	pub queue_family_index : u32,
	pub queue_index : u32,
}

impl DeviceQueueInfo2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000145003,
			p_next : null(),
			flags : <_>::default(),
			queue_family_index : <_>::default(),
			queue_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DeviceQueueCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn queue_family_index(mut self, val : u32) -> Self {
		self.queue_family_index = val;
		self
	}
	pub fn queue_index(mut self, val : u32) -> Self {
		self.queue_index = val;
		self
	}
}

impl std::default::Default for DeviceQueueInfo2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalMemoryImageCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_types : ExternalMemoryHandleTypeFlags,
}

impl ExternalMemoryImageCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000072001,
			p_next : null(),
			handle_types : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_types(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_types = val;
		self
	}
}

impl std::default::Default for ExternalMemoryImageCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ProtectedSubmitInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub protected_submit : Bool32,
}

impl ProtectedSubmitInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000145000,
			p_next : null(),
			protected_submit : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn protected_submit(mut self, val : Bool32) -> Self {
		self.protected_submit = val;
		self
	}
}

impl std::default::Default for ProtectedSubmitInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub pipeline_executable_info : Bool32,
}

impl PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000269000,
			p_next : null_mut(),
			pipeline_executable_info : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn pipeline_executable_info(mut self, val : Bool32) -> Self {
		self.pipeline_executable_info = val;
		self
	}
}

impl std::default::Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SamplerYcbcrConversionCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub format : Format,
	pub ycbcr_model : SamplerYcbcrModelConversion,
	pub ycbcr_range : SamplerYcbcrRange,
	pub components : ComponentMapping,
	pub x_chroma_offset : ChromaLocation,
	pub y_chroma_offset : ChromaLocation,
	pub chroma_filter : Filter,
	pub force_explicit_reconstruction : Bool32,
}

impl SamplerYcbcrConversionCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000156000,
			p_next : null(),
			format : Format::default(),
			ycbcr_model : SamplerYcbcrModelConversion::default(),
			ycbcr_range : SamplerYcbcrRange::default(),
			components : ComponentMapping::new(),
			x_chroma_offset : ChromaLocation::default(),
			y_chroma_offset : ChromaLocation::default(),
			chroma_filter : Filter::default(),
			force_explicit_reconstruction : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn ycbcr_model(mut self, val : SamplerYcbcrModelConversion) -> Self {
		self.ycbcr_model = val;
		self
	}
	pub fn ycbcr_range(mut self, val : SamplerYcbcrRange) -> Self {
		self.ycbcr_range = val;
		self
	}
	pub fn components(mut self, val : ComponentMapping) -> Self {
		self.components = val;
		self
	}
	pub fn x_chroma_offset(mut self, val : ChromaLocation) -> Self {
		self.x_chroma_offset = val;
		self
	}
	pub fn y_chroma_offset(mut self, val : ChromaLocation) -> Self {
		self.y_chroma_offset = val;
		self
	}
	pub fn chroma_filter(mut self, val : Filter) -> Self {
		self.chroma_filter = val;
		self
	}
	pub fn force_explicit_reconstruction(mut self, val : Bool32) -> Self {
		self.force_explicit_reconstruction = val;
		self
	}
}

impl std::default::Default for SamplerYcbcrConversionCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindImagePlaneMemoryInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub plane_aspect : ImageAspectFlags,
}

impl BindImagePlaneMemoryInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000156002,
			p_next : null(),
			plane_aspect : ImageAspectFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn plane_aspect(mut self, val : ImageAspectFlags) -> Self {
		self.plane_aspect = val;
		self
	}
}

impl std::default::Default for BindImagePlaneMemoryInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shared_present_supported_usage_flags : ImageUsageFlags,
}

impl SharedPresentSurfaceCapabilitiesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000111000,
			p_next : null_mut(),
			shared_present_supported_usage_flags : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shared_present_supported_usage_flags(mut self, val : ImageUsageFlags) -> Self {
		self.shared_present_supported_usage_flags = val;
		self
	}
}

impl std::default::Default for SharedPresentSurfaceCapabilitiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceImageRobustnessFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub robust_image_access : Bool32,
}

impl PhysicalDeviceImageRobustnessFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000335000,
			p_next : null_mut(),
			robust_image_access : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn robust_image_access(mut self, val : Bool32) -> Self {
		self.robust_image_access = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceImageRobustnessFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SamplerYcbcrConversionImageFormatProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub combined_image_sampler_descriptor_count : u32,
}

impl SamplerYcbcrConversionImageFormatProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000156005,
			p_next : null_mut(),
			combined_image_sampler_descriptor_count : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn combined_image_sampler_descriptor_count(mut self, val : u32) -> Self {
		self.combined_image_sampler_descriptor_count = val;
		self
	}
}

impl std::default::Default for SamplerYcbcrConversionImageFormatProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorUpdateTemplateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DescriptorUpdateTemplateCreateFlags,
	pub descriptor_update_entry_count : u32,
	pub p_descriptor_update_entries : *const DescriptorUpdateTemplateEntry,
	pub template_type : DescriptorUpdateTemplateType,
	pub descriptor_set_layout : DescriptorSetLayout,
	pub pipeline_bind_point : PipelineBindPoint,
	pub pipeline_layout : PipelineLayout,
	pub set : u32,
}

impl DescriptorUpdateTemplateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000085000,
			p_next : null(),
			flags : <_>::default(),
			descriptor_update_entry_count : <_>::default(),
			p_descriptor_update_entries : null(),
			template_type : DescriptorUpdateTemplateType::default(),
			descriptor_set_layout : DescriptorSetLayout(0),
			pipeline_bind_point : PipelineBindPoint::default(),
			pipeline_layout : PipelineLayout(0),
			set : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DescriptorUpdateTemplateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn template_type(mut self, val : DescriptorUpdateTemplateType) -> Self {
		self.template_type = val;
		self
	}
	pub fn descriptor_set_layout(mut self, val : DescriptorSetLayout) -> Self {
		self.descriptor_set_layout = val;
		self
	}
	pub fn pipeline_bind_point(mut self, val : PipelineBindPoint) -> Self {
		self.pipeline_bind_point = val;
		self
	}
	pub fn pipeline_layout(mut self, val : PipelineLayout) -> Self {
		self.pipeline_layout = val;
		self
	}
	pub fn set(mut self, val : u32) -> Self {
		self.set = val;
		self
	}
	pub fn descriptor_update_entries(mut self, val : &[DescriptorUpdateTemplateEntry]) -> Self {
		self.descriptor_update_entry_count = val.len() as _;
		self.p_descriptor_update_entries = val.as_ptr();
		self
	}
	pub fn descriptor_update_entry(mut self, val : &DescriptorUpdateTemplateEntry) -> Self {
		self.descriptor_update_entry_count = 1;
		self.p_descriptor_update_entries = val;
		self
	}
}

impl std::default::Default for DescriptorUpdateTemplateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineLayoutCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineLayoutCreateFlags,
	pub set_layout_count : u32,
	pub p_set_layouts : *const DescriptorSetLayout,
	pub push_constant_range_count : u32,
	pub p_push_constant_ranges : *const PushConstantRange,
}

impl PipelineLayoutCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 30,
			p_next : null(),
			flags : <_>::default(),
			set_layout_count : <_>::default(),
			p_set_layouts : null(),
			push_constant_range_count : <_>::default(),
			p_push_constant_ranges : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineLayoutCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn set_layouts(mut self, val : &[DescriptorSetLayout]) -> Self {
		self.set_layout_count = val.len() as _;
		self.p_set_layouts = val.as_ptr();
		self
	}
	pub fn set_layout(mut self, val : &DescriptorSetLayout) -> Self {
		self.set_layout_count = 1;
		self.p_set_layouts = val;
		self
	}
	pub fn push_constant_ranges(mut self, val : &[PushConstantRange]) -> Self {
		self.push_constant_range_count = val.len() as _;
		self.p_push_constant_ranges = val.as_ptr();
		self
	}
	pub fn push_constant_range(mut self, val : &PushConstantRange) -> Self {
		self.push_constant_range_count = 1;
		self.p_push_constant_ranges = val;
		self
	}
}

impl std::default::Default for PipelineLayoutCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalImageFormatProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub external_memory_properties : ExternalMemoryProperties,
}

impl ExternalImageFormatProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000071001,
			p_next : null_mut(),
			external_memory_properties : ExternalMemoryProperties::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn external_memory_properties(mut self, val : ExternalMemoryProperties) -> Self {
		self.external_memory_properties = val;
		self
	}
}

impl std::default::Default for ExternalImageFormatProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GeometryAABBNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub aabb_data : Buffer,
	pub num_aabbs : u32,
	pub stride : u32,
	pub offset : DeviceSize,
}

impl GeometryAABBNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165005,
			p_next : null(),
			aabb_data : Buffer(0),
			num_aabbs : <_>::default(),
			stride : <_>::default(),
			offset : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn aabb_data(mut self, val : Buffer) -> Self {
		self.aabb_data = val;
		self
	}
	pub fn num_aabbs(mut self, val : u32) -> Self {
		self.num_aabbs = val;
		self
	}
	pub fn stride(mut self, val : u32) -> Self {
		self.stride = val;
		self
	}
	pub fn offset(mut self, val : DeviceSize) -> Self {
		self.offset = val;
		self
	}
}

impl std::default::Default for GeometryAABBNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub primitive_overestimation_size : f32,
	pub max_extra_primitive_overestimation_size : f32,
	pub extra_primitive_overestimation_size_granularity : f32,
	pub primitive_underestimation : Bool32,
	pub conservative_point_and_line_rasterization : Bool32,
	pub degenerate_triangles_rasterized : Bool32,
	pub degenerate_lines_rasterized : Bool32,
	pub fully_covered_fragment_shader_input_variable : Bool32,
	pub conservative_rasterization_post_depth_coverage : Bool32,
}

impl PhysicalDeviceConservativeRasterizationPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000101000,
			p_next : null_mut(),
			primitive_overestimation_size : <_>::default(),
			max_extra_primitive_overestimation_size : <_>::default(),
			extra_primitive_overestimation_size_granularity : <_>::default(),
			primitive_underestimation : <_>::default(),
			conservative_point_and_line_rasterization : <_>::default(),
			degenerate_triangles_rasterized : <_>::default(),
			degenerate_lines_rasterized : <_>::default(),
			fully_covered_fragment_shader_input_variable : <_>::default(),
			conservative_rasterization_post_depth_coverage : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn primitive_overestimation_size(mut self, val : f32) -> Self {
		self.primitive_overestimation_size = val;
		self
	}
	pub fn max_extra_primitive_overestimation_size(mut self, val : f32) -> Self {
		self.max_extra_primitive_overestimation_size = val;
		self
	}
	pub fn extra_primitive_overestimation_size_granularity(mut self, val : f32) -> Self {
		self.extra_primitive_overestimation_size_granularity = val;
		self
	}
	pub fn primitive_underestimation(mut self, val : Bool32) -> Self {
		self.primitive_underestimation = val;
		self
	}
	pub fn conservative_point_and_line_rasterization(mut self, val : Bool32) -> Self {
		self.conservative_point_and_line_rasterization = val;
		self
	}
	pub fn degenerate_triangles_rasterized(mut self, val : Bool32) -> Self {
		self.degenerate_triangles_rasterized = val;
		self
	}
	pub fn degenerate_lines_rasterized(mut self, val : Bool32) -> Self {
		self.degenerate_lines_rasterized = val;
		self
	}
	pub fn fully_covered_fragment_shader_input_variable(mut self, val : Bool32) -> Self {
		self.fully_covered_fragment_shader_input_variable = val;
		self
	}
	pub fn conservative_rasterization_post_depth_coverage(mut self, val : Bool32) -> Self {
		self.conservative_rasterization_post_depth_coverage = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceConservativeRasterizationPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DebugReportCallbackCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DebugReportFlagsEXT,
	pub pfn_callback : Option<extern "C" fn()>,
	pub p_user_data : *mut c_void,
}

impl DebugReportCallbackCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000011000,
			p_next : null(),
			flags : <_>::default(),
			pfn_callback : None,
			p_user_data : null_mut(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DebugReportFlagsEXT) -> Self {
		self.flags = val;
		self
	}
	pub fn pfn_callback(mut self, val : Option<extern "C" fn()>) -> Self {
		self.pfn_callback = val;
		self
	}
	pub fn user_data(mut self, val : *mut c_void) -> Self {
		self.p_user_data = val;
		self
	}
}

impl std::default::Default for DebugReportCallbackCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceExternalBufferInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : BufferCreateFlags,
	pub usage : BufferUsageFlags,
	pub handle_type : ExternalMemoryHandleTypeFlags,
}

impl PhysicalDeviceExternalBufferInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000071002,
			p_next : null(),
			flags : <_>::default(),
			usage : <_>::default(),
			handle_type : ExternalMemoryHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : BufferCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn usage(mut self, val : BufferUsageFlags) -> Self {
		self.usage = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceExternalBufferInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceIDProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub device_uuid : [u8; VK_UUID_SIZE],
	pub driver_uuid : [u8; VK_UUID_SIZE],
	pub device_luid : [u8; VK_LUID_SIZE],
	pub device_node_mask : u32,
	pub device_luidvalid : Bool32,
}

impl PhysicalDeviceIDProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000071004,
			p_next : null_mut(),
			device_uuid : [0 as _ ;VK_UUID_SIZE],
			driver_uuid : [0 as _ ;VK_UUID_SIZE],
			device_luid : [0 as _ ;VK_LUID_SIZE],
			device_node_mask : <_>::default(),
			device_luidvalid : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_uuid(mut self, val : [u8; VK_UUID_SIZE]) -> Self {
		self.device_uuid = val;
		self
	}
	pub fn driver_uuid(mut self, val : [u8; VK_UUID_SIZE]) -> Self {
		self.driver_uuid = val;
		self
	}
	pub fn device_luid(mut self, val : [u8; VK_LUID_SIZE]) -> Self {
		self.device_luid = val;
		self
	}
	pub fn device_node_mask(mut self, val : u32) -> Self {
		self.device_node_mask = val;
		self
	}
	pub fn device_luidvalid(mut self, val : Bool32) -> Self {
		self.device_luidvalid = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceIDProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceExternalFenceInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_type : ExternalFenceHandleTypeFlags,
}

impl PhysicalDeviceExternalFenceInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000112000,
			p_next : null(),
			handle_type : ExternalFenceHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalFenceHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceExternalFenceInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayModeParametersKHR {
	pub visible_region : Extent2D,
	pub refresh_rate : u32,
}

impl DisplayModeParametersKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn visible_region(mut self, val : Extent2D) -> Self {
		self.visible_region = val;
		self
	}
	pub fn refresh_rate(mut self, val : u32) -> Self {
		self.refresh_rate = val;
		self
	}
}

impl std::default::Default for DisplayModeParametersKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ComputePipelineCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineCreateFlags,
	pub stage : PipelineShaderStageCreateInfo,
	pub layout : PipelineLayout,
	pub base_pipeline_handle : Pipeline,
	pub base_pipeline_index : i32,
}

impl ComputePipelineCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 29,
			p_next : null(),
			flags : <_>::default(),
			stage : PipelineShaderStageCreateInfo::new(),
			layout : PipelineLayout(0),
			base_pipeline_handle : Pipeline(0),
			base_pipeline_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn stage(mut self, val : PipelineShaderStageCreateInfo) -> Self {
		self.stage = val;
		self
	}
	pub fn layout(mut self, val : PipelineLayout) -> Self {
		self.layout = val;
		self
	}
	pub fn base_pipeline_handle(mut self, val : Pipeline) -> Self {
		self.base_pipeline_handle = val;
		self
	}
	pub fn base_pipeline_index(mut self, val : i32) -> Self {
		self.base_pipeline_index = val;
		self
	}
}

impl std::default::Default for ComputePipelineCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExportFenceCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_types : ExternalFenceHandleTypeFlags,
}

impl ExportFenceCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000113000,
			p_next : null(),
			handle_types : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_types(mut self, val : ExternalFenceHandleTypeFlags) -> Self {
		self.handle_types = val;
		self
	}
}

impl std::default::Default for ExportFenceCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub sampler_ycbcr_conversion : Bool32,
}

impl PhysicalDeviceSamplerYcbcrConversionFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000156004,
			p_next : null_mut(),
			sampler_ycbcr_conversion : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn sampler_ycbcr_conversion(mut self, val : Bool32) -> Self {
		self.sampler_ycbcr_conversion = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSamplerYcbcrConversionFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PresentRegionsKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub swapchain_count : u32,
	pub p_regions : *const PresentRegionKHR,
}

impl PresentRegionsKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000084000,
			p_next : null(),
			swapchain_count : <_>::default(),
			p_regions : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn regions(mut self, val : &[PresentRegionKHR]) -> Self {
		self.swapchain_count = val.len() as _;
		self.p_regions = val.as_ptr();
		self
	}
	pub fn region(mut self, val : &PresentRegionKHR) -> Self {
		self.swapchain_count = 1;
		self.p_regions = val;
		self
	}
}

impl std::default::Default for PresentRegionsKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceRobustness2FeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub robust_buffer_access_2 : Bool32,
	pub robust_image_access_2 : Bool32,
	pub null_descriptor : Bool32,
}

impl PhysicalDeviceRobustness2FeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000286000,
			p_next : null_mut(),
			robust_buffer_access_2 : <_>::default(),
			robust_image_access_2 : <_>::default(),
			null_descriptor : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn robust_buffer_access_2(mut self, val : Bool32) -> Self {
		self.robust_buffer_access_2 = val;
		self
	}
	pub fn robust_image_access_2(mut self, val : Bool32) -> Self {
		self.robust_image_access_2 = val;
		self
	}
	pub fn null_descriptor(mut self, val : Bool32) -> Self {
		self.null_descriptor = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceRobustness2FeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct QueryPoolCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : QueryPoolCreateFlags,
	pub query_type : QueryType,
	pub query_count : u32,
	pub pipeline_statistics : QueryPipelineStatisticFlags,
}

impl QueryPoolCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 11,
			p_next : null(),
			flags : <_>::default(),
			query_type : QueryType::default(),
			query_count : <_>::default(),
			pipeline_statistics : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : QueryPoolCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn query_type(mut self, val : QueryType) -> Self {
		self.query_type = val;
		self
	}
	pub fn query_count(mut self, val : u32) -> Self {
		self.query_count = val;
		self
	}
	pub fn pipeline_statistics(mut self, val : QueryPipelineStatisticFlags) -> Self {
		self.pipeline_statistics = val;
		self
	}
}

impl std::default::Default for QueryPoolCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_type : ExternalSemaphoreHandleTypeFlags,
}

impl PhysicalDeviceExternalSemaphoreInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000076000,
			p_next : null(),
			handle_type : ExternalSemaphoreHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalSemaphoreHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceExternalSemaphoreInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub pipeline_creation_cache_control : Bool32,
}

impl PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000297000,
			p_next : null_mut(),
			pipeline_creation_cache_control : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn pipeline_creation_cache_control(mut self, val : Bool32) -> Self {
		self.pipeline_creation_cache_control = val;
		self
	}
}

impl std::default::Default for PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImportSemaphoreFdInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub semaphore : Semaphore,
	pub flags : SemaphoreImportFlags,
	pub handle_type : ExternalSemaphoreHandleTypeFlags,
	pub fd : i32,
}

impl ImportSemaphoreFdInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000079000,
			p_next : null(),
			semaphore : Semaphore(0),
			flags : <_>::default(),
			handle_type : ExternalSemaphoreHandleTypeFlags::default(),
			fd : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn semaphore(mut self, val : Semaphore) -> Self {
		self.semaphore = val;
		self
	}
	pub fn flags(mut self, val : SemaphoreImportFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalSemaphoreHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
	pub fn fd(mut self, val : i32) -> Self {
		self.fd = val;
		self
	}
}

impl std::default::Default for ImportSemaphoreFdInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayPlaneProperties2KHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub display_plane_properties : DisplayPlanePropertiesKHR,
}

impl DisplayPlaneProperties2KHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000121001,
			p_next : null_mut(),
			display_plane_properties : DisplayPlanePropertiesKHR::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn display_plane_properties(mut self, val : DisplayPlanePropertiesKHR) -> Self {
		self.display_plane_properties = val;
		self
	}
}

impl std::default::Default for DisplayPlaneProperties2KHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_draw_parameters : Bool32,
}

impl PhysicalDeviceShaderDrawParametersFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000063000,
			p_next : null_mut(),
			shader_draw_parameters : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_draw_parameters(mut self, val : Bool32) -> Self {
		self.shader_draw_parameters = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderDrawParametersFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub r#type : AccelerationStructureMemoryRequirementsTypeNV,
	pub acceleration_structure : AccelerationStructureNV,
}

impl AccelerationStructureMemoryRequirementsInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165008,
			p_next : null(),
			r#type : AccelerationStructureMemoryRequirementsTypeKHR::default(),
			acceleration_structure : AccelerationStructureKHR(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn r#type(mut self, val : AccelerationStructureMemoryRequirementsTypeNV) -> Self {
		self.r#type = val;
		self
	}
	pub fn acceleration_structure(mut self, val : AccelerationStructureNV) -> Self {
		self.acceleration_structure = val;
		self
	}
}

impl std::default::Default for AccelerationStructureMemoryRequirementsInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferDeviceAddressInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub buffer : Buffer,
}

impl BufferDeviceAddressInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000244001,
			p_next : null(),
			buffer : Buffer(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
}

impl std::default::Default for BufferDeviceAddressInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceVulkan11Properties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub device_uuid : [u8; VK_UUID_SIZE],
	pub driver_uuid : [u8; VK_UUID_SIZE],
	pub device_luid : [u8; VK_LUID_SIZE],
	pub device_node_mask : u32,
	pub device_luidvalid : Bool32,
	pub subgroup_size : u32,
	pub subgroup_supported_stages : ShaderStageFlags,
	pub subgroup_supported_operations : SubgroupFeatureFlags,
	pub subgroup_quad_operations_in_all_stages : Bool32,
	pub point_clipping_behavior : PointClippingBehavior,
	pub max_multiview_view_count : u32,
	pub max_multiview_instance_index : u32,
	pub protected_no_fault : Bool32,
	pub max_per_set_descriptors : u32,
	pub max_memory_allocation_size : DeviceSize,
}

impl PhysicalDeviceVulkan11Properties {
	pub fn new() -> Self {
		Self {
			s_type : 50,
			p_next : null_mut(),
			device_uuid : [0 as _ ;VK_UUID_SIZE],
			driver_uuid : [0 as _ ;VK_UUID_SIZE],
			device_luid : [0 as _ ;VK_LUID_SIZE],
			device_node_mask : <_>::default(),
			device_luidvalid : <_>::default(),
			subgroup_size : <_>::default(),
			subgroup_supported_stages : <_>::default(),
			subgroup_supported_operations : <_>::default(),
			subgroup_quad_operations_in_all_stages : <_>::default(),
			point_clipping_behavior : PointClippingBehavior::default(),
			max_multiview_view_count : <_>::default(),
			max_multiview_instance_index : <_>::default(),
			protected_no_fault : <_>::default(),
			max_per_set_descriptors : <_>::default(),
			max_memory_allocation_size : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn device_uuid(mut self, val : [u8; VK_UUID_SIZE]) -> Self {
		self.device_uuid = val;
		self
	}
	pub fn driver_uuid(mut self, val : [u8; VK_UUID_SIZE]) -> Self {
		self.driver_uuid = val;
		self
	}
	pub fn device_luid(mut self, val : [u8; VK_LUID_SIZE]) -> Self {
		self.device_luid = val;
		self
	}
	pub fn device_node_mask(mut self, val : u32) -> Self {
		self.device_node_mask = val;
		self
	}
	pub fn device_luidvalid(mut self, val : Bool32) -> Self {
		self.device_luidvalid = val;
		self
	}
	pub fn subgroup_size(mut self, val : u32) -> Self {
		self.subgroup_size = val;
		self
	}
	pub fn subgroup_supported_stages(mut self, val : ShaderStageFlags) -> Self {
		self.subgroup_supported_stages = val;
		self
	}
	pub fn subgroup_supported_operations(mut self, val : SubgroupFeatureFlags) -> Self {
		self.subgroup_supported_operations = val;
		self
	}
	pub fn subgroup_quad_operations_in_all_stages(mut self, val : Bool32) -> Self {
		self.subgroup_quad_operations_in_all_stages = val;
		self
	}
	pub fn point_clipping_behavior(mut self, val : PointClippingBehavior) -> Self {
		self.point_clipping_behavior = val;
		self
	}
	pub fn max_multiview_view_count(mut self, val : u32) -> Self {
		self.max_multiview_view_count = val;
		self
	}
	pub fn max_multiview_instance_index(mut self, val : u32) -> Self {
		self.max_multiview_instance_index = val;
		self
	}
	pub fn protected_no_fault(mut self, val : Bool32) -> Self {
		self.protected_no_fault = val;
		self
	}
	pub fn max_per_set_descriptors(mut self, val : u32) -> Self {
		self.max_per_set_descriptors = val;
		self
	}
	pub fn max_memory_allocation_size(mut self, val : DeviceSize) -> Self {
		self.max_memory_allocation_size = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceVulkan11Properties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceVulkan12Features {
	s_type : i32,
	pub p_next : *mut c_void,
	pub sampler_mirror_clamp_to_edge : Bool32,
	pub draw_indirect_count : Bool32,
	pub storage_buffer_8_bit_access : Bool32,
	pub uniform_and_storage_buffer_8_bit_access : Bool32,
	pub storage_push_constant_8 : Bool32,
	pub shader_buffer_int_64_atomics : Bool32,
	pub shader_shared_int_64_atomics : Bool32,
	pub shader_float_16 : Bool32,
	pub shader_int_8 : Bool32,
	pub descriptor_indexing : Bool32,
	pub shader_input_attachment_array_dynamic_indexing : Bool32,
	pub shader_uniform_texel_buffer_array_dynamic_indexing : Bool32,
	pub shader_storage_texel_buffer_array_dynamic_indexing : Bool32,
	pub shader_uniform_buffer_array_non_uniform_indexing : Bool32,
	pub shader_sampled_image_array_non_uniform_indexing : Bool32,
	pub shader_storage_buffer_array_non_uniform_indexing : Bool32,
	pub shader_storage_image_array_non_uniform_indexing : Bool32,
	pub shader_input_attachment_array_non_uniform_indexing : Bool32,
	pub shader_uniform_texel_buffer_array_non_uniform_indexing : Bool32,
	pub shader_storage_texel_buffer_array_non_uniform_indexing : Bool32,
	pub descriptor_binding_uniform_buffer_update_after_bind : Bool32,
	pub descriptor_binding_sampled_image_update_after_bind : Bool32,
	pub descriptor_binding_storage_image_update_after_bind : Bool32,
	pub descriptor_binding_storage_buffer_update_after_bind : Bool32,
	pub descriptor_binding_uniform_texel_buffer_update_after_bind : Bool32,
	pub descriptor_binding_storage_texel_buffer_update_after_bind : Bool32,
	pub descriptor_binding_update_unused_while_pending : Bool32,
	pub descriptor_binding_partially_bound : Bool32,
	pub descriptor_binding_variable_descriptor_count : Bool32,
	pub runtime_descriptor_array : Bool32,
	pub sampler_filter_minmax : Bool32,
	pub scalar_block_layout : Bool32,
	pub imageless_framebuffer : Bool32,
	pub uniform_buffer_standard_layout : Bool32,
	pub shader_subgroup_extended_types : Bool32,
	pub separate_depth_stencil_layouts : Bool32,
	pub host_query_reset : Bool32,
	pub timeline_semaphore : Bool32,
	pub buffer_device_address : Bool32,
	pub buffer_device_address_capture_replay : Bool32,
	pub buffer_device_address_multi_device : Bool32,
	pub vulkan_memory_model : Bool32,
	pub vulkan_memory_model_device_scope : Bool32,
	pub vulkan_memory_model_availability_visibility_chains : Bool32,
	pub shader_output_viewport_index : Bool32,
	pub shader_output_layer : Bool32,
	pub subgroup_broadcast_dynamic_id : Bool32,
}

impl PhysicalDeviceVulkan12Features {
	pub fn new() -> Self {
		Self {
			s_type : 51,
			p_next : null_mut(),
			sampler_mirror_clamp_to_edge : <_>::default(),
			draw_indirect_count : <_>::default(),
			storage_buffer_8_bit_access : <_>::default(),
			uniform_and_storage_buffer_8_bit_access : <_>::default(),
			storage_push_constant_8 : <_>::default(),
			shader_buffer_int_64_atomics : <_>::default(),
			shader_shared_int_64_atomics : <_>::default(),
			shader_float_16 : <_>::default(),
			shader_int_8 : <_>::default(),
			descriptor_indexing : <_>::default(),
			shader_input_attachment_array_dynamic_indexing : <_>::default(),
			shader_uniform_texel_buffer_array_dynamic_indexing : <_>::default(),
			shader_storage_texel_buffer_array_dynamic_indexing : <_>::default(),
			shader_uniform_buffer_array_non_uniform_indexing : <_>::default(),
			shader_sampled_image_array_non_uniform_indexing : <_>::default(),
			shader_storage_buffer_array_non_uniform_indexing : <_>::default(),
			shader_storage_image_array_non_uniform_indexing : <_>::default(),
			shader_input_attachment_array_non_uniform_indexing : <_>::default(),
			shader_uniform_texel_buffer_array_non_uniform_indexing : <_>::default(),
			shader_storage_texel_buffer_array_non_uniform_indexing : <_>::default(),
			descriptor_binding_uniform_buffer_update_after_bind : <_>::default(),
			descriptor_binding_sampled_image_update_after_bind : <_>::default(),
			descriptor_binding_storage_image_update_after_bind : <_>::default(),
			descriptor_binding_storage_buffer_update_after_bind : <_>::default(),
			descriptor_binding_uniform_texel_buffer_update_after_bind : <_>::default(),
			descriptor_binding_storage_texel_buffer_update_after_bind : <_>::default(),
			descriptor_binding_update_unused_while_pending : <_>::default(),
			descriptor_binding_partially_bound : <_>::default(),
			descriptor_binding_variable_descriptor_count : <_>::default(),
			runtime_descriptor_array : <_>::default(),
			sampler_filter_minmax : <_>::default(),
			scalar_block_layout : <_>::default(),
			imageless_framebuffer : <_>::default(),
			uniform_buffer_standard_layout : <_>::default(),
			shader_subgroup_extended_types : <_>::default(),
			separate_depth_stencil_layouts : <_>::default(),
			host_query_reset : <_>::default(),
			timeline_semaphore : <_>::default(),
			buffer_device_address : <_>::default(),
			buffer_device_address_capture_replay : <_>::default(),
			buffer_device_address_multi_device : <_>::default(),
			vulkan_memory_model : <_>::default(),
			vulkan_memory_model_device_scope : <_>::default(),
			vulkan_memory_model_availability_visibility_chains : <_>::default(),
			shader_output_viewport_index : <_>::default(),
			shader_output_layer : <_>::default(),
			subgroup_broadcast_dynamic_id : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn sampler_mirror_clamp_to_edge(mut self, val : Bool32) -> Self {
		self.sampler_mirror_clamp_to_edge = val;
		self
	}
	pub fn draw_indirect_count(mut self, val : Bool32) -> Self {
		self.draw_indirect_count = val;
		self
	}
	pub fn storage_buffer_8_bit_access(mut self, val : Bool32) -> Self {
		self.storage_buffer_8_bit_access = val;
		self
	}
	pub fn uniform_and_storage_buffer_8_bit_access(mut self, val : Bool32) -> Self {
		self.uniform_and_storage_buffer_8_bit_access = val;
		self
	}
	pub fn storage_push_constant_8(mut self, val : Bool32) -> Self {
		self.storage_push_constant_8 = val;
		self
	}
	pub fn shader_buffer_int_64_atomics(mut self, val : Bool32) -> Self {
		self.shader_buffer_int_64_atomics = val;
		self
	}
	pub fn shader_shared_int_64_atomics(mut self, val : Bool32) -> Self {
		self.shader_shared_int_64_atomics = val;
		self
	}
	pub fn shader_float_16(mut self, val : Bool32) -> Self {
		self.shader_float_16 = val;
		self
	}
	pub fn shader_int_8(mut self, val : Bool32) -> Self {
		self.shader_int_8 = val;
		self
	}
	pub fn descriptor_indexing(mut self, val : Bool32) -> Self {
		self.descriptor_indexing = val;
		self
	}
	pub fn shader_input_attachment_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_input_attachment_array_dynamic_indexing = val;
		self
	}
	pub fn shader_uniform_texel_buffer_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_uniform_texel_buffer_array_dynamic_indexing = val;
		self
	}
	pub fn shader_storage_texel_buffer_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_texel_buffer_array_dynamic_indexing = val;
		self
	}
	pub fn shader_uniform_buffer_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_uniform_buffer_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_sampled_image_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_sampled_image_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_storage_buffer_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_buffer_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_storage_image_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_image_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_input_attachment_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_input_attachment_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_uniform_texel_buffer_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_storage_texel_buffer_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_texel_buffer_array_non_uniform_indexing = val;
		self
	}
	pub fn descriptor_binding_uniform_buffer_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_uniform_buffer_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_sampled_image_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_sampled_image_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_storage_image_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_storage_image_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_storage_buffer_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_storage_buffer_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_uniform_texel_buffer_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_storage_texel_buffer_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_storage_texel_buffer_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_update_unused_while_pending(mut self, val : Bool32) -> Self {
		self.descriptor_binding_update_unused_while_pending = val;
		self
	}
	pub fn descriptor_binding_partially_bound(mut self, val : Bool32) -> Self {
		self.descriptor_binding_partially_bound = val;
		self
	}
	pub fn descriptor_binding_variable_descriptor_count(mut self, val : Bool32) -> Self {
		self.descriptor_binding_variable_descriptor_count = val;
		self
	}
	pub fn runtime_descriptor_array(mut self, val : Bool32) -> Self {
		self.runtime_descriptor_array = val;
		self
	}
	pub fn sampler_filter_minmax(mut self, val : Bool32) -> Self {
		self.sampler_filter_minmax = val;
		self
	}
	pub fn scalar_block_layout(mut self, val : Bool32) -> Self {
		self.scalar_block_layout = val;
		self
	}
	pub fn imageless_framebuffer(mut self, val : Bool32) -> Self {
		self.imageless_framebuffer = val;
		self
	}
	pub fn uniform_buffer_standard_layout(mut self, val : Bool32) -> Self {
		self.uniform_buffer_standard_layout = val;
		self
	}
	pub fn shader_subgroup_extended_types(mut self, val : Bool32) -> Self {
		self.shader_subgroup_extended_types = val;
		self
	}
	pub fn separate_depth_stencil_layouts(mut self, val : Bool32) -> Self {
		self.separate_depth_stencil_layouts = val;
		self
	}
	pub fn host_query_reset(mut self, val : Bool32) -> Self {
		self.host_query_reset = val;
		self
	}
	pub fn timeline_semaphore(mut self, val : Bool32) -> Self {
		self.timeline_semaphore = val;
		self
	}
	pub fn buffer_device_address(mut self, val : Bool32) -> Self {
		self.buffer_device_address = val;
		self
	}
	pub fn buffer_device_address_capture_replay(mut self, val : Bool32) -> Self {
		self.buffer_device_address_capture_replay = val;
		self
	}
	pub fn buffer_device_address_multi_device(mut self, val : Bool32) -> Self {
		self.buffer_device_address_multi_device = val;
		self
	}
	pub fn vulkan_memory_model(mut self, val : Bool32) -> Self {
		self.vulkan_memory_model = val;
		self
	}
	pub fn vulkan_memory_model_device_scope(mut self, val : Bool32) -> Self {
		self.vulkan_memory_model_device_scope = val;
		self
	}
	pub fn vulkan_memory_model_availability_visibility_chains(mut self, val : Bool32) -> Self {
		self.vulkan_memory_model_availability_visibility_chains = val;
		self
	}
	pub fn shader_output_viewport_index(mut self, val : Bool32) -> Self {
		self.shader_output_viewport_index = val;
		self
	}
	pub fn shader_output_layer(mut self, val : Bool32) -> Self {
		self.shader_output_layer = val;
		self
	}
	pub fn subgroup_broadcast_dynamic_id(mut self, val : Bool32) -> Self {
		self.subgroup_broadcast_dynamic_id = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceVulkan12Features {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ConformanceVersion {
	pub major : u8,
	pub minor : u8,
	pub subminor : u8,
	pub patch : u8,
}

impl ConformanceVersion {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn major(mut self, val : u8) -> Self {
		self.major = val;
		self
	}
	pub fn minor(mut self, val : u8) -> Self {
		self.minor = val;
		self
	}
	pub fn subminor(mut self, val : u8) -> Self {
		self.subminor = val;
		self
	}
	pub fn patch(mut self, val : u8) -> Self {
		self.patch = val;
		self
	}
}

impl std::default::Default for ConformanceVersion {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AttachmentReference {
	pub attachment : u32,
	pub layout : ImageLayout,
}

impl AttachmentReference {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn attachment(mut self, val : u32) -> Self {
		self.attachment = val;
		self
	}
	pub fn layout(mut self, val : ImageLayout) -> Self {
		self.layout = val;
		self
	}
}

impl std::default::Default for AttachmentReference {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceVulkan12Properties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub driver_id : DriverId,
	pub driver_name : [u8; VK_MAX_DRIVER_NAME_SIZE],
	pub driver_info : [u8; VK_MAX_DRIVER_INFO_SIZE],
	pub conformance_version : ConformanceVersion,
	pub denorm_behavior_independence : ShaderFloatControlsIndependence,
	pub rounding_mode_independence : ShaderFloatControlsIndependence,
	pub shader_signed_zero_inf_nan_preserve_float_16 : Bool32,
	pub shader_signed_zero_inf_nan_preserve_float_32 : Bool32,
	pub shader_signed_zero_inf_nan_preserve_float_64 : Bool32,
	pub shader_denorm_preserve_float_16 : Bool32,
	pub shader_denorm_preserve_float_32 : Bool32,
	pub shader_denorm_preserve_float_64 : Bool32,
	pub shader_denorm_flush_to_zero_float_16 : Bool32,
	pub shader_denorm_flush_to_zero_float_32 : Bool32,
	pub shader_denorm_flush_to_zero_float_64 : Bool32,
	pub shader_rounding_mode_rtefloat_16 : Bool32,
	pub shader_rounding_mode_rtefloat_32 : Bool32,
	pub shader_rounding_mode_rtefloat_64 : Bool32,
	pub shader_rounding_mode_rtzfloat_16 : Bool32,
	pub shader_rounding_mode_rtzfloat_32 : Bool32,
	pub shader_rounding_mode_rtzfloat_64 : Bool32,
	pub max_update_after_bind_descriptors_in_all_pools : u32,
	pub shader_uniform_buffer_array_non_uniform_indexing_native : Bool32,
	pub shader_sampled_image_array_non_uniform_indexing_native : Bool32,
	pub shader_storage_buffer_array_non_uniform_indexing_native : Bool32,
	pub shader_storage_image_array_non_uniform_indexing_native : Bool32,
	pub shader_input_attachment_array_non_uniform_indexing_native : Bool32,
	pub robust_buffer_access_update_after_bind : Bool32,
	pub quad_divergent_implicit_lod : Bool32,
	pub max_per_stage_descriptor_update_after_bind_samplers : u32,
	pub max_per_stage_descriptor_update_after_bind_uniform_buffers : u32,
	pub max_per_stage_descriptor_update_after_bind_storage_buffers : u32,
	pub max_per_stage_descriptor_update_after_bind_sampled_images : u32,
	pub max_per_stage_descriptor_update_after_bind_storage_images : u32,
	pub max_per_stage_descriptor_update_after_bind_input_attachments : u32,
	pub max_per_stage_update_after_bind_resources : u32,
	pub max_descriptor_set_update_after_bind_samplers : u32,
	pub max_descriptor_set_update_after_bind_uniform_buffers : u32,
	pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic : u32,
	pub max_descriptor_set_update_after_bind_storage_buffers : u32,
	pub max_descriptor_set_update_after_bind_storage_buffers_dynamic : u32,
	pub max_descriptor_set_update_after_bind_sampled_images : u32,
	pub max_descriptor_set_update_after_bind_storage_images : u32,
	pub max_descriptor_set_update_after_bind_input_attachments : u32,
	pub supported_depth_resolve_modes : ResolveModeFlags,
	pub supported_stencil_resolve_modes : ResolveModeFlags,
	pub independent_resolve_none : Bool32,
	pub independent_resolve : Bool32,
	pub filter_minmax_single_component_formats : Bool32,
	pub filter_minmax_image_component_mapping : Bool32,
	pub max_timeline_semaphore_value_difference : u64,
	pub framebuffer_integer_color_sample_counts : SampleCountFlags,
}

impl PhysicalDeviceVulkan12Properties {
	pub fn new() -> Self {
		Self {
			s_type : 52,
			p_next : null_mut(),
			driver_id : DriverId::default(),
			driver_name : [0 as _ ;VK_MAX_DRIVER_NAME_SIZE],
			driver_info : [0 as _ ;VK_MAX_DRIVER_INFO_SIZE],
			conformance_version : ConformanceVersion::new(),
			denorm_behavior_independence : ShaderFloatControlsIndependence::default(),
			rounding_mode_independence : ShaderFloatControlsIndependence::default(),
			shader_signed_zero_inf_nan_preserve_float_16 : <_>::default(),
			shader_signed_zero_inf_nan_preserve_float_32 : <_>::default(),
			shader_signed_zero_inf_nan_preserve_float_64 : <_>::default(),
			shader_denorm_preserve_float_16 : <_>::default(),
			shader_denorm_preserve_float_32 : <_>::default(),
			shader_denorm_preserve_float_64 : <_>::default(),
			shader_denorm_flush_to_zero_float_16 : <_>::default(),
			shader_denorm_flush_to_zero_float_32 : <_>::default(),
			shader_denorm_flush_to_zero_float_64 : <_>::default(),
			shader_rounding_mode_rtefloat_16 : <_>::default(),
			shader_rounding_mode_rtefloat_32 : <_>::default(),
			shader_rounding_mode_rtefloat_64 : <_>::default(),
			shader_rounding_mode_rtzfloat_16 : <_>::default(),
			shader_rounding_mode_rtzfloat_32 : <_>::default(),
			shader_rounding_mode_rtzfloat_64 : <_>::default(),
			max_update_after_bind_descriptors_in_all_pools : <_>::default(),
			shader_uniform_buffer_array_non_uniform_indexing_native : <_>::default(),
			shader_sampled_image_array_non_uniform_indexing_native : <_>::default(),
			shader_storage_buffer_array_non_uniform_indexing_native : <_>::default(),
			shader_storage_image_array_non_uniform_indexing_native : <_>::default(),
			shader_input_attachment_array_non_uniform_indexing_native : <_>::default(),
			robust_buffer_access_update_after_bind : <_>::default(),
			quad_divergent_implicit_lod : <_>::default(),
			max_per_stage_descriptor_update_after_bind_samplers : <_>::default(),
			max_per_stage_descriptor_update_after_bind_uniform_buffers : <_>::default(),
			max_per_stage_descriptor_update_after_bind_storage_buffers : <_>::default(),
			max_per_stage_descriptor_update_after_bind_sampled_images : <_>::default(),
			max_per_stage_descriptor_update_after_bind_storage_images : <_>::default(),
			max_per_stage_descriptor_update_after_bind_input_attachments : <_>::default(),
			max_per_stage_update_after_bind_resources : <_>::default(),
			max_descriptor_set_update_after_bind_samplers : <_>::default(),
			max_descriptor_set_update_after_bind_uniform_buffers : <_>::default(),
			max_descriptor_set_update_after_bind_uniform_buffers_dynamic : <_>::default(),
			max_descriptor_set_update_after_bind_storage_buffers : <_>::default(),
			max_descriptor_set_update_after_bind_storage_buffers_dynamic : <_>::default(),
			max_descriptor_set_update_after_bind_sampled_images : <_>::default(),
			max_descriptor_set_update_after_bind_storage_images : <_>::default(),
			max_descriptor_set_update_after_bind_input_attachments : <_>::default(),
			supported_depth_resolve_modes : <_>::default(),
			supported_stencil_resolve_modes : <_>::default(),
			independent_resolve_none : <_>::default(),
			independent_resolve : <_>::default(),
			filter_minmax_single_component_formats : <_>::default(),
			filter_minmax_image_component_mapping : <_>::default(),
			max_timeline_semaphore_value_difference : <_>::default(),
			framebuffer_integer_color_sample_counts : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn driver_id(mut self, val : DriverId) -> Self {
		self.driver_id = val;
		self
	}
	pub fn driver_name(mut self, val : [u8; VK_MAX_DRIVER_NAME_SIZE]) -> Self {
		self.driver_name = val;
		self
	}
	pub fn driver_info(mut self, val : [u8; VK_MAX_DRIVER_INFO_SIZE]) -> Self {
		self.driver_info = val;
		self
	}
	pub fn conformance_version(mut self, val : ConformanceVersion) -> Self {
		self.conformance_version = val;
		self
	}
	pub fn denorm_behavior_independence(mut self, val : ShaderFloatControlsIndependence) -> Self {
		self.denorm_behavior_independence = val;
		self
	}
	pub fn rounding_mode_independence(mut self, val : ShaderFloatControlsIndependence) -> Self {
		self.rounding_mode_independence = val;
		self
	}
	pub fn shader_signed_zero_inf_nan_preserve_float_16(mut self, val : Bool32) -> Self {
		self.shader_signed_zero_inf_nan_preserve_float_16 = val;
		self
	}
	pub fn shader_signed_zero_inf_nan_preserve_float_32(mut self, val : Bool32) -> Self {
		self.shader_signed_zero_inf_nan_preserve_float_32 = val;
		self
	}
	pub fn shader_signed_zero_inf_nan_preserve_float_64(mut self, val : Bool32) -> Self {
		self.shader_signed_zero_inf_nan_preserve_float_64 = val;
		self
	}
	pub fn shader_denorm_preserve_float_16(mut self, val : Bool32) -> Self {
		self.shader_denorm_preserve_float_16 = val;
		self
	}
	pub fn shader_denorm_preserve_float_32(mut self, val : Bool32) -> Self {
		self.shader_denorm_preserve_float_32 = val;
		self
	}
	pub fn shader_denorm_preserve_float_64(mut self, val : Bool32) -> Self {
		self.shader_denorm_preserve_float_64 = val;
		self
	}
	pub fn shader_denorm_flush_to_zero_float_16(mut self, val : Bool32) -> Self {
		self.shader_denorm_flush_to_zero_float_16 = val;
		self
	}
	pub fn shader_denorm_flush_to_zero_float_32(mut self, val : Bool32) -> Self {
		self.shader_denorm_flush_to_zero_float_32 = val;
		self
	}
	pub fn shader_denorm_flush_to_zero_float_64(mut self, val : Bool32) -> Self {
		self.shader_denorm_flush_to_zero_float_64 = val;
		self
	}
	pub fn shader_rounding_mode_rtefloat_16(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtefloat_16 = val;
		self
	}
	pub fn shader_rounding_mode_rtefloat_32(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtefloat_32 = val;
		self
	}
	pub fn shader_rounding_mode_rtefloat_64(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtefloat_64 = val;
		self
	}
	pub fn shader_rounding_mode_rtzfloat_16(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtzfloat_16 = val;
		self
	}
	pub fn shader_rounding_mode_rtzfloat_32(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtzfloat_32 = val;
		self
	}
	pub fn shader_rounding_mode_rtzfloat_64(mut self, val : Bool32) -> Self {
		self.shader_rounding_mode_rtzfloat_64 = val;
		self
	}
	pub fn max_update_after_bind_descriptors_in_all_pools(mut self, val : u32) -> Self {
		self.max_update_after_bind_descriptors_in_all_pools = val;
		self
	}
	pub fn shader_uniform_buffer_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_uniform_buffer_array_non_uniform_indexing_native = val;
		self
	}
	pub fn shader_sampled_image_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_sampled_image_array_non_uniform_indexing_native = val;
		self
	}
	pub fn shader_storage_buffer_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_storage_buffer_array_non_uniform_indexing_native = val;
		self
	}
	pub fn shader_storage_image_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_storage_image_array_non_uniform_indexing_native = val;
		self
	}
	pub fn shader_input_attachment_array_non_uniform_indexing_native(mut self, val : Bool32) -> Self {
		self.shader_input_attachment_array_non_uniform_indexing_native = val;
		self
	}
	pub fn robust_buffer_access_update_after_bind(mut self, val : Bool32) -> Self {
		self.robust_buffer_access_update_after_bind = val;
		self
	}
	pub fn quad_divergent_implicit_lod(mut self, val : Bool32) -> Self {
		self.quad_divergent_implicit_lod = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_samplers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_samplers = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_uniform_buffers = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_storage_buffers = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_sampled_images(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_sampled_images = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_storage_images(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_storage_images = val;
		self
	}
	pub fn max_per_stage_descriptor_update_after_bind_input_attachments(mut self, val : u32) -> Self {
		self.max_per_stage_descriptor_update_after_bind_input_attachments = val;
		self
	}
	pub fn max_per_stage_update_after_bind_resources(mut self, val : u32) -> Self {
		self.max_per_stage_update_after_bind_resources = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_samplers(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_samplers = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_uniform_buffers(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_uniform_buffers = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_storage_buffers(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_storage_buffers = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_storage_buffers_dynamic = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_sampled_images(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_sampled_images = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_storage_images(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_storage_images = val;
		self
	}
	pub fn max_descriptor_set_update_after_bind_input_attachments(mut self, val : u32) -> Self {
		self.max_descriptor_set_update_after_bind_input_attachments = val;
		self
	}
	pub fn supported_depth_resolve_modes(mut self, val : ResolveModeFlags) -> Self {
		self.supported_depth_resolve_modes = val;
		self
	}
	pub fn supported_stencil_resolve_modes(mut self, val : ResolveModeFlags) -> Self {
		self.supported_stencil_resolve_modes = val;
		self
	}
	pub fn independent_resolve_none(mut self, val : Bool32) -> Self {
		self.independent_resolve_none = val;
		self
	}
	pub fn independent_resolve(mut self, val : Bool32) -> Self {
		self.independent_resolve = val;
		self
	}
	pub fn filter_minmax_single_component_formats(mut self, val : Bool32) -> Self {
		self.filter_minmax_single_component_formats = val;
		self
	}
	pub fn filter_minmax_image_component_mapping(mut self, val : Bool32) -> Self {
		self.filter_minmax_image_component_mapping = val;
		self
	}
	pub fn max_timeline_semaphore_value_difference(mut self, val : u64) -> Self {
		self.max_timeline_semaphore_value_difference = val;
		self
	}
	pub fn framebuffer_integer_color_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.framebuffer_integer_color_sample_counts = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceVulkan12Properties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CommandBufferBeginInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : CommandBufferUsageFlags,
	pub p_inheritance_info : *const CommandBufferInheritanceInfo,
}

impl CommandBufferBeginInfo {
	pub fn new() -> Self {
		Self {
			s_type : 42,
			p_next : null(),
			flags : <_>::default(),
			p_inheritance_info : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : CommandBufferUsageFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn inheritance_info(mut self, val : *const CommandBufferInheritanceInfo) -> Self {
		self.p_inheritance_info = val;
		self
	}
}

impl std::default::Default for CommandBufferBeginInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubpassBeginInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub contents : SubpassContents,
}

impl SubpassBeginInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000109005,
			p_next : null(),
			contents : SubpassContents::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn contents(mut self, val : SubpassContents) -> Self {
		self.contents = val;
		self
	}
}

impl std::default::Default for SubpassBeginInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_buffer_int_64_atomics : Bool32,
	pub shader_shared_int_64_atomics : Bool32,
}

impl PhysicalDeviceShaderAtomicInt64Features {
	pub fn new() -> Self {
		Self {
			s_type : 1000180000,
			p_next : null_mut(),
			shader_buffer_int_64_atomics : <_>::default(),
			shader_shared_int_64_atomics : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_buffer_int_64_atomics(mut self, val : Bool32) -> Self {
		self.shader_buffer_int_64_atomics = val;
		self
	}
	pub fn shader_shared_int_64_atomics(mut self, val : Bool32) -> Self {
		self.shader_shared_int_64_atomics = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderAtomicInt64Features {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub extended_dynamic_state : Bool32,
}

impl PhysicalDeviceExtendedDynamicStateFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000267000,
			p_next : null_mut(),
			extended_dynamic_state : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn extended_dynamic_state(mut self, val : Bool32) -> Self {
		self.extended_dynamic_state = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExternalMemoryBufferCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub handle_types : ExternalMemoryHandleTypeFlags,
}

impl ExternalMemoryBufferCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000072000,
			p_next : null(),
			handle_types : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn handle_types(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_types = val;
		self
	}
}

impl std::default::Default for ExternalMemoryBufferCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_float_16 : Bool32,
	pub shader_int_8 : Bool32,
}

impl PhysicalDeviceShaderFloat16Int8Features {
	pub fn new() -> Self {
		Self {
			s_type : 1000082000,
			p_next : null_mut(),
			shader_float_16 : <_>::default(),
			shader_int_8 : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_float_16(mut self, val : Bool32) -> Self {
		self.shader_float_16 = val;
		self
	}
	pub fn shader_int_8(mut self, val : Bool32) -> Self {
		self.shader_int_8 = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderFloat16Int8Features {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DispatchIndirectCommand {
	pub x : u32,
	pub y : u32,
	pub z : u32,
}

impl DispatchIndirectCommand {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn x(mut self, val : u32) -> Self {
		self.x = val;
		self
	}
	pub fn y(mut self, val : u32) -> Self {
		self.y = val;
		self
	}
	pub fn z(mut self, val : u32) -> Self {
		self.z = val;
		self
	}
}

impl std::default::Default for DispatchIndirectCommand {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_input_attachment_array_dynamic_indexing : Bool32,
	pub shader_uniform_texel_buffer_array_dynamic_indexing : Bool32,
	pub shader_storage_texel_buffer_array_dynamic_indexing : Bool32,
	pub shader_uniform_buffer_array_non_uniform_indexing : Bool32,
	pub shader_sampled_image_array_non_uniform_indexing : Bool32,
	pub shader_storage_buffer_array_non_uniform_indexing : Bool32,
	pub shader_storage_image_array_non_uniform_indexing : Bool32,
	pub shader_input_attachment_array_non_uniform_indexing : Bool32,
	pub shader_uniform_texel_buffer_array_non_uniform_indexing : Bool32,
	pub shader_storage_texel_buffer_array_non_uniform_indexing : Bool32,
	pub descriptor_binding_uniform_buffer_update_after_bind : Bool32,
	pub descriptor_binding_sampled_image_update_after_bind : Bool32,
	pub descriptor_binding_storage_image_update_after_bind : Bool32,
	pub descriptor_binding_storage_buffer_update_after_bind : Bool32,
	pub descriptor_binding_uniform_texel_buffer_update_after_bind : Bool32,
	pub descriptor_binding_storage_texel_buffer_update_after_bind : Bool32,
	pub descriptor_binding_update_unused_while_pending : Bool32,
	pub descriptor_binding_partially_bound : Bool32,
	pub descriptor_binding_variable_descriptor_count : Bool32,
	pub runtime_descriptor_array : Bool32,
}

impl PhysicalDeviceDescriptorIndexingFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000161001,
			p_next : null_mut(),
			shader_input_attachment_array_dynamic_indexing : <_>::default(),
			shader_uniform_texel_buffer_array_dynamic_indexing : <_>::default(),
			shader_storage_texel_buffer_array_dynamic_indexing : <_>::default(),
			shader_uniform_buffer_array_non_uniform_indexing : <_>::default(),
			shader_sampled_image_array_non_uniform_indexing : <_>::default(),
			shader_storage_buffer_array_non_uniform_indexing : <_>::default(),
			shader_storage_image_array_non_uniform_indexing : <_>::default(),
			shader_input_attachment_array_non_uniform_indexing : <_>::default(),
			shader_uniform_texel_buffer_array_non_uniform_indexing : <_>::default(),
			shader_storage_texel_buffer_array_non_uniform_indexing : <_>::default(),
			descriptor_binding_uniform_buffer_update_after_bind : <_>::default(),
			descriptor_binding_sampled_image_update_after_bind : <_>::default(),
			descriptor_binding_storage_image_update_after_bind : <_>::default(),
			descriptor_binding_storage_buffer_update_after_bind : <_>::default(),
			descriptor_binding_uniform_texel_buffer_update_after_bind : <_>::default(),
			descriptor_binding_storage_texel_buffer_update_after_bind : <_>::default(),
			descriptor_binding_update_unused_while_pending : <_>::default(),
			descriptor_binding_partially_bound : <_>::default(),
			descriptor_binding_variable_descriptor_count : <_>::default(),
			runtime_descriptor_array : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_input_attachment_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_input_attachment_array_dynamic_indexing = val;
		self
	}
	pub fn shader_uniform_texel_buffer_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_uniform_texel_buffer_array_dynamic_indexing = val;
		self
	}
	pub fn shader_storage_texel_buffer_array_dynamic_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_texel_buffer_array_dynamic_indexing = val;
		self
	}
	pub fn shader_uniform_buffer_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_uniform_buffer_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_sampled_image_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_sampled_image_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_storage_buffer_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_buffer_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_storage_image_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_image_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_input_attachment_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_input_attachment_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_uniform_texel_buffer_array_non_uniform_indexing = val;
		self
	}
	pub fn shader_storage_texel_buffer_array_non_uniform_indexing(mut self, val : Bool32) -> Self {
		self.shader_storage_texel_buffer_array_non_uniform_indexing = val;
		self
	}
	pub fn descriptor_binding_uniform_buffer_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_uniform_buffer_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_sampled_image_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_sampled_image_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_storage_image_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_storage_image_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_storage_buffer_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_storage_buffer_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_uniform_texel_buffer_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_storage_texel_buffer_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_storage_texel_buffer_update_after_bind = val;
		self
	}
	pub fn descriptor_binding_update_unused_while_pending(mut self, val : Bool32) -> Self {
		self.descriptor_binding_update_unused_while_pending = val;
		self
	}
	pub fn descriptor_binding_partially_bound(mut self, val : Bool32) -> Self {
		self.descriptor_binding_partially_bound = val;
		self
	}
	pub fn descriptor_binding_variable_descriptor_count(mut self, val : Bool32) -> Self {
		self.descriptor_binding_variable_descriptor_count = val;
		self
	}
	pub fn runtime_descriptor_array(mut self, val : Bool32) -> Self {
		self.runtime_descriptor_array = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDescriptorIndexingFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub descriptor_set_count : u32,
	pub p_descriptor_counts : *const u32,
}

impl DescriptorSetVariableDescriptorCountAllocateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000161003,
			p_next : null(),
			descriptor_set_count : <_>::default(),
			p_descriptor_counts : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn descriptor_counts(mut self, val : &[u32]) -> Self {
		self.descriptor_set_count = val.len() as _;
		self.p_descriptor_counts = val.as_ptr();
		self
	}
	pub fn descriptor_count(mut self, val : &u32) -> Self {
		self.descriptor_set_count = 1;
		self.p_descriptor_counts = val;
		self
	}
}

impl std::default::Default for DescriptorSetVariableDescriptorCountAllocateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub sample_location_sample_counts : SampleCountFlags,
	pub max_sample_location_grid_size : Extent2D,
	pub sample_location_coordinate_range : [f32; 2],
	pub sample_location_sub_pixel_bits : u32,
	pub variable_sample_locations : Bool32,
}

impl PhysicalDeviceSampleLocationsPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000143003,
			p_next : null_mut(),
			sample_location_sample_counts : <_>::default(),
			max_sample_location_grid_size : Extent2D::new(),
			sample_location_coordinate_range : [0 as _ ;2],
			sample_location_sub_pixel_bits : <_>::default(),
			variable_sample_locations : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn sample_location_sample_counts(mut self, val : SampleCountFlags) -> Self {
		self.sample_location_sample_counts = val;
		self
	}
	pub fn max_sample_location_grid_size(mut self, val : Extent2D) -> Self {
		self.max_sample_location_grid_size = val;
		self
	}
	pub fn sample_location_coordinate_range(mut self, val : [f32; 2]) -> Self {
		self.sample_location_coordinate_range = val;
		self
	}
	pub fn sample_location_sub_pixel_bits(mut self, val : u32) -> Self {
		self.sample_location_sub_pixel_bits = val;
		self
	}
	pub fn variable_sample_locations(mut self, val : Bool32) -> Self {
		self.variable_sample_locations = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSampleLocationsPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub supported_depth_resolve_modes : ResolveModeFlags,
	pub supported_stencil_resolve_modes : ResolveModeFlags,
	pub independent_resolve_none : Bool32,
	pub independent_resolve : Bool32,
}

impl PhysicalDeviceDepthStencilResolveProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000199000,
			p_next : null_mut(),
			supported_depth_resolve_modes : <_>::default(),
			supported_stencil_resolve_modes : <_>::default(),
			independent_resolve_none : <_>::default(),
			independent_resolve : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn supported_depth_resolve_modes(mut self, val : ResolveModeFlags) -> Self {
		self.supported_depth_resolve_modes = val;
		self
	}
	pub fn supported_stencil_resolve_modes(mut self, val : ResolveModeFlags) -> Self {
		self.supported_stencil_resolve_modes = val;
		self
	}
	pub fn independent_resolve_none(mut self, val : Bool32) -> Self {
		self.independent_resolve_none = val;
		self
	}
	pub fn independent_resolve(mut self, val : Bool32) -> Self {
		self.independent_resolve = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDepthStencilResolveProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorPoolSize {
	pub r#type : DescriptorType,
	pub descriptor_count : u32,
}

impl DescriptorPoolSize {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn r#type(mut self, val : DescriptorType) -> Self {
		self.r#type = val;
		self
	}
	pub fn descriptor_count(mut self, val : u32) -> Self {
		self.descriptor_count = val;
		self
	}
}

impl std::default::Default for DescriptorPoolSize {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub scalar_block_layout : Bool32,
}

impl PhysicalDeviceScalarBlockLayoutFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000221000,
			p_next : null_mut(),
			scalar_block_layout : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn scalar_block_layout(mut self, val : Bool32) -> Self {
		self.scalar_block_layout = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceScalarBlockLayoutFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassCreateInfo2 {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : RenderPassCreateFlags,
	pub attachment_count : u32,
	pub p_attachments : *const AttachmentDescription2,
	pub subpass_count : u32,
	pub p_subpasses : *const SubpassDescription2,
	pub dependency_count : u32,
	pub p_dependencies : *const SubpassDependency2,
	pub correlated_view_mask_count : u32,
	pub p_correlated_view_masks : *const u32,
}

impl RenderPassCreateInfo2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000109004,
			p_next : null(),
			flags : <_>::default(),
			attachment_count : <_>::default(),
			p_attachments : null(),
			subpass_count : <_>::default(),
			p_subpasses : null(),
			dependency_count : <_>::default(),
			p_dependencies : null(),
			correlated_view_mask_count : <_>::default(),
			p_correlated_view_masks : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : RenderPassCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn attachments(mut self, val : &[AttachmentDescription2]) -> Self {
		self.attachment_count = val.len() as _;
		self.p_attachments = val.as_ptr();
		self
	}
	pub fn attachment(mut self, val : &AttachmentDescription2) -> Self {
		self.attachment_count = 1;
		self.p_attachments = val;
		self
	}
	pub fn subpasses(mut self, val : &[SubpassDescription2]) -> Self {
		self.subpass_count = val.len() as _;
		self.p_subpasses = val.as_ptr();
		self
	}
	pub fn subpasse(mut self, val : &SubpassDescription2) -> Self {
		self.subpass_count = 1;
		self.p_subpasses = val;
		self
	}
	pub fn dependencies(mut self, val : &[SubpassDependency2]) -> Self {
		self.dependency_count = val.len() as _;
		self.p_dependencies = val.as_ptr();
		self
	}
	pub fn dependency(mut self, val : &SubpassDependency2) -> Self {
		self.dependency_count = 1;
		self.p_dependencies = val;
		self
	}
	pub fn correlated_view_masks(mut self, val : &[u32]) -> Self {
		self.correlated_view_mask_count = val.len() as _;
		self.p_correlated_view_masks = val.as_ptr();
		self
	}
	pub fn correlated_view_mask(mut self, val : &u32) -> Self {
		self.correlated_view_mask_count = 1;
		self.p_correlated_view_masks = val;
		self
	}
}

impl std::default::Default for RenderPassCreateInfo2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineColorBlendAttachmentState {
	pub blend_enable : Bool32,
	pub src_color_blend_factor : BlendFactor,
	pub dst_color_blend_factor : BlendFactor,
	pub color_blend_op : BlendOp,
	pub src_alpha_blend_factor : BlendFactor,
	pub dst_alpha_blend_factor : BlendFactor,
	pub alpha_blend_op : BlendOp,
	pub color_write_mask : ColorComponentFlags,
}

impl PipelineColorBlendAttachmentState {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn blend_enable(mut self, val : Bool32) -> Self {
		self.blend_enable = val;
		self
	}
	pub fn src_color_blend_factor(mut self, val : BlendFactor) -> Self {
		self.src_color_blend_factor = val;
		self
	}
	pub fn dst_color_blend_factor(mut self, val : BlendFactor) -> Self {
		self.dst_color_blend_factor = val;
		self
	}
	pub fn color_blend_op(mut self, val : BlendOp) -> Self {
		self.color_blend_op = val;
		self
	}
	pub fn src_alpha_blend_factor(mut self, val : BlendFactor) -> Self {
		self.src_alpha_blend_factor = val;
		self
	}
	pub fn dst_alpha_blend_factor(mut self, val : BlendFactor) -> Self {
		self.dst_alpha_blend_factor = val;
		self
	}
	pub fn alpha_blend_op(mut self, val : BlendOp) -> Self {
		self.alpha_blend_op = val;
		self
	}
	pub fn color_write_mask(mut self, val : ColorComponentFlags) -> Self {
		self.color_write_mask = val;
		self
	}
}

impl std::default::Default for PipelineColorBlendAttachmentState {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineTessellationStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineTessellationStateCreateFlags,
	pub patch_control_points : u32,
}

impl PipelineTessellationStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 21,
			p_next : null(),
			flags : <_>::default(),
			patch_control_points : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineTessellationStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn patch_control_points(mut self, val : u32) -> Self {
		self.patch_control_points = val;
		self
	}
}

impl std::default::Default for PipelineTessellationStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageStencilUsageCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub stencil_usage : ImageUsageFlags,
}

impl ImageStencilUsageCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000246000,
			p_next : null(),
			stencil_usage : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn stencil_usage(mut self, val : ImageUsageFlags) -> Self {
		self.stencil_usage = val;
		self
	}
}

impl std::default::Default for ImageStencilUsageCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub imageless_framebuffer : Bool32,
}

impl PhysicalDeviceImagelessFramebufferFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000108000,
			p_next : null_mut(),
			imageless_framebuffer : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn imageless_framebuffer(mut self, val : Bool32) -> Self {
		self.imageless_framebuffer = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceImagelessFramebufferFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AccelerationStructureInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub r#type : AccelerationStructureTypeNV,
	pub flags : BuildAccelerationStructureFlagsNV,
	pub instance_count : u32,
	pub geometry_count : u32,
	pub p_geometries : *const GeometryNV,
}

impl AccelerationStructureInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000165012,
			p_next : null(),
			r#type : AccelerationStructureTypeKHR::default(),
			flags : <_>::default(),
			instance_count : <_>::default(),
			geometry_count : <_>::default(),
			p_geometries : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn r#type(mut self, val : AccelerationStructureTypeNV) -> Self {
		self.r#type = val;
		self
	}
	pub fn flags(mut self, val : BuildAccelerationStructureFlagsNV) -> Self {
		self.flags = val;
		self
	}
	pub fn instance_count(mut self, val : u32) -> Self {
		self.instance_count = val;
		self
	}
	pub fn geometries(mut self, val : &[GeometryNV]) -> Self {
		self.geometry_count = val.len() as _;
		self.p_geometries = val.as_ptr();
		self
	}
	pub fn geometry(mut self, val : &GeometryNV) -> Self {
		self.geometry_count = 1;
		self.p_geometries = val;
		self
	}
}

impl std::default::Default for AccelerationStructureInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineInputAssemblyStateCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineInputAssemblyStateCreateFlags,
	pub topology : PrimitiveTopology,
	pub primitive_restart_enable : Bool32,
}

impl PipelineInputAssemblyStateCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 20,
			p_next : null(),
			flags : <_>::default(),
			topology : PrimitiveTopology::default(),
			primitive_restart_enable : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineInputAssemblyStateCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn topology(mut self, val : PrimitiveTopology) -> Self {
		self.topology = val;
		self
	}
	pub fn primitive_restart_enable(mut self, val : Bool32) -> Self {
		self.primitive_restart_enable = val;
		self
	}
}

impl std::default::Default for PipelineInputAssemblyStateCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FramebufferAttachmentsCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub attachment_image_info_count : u32,
	pub p_attachment_image_infos : *const FramebufferAttachmentImageInfo,
}

impl FramebufferAttachmentsCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000108001,
			p_next : null(),
			attachment_image_info_count : <_>::default(),
			p_attachment_image_infos : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn attachment_image_infos(mut self, val : &[FramebufferAttachmentImageInfo]) -> Self {
		self.attachment_image_info_count = val.len() as _;
		self.p_attachment_image_infos = val.as_ptr();
		self
	}
	pub fn attachment_image_info(mut self, val : &FramebufferAttachmentImageInfo) -> Self {
		self.attachment_image_info_count = 1;
		self.p_attachment_image_infos = val;
		self
	}
}

impl std::default::Default for FramebufferAttachmentsCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub sample_locations_enable : Bool32,
	pub sample_locations_info : SampleLocationsInfoEXT,
}

impl PipelineSampleLocationsStateCreateInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000143002,
			p_next : null(),
			sample_locations_enable : <_>::default(),
			sample_locations_info : SampleLocationsInfoEXT::new(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn sample_locations_enable(mut self, val : Bool32) -> Self {
		self.sample_locations_enable = val;
		self
	}
	pub fn sample_locations_info(mut self, val : SampleLocationsInfoEXT) -> Self {
		self.sample_locations_info = val;
		self
	}
}

impl std::default::Default for PipelineSampleLocationsStateCreateInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPassAttachmentBeginInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub attachment_count : u32,
	pub p_attachments : *const ImageView,
}

impl RenderPassAttachmentBeginInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000108003,
			p_next : null(),
			attachment_count : <_>::default(),
			p_attachments : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn attachments(mut self, val : &[ImageView]) -> Self {
		self.attachment_count = val.len() as _;
		self.p_attachments = val.as_ptr();
		self
	}
	pub fn attachment(mut self, val : &ImageView) -> Self {
		self.attachment_count = 1;
		self.p_attachments = val;
		self
	}
}

impl std::default::Default for RenderPassAttachmentBeginInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub uniform_buffer_standard_layout : Bool32,
}

impl PhysicalDeviceUniformBufferStandardLayoutFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000253000,
			p_next : null_mut(),
			uniform_buffer_standard_layout : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn uniform_buffer_standard_layout(mut self, val : Bool32) -> Self {
		self.uniform_buffer_standard_layout = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceUniformBufferStandardLayoutFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_subgroup_extended_types : Bool32,
}

impl PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000175000,
			p_next : null_mut(),
			shader_subgroup_extended_types : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_subgroup_extended_types(mut self, val : Bool32) -> Self {
		self.shader_subgroup_extended_types = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DrawIndirectCommand {
	pub vertex_count : u32,
	pub instance_count : u32,
	pub first_vertex : u32,
	pub first_instance : u32,
}

impl DrawIndirectCommand {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn vertex_count(mut self, val : u32) -> Self {
		self.vertex_count = val;
		self
	}
	pub fn instance_count(mut self, val : u32) -> Self {
		self.instance_count = val;
		self
	}
	pub fn first_vertex(mut self, val : u32) -> Self {
		self.first_vertex = val;
		self
	}
	pub fn first_instance(mut self, val : u32) -> Self {
		self.first_instance = val;
		self
	}
}

impl std::default::Default for DrawIndirectCommand {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub protected_memory : Bool32,
}

impl PhysicalDeviceProtectedMemoryFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000145001,
			p_next : null_mut(),
			protected_memory : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn protected_memory(mut self, val : Bool32) -> Self {
		self.protected_memory = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceProtectedMemoryFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub separate_depth_stencil_layouts : Bool32,
}

impl PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000241000,
			p_next : null_mut(),
			separate_depth_stencil_layouts : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn separate_depth_stencil_layouts(mut self, val : Bool32) -> Self {
		self.separate_depth_stencil_layouts = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PipelineCacheCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : PipelineCacheCreateFlags,
	pub initial_data_size : usize,
	pub p_initial_data : *const c_void,
}

impl PipelineCacheCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 17,
			p_next : null(),
			flags : <_>::default(),
			initial_data_size : <_>::default(),
			p_initial_data : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : PipelineCacheCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn initial_data_size(mut self, val : usize) -> Self {
		self.initial_data_size = val;
		self
	}
	pub fn initial_data(mut self, val : *const c_void) -> Self {
		self.p_initial_data = val;
		self
	}
}

impl std::default::Default for PipelineCacheCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AttachmentReferenceStencilLayout {
	s_type : i32,
	pub p_next : *mut c_void,
	pub stencil_layout : ImageLayout,
}

impl AttachmentReferenceStencilLayout {
	pub fn new() -> Self {
		Self {
			s_type : 1000241001,
			p_next : null_mut(),
			stencil_layout : ImageLayout::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn stencil_layout(mut self, val : ImageLayout) -> Self {
		self.stencil_layout = val;
		self
	}
}

impl std::default::Default for AttachmentReferenceStencilLayout {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub diagnostics_config : Bool32,
}

impl PhysicalDeviceDiagnosticsConfigFeaturesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000300000,
			p_next : null_mut(),
			diagnostics_config : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn diagnostics_config(mut self, val : Bool32) -> Self {
		self.diagnostics_config = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceDiagnosticsConfigFeaturesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryGetFdInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub memory : DeviceMemory,
	pub handle_type : ExternalMemoryHandleTypeFlags,
}

impl MemoryGetFdInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000074002,
			p_next : null(),
			memory : DeviceMemory(0),
			handle_type : ExternalMemoryHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalMemoryHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for MemoryGetFdInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AttachmentDescriptionStencilLayout {
	s_type : i32,
	pub p_next : *mut c_void,
	pub stencil_initial_layout : ImageLayout,
	pub stencil_final_layout : ImageLayout,
}

impl AttachmentDescriptionStencilLayout {
	pub fn new() -> Self {
		Self {
			s_type : 1000241002,
			p_next : null_mut(),
			stencil_initial_layout : ImageLayout::default(),
			stencil_final_layout : ImageLayout::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn stencil_initial_layout(mut self, val : ImageLayout) -> Self {
		self.stencil_initial_layout = val;
		self
	}
	pub fn stencil_final_layout(mut self, val : ImageLayout) -> Self {
		self.stencil_final_layout = val;
		self
	}
}

impl std::default::Default for AttachmentDescriptionStencilLayout {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
	s_type : i32,
	pub p_next : *const c_void,
	pub image : Image,
	pub buffer : Buffer,
}

impl DedicatedAllocationMemoryAllocateInfoNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000026002,
			p_next : null(),
			image : Image(0),
			buffer : Buffer(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn image(mut self, val : Image) -> Self {
		self.image = val;
		self
	}
	pub fn buffer(mut self, val : Buffer) -> Self {
		self.buffer = val;
		self
	}
}

impl std::default::Default for DedicatedAllocationMemoryAllocateInfoNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DebugMarkerObjectNameInfoEXT {
	s_type : i32,
	pub p_next : *const c_void,
	pub object_type : DebugReportObjectTypeEXT,
	pub object : u64,
	pub p_object_name : *const u8,
}

impl DebugMarkerObjectNameInfoEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000022000,
			p_next : null(),
			object_type : DebugReportObjectTypeEXT::default(),
			object : <_>::default(),
			p_object_name : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn object_type(mut self, val : DebugReportObjectTypeEXT) -> Self {
		self.object_type = val;
		self
	}
	pub fn object(mut self, val : u64) -> Self {
		self.object = val;
		self
	}
	pub fn object_name(mut self, val : *const u8) -> Self {
		self.p_object_name = val;
		self
	}
}

impl std::default::Default for DebugMarkerObjectNameInfoEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceHostQueryResetFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub host_query_reset : Bool32,
}

impl PhysicalDeviceHostQueryResetFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000261000,
			p_next : null_mut(),
			host_query_reset : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn host_query_reset(mut self, val : Bool32) -> Self {
		self.host_query_reset = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceHostQueryResetFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
	s_type : i32,
	pub p_next : *mut c_void,
	pub timeline_semaphore : Bool32,
}

impl PhysicalDeviceTimelineSemaphoreFeatures {
	pub fn new() -> Self {
		Self {
			s_type : 1000207000,
			p_next : null_mut(),
			timeline_semaphore : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn timeline_semaphore(mut self, val : Bool32) -> Self {
		self.timeline_semaphore = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceTimelineSemaphoreFeatures {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MemoryWin32HandlePropertiesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub memory_type_bits : u32,
}

impl MemoryWin32HandlePropertiesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000073002,
			p_next : null_mut(),
			memory_type_bits : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory_type_bits(mut self, val : u32) -> Self {
		self.memory_type_bits = val;
		self
	}
}

impl std::default::Default for MemoryWin32HandlePropertiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CooperativeMatrixPropertiesNV {
	s_type : i32,
	pub p_next : *mut c_void,
	pub m_size : u32,
	pub n_size : u32,
	pub k_size : u32,
	pub a_type : ComponentTypeNV,
	pub b_type : ComponentTypeNV,
	pub c_type : ComponentTypeNV,
	pub d_type : ComponentTypeNV,
	pub scope : ScopeNV,
}

impl CooperativeMatrixPropertiesNV {
	pub fn new() -> Self {
		Self {
			s_type : 1000249001,
			p_next : null_mut(),
			m_size : <_>::default(),
			n_size : <_>::default(),
			k_size : <_>::default(),
			a_type : ComponentTypeNV::default(),
			b_type : ComponentTypeNV::default(),
			c_type : ComponentTypeNV::default(),
			d_type : ComponentTypeNV::default(),
			scope : ScopeNV::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn m_size(mut self, val : u32) -> Self {
		self.m_size = val;
		self
	}
	pub fn n_size(mut self, val : u32) -> Self {
		self.n_size = val;
		self
	}
	pub fn k_size(mut self, val : u32) -> Self {
		self.k_size = val;
		self
	}
	pub fn a_type(mut self, val : ComponentTypeNV) -> Self {
		self.a_type = val;
		self
	}
	pub fn b_type(mut self, val : ComponentTypeNV) -> Self {
		self.b_type = val;
		self
	}
	pub fn c_type(mut self, val : ComponentTypeNV) -> Self {
		self.c_type = val;
		self
	}
	pub fn d_type(mut self, val : ComponentTypeNV) -> Self {
		self.d_type = val;
		self
	}
	pub fn scope(mut self, val : ScopeNV) -> Self {
		self.scope = val;
		self
	}
}

impl std::default::Default for CooperativeMatrixPropertiesNV {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : ImageCreateFlags,
	pub image_type : ImageType,
	pub format : Format,
	pub extent : Extent3D,
	pub mip_levels : u32,
	pub array_layers : u32,
	pub samples : SampleCountFlags,
	pub tiling : ImageTiling,
	pub usage : ImageUsageFlags,
	pub sharing_mode : SharingMode,
	pub queue_family_index_count : u32,
	pub p_queue_family_indices : *const u32,
	pub initial_layout : ImageLayout,
}

impl ImageCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 14,
			p_next : null(),
			flags : <_>::default(),
			image_type : ImageType::default(),
			format : Format::default(),
			extent : Extent3D::new(),
			mip_levels : <_>::default(),
			array_layers : <_>::default(),
			samples : SampleCountFlags::default(),
			tiling : ImageTiling::default(),
			usage : <_>::default(),
			sharing_mode : SharingMode::default(),
			queue_family_index_count : <_>::default(),
			p_queue_family_indices : null(),
			initial_layout : ImageLayout::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : ImageCreateFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn image_type(mut self, val : ImageType) -> Self {
		self.image_type = val;
		self
	}
	pub fn format(mut self, val : Format) -> Self {
		self.format = val;
		self
	}
	pub fn extent(mut self, val : Extent3D) -> Self {
		self.extent = val;
		self
	}
	pub fn mip_levels(mut self, val : u32) -> Self {
		self.mip_levels = val;
		self
	}
	pub fn array_layers(mut self, val : u32) -> Self {
		self.array_layers = val;
		self
	}
	pub fn samples(mut self, val : SampleCountFlags) -> Self {
		self.samples = val;
		self
	}
	pub fn tiling(mut self, val : ImageTiling) -> Self {
		self.tiling = val;
		self
	}
	pub fn usage(mut self, val : ImageUsageFlags) -> Self {
		self.usage = val;
		self
	}
	pub fn sharing_mode(mut self, val : SharingMode) -> Self {
		self.sharing_mode = val;
		self
	}
	pub fn queue_family_indices(mut self, val : &[u32]) -> Self {
		self.queue_family_index_count = val.len() as _;
		self.p_queue_family_indices = val.as_ptr();
		self
	}
	pub fn queue_family_indice(mut self, val : &u32) -> Self {
		self.queue_family_index_count = 1;
		self.p_queue_family_indices = val;
		self
	}
}

impl std::default::Default for ImageCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct QueueFamilyProperties2 {
	s_type : i32,
	pub p_next : *mut c_void,
	pub queue_family_properties : QueueFamilyProperties,
}

impl QueueFamilyProperties2 {
	pub fn new() -> Self {
		Self {
			s_type : 1000059005,
			p_next : null_mut(),
			queue_family_properties : QueueFamilyProperties::new(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn queue_family_properties(mut self, val : QueueFamilyProperties) -> Self {
		self.queue_family_properties = val;
		self
	}
}

impl std::default::Default for QueueFamilyProperties2 {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_timeline_semaphore_value_difference : u64,
}

impl PhysicalDeviceTimelineSemaphoreProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000207001,
			p_next : null_mut(),
			max_timeline_semaphore_value_difference : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_timeline_semaphore_value_difference(mut self, val : u64) -> Self {
		self.max_timeline_semaphore_value_difference = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceTimelineSemaphoreProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SwapchainCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : SwapchainCreateFlagsKHR,
	pub surface : SurfaceKHR,
	pub min_image_count : u32,
	pub image_format : Format,
	pub image_color_space : ColorSpaceKHR,
	pub image_extent : Extent2D,
	pub image_array_layers : u32,
	pub image_usage : ImageUsageFlags,
	pub image_sharing_mode : SharingMode,
	pub queue_family_index_count : u32,
	pub p_queue_family_indices : *const u32,
	pub pre_transform : SurfaceTransformFlagsKHR,
	pub composite_alpha : CompositeAlphaFlagsKHR,
	pub present_mode : PresentModeKHR,
	pub clipped : Bool32,
	pub old_swapchain : SwapchainKHR,
}

impl SwapchainCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000001000,
			p_next : null(),
			flags : <_>::default(),
			surface : SurfaceKHR(0),
			min_image_count : <_>::default(),
			image_format : Format::default(),
			image_color_space : ColorSpaceKHR::default(),
			image_extent : Extent2D::new(),
			image_array_layers : <_>::default(),
			image_usage : <_>::default(),
			image_sharing_mode : SharingMode::default(),
			queue_family_index_count : <_>::default(),
			p_queue_family_indices : null(),
			pre_transform : SurfaceTransformFlagsKHR::default(),
			composite_alpha : CompositeAlphaFlagsKHR::default(),
			present_mode : PresentModeKHR::default(),
			clipped : <_>::default(),
			old_swapchain : SwapchainKHR(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : SwapchainCreateFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn surface(mut self, val : SurfaceKHR) -> Self {
		self.surface = val;
		self
	}
	pub fn min_image_count(mut self, val : u32) -> Self {
		self.min_image_count = val;
		self
	}
	pub fn image_format(mut self, val : Format) -> Self {
		self.image_format = val;
		self
	}
	pub fn image_color_space(mut self, val : ColorSpaceKHR) -> Self {
		self.image_color_space = val;
		self
	}
	pub fn image_extent(mut self, val : Extent2D) -> Self {
		self.image_extent = val;
		self
	}
	pub fn image_array_layers(mut self, val : u32) -> Self {
		self.image_array_layers = val;
		self
	}
	pub fn image_usage(mut self, val : ImageUsageFlags) -> Self {
		self.image_usage = val;
		self
	}
	pub fn image_sharing_mode(mut self, val : SharingMode) -> Self {
		self.image_sharing_mode = val;
		self
	}
	pub fn pre_transform(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.pre_transform = val;
		self
	}
	pub fn composite_alpha(mut self, val : CompositeAlphaFlagsKHR) -> Self {
		self.composite_alpha = val;
		self
	}
	pub fn present_mode(mut self, val : PresentModeKHR) -> Self {
		self.present_mode = val;
		self
	}
	pub fn clipped(mut self, val : Bool32) -> Self {
		self.clipped = val;
		self
	}
	pub fn old_swapchain(mut self, val : SwapchainKHR) -> Self {
		self.old_swapchain = val;
		self
	}
	pub fn queue_family_indices(mut self, val : &[u32]) -> Self {
		self.queue_family_index_count = val.len() as _;
		self.p_queue_family_indices = val.as_ptr();
		self
	}
	pub fn queue_family_indice(mut self, val : &u32) -> Self {
		self.queue_family_index_count = 1;
		self.p_queue_family_indices = val;
		self
	}
}

impl std::default::Default for SwapchainCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SemaphoreTypeCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub semaphore_type : SemaphoreType,
	pub initial_value : u64,
}

impl SemaphoreTypeCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000207002,
			p_next : null(),
			semaphore_type : SemaphoreType::default(),
			initial_value : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn semaphore_type(mut self, val : SemaphoreType) -> Self {
		self.semaphore_type = val;
		self
	}
	pub fn initial_value(mut self, val : u64) -> Self {
		self.initial_value = val;
		self
	}
}

impl std::default::Default for SemaphoreTypeCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone)]
pub struct PipelineExecutableInternalRepresentationKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub name : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub description : [u8; VK_MAX_DESCRIPTION_SIZE],
	pub is_text : Bool32,
	pub data_size : usize,
	pub p_data : *mut c_void,
}

impl PipelineExecutableInternalRepresentationKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000269005,
			p_next : null_mut(),
			name : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			description : [0 as _ ;VK_MAX_DESCRIPTION_SIZE],
			is_text : <_>::default(),
			data_size : <_>::default(),
			p_data : null_mut(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn name(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.name = val;
		self
	}
	pub fn description(mut self, val : [u8; VK_MAX_DESCRIPTION_SIZE]) -> Self {
		self.description = val;
		self
	}
	pub fn is_text(mut self, val : Bool32) -> Self {
		self.is_text = val;
		self
	}
	pub fn data_size(mut self, val : usize) -> Self {
		self.data_size = val;
		self
	}
	pub fn data(mut self, val : *mut c_void) -> Self {
		self.p_data = val;
		self
	}
}

impl std::default::Default for PipelineExecutableInternalRepresentationKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SemaphoreWaitInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : SemaphoreWaitFlags,
	pub semaphore_count : u32,
	pub p_semaphores : *const Semaphore,
	pub p_values : *const u64,
}

impl SemaphoreWaitInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000207004,
			p_next : null(),
			flags : <_>::default(),
			semaphore_count : <_>::default(),
			p_semaphores : null(),
			p_values : null(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : SemaphoreWaitFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn semaphores(mut self, val : &[Semaphore]) -> Self {
		self.semaphore_count = val.len() as _;
		self.p_semaphores = val.as_ptr();
		self
	}
	pub fn semaphore(mut self, val : &Semaphore) -> Self {
		self.semaphore_count = 1;
		self.p_semaphores = val;
		self
	}
}

impl std::default::Default for SemaphoreWaitInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub opaque_capture_address : u64,
}

impl BufferOpaqueCaptureAddressCreateInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000257002,
			p_next : null(),
			opaque_capture_address : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn opaque_capture_address(mut self, val : u64) -> Self {
		self.opaque_capture_address = val;
		self
	}
}

impl std::default::Default for BufferOpaqueCaptureAddressCreateInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub max_custom_border_color_samplers : u32,
}

impl PhysicalDeviceCustomBorderColorPropertiesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000287001,
			p_next : null_mut(),
			max_custom_border_color_samplers : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn max_custom_border_color_samplers(mut self, val : u32) -> Self {
		self.max_custom_border_color_samplers = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceCustomBorderColorPropertiesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
	s_type : i32,
	pub p_next : *const c_void,
	pub memory : DeviceMemory,
}

impl DeviceMemoryOpaqueCaptureAddressInfo {
	pub fn new() -> Self {
		Self {
			s_type : 1000257004,
			p_next : null(),
			memory : DeviceMemory(0),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
}

impl std::default::Default for DeviceMemoryOpaqueCaptureAddressInfo {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SurfaceCapabilitiesKHR {
	pub min_image_count : u32,
	pub max_image_count : u32,
	pub current_extent : Extent2D,
	pub min_image_extent : Extent2D,
	pub max_image_extent : Extent2D,
	pub max_image_array_layers : u32,
	pub supported_transforms : SurfaceTransformFlagsKHR,
	pub current_transform : SurfaceTransformFlagsKHR,
	pub supported_composite_alpha : CompositeAlphaFlagsKHR,
	pub supported_usage_flags : ImageUsageFlags,
}

impl SurfaceCapabilitiesKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn min_image_count(mut self, val : u32) -> Self {
		self.min_image_count = val;
		self
	}
	pub fn max_image_count(mut self, val : u32) -> Self {
		self.max_image_count = val;
		self
	}
	pub fn current_extent(mut self, val : Extent2D) -> Self {
		self.current_extent = val;
		self
	}
	pub fn min_image_extent(mut self, val : Extent2D) -> Self {
		self.min_image_extent = val;
		self
	}
	pub fn max_image_extent(mut self, val : Extent2D) -> Self {
		self.max_image_extent = val;
		self
	}
	pub fn max_image_array_layers(mut self, val : u32) -> Self {
		self.max_image_array_layers = val;
		self
	}
	pub fn supported_transforms(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.supported_transforms = val;
		self
	}
	pub fn current_transform(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.current_transform = val;
		self
	}
	pub fn supported_composite_alpha(mut self, val : CompositeAlphaFlagsKHR) -> Self {
		self.supported_composite_alpha = val;
		self
	}
	pub fn supported_usage_flags(mut self, val : ImageUsageFlags) -> Self {
		self.supported_usage_flags = val;
		self
	}
}

impl std::default::Default for SurfaceCapabilitiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BindImageMemorySwapchainInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub swapchain : SwapchainKHR,
	pub image_index : u32,
}

impl BindImageMemorySwapchainInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000060009,
			p_next : null(),
			swapchain : SwapchainKHR(0),
			image_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn swapchain(mut self, val : SwapchainKHR) -> Self {
		self.swapchain = val;
		self
	}
	pub fn image_index(mut self, val : u32) -> Self {
		self.image_index = val;
		self
	}
}

impl std::default::Default for BindImageMemorySwapchainInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
	s_type : i32,
	pub p_next : *mut c_void,
	pub shader_subgroup_clock : Bool32,
	pub shader_device_clock : Bool32,
}

impl PhysicalDeviceShaderClockFeaturesKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000181000,
			p_next : null_mut(),
			shader_subgroup_clock : <_>::default(),
			shader_device_clock : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn shader_subgroup_clock(mut self, val : Bool32) -> Self {
		self.shader_subgroup_clock = val;
		self
	}
	pub fn shader_device_clock(mut self, val : Bool32) -> Self {
		self.shader_device_clock = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceShaderClockFeaturesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AcquireNextImageInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub swapchain : SwapchainKHR,
	pub timeout : u64,
	pub semaphore : Semaphore,
	pub fence : Fence,
	pub device_mask : u32,
}

impl AcquireNextImageInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000060010,
			p_next : null(),
			swapchain : SwapchainKHR(0),
			timeout : <_>::default(),
			semaphore : Semaphore(0),
			fence : Fence(0),
			device_mask : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn swapchain(mut self, val : SwapchainKHR) -> Self {
		self.swapchain = val;
		self
	}
	pub fn timeout(mut self, val : u64) -> Self {
		self.timeout = val;
		self
	}
	pub fn semaphore(mut self, val : Semaphore) -> Self {
		self.semaphore = val;
		self
	}
	pub fn fence(mut self, val : Fence) -> Self {
		self.fence = val;
		self
	}
	pub fn device_mask(mut self, val : u32) -> Self {
		self.device_mask = val;
		self
	}
}

impl std::default::Default for AcquireNextImageInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub modes : DeviceGroupPresentModeFlagsKHR,
}

impl DeviceGroupSwapchainCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000060012,
			p_next : null(),
			modes : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn modes(mut self, val : DeviceGroupPresentModeFlagsKHR) -> Self {
		self.modes = val;
		self
	}
}

impl std::default::Default for DeviceGroupSwapchainCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubpassDescription {
	pub flags : SubpassDescriptionFlags,
	pub pipeline_bind_point : PipelineBindPoint,
	pub input_attachment_count : u32,
	pub p_input_attachments : *const AttachmentReference,
	pub color_attachment_count : u32,
	pub p_color_attachments : *const AttachmentReference,
	pub p_resolve_attachments : *const AttachmentReference,
	pub p_depth_stencil_attachment : *const AttachmentReference,
	pub preserve_attachment_count : u32,
	pub p_preserve_attachments : *const u32,
}

impl SubpassDescription {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn flags(mut self, val : SubpassDescriptionFlags) -> Self {
		self.flags = val;
		self
	}
	pub fn pipeline_bind_point(mut self, val : PipelineBindPoint) -> Self {
		self.pipeline_bind_point = val;
		self
	}
	pub fn resolve_attachments(mut self, val : *const AttachmentReference) -> Self {
		self.p_resolve_attachments = val;
		self
	}
	pub fn depth_stencil_attachment(mut self, val : *const AttachmentReference) -> Self {
		self.p_depth_stencil_attachment = val;
		self
	}
	pub fn input_attachments(mut self, val : &[AttachmentReference]) -> Self {
		self.input_attachment_count = val.len() as _;
		self.p_input_attachments = val.as_ptr();
		self
	}
	pub fn input_attachment(mut self, val : &AttachmentReference) -> Self {
		self.input_attachment_count = 1;
		self.p_input_attachments = val;
		self
	}
	pub fn color_attachments(mut self, val : &[AttachmentReference]) -> Self {
		self.color_attachment_count = val.len() as _;
		self.p_color_attachments = val.as_ptr();
		self
	}
	pub fn color_attachment(mut self, val : &AttachmentReference) -> Self {
		self.color_attachment_count = 1;
		self.p_color_attachments = val;
		self
	}
	pub fn preserve_attachments(mut self, val : &[u32]) -> Self {
		self.preserve_attachment_count = val.len() as _;
		self.p_preserve_attachments = val.as_ptr();
		self
	}
	pub fn preserve_attachment(mut self, val : &u32) -> Self {
		self.preserve_attachment_count = 1;
		self.p_preserve_attachments = val;
		self
	}
}

impl std::default::Default for SubpassDescription {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayModeCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DisplayModeCreateFlagsKHR,
	pub parameters : DisplayModeParametersKHR,
}

impl DisplayModeCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000002000,
			p_next : null(),
			flags : <_>::default(),
			parameters : DisplayModeParametersKHR::new(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DisplayModeCreateFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn parameters(mut self, val : DisplayModeParametersKHR) -> Self {
		self.parameters = val;
		self
	}
}

impl std::default::Default for DisplayModeCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceProtectedMemoryProperties {
	s_type : i32,
	pub p_next : *mut c_void,
	pub protected_no_fault : Bool32,
}

impl PhysicalDeviceProtectedMemoryProperties {
	pub fn new() -> Self {
		Self {
			s_type : 1000145002,
			p_next : null_mut(),
			protected_no_fault : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn protected_no_fault(mut self, val : Bool32) -> Self {
		self.protected_no_fault = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceProtectedMemoryProperties {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayPlaneCapabilitiesKHR {
	pub supported_alpha : DisplayPlaneAlphaFlagsKHR,
	pub min_src_position : Offset2D,
	pub max_src_position : Offset2D,
	pub min_src_extent : Extent2D,
	pub max_src_extent : Extent2D,
	pub min_dst_position : Offset2D,
	pub max_dst_position : Offset2D,
	pub min_dst_extent : Extent2D,
	pub max_dst_extent : Extent2D,
}

impl DisplayPlaneCapabilitiesKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn supported_alpha(mut self, val : DisplayPlaneAlphaFlagsKHR) -> Self {
		self.supported_alpha = val;
		self
	}
	pub fn min_src_position(mut self, val : Offset2D) -> Self {
		self.min_src_position = val;
		self
	}
	pub fn max_src_position(mut self, val : Offset2D) -> Self {
		self.max_src_position = val;
		self
	}
	pub fn min_src_extent(mut self, val : Extent2D) -> Self {
		self.min_src_extent = val;
		self
	}
	pub fn max_src_extent(mut self, val : Extent2D) -> Self {
		self.max_src_extent = val;
		self
	}
	pub fn min_dst_position(mut self, val : Offset2D) -> Self {
		self.min_dst_position = val;
		self
	}
	pub fn max_dst_position(mut self, val : Offset2D) -> Self {
		self.max_dst_position = val;
		self
	}
	pub fn min_dst_extent(mut self, val : Extent2D) -> Self {
		self.min_dst_extent = val;
		self
	}
	pub fn max_dst_extent(mut self, val : Extent2D) -> Self {
		self.max_dst_extent = val;
		self
	}
}

impl std::default::Default for DisplayPlaneCapabilitiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayPropertiesKHR {
	pub display : DisplayKHR,
	pub display_name : *const u8,
	pub physical_dimensions : Extent2D,
	pub physical_resolution : Extent2D,
	pub supported_transforms : SurfaceTransformFlagsKHR,
	pub plane_reorder_possible : Bool32,
	pub persistent_content : Bool32,
}

impl DisplayPropertiesKHR {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn display(mut self, val : DisplayKHR) -> Self {
		self.display = val;
		self
	}
	pub fn display_name(mut self, val : *const u8) -> Self {
		self.display_name = val;
		self
	}
	pub fn physical_dimensions(mut self, val : Extent2D) -> Self {
		self.physical_dimensions = val;
		self
	}
	pub fn physical_resolution(mut self, val : Extent2D) -> Self {
		self.physical_resolution = val;
		self
	}
	pub fn supported_transforms(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.supported_transforms = val;
		self
	}
	pub fn plane_reorder_possible(mut self, val : Bool32) -> Self {
		self.plane_reorder_possible = val;
		self
	}
	pub fn persistent_content(mut self, val : Bool32) -> Self {
		self.persistent_content = val;
		self
	}
}

impl std::default::Default for DisplayPropertiesKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SparseMemoryBind {
	pub resource_offset : DeviceSize,
	pub size : DeviceSize,
	pub memory : DeviceMemory,
	pub memory_offset : DeviceSize,
	pub flags : SparseMemoryBindFlags,
}

impl SparseMemoryBind {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}
	pub fn resource_offset(mut self, val : DeviceSize) -> Self {
		self.resource_offset = val;
		self
	}
	pub fn size(mut self, val : DeviceSize) -> Self {
		self.size = val;
		self
	}
	pub fn memory(mut self, val : DeviceMemory) -> Self {
		self.memory = val;
		self
	}
	pub fn memory_offset(mut self, val : DeviceSize) -> Self {
		self.memory_offset = val;
		self
	}
	pub fn flags(mut self, val : SparseMemoryBindFlags) -> Self {
		self.flags = val;
		self
	}
}

impl std::default::Default for SparseMemoryBind {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplaySurfaceCreateInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub flags : DisplaySurfaceCreateFlagsKHR,
	pub display_mode : DisplayModeKHR,
	pub plane_index : u32,
	pub plane_stack_index : u32,
	pub transform : SurfaceTransformFlagsKHR,
	pub global_alpha : f32,
	pub alpha_mode : DisplayPlaneAlphaFlagsKHR,
	pub image_extent : Extent2D,
}

impl DisplaySurfaceCreateInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000002001,
			p_next : null(),
			flags : <_>::default(),
			display_mode : DisplayModeKHR(0),
			plane_index : <_>::default(),
			plane_stack_index : <_>::default(),
			transform : SurfaceTransformFlagsKHR::default(),
			global_alpha : <_>::default(),
			alpha_mode : DisplayPlaneAlphaFlagsKHR::default(),
			image_extent : Extent2D::new(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn flags(mut self, val : DisplaySurfaceCreateFlagsKHR) -> Self {
		self.flags = val;
		self
	}
	pub fn display_mode(mut self, val : DisplayModeKHR) -> Self {
		self.display_mode = val;
		self
	}
	pub fn plane_index(mut self, val : u32) -> Self {
		self.plane_index = val;
		self
	}
	pub fn plane_stack_index(mut self, val : u32) -> Self {
		self.plane_stack_index = val;
		self
	}
	pub fn transform(mut self, val : SurfaceTransformFlagsKHR) -> Self {
		self.transform = val;
		self
	}
	pub fn global_alpha(mut self, val : f32) -> Self {
		self.global_alpha = val;
		self
	}
	pub fn alpha_mode(mut self, val : DisplayPlaneAlphaFlagsKHR) -> Self {
		self.alpha_mode = val;
		self
	}
	pub fn image_extent(mut self, val : Extent2D) -> Self {
		self.image_extent = val;
		self
	}
}

impl std::default::Default for DisplaySurfaceCreateInfoKHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesEXT {
	s_type : i32,
	pub p_next : *mut c_void,
	pub inline_uniform_block : Bool32,
	pub descriptor_binding_inline_uniform_block_update_after_bind : Bool32,
}

impl PhysicalDeviceInlineUniformBlockFeaturesEXT {
	pub fn new() -> Self {
		Self {
			s_type : 1000138000,
			p_next : null_mut(),
			inline_uniform_block : <_>::default(),
			descriptor_binding_inline_uniform_block_update_after_bind : <_>::default(),
		}
	}
	pub fn next(mut self, val : *mut c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn inline_uniform_block(mut self, val : Bool32) -> Self {
		self.inline_uniform_block = val;
		self
	}
	pub fn descriptor_binding_inline_uniform_block_update_after_bind(mut self, val : Bool32) -> Self {
		self.descriptor_binding_inline_uniform_block_update_after_bind = val;
		self
	}
}

impl std::default::Default for PhysicalDeviceInlineUniformBlockFeaturesEXT {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DisplayPlaneInfo2KHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub mode : DisplayModeKHR,
	pub plane_index : u32,
}

impl DisplayPlaneInfo2KHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000121003,
			p_next : null(),
			mode : DisplayModeKHR(0),
			plane_index : <_>::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn mode(mut self, val : DisplayModeKHR) -> Self {
		self.mode = val;
		self
	}
	pub fn plane_index(mut self, val : u32) -> Self {
		self.plane_index = val;
		self
	}
}

impl std::default::Default for DisplayPlaneInfo2KHR {
	fn default() -> Self { Self::new() }
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FenceGetFdInfoKHR {
	s_type : i32,
	pub p_next : *const c_void,
	pub fence : Fence,
	pub handle_type : ExternalFenceHandleTypeFlags,
}

impl FenceGetFdInfoKHR {
	pub fn new() -> Self {
		Self {
			s_type : 1000115001,
			p_next : null(),
			fence : Fence(0),
			handle_type : ExternalFenceHandleTypeFlags::default(),
		}
	}
	pub fn next(mut self, val : *const c_void) -> Self {
		self.p_next = val;
		self
	}
	pub fn fence(mut self, val : Fence) -> Self {
		self.fence = val;
		self
	}
	pub fn handle_type(mut self, val : ExternalFenceHandleTypeFlags) -> Self {
		self.handle_type = val;
		self
	}
}

impl std::default::Default for FenceGetFdInfoKHR {
	fn default() -> Self { Self::new() }
}
extern "C" {
	pub static vkCreateInstance: fn(*const InstanceCreateInfo, *const AllocationCallbacks, *mut Instance) -> VkResult;
	pub static vkEnumerateInstanceExtensionProperties: fn(*const u8, *mut u32, *mut ExtensionProperties) -> VkResult;
	pub static vkEnumerateInstanceLayerProperties: fn(*mut u32, *mut LayerProperties) -> VkResult;
	pub static vkEnumerateInstanceVersion: fn(*mut u32) -> VkResult;
	pub static vkBeginCommandBuffer: fn(CommandBuffer, *const CommandBufferBeginInfo) -> VkResult;
	pub static vkEndCommandBuffer: fn(CommandBuffer) -> VkResult;
	pub static vkResetCommandBuffer: fn(CommandBuffer, CommandBufferResetFlags) -> VkResult;
	pub static vkCmdBindPipeline: fn(CommandBuffer, PipelineBindPoint, Pipeline);
	pub static vkCmdSetViewport: fn(CommandBuffer, u32, u32, *const Viewport);
	pub static vkCmdSetScissor: fn(CommandBuffer, u32, u32, *const Rect2D);
	pub static vkCmdSetLineWidth: fn(CommandBuffer, f32);
	pub static vkCmdSetDepthBias: fn(CommandBuffer, f32, f32, f32);
	pub static vkCmdSetBlendConstants: fn(CommandBuffer, &[f32; 4]);
	pub static vkCmdSetDepthBounds: fn(CommandBuffer, f32, f32);
	pub static vkCmdSetStencilCompareMask: fn(CommandBuffer, StencilFaceFlags, u32);
	pub static vkCmdSetStencilWriteMask: fn(CommandBuffer, StencilFaceFlags, u32);
	pub static vkCmdSetStencilReference: fn(CommandBuffer, StencilFaceFlags, u32);
	pub static vkCmdBindDescriptorSets: fn(CommandBuffer, PipelineBindPoint, PipelineLayout, u32, u32, *const DescriptorSet, u32, *const u32);
	pub static vkCmdBindIndexBuffer: fn(CommandBuffer, Buffer, DeviceSize, IndexType);
	pub static vkCmdBindVertexBuffers: fn(CommandBuffer, u32, u32, *const Buffer, *const DeviceSize);
	pub static vkCmdDraw: fn(CommandBuffer, u32, u32, u32, u32);
	pub static vkCmdDrawIndexed: fn(CommandBuffer, u32, u32, u32, i32, u32);
	pub static vkCmdDrawIndirect: fn(CommandBuffer, Buffer, DeviceSize, u32, u32);
	pub static vkCmdDrawIndexedIndirect: fn(CommandBuffer, Buffer, DeviceSize, u32, u32);
	pub static vkCmdDispatch: fn(CommandBuffer, u32, u32, u32);
	pub static vkCmdDispatchIndirect: fn(CommandBuffer, Buffer, DeviceSize);
	pub static vkCmdCopyBuffer: fn(CommandBuffer, Buffer, Buffer, u32, *const BufferCopy);
	pub static vkCmdCopyImage: fn(CommandBuffer, Image, ImageLayout, Image, ImageLayout, u32, *const ImageCopy);
	pub static vkCmdBlitImage: fn(CommandBuffer, Image, ImageLayout, Image, ImageLayout, u32, *const ImageBlit, Filter);
	pub static vkCmdCopyBufferToImage: fn(CommandBuffer, Buffer, Image, ImageLayout, u32, *const BufferImageCopy);
	pub static vkCmdCopyImageToBuffer: fn(CommandBuffer, Image, ImageLayout, Buffer, u32, *const BufferImageCopy);
	pub static vkCmdUpdateBuffer: fn(CommandBuffer, Buffer, DeviceSize, DeviceSize, *const c_void);
	pub static vkCmdFillBuffer: fn(CommandBuffer, Buffer, DeviceSize, DeviceSize, u32);
	pub static vkCmdClearColorImage: fn(CommandBuffer, Image, ImageLayout, *const ClearColorValue, u32, *const ImageSubresourceRange);
	pub static vkCmdClearDepthStencilImage: fn(CommandBuffer, Image, ImageLayout, *const ClearDepthStencilValue, u32, *const ImageSubresourceRange);
	pub static vkCmdClearAttachments: fn(CommandBuffer, u32, *const ClearAttachment, u32, *const ClearRect);
	pub static vkCmdResolveImage: fn(CommandBuffer, Image, ImageLayout, Image, ImageLayout, u32, *const ImageResolve);
	pub static vkCmdSetEvent: fn(CommandBuffer, Event, PipelineStageFlags);
	pub static vkCmdResetEvent: fn(CommandBuffer, Event, PipelineStageFlags);
	pub static vkCmdWaitEvents: fn(CommandBuffer, u32, *const Event, PipelineStageFlags, PipelineStageFlags, u32, *const MemoryBarrier, u32, *const BufferMemoryBarrier, u32, *const ImageMemoryBarrier);
	pub static vkCmdPipelineBarrier: fn(CommandBuffer, PipelineStageFlags, PipelineStageFlags, DependencyFlags, u32, *const MemoryBarrier, u32, *const BufferMemoryBarrier, u32, *const ImageMemoryBarrier);
	pub static vkCmdBeginQuery: fn(CommandBuffer, QueryPool, u32, QueryControlFlags);
	pub static vkCmdEndQuery: fn(CommandBuffer, QueryPool, u32);
	pub static vkCmdResetQueryPool: fn(CommandBuffer, QueryPool, u32, u32);
	pub static vkCmdWriteTimestamp: fn(CommandBuffer, PipelineStageFlags, QueryPool, u32);
	pub static vkCmdCopyQueryPoolResults: fn(CommandBuffer, QueryPool, u32, u32, Buffer, DeviceSize, DeviceSize, QueryResultFlags);
	pub static vkCmdPushConstants: fn(CommandBuffer, PipelineLayout, ShaderStageFlags, u32, u32, *const c_void);
	pub static vkCmdBeginRenderPass: fn(CommandBuffer, *const RenderPassBeginInfo, SubpassContents);
	pub static vkCmdNextSubpass: fn(CommandBuffer, SubpassContents);
	pub static vkCmdEndRenderPass: fn(CommandBuffer);
	pub static vkCmdExecuteCommands: fn(CommandBuffer, u32, *const CommandBuffer);
	pub static vkCmdSetDeviceMask: fn(CommandBuffer, u32);
	pub static vkCmdDispatchBase: fn(CommandBuffer, u32, u32, u32, u32, u32, u32);
	pub static vkCmdDrawIndirectCount: fn(CommandBuffer, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
	pub static vkCmdDrawIndexedIndirectCount: fn(CommandBuffer, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
	pub static vkCmdBeginRenderPass2: fn(CommandBuffer, *const RenderPassBeginInfo, *const SubpassBeginInfo);
	pub static vkCmdNextSubpass2: fn(CommandBuffer, *const SubpassBeginInfo, *const SubpassEndInfo);
	pub static vkCmdEndRenderPass2: fn(CommandBuffer, *const SubpassEndInfo);
	pub static vkCmdSetDeviceMaskKHR: fn(CommandBuffer, u32);
	pub static vkCmdDispatchBaseKHR: fn(CommandBuffer, u32, u32, u32, u32, u32, u32);
	pub static vkCmdPushDescriptorSetKHR: fn(CommandBuffer, PipelineBindPoint, PipelineLayout, u32, u32, *const WriteDescriptorSet);
	pub static vkCmdPushDescriptorSetWithTemplateKHR: fn(CommandBuffer, DescriptorUpdateTemplate, PipelineLayout, u32, *const c_void);
	pub static vkCmdBeginRenderPass2KHR: fn(CommandBuffer, *const RenderPassBeginInfo, *const SubpassBeginInfo);
	pub static vkCmdNextSubpass2KHR: fn(CommandBuffer, *const SubpassBeginInfo, *const SubpassEndInfo);
	pub static vkCmdEndRenderPass2KHR: fn(CommandBuffer, *const SubpassEndInfo);
	pub static vkCmdDrawIndirectCountKHR: fn(CommandBuffer, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
	pub static vkCmdDrawIndexedIndirectCountKHR: fn(CommandBuffer, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
	pub static vkCmdDebugMarkerBeginEXT: fn(CommandBuffer, *const DebugMarkerMarkerInfoEXT);
	pub static vkCmdDebugMarkerEndEXT: fn(CommandBuffer);
	pub static vkCmdDebugMarkerInsertEXT: fn(CommandBuffer, *const DebugMarkerMarkerInfoEXT);
	pub static vkCmdBindTransformFeedbackBuffersEXT: fn(CommandBuffer, u32, u32, *const Buffer, *const DeviceSize, *const DeviceSize);
	pub static vkCmdBeginTransformFeedbackEXT: fn(CommandBuffer, u32, u32, *const Buffer, *const DeviceSize);
	pub static vkCmdEndTransformFeedbackEXT: fn(CommandBuffer, u32, u32, *const Buffer, *const DeviceSize);
	pub static vkCmdBeginQueryIndexedEXT: fn(CommandBuffer, QueryPool, u32, QueryControlFlags, u32);
	pub static vkCmdEndQueryIndexedEXT: fn(CommandBuffer, QueryPool, u32, u32);
	pub static vkCmdDrawIndirectByteCountEXT: fn(CommandBuffer, u32, u32, Buffer, DeviceSize, u32, u32);
	pub static vkCmdDrawIndirectCountAMD: fn(CommandBuffer, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
	pub static vkCmdDrawIndexedIndirectCountAMD: fn(CommandBuffer, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
	pub static vkCmdBeginConditionalRenderingEXT: fn(CommandBuffer, *const ConditionalRenderingBeginInfoEXT);
	pub static vkCmdEndConditionalRenderingEXT: fn(CommandBuffer);
	pub static vkCmdSetViewportWScalingNV: fn(CommandBuffer, u32, u32, *const ViewportWScalingNV);
	pub static vkCmdSetDiscardRectangleEXT: fn(CommandBuffer, u32, u32, *const Rect2D);
	pub static vkCmdBeginDebugUtilsLabelEXT: fn(CommandBuffer, *const DebugUtilsLabelEXT);
	pub static vkCmdEndDebugUtilsLabelEXT: fn(CommandBuffer);
	pub static vkCmdInsertDebugUtilsLabelEXT: fn(CommandBuffer, *const DebugUtilsLabelEXT);
	pub static vkCmdSetSampleLocationsEXT: fn(CommandBuffer, *const SampleLocationsInfoEXT);
	pub static vkCmdBindShadingRateImageNV: fn(CommandBuffer, ImageView, ImageLayout);
	pub static vkCmdSetViewportShadingRatePaletteNV: fn(CommandBuffer, u32, u32, *const ShadingRatePaletteNV);
	pub static vkCmdSetCoarseSampleOrderNV: fn(CommandBuffer, CoarseSampleOrderTypeNV, u32, *const CoarseSampleOrderCustomNV);
	pub static vkCmdBuildAccelerationStructureNV: fn(CommandBuffer, *const AccelerationStructureInfoNV, Buffer, DeviceSize, Bool32, AccelerationStructureKHR, AccelerationStructureKHR, Buffer, DeviceSize);
	pub static vkCmdCopyAccelerationStructureNV: fn(CommandBuffer, AccelerationStructureKHR, AccelerationStructureKHR, CopyAccelerationStructureModeKHR);
	pub static vkCmdTraceRaysNV: fn(CommandBuffer, Buffer, DeviceSize, Buffer, DeviceSize, DeviceSize, Buffer, DeviceSize, DeviceSize, Buffer, DeviceSize, DeviceSize, u32, u32, u32);
	pub static vkCmdWriteAccelerationStructuresPropertiesKHR: fn(CommandBuffer, u32, *const AccelerationStructureKHR, QueryType, QueryPool, u32);
	pub static vkCmdWriteAccelerationStructuresPropertiesNV: fn(CommandBuffer, u32, *const AccelerationStructureKHR, QueryType, QueryPool, u32);
	pub static vkCmdWriteBufferMarkerAMD: fn(CommandBuffer, PipelineStageFlags, Buffer, DeviceSize, u32);
	pub static vkCmdDrawMeshTasksNV: fn(CommandBuffer, u32, u32);
	pub static vkCmdDrawMeshTasksIndirectNV: fn(CommandBuffer, Buffer, DeviceSize, u32, u32);
	pub static vkCmdDrawMeshTasksIndirectCountNV: fn(CommandBuffer, Buffer, DeviceSize, Buffer, DeviceSize, u32, u32);
	pub static vkCmdSetExclusiveScissorNV: fn(CommandBuffer, u32, u32, *const Rect2D);
	pub static vkCmdSetCheckpointNV: fn(CommandBuffer, *const c_void);
	pub static vkCmdSetPerformanceMarkerINTEL: fn(CommandBuffer, *const PerformanceMarkerInfoINTEL) -> VkResult;
	pub static vkCmdSetPerformanceStreamMarkerINTEL: fn(CommandBuffer, *const PerformanceStreamMarkerInfoINTEL) -> VkResult;
	pub static vkCmdSetPerformanceOverrideINTEL: fn(CommandBuffer, *const PerformanceOverrideInfoINTEL) -> VkResult;
	pub static vkCmdSetLineStippleEXT: fn(CommandBuffer, u32, u16);
	pub static vkCmdSetCullModeEXT: fn(CommandBuffer, CullModeFlags);
	pub static vkCmdSetFrontFaceEXT: fn(CommandBuffer, FrontFace);
	pub static vkCmdSetPrimitiveTopologyEXT: fn(CommandBuffer, PrimitiveTopology);
	pub static vkCmdSetViewportWithCountEXT: fn(CommandBuffer, u32, *const Viewport);
	pub static vkCmdSetScissorWithCountEXT: fn(CommandBuffer, u32, *const Rect2D);
	pub static vkCmdBindVertexBuffers2EXT: fn(CommandBuffer, u32, u32, *const Buffer, *const DeviceSize, *const DeviceSize, *const DeviceSize);
	pub static vkCmdSetDepthTestEnableEXT: fn(CommandBuffer, Bool32);
	pub static vkCmdSetDepthWriteEnableEXT: fn(CommandBuffer, Bool32);
	pub static vkCmdSetDepthCompareOpEXT: fn(CommandBuffer, CompareOp);
	pub static vkCmdSetDepthBoundsTestEnableEXT: fn(CommandBuffer, Bool32);
	pub static vkCmdSetStencilTestEnableEXT: fn(CommandBuffer, Bool32);
	pub static vkCmdSetStencilOpEXT: fn(CommandBuffer, StencilFaceFlags, StencilOp, StencilOp, StencilOp, CompareOp);
	pub static vkCmdPreprocessGeneratedCommandsNV: fn(CommandBuffer, *const GeneratedCommandsInfoNV);
	pub static vkCmdExecuteGeneratedCommandsNV: fn(CommandBuffer, Bool32, *const GeneratedCommandsInfoNV);
	pub static vkCmdBindPipelineShaderGroupNV: fn(CommandBuffer, PipelineBindPoint, Pipeline, u32);
	pub static vkQueueSubmit: fn(Queue, u32, *const SubmitInfo, Fence) -> VkResult;
	pub static vkQueueWaitIdle: fn(Queue) -> VkResult;
	pub static vkQueueBindSparse: fn(Queue, u32, *const BindSparseInfo, Fence) -> VkResult;
	pub static vkQueuePresentKHR: fn(Queue, *const PresentInfoKHR) -> VkResult;
	pub static vkQueueBeginDebugUtilsLabelEXT: fn(Queue, *const DebugUtilsLabelEXT);
	pub static vkQueueEndDebugUtilsLabelEXT: fn(Queue);
	pub static vkQueueInsertDebugUtilsLabelEXT: fn(Queue, *const DebugUtilsLabelEXT);
	pub static vkGetQueueCheckpointDataNV: fn(Queue, *mut u32, *mut CheckpointDataNV);
	pub static vkQueueSetPerformanceConfigurationINTEL: fn(Queue, PerformanceConfigurationINTEL) -> VkResult;
	pub static vkGetDeviceProcAddr: fn(Device, *const u8) -> Option<extern "C" fn()>;
	pub static vkDestroyDevice: fn(Device, *const AllocationCallbacks);
	pub static vkGetDeviceQueue: fn(Device, u32, u32, *mut Queue);
	pub static vkDeviceWaitIdle: fn(Device) -> VkResult;
	pub static vkAllocateMemory: fn(Device, *const MemoryAllocateInfo, *const AllocationCallbacks, *mut DeviceMemory) -> VkResult;
	pub static vkFreeMemory: fn(Device, DeviceMemory, *const AllocationCallbacks);
	pub static vkMapMemory: fn(Device, DeviceMemory, DeviceSize, DeviceSize, MemoryMapFlags, *mut *mut c_void) -> VkResult;
	pub static vkUnmapMemory: fn(Device, DeviceMemory);
	pub static vkFlushMappedMemoryRanges: fn(Device, u32, *const MappedMemoryRange) -> VkResult;
	pub static vkInvalidateMappedMemoryRanges: fn(Device, u32, *const MappedMemoryRange) -> VkResult;
	pub static vkGetDeviceMemoryCommitment: fn(Device, DeviceMemory, *mut DeviceSize);
	pub static vkBindBufferMemory: fn(Device, Buffer, DeviceMemory, DeviceSize) -> VkResult;
	pub static vkBindImageMemory: fn(Device, Image, DeviceMemory, DeviceSize) -> VkResult;
	pub static vkGetBufferMemoryRequirements: fn(Device, Buffer, *mut MemoryRequirements);
	pub static vkGetImageMemoryRequirements: fn(Device, Image, *mut MemoryRequirements);
	pub static vkGetImageSparseMemoryRequirements: fn(Device, Image, *mut u32, *mut SparseImageMemoryRequirements);
	pub static vkCreateFence: fn(Device, *const FenceCreateInfo, *const AllocationCallbacks, *mut Fence) -> VkResult;
	pub static vkDestroyFence: fn(Device, Fence, *const AllocationCallbacks);
	pub static vkResetFences: fn(Device, u32, *const Fence) -> VkResult;
	pub static vkGetFenceStatus: fn(Device, Fence) -> VkResult;
	pub static vkWaitForFences: fn(Device, u32, *const Fence, Bool32, u64) -> VkResult;
	pub static vkCreateSemaphore: fn(Device, *const SemaphoreCreateInfo, *const AllocationCallbacks, *mut Semaphore) -> VkResult;
	pub static vkDestroySemaphore: fn(Device, Semaphore, *const AllocationCallbacks);
	pub static vkCreateEvent: fn(Device, *const EventCreateInfo, *const AllocationCallbacks, *mut Event) -> VkResult;
	pub static vkDestroyEvent: fn(Device, Event, *const AllocationCallbacks);
	pub static vkGetEventStatus: fn(Device, Event) -> VkResult;
	pub static vkSetEvent: fn(Device, Event) -> VkResult;
	pub static vkResetEvent: fn(Device, Event) -> VkResult;
	pub static vkCreateQueryPool: fn(Device, *const QueryPoolCreateInfo, *const AllocationCallbacks, *mut QueryPool) -> VkResult;
	pub static vkDestroyQueryPool: fn(Device, QueryPool, *const AllocationCallbacks);
	pub static vkGetQueryPoolResults: fn(Device, QueryPool, u32, u32, usize, *mut c_void, DeviceSize, QueryResultFlags) -> VkResult;
	pub static vkCreateBuffer: fn(Device, *const BufferCreateInfo, *const AllocationCallbacks, *mut Buffer) -> VkResult;
	pub static vkDestroyBuffer: fn(Device, Buffer, *const AllocationCallbacks);
	pub static vkCreateBufferView: fn(Device, *const BufferViewCreateInfo, *const AllocationCallbacks, *mut BufferView) -> VkResult;
	pub static vkDestroyBufferView: fn(Device, BufferView, *const AllocationCallbacks);
	pub static vkCreateImage: fn(Device, *const ImageCreateInfo, *const AllocationCallbacks, *mut Image) -> VkResult;
	pub static vkDestroyImage: fn(Device, Image, *const AllocationCallbacks);
	pub static vkGetImageSubresourceLayout: fn(Device, Image, *const ImageSubresource, *mut SubresourceLayout);
	pub static vkCreateImageView: fn(Device, *const ImageViewCreateInfo, *const AllocationCallbacks, *mut ImageView) -> VkResult;
	pub static vkDestroyImageView: fn(Device, ImageView, *const AllocationCallbacks);
	pub static vkCreateShaderModule: fn(Device, *const ShaderModuleCreateInfo, *const AllocationCallbacks, *mut ShaderModule) -> VkResult;
	pub static vkDestroyShaderModule: fn(Device, ShaderModule, *const AllocationCallbacks);
	pub static vkCreatePipelineCache: fn(Device, *const PipelineCacheCreateInfo, *const AllocationCallbacks, *mut PipelineCache) -> VkResult;
	pub static vkDestroyPipelineCache: fn(Device, PipelineCache, *const AllocationCallbacks);
	pub static vkGetPipelineCacheData: fn(Device, PipelineCache, *mut usize, *mut c_void) -> VkResult;
	pub static vkMergePipelineCaches: fn(Device, PipelineCache, u32, *const PipelineCache) -> VkResult;
	pub static vkCreateGraphicsPipelines: fn(Device, PipelineCache, u32, *const GraphicsPipelineCreateInfo, *const AllocationCallbacks, *mut Pipeline) -> VkResult;
	pub static vkCreateComputePipelines: fn(Device, PipelineCache, u32, *const ComputePipelineCreateInfo, *const AllocationCallbacks, *mut Pipeline) -> VkResult;
	pub static vkDestroyPipeline: fn(Device, Pipeline, *const AllocationCallbacks);
	pub static vkCreatePipelineLayout: fn(Device, *const PipelineLayoutCreateInfo, *const AllocationCallbacks, *mut PipelineLayout) -> VkResult;
	pub static vkDestroyPipelineLayout: fn(Device, PipelineLayout, *const AllocationCallbacks);
	pub static vkCreateSampler: fn(Device, *const SamplerCreateInfo, *const AllocationCallbacks, *mut Sampler) -> VkResult;
	pub static vkDestroySampler: fn(Device, Sampler, *const AllocationCallbacks);
	pub static vkCreateDescriptorSetLayout: fn(Device, *const DescriptorSetLayoutCreateInfo, *const AllocationCallbacks, *mut DescriptorSetLayout) -> VkResult;
	pub static vkDestroyDescriptorSetLayout: fn(Device, DescriptorSetLayout, *const AllocationCallbacks);
	pub static vkCreateDescriptorPool: fn(Device, *const DescriptorPoolCreateInfo, *const AllocationCallbacks, *mut DescriptorPool) -> VkResult;
	pub static vkDestroyDescriptorPool: fn(Device, DescriptorPool, *const AllocationCallbacks);
	pub static vkResetDescriptorPool: fn(Device, DescriptorPool, DescriptorPoolResetFlags) -> VkResult;
	pub static vkAllocateDescriptorSets: fn(Device, *const DescriptorSetAllocateInfo, *mut DescriptorSet) -> VkResult;
	pub static vkFreeDescriptorSets: fn(Device, DescriptorPool, u32, *const DescriptorSet) -> VkResult;
	pub static vkUpdateDescriptorSets: fn(Device, u32, *const WriteDescriptorSet, u32, *const CopyDescriptorSet);
	pub static vkCreateFramebuffer: fn(Device, *const FramebufferCreateInfo, *const AllocationCallbacks, *mut Framebuffer) -> VkResult;
	pub static vkDestroyFramebuffer: fn(Device, Framebuffer, *const AllocationCallbacks);
	pub static vkCreateRenderPass: fn(Device, *const RenderPassCreateInfo, *const AllocationCallbacks, *mut RenderPass) -> VkResult;
	pub static vkDestroyRenderPass: fn(Device, RenderPass, *const AllocationCallbacks);
	pub static vkGetRenderAreaGranularity: fn(Device, RenderPass, *mut Extent2D);
	pub static vkCreateCommandPool: fn(Device, *const CommandPoolCreateInfo, *const AllocationCallbacks, *mut CommandPool) -> VkResult;
	pub static vkDestroyCommandPool: fn(Device, CommandPool, *const AllocationCallbacks);
	pub static vkResetCommandPool: fn(Device, CommandPool, CommandPoolResetFlags) -> VkResult;
	pub static vkAllocateCommandBuffers: fn(Device, *const CommandBufferAllocateInfo, *mut CommandBuffer) -> VkResult;
	pub static vkFreeCommandBuffers: fn(Device, CommandPool, u32, *const CommandBuffer);
	pub static vkBindBufferMemory2: fn(Device, u32, *const BindBufferMemoryInfo) -> VkResult;
	pub static vkBindImageMemory2: fn(Device, u32, *const BindImageMemoryInfo) -> VkResult;
	pub static vkGetDeviceGroupPeerMemoryFeatures: fn(Device, u32, u32, u32, *mut PeerMemoryFeatureFlags);
	pub static vkGetImageMemoryRequirements2: fn(Device, *const ImageMemoryRequirementsInfo2, *mut MemoryRequirements2);
	pub static vkGetBufferMemoryRequirements2: fn(Device, *const BufferMemoryRequirementsInfo2, *mut MemoryRequirements2);
	pub static vkGetImageSparseMemoryRequirements2: fn(Device, *const ImageSparseMemoryRequirementsInfo2, *mut u32, *mut SparseImageMemoryRequirements2);
	pub static vkTrimCommandPool: fn(Device, CommandPool, CommandPoolTrimFlags);
	pub static vkGetDeviceQueue2: fn(Device, *const DeviceQueueInfo2, *mut Queue);
	pub static vkCreateSamplerYcbcrConversion: fn(Device, *const SamplerYcbcrConversionCreateInfo, *const AllocationCallbacks, *mut SamplerYcbcrConversion) -> VkResult;
	pub static vkDestroySamplerYcbcrConversion: fn(Device, SamplerYcbcrConversion, *const AllocationCallbacks);
	pub static vkCreateDescriptorUpdateTemplate: fn(Device, *const DescriptorUpdateTemplateCreateInfo, *const AllocationCallbacks, *mut DescriptorUpdateTemplate) -> VkResult;
	pub static vkDestroyDescriptorUpdateTemplate: fn(Device, DescriptorUpdateTemplate, *const AllocationCallbacks);
	pub static vkUpdateDescriptorSetWithTemplate: fn(Device, DescriptorSet, DescriptorUpdateTemplate, *const c_void);
	pub static vkGetDescriptorSetLayoutSupport: fn(Device, *const DescriptorSetLayoutCreateInfo, *mut DescriptorSetLayoutSupport);
	pub static vkCreateRenderPass2: fn(Device, *const RenderPassCreateInfo2, *const AllocationCallbacks, *mut RenderPass) -> VkResult;
	pub static vkResetQueryPool: fn(Device, QueryPool, u32, u32);
	pub static vkGetSemaphoreCounterValue: fn(Device, Semaphore, *mut u64) -> VkResult;
	pub static vkWaitSemaphores: fn(Device, *const SemaphoreWaitInfo, u64) -> VkResult;
	pub static vkSignalSemaphore: fn(Device, *const SemaphoreSignalInfo) -> VkResult;
	pub static vkGetBufferDeviceAddress: fn(Device, *const BufferDeviceAddressInfo) -> DeviceAddress;
	pub static vkGetBufferOpaqueCaptureAddress: fn(Device, *const BufferDeviceAddressInfo) -> u64;
	pub static vkGetDeviceMemoryOpaqueCaptureAddress: fn(Device, *const DeviceMemoryOpaqueCaptureAddressInfo) -> u64;
	pub static vkCreateSwapchainKHR: fn(Device, *const SwapchainCreateInfoKHR, *const AllocationCallbacks, *mut SwapchainKHR) -> VkResult;
	pub static vkDestroySwapchainKHR: fn(Device, SwapchainKHR, *const AllocationCallbacks);
	pub static vkGetSwapchainImagesKHR: fn(Device, SwapchainKHR, *mut u32, *mut Image) -> VkResult;
	pub static vkAcquireNextImageKHR: fn(Device, SwapchainKHR, u64, Semaphore, Fence, *mut u32) -> VkResult;
	pub static vkGetDeviceGroupPresentCapabilitiesKHR: fn(Device, *mut DeviceGroupPresentCapabilitiesKHR) -> VkResult;
	pub static vkGetDeviceGroupSurfacePresentModesKHR: fn(Device, SurfaceKHR, *mut DeviceGroupPresentModeFlagsKHR) -> VkResult;
	pub static vkAcquireNextImage2KHR: fn(Device, *const AcquireNextImageInfoKHR, *mut u32) -> VkResult;
	pub static vkCreateSharedSwapchainsKHR: fn(Device, u32, *const SwapchainCreateInfoKHR, *const AllocationCallbacks, *mut SwapchainKHR) -> VkResult;
	pub static vkGetDeviceGroupPeerMemoryFeaturesKHR: fn(Device, u32, u32, u32, *mut PeerMemoryFeatureFlags);
	pub static vkTrimCommandPoolKHR: fn(Device, CommandPool, CommandPoolTrimFlags);
	pub static vkGetMemoryFdKHR: fn(Device, *const MemoryGetFdInfoKHR, *mut i32) -> VkResult;
	pub static vkGetMemoryFdPropertiesKHR: fn(Device, ExternalMemoryHandleTypeFlags, i32, *mut MemoryFdPropertiesKHR) -> VkResult;
	pub static vkImportSemaphoreFdKHR: fn(Device, *const ImportSemaphoreFdInfoKHR) -> VkResult;
	pub static vkGetSemaphoreFdKHR: fn(Device, *const SemaphoreGetFdInfoKHR, *mut i32) -> VkResult;
	pub static vkCreateDescriptorUpdateTemplateKHR: fn(Device, *const DescriptorUpdateTemplateCreateInfo, *const AllocationCallbacks, *mut DescriptorUpdateTemplate) -> VkResult;
	pub static vkDestroyDescriptorUpdateTemplateKHR: fn(Device, DescriptorUpdateTemplate, *const AllocationCallbacks);
	pub static vkUpdateDescriptorSetWithTemplateKHR: fn(Device, DescriptorSet, DescriptorUpdateTemplate, *const c_void);
	pub static vkCreateRenderPass2KHR: fn(Device, *const RenderPassCreateInfo2, *const AllocationCallbacks, *mut RenderPass) -> VkResult;
	pub static vkGetSwapchainStatusKHR: fn(Device, SwapchainKHR) -> VkResult;
	pub static vkImportFenceFdKHR: fn(Device, *const ImportFenceFdInfoKHR) -> VkResult;
	pub static vkGetFenceFdKHR: fn(Device, *const FenceGetFdInfoKHR, *mut i32) -> VkResult;
	pub static vkAcquireProfilingLockKHR: fn(Device, *const AcquireProfilingLockInfoKHR) -> VkResult;
	pub static vkReleaseProfilingLockKHR: fn(Device);
	pub static vkGetImageMemoryRequirements2KHR: fn(Device, *const ImageMemoryRequirementsInfo2, *mut MemoryRequirements2);
	pub static vkGetBufferMemoryRequirements2KHR: fn(Device, *const BufferMemoryRequirementsInfo2, *mut MemoryRequirements2);
	pub static vkGetImageSparseMemoryRequirements2KHR: fn(Device, *const ImageSparseMemoryRequirementsInfo2, *mut u32, *mut SparseImageMemoryRequirements2);
	pub static vkCreateSamplerYcbcrConversionKHR: fn(Device, *const SamplerYcbcrConversionCreateInfo, *const AllocationCallbacks, *mut SamplerYcbcrConversion) -> VkResult;
	pub static vkDestroySamplerYcbcrConversionKHR: fn(Device, SamplerYcbcrConversion, *const AllocationCallbacks);
	pub static vkBindBufferMemory2KHR: fn(Device, u32, *const BindBufferMemoryInfo) -> VkResult;
	pub static vkBindImageMemory2KHR: fn(Device, u32, *const BindImageMemoryInfo) -> VkResult;
	pub static vkGetDescriptorSetLayoutSupportKHR: fn(Device, *const DescriptorSetLayoutCreateInfo, *mut DescriptorSetLayoutSupport);
	pub static vkGetSemaphoreCounterValueKHR: fn(Device, Semaphore, *mut u64) -> VkResult;
	pub static vkWaitSemaphoresKHR: fn(Device, *const SemaphoreWaitInfo, u64) -> VkResult;
	pub static vkSignalSemaphoreKHR: fn(Device, *const SemaphoreSignalInfo) -> VkResult;
	pub static vkGetBufferDeviceAddressKHR: fn(Device, *const BufferDeviceAddressInfo) -> DeviceAddress;
	pub static vkGetBufferOpaqueCaptureAddressKHR: fn(Device, *const BufferDeviceAddressInfo) -> u64;
	pub static vkGetDeviceMemoryOpaqueCaptureAddressKHR: fn(Device, *const DeviceMemoryOpaqueCaptureAddressInfo) -> u64;
	pub static vkGetPipelineExecutablePropertiesKHR: fn(Device, *const PipelineInfoKHR, *mut u32, *mut PipelineExecutablePropertiesKHR) -> VkResult;
	pub static vkGetPipelineExecutableStatisticsKHR: fn(Device, *const PipelineExecutableInfoKHR, *mut u32, *mut PipelineExecutableStatisticKHR) -> VkResult;
	pub static vkGetPipelineExecutableInternalRepresentationsKHR: fn(Device, *const PipelineExecutableInfoKHR, *mut u32, *mut PipelineExecutableInternalRepresentationKHR) -> VkResult;
	pub static vkDebugMarkerSetObjectTagEXT: fn(Device, *const DebugMarkerObjectTagInfoEXT) -> VkResult;
	pub static vkDebugMarkerSetObjectNameEXT: fn(Device, *const DebugMarkerObjectNameInfoEXT) -> VkResult;
	pub static vkGetImageViewHandleNVX: fn(Device, *const ImageViewHandleInfoNVX) -> u32;
	pub static vkGetImageViewAddressNVX: fn(Device, ImageView, *mut ImageViewAddressPropertiesNVX) -> VkResult;
	pub static vkGetShaderInfoAMD: fn(Device, Pipeline, ShaderStageFlags, ShaderInfoTypeAMD, *mut usize, *mut c_void) -> VkResult;
	pub static vkDisplayPowerControlEXT: fn(Device, DisplayKHR, *const DisplayPowerInfoEXT) -> VkResult;
	pub static vkRegisterDeviceEventEXT: fn(Device, *const DeviceEventInfoEXT, *const AllocationCallbacks, *mut Fence) -> VkResult;
	pub static vkRegisterDisplayEventEXT: fn(Device, DisplayKHR, *const DisplayEventInfoEXT, *const AllocationCallbacks, *mut Fence) -> VkResult;
	pub static vkGetSwapchainCounterEXT: fn(Device, SwapchainKHR, SurfaceCounterFlagsEXT, *mut u64) -> VkResult;
	pub static vkGetRefreshCycleDurationGOOGLE: fn(Device, SwapchainKHR, *mut RefreshCycleDurationGOOGLE) -> VkResult;
	pub static vkGetPastPresentationTimingGOOGLE: fn(Device, SwapchainKHR, *mut u32, *mut PastPresentationTimingGOOGLE) -> VkResult;
	pub static vkSetHdrMetadataEXT: fn(Device, u32, *const SwapchainKHR, *const HdrMetadataEXT);
	pub static vkSetDebugUtilsObjectNameEXT: fn(Device, *const DebugUtilsObjectNameInfoEXT) -> VkResult;
	pub static vkSetDebugUtilsObjectTagEXT: fn(Device, *const DebugUtilsObjectTagInfoEXT) -> VkResult;
	pub static vkGetImageDrmFormatModifierPropertiesEXT: fn(Device, Image, *mut ImageDrmFormatModifierPropertiesEXT) -> VkResult;
	pub static vkCreateValidationCacheEXT: fn(Device, *const ValidationCacheCreateInfoEXT, *const AllocationCallbacks, *mut ValidationCacheEXT) -> VkResult;
	pub static vkDestroyValidationCacheEXT: fn(Device, ValidationCacheEXT, *const AllocationCallbacks);
	pub static vkMergeValidationCachesEXT: fn(Device, ValidationCacheEXT, u32, *const ValidationCacheEXT) -> VkResult;
	pub static vkGetValidationCacheDataEXT: fn(Device, ValidationCacheEXT, *mut usize, *mut c_void) -> VkResult;
	pub static vkCreateAccelerationStructureNV: fn(Device, *const AccelerationStructureCreateInfoNV, *const AllocationCallbacks, *mut AccelerationStructureNV) -> VkResult;
	pub static vkDestroyAccelerationStructureKHR: fn(Device, AccelerationStructureKHR, *const AllocationCallbacks);
	pub static vkDestroyAccelerationStructureNV: fn(Device, AccelerationStructureKHR, *const AllocationCallbacks);
	pub static vkGetAccelerationStructureMemoryRequirementsNV: fn(Device, *const AccelerationStructureMemoryRequirementsInfoNV, *mut MemoryRequirements2KHR);
	pub static vkBindAccelerationStructureMemoryKHR: fn(Device, u32, *const BindAccelerationStructureMemoryInfoKHR) -> VkResult;
	pub static vkBindAccelerationStructureMemoryNV: fn(Device, u32, *const BindAccelerationStructureMemoryInfoKHR) -> VkResult;
	pub static vkCreateRayTracingPipelinesNV: fn(Device, PipelineCache, u32, *const RayTracingPipelineCreateInfoNV, *const AllocationCallbacks, *mut Pipeline) -> VkResult;
	pub static vkGetRayTracingShaderGroupHandlesKHR: fn(Device, Pipeline, u32, u32, usize, *mut c_void) -> VkResult;
	pub static vkGetRayTracingShaderGroupHandlesNV: fn(Device, Pipeline, u32, u32, usize, *mut c_void) -> VkResult;
	pub static vkGetAccelerationStructureHandleNV: fn(Device, AccelerationStructureKHR, usize, *mut c_void) -> VkResult;
	pub static vkCompileDeferredNV: fn(Device, Pipeline, u32) -> VkResult;
	pub static vkGetMemoryHostPointerPropertiesEXT: fn(Device, ExternalMemoryHandleTypeFlags, *const c_void, *mut MemoryHostPointerPropertiesEXT) -> VkResult;
	pub static vkGetCalibratedTimestampsEXT: fn(Device, u32, *const CalibratedTimestampInfoEXT, *mut u64, *mut u64) -> VkResult;
	pub static vkInitializePerformanceApiINTEL: fn(Device, *const InitializePerformanceApiInfoINTEL) -> VkResult;
	pub static vkUninitializePerformanceApiINTEL: fn(Device);
	pub static vkAcquirePerformanceConfigurationINTEL: fn(Device, *const PerformanceConfigurationAcquireInfoINTEL, *mut PerformanceConfigurationINTEL) -> VkResult;
	pub static vkReleasePerformanceConfigurationINTEL: fn(Device, PerformanceConfigurationINTEL) -> VkResult;
	pub static vkGetPerformanceParameterINTEL: fn(Device, PerformanceParameterTypeINTEL, *mut PerformanceValueINTEL) -> VkResult;
	pub static vkSetLocalDimmingAMD: fn(Device, SwapchainKHR, Bool32);
	pub static vkGetBufferDeviceAddressEXT: fn(Device, *const BufferDeviceAddressInfo) -> DeviceAddress;
	pub static vkResetQueryPoolEXT: fn(Device, QueryPool, u32, u32);
	pub static vkGetGeneratedCommandsMemoryRequirementsNV: fn(Device, *const GeneratedCommandsMemoryRequirementsInfoNV, *mut MemoryRequirements2);
	pub static vkCreateIndirectCommandsLayoutNV: fn(Device, *const IndirectCommandsLayoutCreateInfoNV, *const AllocationCallbacks, *mut IndirectCommandsLayoutNV) -> VkResult;
	pub static vkDestroyIndirectCommandsLayoutNV: fn(Device, IndirectCommandsLayoutNV, *const AllocationCallbacks);
	pub static vkCreatePrivateDataSlotEXT: fn(Device, *const PrivateDataSlotCreateInfoEXT, *const AllocationCallbacks, *mut PrivateDataSlotEXT) -> VkResult;
	pub static vkDestroyPrivateDataSlotEXT: fn(Device, PrivateDataSlotEXT, *const AllocationCallbacks);
	pub static vkSetPrivateDataEXT: fn(Device, ObjectType, u64, PrivateDataSlotEXT, u64) -> VkResult;
	pub static vkGetPrivateDataEXT: fn(Device, ObjectType, u64, PrivateDataSlotEXT, *mut u64);
	pub static vkGetAndroidHardwareBufferPropertiesANDROID: fn(Device, *const AHardwareBuffer, *mut AndroidHardwareBufferPropertiesANDROID) -> VkResult;
	pub static vkGetMemoryAndroidHardwareBufferANDROID: fn(Device, *const MemoryGetAndroidHardwareBufferInfoANDROID, *mut *mut AHardwareBuffer) -> VkResult;
	pub static vkGetMemoryWin32HandleKHR: fn(Device, *const MemoryGetWin32HandleInfoKHR, *mut HANDLE) -> VkResult;
	pub static vkGetMemoryWin32HandlePropertiesKHR: fn(Device, ExternalMemoryHandleTypeFlags, HANDLE, *mut MemoryWin32HandlePropertiesKHR) -> VkResult;
	pub static vkImportSemaphoreWin32HandleKHR: fn(Device, *const ImportSemaphoreWin32HandleInfoKHR) -> VkResult;
	pub static vkGetSemaphoreWin32HandleKHR: fn(Device, *const SemaphoreGetWin32HandleInfoKHR, *mut HANDLE) -> VkResult;
	pub static vkImportFenceWin32HandleKHR: fn(Device, *const ImportFenceWin32HandleInfoKHR) -> VkResult;
	pub static vkGetFenceWin32HandleKHR: fn(Device, *const FenceGetWin32HandleInfoKHR, *mut HANDLE) -> VkResult;
	pub static vkGetMemoryWin32HandleNV: fn(Device, DeviceMemory, ExternalMemoryHandleTypeFlagsNV, *mut HANDLE) -> VkResult;
	pub static vkAcquireFullScreenExclusiveModeEXT: fn(Device, SwapchainKHR) -> VkResult;
	pub static vkReleaseFullScreenExclusiveModeEXT: fn(Device, SwapchainKHR) -> VkResult;
	pub static vkGetDeviceGroupSurfacePresentModes2EXT: fn(Device, *const PhysicalDeviceSurfaceInfo2KHR, *mut DeviceGroupPresentModeFlagsKHR) -> VkResult;
	pub static vkGetPhysicalDeviceFeatures: fn(PhysicalDevice, *mut PhysicalDeviceFeatures);
	pub static vkGetPhysicalDeviceFormatProperties: fn(PhysicalDevice, Format, *mut FormatProperties);
	pub static vkGetPhysicalDeviceImageFormatProperties: fn(PhysicalDevice, Format, ImageType, ImageTiling, ImageUsageFlags, ImageCreateFlags, *mut ImageFormatProperties) -> VkResult;
	pub static vkGetPhysicalDeviceProperties: fn(PhysicalDevice, *mut PhysicalDeviceProperties);
	pub static vkGetPhysicalDeviceQueueFamilyProperties: fn(PhysicalDevice, *mut u32, *mut QueueFamilyProperties);
	pub static vkGetPhysicalDeviceMemoryProperties: fn(PhysicalDevice, *mut PhysicalDeviceMemoryProperties);
	pub static vkCreateDevice: fn(PhysicalDevice, *const DeviceCreateInfo, *const AllocationCallbacks, *mut Device) -> VkResult;
	pub static vkEnumerateDeviceExtensionProperties: fn(PhysicalDevice, *const u8, *mut u32, *mut ExtensionProperties) -> VkResult;
	pub static vkEnumerateDeviceLayerProperties: fn(PhysicalDevice, *mut u32, *mut LayerProperties) -> VkResult;
	pub static vkGetPhysicalDeviceSparseImageFormatProperties: fn(PhysicalDevice, Format, ImageType, SampleCountFlags, ImageUsageFlags, ImageTiling, *mut u32, *mut SparseImageFormatProperties);
	pub static vkGetPhysicalDeviceFeatures2: fn(PhysicalDevice, *mut PhysicalDeviceFeatures2);
	pub static vkGetPhysicalDeviceProperties2: fn(PhysicalDevice, *mut PhysicalDeviceProperties2);
	pub static vkGetPhysicalDeviceFormatProperties2: fn(PhysicalDevice, Format, *mut FormatProperties2);
	pub static vkGetPhysicalDeviceImageFormatProperties2: fn(PhysicalDevice, *const PhysicalDeviceImageFormatInfo2, *mut ImageFormatProperties2) -> VkResult;
	pub static vkGetPhysicalDeviceQueueFamilyProperties2: fn(PhysicalDevice, *mut u32, *mut QueueFamilyProperties2);
	pub static vkGetPhysicalDeviceMemoryProperties2: fn(PhysicalDevice, *mut PhysicalDeviceMemoryProperties2);
	pub static vkGetPhysicalDeviceSparseImageFormatProperties2: fn(PhysicalDevice, *const PhysicalDeviceSparseImageFormatInfo2, *mut u32, *mut SparseImageFormatProperties2);
	pub static vkGetPhysicalDeviceExternalBufferProperties: fn(PhysicalDevice, *const PhysicalDeviceExternalBufferInfo, *mut ExternalBufferProperties);
	pub static vkGetPhysicalDeviceExternalFenceProperties: fn(PhysicalDevice, *const PhysicalDeviceExternalFenceInfo, *mut ExternalFenceProperties);
	pub static vkGetPhysicalDeviceExternalSemaphoreProperties: fn(PhysicalDevice, *const PhysicalDeviceExternalSemaphoreInfo, *mut ExternalSemaphoreProperties);
	pub static vkGetPhysicalDeviceSurfaceSupportKHR: fn(PhysicalDevice, u32, SurfaceKHR, *mut Bool32) -> VkResult;
	pub static vkGetPhysicalDeviceSurfaceCapabilitiesKHR: fn(PhysicalDevice, SurfaceKHR, *mut SurfaceCapabilitiesKHR) -> VkResult;
	pub static vkGetPhysicalDeviceSurfaceFormatsKHR: fn(PhysicalDevice, SurfaceKHR, *mut u32, *mut SurfaceFormatKHR) -> VkResult;
	pub static vkGetPhysicalDeviceSurfacePresentModesKHR: fn(PhysicalDevice, SurfaceKHR, *mut u32, *mut PresentModeKHR) -> VkResult;
	pub static vkGetPhysicalDevicePresentRectanglesKHR: fn(PhysicalDevice, SurfaceKHR, *mut u32, *mut Rect2D) -> VkResult;
	pub static vkGetPhysicalDeviceDisplayPropertiesKHR: fn(PhysicalDevice, *mut u32, *mut DisplayPropertiesKHR) -> VkResult;
	pub static vkGetPhysicalDeviceDisplayPlanePropertiesKHR: fn(PhysicalDevice, *mut u32, *mut DisplayPlanePropertiesKHR) -> VkResult;
	pub static vkGetDisplayPlaneSupportedDisplaysKHR: fn(PhysicalDevice, u32, *mut u32, *mut DisplayKHR) -> VkResult;
	pub static vkGetDisplayModePropertiesKHR: fn(PhysicalDevice, DisplayKHR, *mut u32, *mut DisplayModePropertiesKHR) -> VkResult;
	pub static vkCreateDisplayModeKHR: fn(PhysicalDevice, DisplayKHR, *const DisplayModeCreateInfoKHR, *const AllocationCallbacks, *mut DisplayModeKHR) -> VkResult;
	pub static vkGetDisplayPlaneCapabilitiesKHR: fn(PhysicalDevice, DisplayModeKHR, u32, *mut DisplayPlaneCapabilitiesKHR) -> VkResult;
	pub static vkGetPhysicalDeviceFeatures2KHR: fn(PhysicalDevice, *mut PhysicalDeviceFeatures2);
	pub static vkGetPhysicalDeviceProperties2KHR: fn(PhysicalDevice, *mut PhysicalDeviceProperties2);
	pub static vkGetPhysicalDeviceFormatProperties2KHR: fn(PhysicalDevice, Format, *mut FormatProperties2);
	pub static vkGetPhysicalDeviceImageFormatProperties2KHR: fn(PhysicalDevice, *const PhysicalDeviceImageFormatInfo2, *mut ImageFormatProperties2) -> VkResult;
	pub static vkGetPhysicalDeviceQueueFamilyProperties2KHR: fn(PhysicalDevice, *mut u32, *mut QueueFamilyProperties2);
	pub static vkGetPhysicalDeviceMemoryProperties2KHR: fn(PhysicalDevice, *mut PhysicalDeviceMemoryProperties2);
	pub static vkGetPhysicalDeviceSparseImageFormatProperties2KHR: fn(PhysicalDevice, *const PhysicalDeviceSparseImageFormatInfo2, *mut u32, *mut SparseImageFormatProperties2);
	pub static vkGetPhysicalDeviceExternalBufferPropertiesKHR: fn(PhysicalDevice, *const PhysicalDeviceExternalBufferInfo, *mut ExternalBufferProperties);
	pub static vkGetPhysicalDeviceExternalSemaphorePropertiesKHR: fn(PhysicalDevice, *const PhysicalDeviceExternalSemaphoreInfo, *mut ExternalSemaphoreProperties);
	pub static vkGetPhysicalDeviceExternalFencePropertiesKHR: fn(PhysicalDevice, *const PhysicalDeviceExternalFenceInfo, *mut ExternalFenceProperties);
	pub static vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR: fn(PhysicalDevice, u32, *mut u32, *mut PerformanceCounterKHR, *mut PerformanceCounterDescriptionKHR) -> VkResult;
	pub static vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR: fn(PhysicalDevice, *const QueryPoolPerformanceCreateInfoKHR, *mut u32);
	pub static vkGetPhysicalDeviceSurfaceCapabilities2KHR: fn(PhysicalDevice, *const PhysicalDeviceSurfaceInfo2KHR, *mut SurfaceCapabilities2KHR) -> VkResult;
	pub static vkGetPhysicalDeviceSurfaceFormats2KHR: fn(PhysicalDevice, *const PhysicalDeviceSurfaceInfo2KHR, *mut u32, *mut SurfaceFormat2KHR) -> VkResult;
	pub static vkGetPhysicalDeviceDisplayProperties2KHR: fn(PhysicalDevice, *mut u32, *mut DisplayProperties2KHR) -> VkResult;
	pub static vkGetPhysicalDeviceDisplayPlaneProperties2KHR: fn(PhysicalDevice, *mut u32, *mut DisplayPlaneProperties2KHR) -> VkResult;
	pub static vkGetDisplayModeProperties2KHR: fn(PhysicalDevice, DisplayKHR, *mut u32, *mut DisplayModeProperties2KHR) -> VkResult;
	pub static vkGetDisplayPlaneCapabilities2KHR: fn(PhysicalDevice, *const DisplayPlaneInfo2KHR, *mut DisplayPlaneCapabilities2KHR) -> VkResult;
	pub static vkGetPhysicalDeviceExternalImageFormatPropertiesNV: fn(PhysicalDevice, Format, ImageType, ImageTiling, ImageUsageFlags, ImageCreateFlags, ExternalMemoryHandleTypeFlagsNV, *mut ExternalImageFormatPropertiesNV) -> VkResult;
	pub static vkReleaseDisplayEXT: fn(PhysicalDevice, DisplayKHR) -> VkResult;
	pub static vkGetPhysicalDeviceSurfaceCapabilities2EXT: fn(PhysicalDevice, SurfaceKHR, *mut SurfaceCapabilities2EXT) -> VkResult;
	pub static vkGetPhysicalDeviceMultisamplePropertiesEXT: fn(PhysicalDevice, SampleCountFlags, *mut MultisamplePropertiesEXT);
	pub static vkGetPhysicalDeviceCalibrateableTimeDomainsEXT: fn(PhysicalDevice, *mut u32, *mut TimeDomainEXT) -> VkResult;
	pub static vkGetPhysicalDeviceToolPropertiesEXT: fn(PhysicalDevice, *mut u32, *mut PhysicalDeviceToolPropertiesEXT) -> VkResult;
	pub static vkGetPhysicalDeviceCooperativeMatrixPropertiesNV: fn(PhysicalDevice, *mut u32, *mut CooperativeMatrixPropertiesNV) -> VkResult;
	pub static vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV: fn(PhysicalDevice, *mut u32, *mut FramebufferMixedSamplesCombinationNV) -> VkResult;
	pub static vkGetPhysicalDeviceWin32PresentationSupportKHR: fn(PhysicalDevice, u32) -> Bool32;
	pub static vkGetPhysicalDeviceSurfacePresentModes2EXT: fn(PhysicalDevice, *const PhysicalDeviceSurfaceInfo2KHR, *mut u32, *mut PresentModeKHR) -> VkResult;
	pub static vkGetPhysicalDeviceWaylandPresentationSupportKHR: fn(PhysicalDevice, u32, *mut wl_display) -> Bool32;
	pub static vkGetPhysicalDeviceXcbPresentationSupportKHR: fn(PhysicalDevice, u32, *mut xcb_connection_t, xcb_visualid_t) -> Bool32;
	pub static vkGetPhysicalDeviceXlibPresentationSupportKHR: fn(PhysicalDevice, u32, *mut Display, VisualID) -> Bool32;
	pub static vkAcquireXlibDisplayEXT: fn(PhysicalDevice, *mut Display, DisplayKHR) -> VkResult;
	pub static vkGetRandROutputDisplayEXT: fn(PhysicalDevice, *mut Display, RROutput, *mut DisplayKHR) -> VkResult;
	pub static vkDestroyInstance: fn(Instance, *const AllocationCallbacks);
	pub static vkEnumeratePhysicalDevices: fn(Instance, *mut u32, *mut PhysicalDevice) -> VkResult;
	pub static vkGetInstanceProcAddr: fn(Instance, *const u8) -> Option<extern "C" fn()>;
	pub static vkEnumeratePhysicalDeviceGroups: fn(Instance, *mut u32, *mut PhysicalDeviceGroupProperties) -> VkResult;
	pub static vkDestroySurfaceKHR: fn(Instance, SurfaceKHR, *const AllocationCallbacks);
	pub static vkCreateDisplayPlaneSurfaceKHR: fn(Instance, *const DisplaySurfaceCreateInfoKHR, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkEnumeratePhysicalDeviceGroupsKHR: fn(Instance, *mut u32, *mut PhysicalDeviceGroupProperties) -> VkResult;
	pub static vkCreateDebugReportCallbackEXT: fn(Instance, *const DebugReportCallbackCreateInfoEXT, *const AllocationCallbacks, *mut DebugReportCallbackEXT) -> VkResult;
	pub static vkDestroyDebugReportCallbackEXT: fn(Instance, DebugReportCallbackEXT, *const AllocationCallbacks);
	pub static vkDebugReportMessageEXT: fn(Instance, DebugReportFlagsEXT, DebugReportObjectTypeEXT, u64, usize, i32, *const u8, *const u8);
	pub static vkCreateDebugUtilsMessengerEXT: fn(Instance, *const DebugUtilsMessengerCreateInfoEXT, *const AllocationCallbacks, *mut DebugUtilsMessengerEXT) -> VkResult;
	pub static vkDestroyDebugUtilsMessengerEXT: fn(Instance, DebugUtilsMessengerEXT, *const AllocationCallbacks);
	pub static vkSubmitDebugUtilsMessageEXT: fn(Instance, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, *const DebugUtilsMessengerCallbackDataEXT);
	pub static vkCreateHeadlessSurfaceEXT: fn(Instance, *const HeadlessSurfaceCreateInfoEXT, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkCreateAndroidSurfaceKHR: fn(Instance, *const AndroidSurfaceCreateInfoKHR, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkCreateWin32SurfaceKHR: fn(Instance, *const Win32SurfaceCreateInfoKHR, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkCreateWaylandSurfaceKHR: fn(Instance, *const WaylandSurfaceCreateInfoKHR, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkCreateXcbSurfaceKHR: fn(Instance, *const XcbSurfaceCreateInfoKHR, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkCreateXlibSurfaceKHR: fn(Instance, *const XlibSurfaceCreateInfoKHR, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkCreateIOSSurfaceMVK: fn(Instance, *const IOSSurfaceCreateInfoMVK, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkCreateMacOSSurfaceMVK: fn(Instance, *const MacOSSurfaceCreateInfoMVK, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;
	pub static vkCreateMetalSurfaceEXT: fn(Instance, *const MetalSurfaceCreateInfoEXT, *const AllocationCallbacks, *mut SurfaceKHR) -> VkResult;

}
impl CommandBuffer {

	pub fn begin(self, pBeginInfo : &CommandBufferBeginInfo) -> VkResult {
		unsafe { vkBeginCommandBuffer(self, pBeginInfo) }
	}

	pub fn end(self) -> VkResult {
		unsafe { vkEndCommandBuffer(self) }
	}

	pub fn reset(self, flags : CommandBufferResetFlags) -> VkResult {
		unsafe { vkResetCommandBuffer(self, flags) }
	}

	pub fn bind_pipeline(self, pipelineBindPoint : PipelineBindPoint, pipeline : Pipeline) -> Self {
		unsafe { vkCmdBindPipeline(self, pipelineBindPoint, pipeline) };
		self
	}

	pub fn set_viewport(self, firstViewport : u32, viewports : &[Viewport]) -> Self {
		unsafe { vkCmdSetViewport(self, firstViewport, viewports.len() as _, viewports.as_ptr()) };
		self
	}

	pub fn set_scissor(self, firstScissor : u32, scissors : &[Rect2D]) -> Self {
		unsafe { vkCmdSetScissor(self, firstScissor, scissors.len() as _, scissors.as_ptr()) };
		self
	}

	pub fn set_line_width(self, lineWidth : f32) -> Self {
		unsafe { vkCmdSetLineWidth(self, lineWidth) };
		self
	}

	pub fn set_depth_bias(self, depthBiasConstantFactor : f32, depthBiasClamp : f32, depthBiasSlopeFactor : f32) -> Self {
		unsafe { vkCmdSetDepthBias(self, depthBiasConstantFactor, depthBiasClamp, depthBiasSlopeFactor) };
		self
	}

	pub fn set_blend_constants(self, blendConstants : &[f32; 4]) -> Self {
		unsafe { vkCmdSetBlendConstants(self, blendConstants) };
		self
	}

	pub fn set_depth_bounds(self, minDepthBounds : f32, maxDepthBounds : f32) -> Self {
		unsafe { vkCmdSetDepthBounds(self, minDepthBounds, maxDepthBounds) };
		self
	}

	pub fn set_stencil_compare_mask(self, faceMask : StencilFaceFlags, compareMask : u32) -> Self {
		unsafe { vkCmdSetStencilCompareMask(self, faceMask, compareMask) };
		self
	}

	pub fn set_stencil_write_mask(self, faceMask : StencilFaceFlags, writeMask : u32) -> Self {
		unsafe { vkCmdSetStencilWriteMask(self, faceMask, writeMask) };
		self
	}

	pub fn set_stencil_reference(self, faceMask : StencilFaceFlags, reference : u32) -> Self {
		unsafe { vkCmdSetStencilReference(self, faceMask, reference) };
		self
	}

	pub fn bind_descriptor_sets(self, pipelineBindPoint : PipelineBindPoint, layout : PipelineLayout, firstSet : u32, descriptorSets : &[DescriptorSet], dynamicOffsets : &[u32]) -> Self {
		unsafe { vkCmdBindDescriptorSets(self, pipelineBindPoint, layout, firstSet, descriptorSets.len() as _, descriptorSets.as_ptr(), dynamicOffsets.len() as _, dynamicOffsets.as_ptr()) };
		self
	}

	pub fn bind_index_buffer(self, buffer : Buffer, offset : DeviceSize, indexType : IndexType) -> Self {
		unsafe { vkCmdBindIndexBuffer(self, buffer, offset, indexType) };
		self
	}

	pub fn bind_vertex_buffers(self, firstBinding : u32, bindingCount : u32, pBuffers : &Buffer, pOffsets : &DeviceSize) -> Self {
		unsafe { vkCmdBindVertexBuffers(self, firstBinding, bindingCount, pBuffers, pOffsets) };
		self
	}

	pub fn draw(self, vertexCount : u32, instanceCount : u32, firstVertex : u32, firstInstance : u32) -> Self {
		unsafe { vkCmdDraw(self, vertexCount, instanceCount, firstVertex, firstInstance) };
		self
	}

	pub fn draw_indexed(self, indexCount : u32, instanceCount : u32, firstIndex : u32, vertexOffset : i32, firstInstance : u32) -> Self {
		unsafe { vkCmdDrawIndexed(self, indexCount, instanceCount, firstIndex, vertexOffset, firstInstance) };
		self
	}

	pub fn draw_indirect(self, buffer : Buffer, offset : DeviceSize, drawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawIndirect(self, buffer, offset, drawCount, stride) };
		self
	}

	pub fn draw_indexed_indirect(self, buffer : Buffer, offset : DeviceSize, drawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawIndexedIndirect(self, buffer, offset, drawCount, stride) };
		self
	}

	pub fn dispatch(self, groupCountX : u32, groupCountY : u32, groupCountZ : u32) -> Self {
		unsafe { vkCmdDispatch(self, groupCountX, groupCountY, groupCountZ) };
		self
	}

	pub fn dispatch_indirect(self, buffer : Buffer, offset : DeviceSize) -> Self {
		unsafe { vkCmdDispatchIndirect(self, buffer, offset) };
		self
	}

	pub fn copy_buffer(self, srcBuffer : Buffer, dstBuffer : Buffer, regions : &[BufferCopy]) -> Self {
		unsafe { vkCmdCopyBuffer(self, srcBuffer, dstBuffer, regions.len() as _, regions.as_ptr()) };
		self
	}

	pub fn copy_image(self, srcImage : Image, srcImageLayout : ImageLayout, dstImage : Image, dstImageLayout : ImageLayout, regions : &[ImageCopy]) -> Self {
		unsafe { vkCmdCopyImage(self, srcImage, srcImageLayout, dstImage, dstImageLayout, regions.len() as _, regions.as_ptr()) };
		self
	}

	pub fn blit_image(self, srcImage : Image, srcImageLayout : ImageLayout, dstImage : Image, dstImageLayout : ImageLayout, regions : &[ImageBlit], filter : Filter) -> Self {
		unsafe { vkCmdBlitImage(self, srcImage, srcImageLayout, dstImage, dstImageLayout, regions.len() as _, regions.as_ptr(), filter) };
		self
	}

	pub fn copy_buffer_to_image(self, srcBuffer : Buffer, dstImage : Image, dstImageLayout : ImageLayout, regions : &[BufferImageCopy]) -> Self {
		unsafe { vkCmdCopyBufferToImage(self, srcBuffer, dstImage, dstImageLayout, regions.len() as _, regions.as_ptr()) };
		self
	}

	pub fn copy_image_to_buffer(self, srcImage : Image, srcImageLayout : ImageLayout, dstBuffer : Buffer, regions : &[BufferImageCopy]) -> Self {
		unsafe { vkCmdCopyImageToBuffer(self, srcImage, srcImageLayout, dstBuffer, regions.len() as _, regions.as_ptr()) };
		self
	}

	pub fn update_buffer(self, dstBuffer : Buffer, dstOffset : DeviceSize, dataSize : DeviceSize, pData : *const c_void) -> Self {
		unsafe { vkCmdUpdateBuffer(self, dstBuffer, dstOffset, dataSize, pData) };
		self
	}

	pub fn fill_buffer(self, dstBuffer : Buffer, dstOffset : DeviceSize, size : DeviceSize, data : u32) -> Self {
		unsafe { vkCmdFillBuffer(self, dstBuffer, dstOffset, size, data) };
		self
	}

	pub fn clear_color_image(self, image : Image, imageLayout : ImageLayout, pColor : &ClearColorValue, rangeCount : u32, pRanges : &ImageSubresourceRange) -> Self {
		unsafe { vkCmdClearColorImage(self, image, imageLayout, pColor, rangeCount, pRanges) };
		self
	}

	pub fn clear_depth_stencil_image(self, image : Image, imageLayout : ImageLayout, pDepthStencil : &ClearDepthStencilValue, rangeCount : u32, pRanges : &ImageSubresourceRange) -> Self {
		unsafe { vkCmdClearDepthStencilImage(self, image, imageLayout, pDepthStencil, rangeCount, pRanges) };
		self
	}

	pub fn clear_attachments(self, attachments : &[ClearAttachment], rects : &[ClearRect]) -> Self {
		unsafe { vkCmdClearAttachments(self, attachments.len() as _, attachments.as_ptr(), rects.len() as _, rects.as_ptr()) };
		self
	}

	pub fn resolve_image(self, srcImage : Image, srcImageLayout : ImageLayout, dstImage : Image, dstImageLayout : ImageLayout, regions : &[ImageResolve]) -> Self {
		unsafe { vkCmdResolveImage(self, srcImage, srcImageLayout, dstImage, dstImageLayout, regions.len() as _, regions.as_ptr()) };
		self
	}

	pub fn set_event(self, event : Event, stageMask : PipelineStageFlags) -> Self {
		unsafe { vkCmdSetEvent(self, event, stageMask) };
		self
	}

	pub fn reset_event(self, event : Event, stageMask : PipelineStageFlags) -> Self {
		unsafe { vkCmdResetEvent(self, event, stageMask) };
		self
	}

	pub fn wait_events(self, events : &[Event], srcStageMask : PipelineStageFlags, dstStageMask : PipelineStageFlags, memoryBarriers : &[MemoryBarrier], bufferMemoryBarriers : &[BufferMemoryBarrier], imageMemoryBarriers : &[ImageMemoryBarrier]) -> Self {
		unsafe { vkCmdWaitEvents(self, events.len() as _, events.as_ptr(), srcStageMask, dstStageMask, memoryBarriers.len() as _, memoryBarriers.as_ptr(), bufferMemoryBarriers.len() as _, bufferMemoryBarriers.as_ptr(), imageMemoryBarriers.len() as _, imageMemoryBarriers.as_ptr()) };
		self
	}

	pub fn pipeline_barrier(self, srcStageMask : PipelineStageFlags, dstStageMask : PipelineStageFlags, dependencyFlags : DependencyFlags, memoryBarriers : &[MemoryBarrier], bufferMemoryBarriers : &[BufferMemoryBarrier], imageMemoryBarriers : &[ImageMemoryBarrier]) -> Self {
		unsafe { vkCmdPipelineBarrier(self, srcStageMask, dstStageMask, dependencyFlags, memoryBarriers.len() as _, memoryBarriers.as_ptr(), bufferMemoryBarriers.len() as _, bufferMemoryBarriers.as_ptr(), imageMemoryBarriers.len() as _, imageMemoryBarriers.as_ptr()) };
		self
	}

	pub fn begin_query(self, queryPool : QueryPool, query : u32, flags : QueryControlFlags) -> Self {
		unsafe { vkCmdBeginQuery(self, queryPool, query, flags) };
		self
	}

	pub fn end_query(self, queryPool : QueryPool, query : u32) -> Self {
		unsafe { vkCmdEndQuery(self, queryPool, query) };
		self
	}

	pub fn reset_query_pool(self, queryPool : QueryPool, firstQuery : u32, queryCount : u32) -> Self {
		unsafe { vkCmdResetQueryPool(self, queryPool, firstQuery, queryCount) };
		self
	}

	pub fn write_timestamp(self, pipelineStage : PipelineStageFlags, queryPool : QueryPool, query : u32) -> Self {
		unsafe { vkCmdWriteTimestamp(self, pipelineStage, queryPool, query) };
		self
	}

	pub fn copy_query_pool_results(self, queryPool : QueryPool, firstQuery : u32, queryCount : u32, dstBuffer : Buffer, dstOffset : DeviceSize, stride : DeviceSize, flags : QueryResultFlags) -> Self {
		unsafe { vkCmdCopyQueryPoolResults(self, queryPool, firstQuery, queryCount, dstBuffer, dstOffset, stride, flags) };
		self
	}

	pub fn push_constants(self, layout : PipelineLayout, stageFlags : ShaderStageFlags, offset : u32, size : u32, pValues : *const c_void) -> Self {
		unsafe { vkCmdPushConstants(self, layout, stageFlags, offset, size, pValues) };
		self
	}

	pub fn begin_render_pass(self, pRenderPassBegin : &RenderPassBeginInfo, contents : SubpassContents) -> Self {
		unsafe { vkCmdBeginRenderPass(self, pRenderPassBegin, contents) };
		self
	}

	pub fn next_subpass(self, contents : SubpassContents) -> Self {
		unsafe { vkCmdNextSubpass(self, contents) };
		self
	}

	pub fn end_render_pass(self) -> Self {
		unsafe { vkCmdEndRenderPass(self) };
		self
	}

	pub fn execute_commands(self, commandBuffers : &[CommandBuffer]) -> Self {
		unsafe { vkCmdExecuteCommands(self, commandBuffers.len() as _, commandBuffers.as_ptr()) };
		self
	}

	pub fn set_device_mask(self, deviceMask : u32) -> Self {
		unsafe { vkCmdSetDeviceMask(self, deviceMask) };
		self
	}

	pub fn dispatch_base(self, baseGroupX : u32, baseGroupY : u32, baseGroupZ : u32, groupCountX : u32, groupCountY : u32, groupCountZ : u32) -> Self {
		unsafe { vkCmdDispatchBase(self, baseGroupX, baseGroupY, baseGroupZ, groupCountX, groupCountY, groupCountZ) };
		self
	}

	pub fn draw_indirect_count(self, buffer : Buffer, offset : DeviceSize, countBuffer : Buffer, countBufferOffset : DeviceSize, maxDrawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawIndirectCount(self, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride) };
		self
	}

	pub fn draw_indexed_indirect_count(self, buffer : Buffer, offset : DeviceSize, countBuffer : Buffer, countBufferOffset : DeviceSize, maxDrawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawIndexedIndirectCount(self, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride) };
		self
	}

	pub fn begin_render_pass_2(self, pRenderPassBegin : &RenderPassBeginInfo, pSubpassBeginInfo : &SubpassBeginInfo) -> Self {
		unsafe { vkCmdBeginRenderPass2(self, pRenderPassBegin, pSubpassBeginInfo) };
		self
	}

	pub fn next_subpass_2(self, pSubpassBeginInfo : &SubpassBeginInfo, pSubpassEndInfo : &SubpassEndInfo) -> Self {
		unsafe { vkCmdNextSubpass2(self, pSubpassBeginInfo, pSubpassEndInfo) };
		self
	}

	pub fn end_render_pass_2(self, pSubpassEndInfo : &SubpassEndInfo) -> Self {
		unsafe { vkCmdEndRenderPass2(self, pSubpassEndInfo) };
		self
	}

	pub fn set_device_mask_khr(self, deviceMask : u32) -> Self {
		unsafe { vkCmdSetDeviceMaskKHR(self, deviceMask) };
		self
	}

	pub fn dispatch_base_khr(self, baseGroupX : u32, baseGroupY : u32, baseGroupZ : u32, groupCountX : u32, groupCountY : u32, groupCountZ : u32) -> Self {
		unsafe { vkCmdDispatchBaseKHR(self, baseGroupX, baseGroupY, baseGroupZ, groupCountX, groupCountY, groupCountZ) };
		self
	}

	pub fn push_descriptor_set_khr(self, pipelineBindPoint : PipelineBindPoint, layout : PipelineLayout, set : u32, descriptorWrites : &[WriteDescriptorSet]) -> Self {
		unsafe { vkCmdPushDescriptorSetKHR(self, pipelineBindPoint, layout, set, descriptorWrites.len() as _, descriptorWrites.as_ptr()) };
		self
	}

	pub fn push_descriptor_set_with_template_khr(self, descriptorUpdateTemplate : DescriptorUpdateTemplate, layout : PipelineLayout, set : u32, pData : *const c_void) -> Self {
		unsafe { vkCmdPushDescriptorSetWithTemplateKHR(self, descriptorUpdateTemplate, layout, set, pData) };
		self
	}

	pub fn begin_render_pass_2_khr(self, pRenderPassBegin : &RenderPassBeginInfo, pSubpassBeginInfo : &SubpassBeginInfo) -> Self {
		unsafe { vkCmdBeginRenderPass2KHR(self, pRenderPassBegin, pSubpassBeginInfo) };
		self
	}

	pub fn next_subpass_2_khr(self, pSubpassBeginInfo : &SubpassBeginInfo, pSubpassEndInfo : &SubpassEndInfo) -> Self {
		unsafe { vkCmdNextSubpass2KHR(self, pSubpassBeginInfo, pSubpassEndInfo) };
		self
	}

	pub fn end_render_pass_2_khr(self, pSubpassEndInfo : &SubpassEndInfo) -> Self {
		unsafe { vkCmdEndRenderPass2KHR(self, pSubpassEndInfo) };
		self
	}

	pub fn draw_indirect_count_khr(self, buffer : Buffer, offset : DeviceSize, countBuffer : Buffer, countBufferOffset : DeviceSize, maxDrawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawIndirectCountKHR(self, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride) };
		self
	}

	pub fn draw_indexed_indirect_count_khr(self, buffer : Buffer, offset : DeviceSize, countBuffer : Buffer, countBufferOffset : DeviceSize, maxDrawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawIndexedIndirectCountKHR(self, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride) };
		self
	}

	pub fn debug_marker_begin_ext(self, pMarkerInfo : &DebugMarkerMarkerInfoEXT) -> Self {
		unsafe { vkCmdDebugMarkerBeginEXT(self, pMarkerInfo) };
		self
	}

	pub fn debug_marker_end_ext(self) -> Self {
		unsafe { vkCmdDebugMarkerEndEXT(self) };
		self
	}

	pub fn debug_marker_insert_ext(self, pMarkerInfo : &DebugMarkerMarkerInfoEXT) -> Self {
		unsafe { vkCmdDebugMarkerInsertEXT(self, pMarkerInfo) };
		self
	}

	pub fn bind_transform_feedback_buffers_ext(self, firstBinding : u32, bindingCount : u32, pBuffers : &Buffer, pOffsets : &DeviceSize, pSizes : &DeviceSize) -> Self {
		unsafe { vkCmdBindTransformFeedbackBuffersEXT(self, firstBinding, bindingCount, pBuffers, pOffsets, pSizes) };
		self
	}

	pub fn begin_transform_feedback_ext(self, firstCounterBuffer : u32, counterBufferCount : u32, pCounterBuffers : &Buffer, pCounterBufferOffsets : &DeviceSize) -> Self {
		unsafe { vkCmdBeginTransformFeedbackEXT(self, firstCounterBuffer, counterBufferCount, pCounterBuffers, pCounterBufferOffsets) };
		self
	}

	pub fn end_transform_feedback_ext(self, firstCounterBuffer : u32, counterBufferCount : u32, pCounterBuffers : &Buffer, pCounterBufferOffsets : &DeviceSize) -> Self {
		unsafe { vkCmdEndTransformFeedbackEXT(self, firstCounterBuffer, counterBufferCount, pCounterBuffers, pCounterBufferOffsets) };
		self
	}

	pub fn begin_query_indexed_ext(self, queryPool : QueryPool, query : u32, flags : QueryControlFlags, index : u32) -> Self {
		unsafe { vkCmdBeginQueryIndexedEXT(self, queryPool, query, flags, index) };
		self
	}

	pub fn end_query_indexed_ext(self, queryPool : QueryPool, query : u32, index : u32) -> Self {
		unsafe { vkCmdEndQueryIndexedEXT(self, queryPool, query, index) };
		self
	}

	pub fn draw_indirect_byte_count_ext(self, instanceCount : u32, firstInstance : u32, counterBuffer : Buffer, counterBufferOffset : DeviceSize, counterOffset : u32, vertexStride : u32) -> Self {
		unsafe { vkCmdDrawIndirectByteCountEXT(self, instanceCount, firstInstance, counterBuffer, counterBufferOffset, counterOffset, vertexStride) };
		self
	}

	pub fn draw_indirect_count_amd(self, buffer : Buffer, offset : DeviceSize, countBuffer : Buffer, countBufferOffset : DeviceSize, maxDrawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawIndirectCountAMD(self, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride) };
		self
	}

	pub fn draw_indexed_indirect_count_amd(self, buffer : Buffer, offset : DeviceSize, countBuffer : Buffer, countBufferOffset : DeviceSize, maxDrawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawIndexedIndirectCountAMD(self, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride) };
		self
	}

	pub fn begin_conditional_rendering_ext(self, pConditionalRenderingBegin : &ConditionalRenderingBeginInfoEXT) -> Self {
		unsafe { vkCmdBeginConditionalRenderingEXT(self, pConditionalRenderingBegin) };
		self
	}

	pub fn end_conditional_rendering_ext(self) -> Self {
		unsafe { vkCmdEndConditionalRenderingEXT(self) };
		self
	}

	pub fn set_viewport_wscaling_nv(self, firstViewport : u32, viewportWScalings : &[ViewportWScalingNV]) -> Self {
		unsafe { vkCmdSetViewportWScalingNV(self, firstViewport, viewportWScalings.len() as _, viewportWScalings.as_ptr()) };
		self
	}

	pub fn set_discard_rectangle_ext(self, firstDiscardRectangle : u32, discardRectangles : &[Rect2D]) -> Self {
		unsafe { vkCmdSetDiscardRectangleEXT(self, firstDiscardRectangle, discardRectangles.len() as _, discardRectangles.as_ptr()) };
		self
	}

	pub fn begin_debug_utils_label_ext(self, pLabelInfo : &DebugUtilsLabelEXT) -> Self {
		unsafe { vkCmdBeginDebugUtilsLabelEXT(self, pLabelInfo) };
		self
	}

	pub fn end_debug_utils_label_ext(self) -> Self {
		unsafe { vkCmdEndDebugUtilsLabelEXT(self) };
		self
	}

	pub fn insert_debug_utils_label_ext(self, pLabelInfo : &DebugUtilsLabelEXT) -> Self {
		unsafe { vkCmdInsertDebugUtilsLabelEXT(self, pLabelInfo) };
		self
	}

	pub fn set_sample_locations_ext(self, pSampleLocationsInfo : &SampleLocationsInfoEXT) -> Self {
		unsafe { vkCmdSetSampleLocationsEXT(self, pSampleLocationsInfo) };
		self
	}

	pub fn bind_shading_rate_image_nv(self, imageView : ImageView, imageLayout : ImageLayout) -> Self {
		unsafe { vkCmdBindShadingRateImageNV(self, imageView, imageLayout) };
		self
	}

	pub fn set_viewport_shading_rate_palette_nv(self, firstViewport : u32, shadingRatePalettes : &[ShadingRatePaletteNV]) -> Self {
		unsafe { vkCmdSetViewportShadingRatePaletteNV(self, firstViewport, shadingRatePalettes.len() as _, shadingRatePalettes.as_ptr()) };
		self
	}

	pub fn set_coarse_sample_order_nv(self, sampleOrderType : CoarseSampleOrderTypeNV, customSampleOrders : &[CoarseSampleOrderCustomNV]) -> Self {
		unsafe { vkCmdSetCoarseSampleOrderNV(self, sampleOrderType, customSampleOrders.len() as _, customSampleOrders.as_ptr()) };
		self
	}

	pub fn build_acceleration_structure_nv(self, pInfo : &AccelerationStructureInfoNV, instanceData : Buffer, instanceOffset : DeviceSize, update : Bool32, dst : AccelerationStructureKHR, src : AccelerationStructureKHR, scratch : Buffer, scratchOffset : DeviceSize) -> Self {
		unsafe { vkCmdBuildAccelerationStructureNV(self, pInfo, instanceData, instanceOffset, update, dst, src, scratch, scratchOffset) };
		self
	}

	pub fn copy_acceleration_structure_nv(self, dst : AccelerationStructureKHR, src : AccelerationStructureKHR, mode : CopyAccelerationStructureModeKHR) -> Self {
		unsafe { vkCmdCopyAccelerationStructureNV(self, dst, src, mode) };
		self
	}

	pub fn trace_rays_nv(self, raygenShaderBindingTableBuffer : Buffer, raygenShaderBindingOffset : DeviceSize, missShaderBindingTableBuffer : Buffer, missShaderBindingOffset : DeviceSize, missShaderBindingStride : DeviceSize, hitShaderBindingTableBuffer : Buffer, hitShaderBindingOffset : DeviceSize, hitShaderBindingStride : DeviceSize, callableShaderBindingTableBuffer : Buffer, callableShaderBindingOffset : DeviceSize, callableShaderBindingStride : DeviceSize, width : u32, height : u32, depth : u32) -> Self {
		unsafe { vkCmdTraceRaysNV(self, raygenShaderBindingTableBuffer, raygenShaderBindingOffset, missShaderBindingTableBuffer, missShaderBindingOffset, missShaderBindingStride, hitShaderBindingTableBuffer, hitShaderBindingOffset, hitShaderBindingStride, callableShaderBindingTableBuffer, callableShaderBindingOffset, callableShaderBindingStride, width, height, depth) };
		self
	}

	pub fn write_acceleration_structures_properties_khr(self, accelerationStructures : &[AccelerationStructureKHR], queryType : QueryType, queryPool : QueryPool, firstQuery : u32) -> Self {
		unsafe { vkCmdWriteAccelerationStructuresPropertiesKHR(self, accelerationStructures.len() as _, accelerationStructures.as_ptr(), queryType, queryPool, firstQuery) };
		self
	}

	pub fn write_acceleration_structures_properties_nv(self, accelerationStructures : &[AccelerationStructureKHR], queryType : QueryType, queryPool : QueryPool, firstQuery : u32) -> Self {
		unsafe { vkCmdWriteAccelerationStructuresPropertiesNV(self, accelerationStructures.len() as _, accelerationStructures.as_ptr(), queryType, queryPool, firstQuery) };
		self
	}

	pub fn write_buffer_marker_amd(self, pipelineStage : PipelineStageFlags, dstBuffer : Buffer, dstOffset : DeviceSize, marker : u32) -> Self {
		unsafe { vkCmdWriteBufferMarkerAMD(self, pipelineStage, dstBuffer, dstOffset, marker) };
		self
	}

	pub fn draw_mesh_tasks_nv(self, taskCount : u32, firstTask : u32) -> Self {
		unsafe { vkCmdDrawMeshTasksNV(self, taskCount, firstTask) };
		self
	}

	pub fn draw_mesh_tasks_indirect_nv(self, buffer : Buffer, offset : DeviceSize, drawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawMeshTasksIndirectNV(self, buffer, offset, drawCount, stride) };
		self
	}

	pub fn draw_mesh_tasks_indirect_count_nv(self, buffer : Buffer, offset : DeviceSize, countBuffer : Buffer, countBufferOffset : DeviceSize, maxDrawCount : u32, stride : u32) -> Self {
		unsafe { vkCmdDrawMeshTasksIndirectCountNV(self, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride) };
		self
	}

	pub fn set_exclusive_scissor_nv(self, firstExclusiveScissor : u32, exclusiveScissors : &[Rect2D]) -> Self {
		unsafe { vkCmdSetExclusiveScissorNV(self, firstExclusiveScissor, exclusiveScissors.len() as _, exclusiveScissors.as_ptr()) };
		self
	}

	pub fn set_checkpoint_nv(self, pCheckpointMarker : *const c_void) -> Self {
		unsafe { vkCmdSetCheckpointNV(self, pCheckpointMarker) };
		self
	}

	pub fn set_performance_marker_intel(self, pMarkerInfo : &PerformanceMarkerInfoINTEL) -> VkResult {
		unsafe { vkCmdSetPerformanceMarkerINTEL(self, pMarkerInfo) }
	}

	pub fn set_performance_stream_marker_intel(self, pMarkerInfo : &PerformanceStreamMarkerInfoINTEL) -> VkResult {
		unsafe { vkCmdSetPerformanceStreamMarkerINTEL(self, pMarkerInfo) }
	}

	pub fn set_performance_override_intel(self, pOverrideInfo : &PerformanceOverrideInfoINTEL) -> VkResult {
		unsafe { vkCmdSetPerformanceOverrideINTEL(self, pOverrideInfo) }
	}

	pub fn set_line_stipple_ext(self, lineStippleFactor : u32, lineStipplePattern : u16) -> Self {
		unsafe { vkCmdSetLineStippleEXT(self, lineStippleFactor, lineStipplePattern) };
		self
	}

	pub fn set_cull_mode_ext(self, cullMode : CullModeFlags) -> Self {
		unsafe { vkCmdSetCullModeEXT(self, cullMode) };
		self
	}

	pub fn set_front_face_ext(self, frontFace : FrontFace) -> Self {
		unsafe { vkCmdSetFrontFaceEXT(self, frontFace) };
		self
	}

	pub fn set_primitive_topology_ext(self, primitiveTopology : PrimitiveTopology) -> Self {
		unsafe { vkCmdSetPrimitiveTopologyEXT(self, primitiveTopology) };
		self
	}

	pub fn set_viewport_with_count_ext(self, viewports : &[Viewport]) -> Self {
		unsafe { vkCmdSetViewportWithCountEXT(self, viewports.len() as _, viewports.as_ptr()) };
		self
	}

	pub fn set_scissor_with_count_ext(self, scissors : &[Rect2D]) -> Self {
		unsafe { vkCmdSetScissorWithCountEXT(self, scissors.len() as _, scissors.as_ptr()) };
		self
	}

	pub fn bind_vertex_buffers_2_ext(self, firstBinding : u32, bindingCount : u32, pBuffers : &Buffer, pOffsets : &DeviceSize, pSizes : &DeviceSize, pStrides : &DeviceSize) -> Self {
		unsafe { vkCmdBindVertexBuffers2EXT(self, firstBinding, bindingCount, pBuffers, pOffsets, pSizes, pStrides) };
		self
	}

	pub fn set_depth_test_enable_ext(self, depthTestEnable : Bool32) -> Self {
		unsafe { vkCmdSetDepthTestEnableEXT(self, depthTestEnable) };
		self
	}

	pub fn set_depth_write_enable_ext(self, depthWriteEnable : Bool32) -> Self {
		unsafe { vkCmdSetDepthWriteEnableEXT(self, depthWriteEnable) };
		self
	}

	pub fn set_depth_compare_op_ext(self, depthCompareOp : CompareOp) -> Self {
		unsafe { vkCmdSetDepthCompareOpEXT(self, depthCompareOp) };
		self
	}

	pub fn set_depth_bounds_test_enable_ext(self, depthBoundsTestEnable : Bool32) -> Self {
		unsafe { vkCmdSetDepthBoundsTestEnableEXT(self, depthBoundsTestEnable) };
		self
	}

	pub fn set_stencil_test_enable_ext(self, stencilTestEnable : Bool32) -> Self {
		unsafe { vkCmdSetStencilTestEnableEXT(self, stencilTestEnable) };
		self
	}

	pub fn set_stencil_op_ext(self, faceMask : StencilFaceFlags, failOp : StencilOp, passOp : StencilOp, depthFailOp : StencilOp, compareOp : CompareOp) -> Self {
		unsafe { vkCmdSetStencilOpEXT(self, faceMask, failOp, passOp, depthFailOp, compareOp) };
		self
	}

	pub fn preprocess_generated_commands_nv(self, pGeneratedCommandsInfo : &GeneratedCommandsInfoNV) -> Self {
		unsafe { vkCmdPreprocessGeneratedCommandsNV(self, pGeneratedCommandsInfo) };
		self
	}

	pub fn execute_generated_commands_nv(self, isPreprocessed : Bool32, pGeneratedCommandsInfo : &GeneratedCommandsInfoNV) -> Self {
		unsafe { vkCmdExecuteGeneratedCommandsNV(self, isPreprocessed, pGeneratedCommandsInfo) };
		self
	}

	pub fn bind_pipeline_shader_group_nv(self, pipelineBindPoint : PipelineBindPoint, pipeline : Pipeline, groupIndex : u32) -> Self {
		unsafe { vkCmdBindPipelineShaderGroupNV(self, pipelineBindPoint, pipeline, groupIndex) };
		self
	}

}

impl Queue {

	pub fn submit(self, submits : &[SubmitInfo], fence : Fence) -> VkResult {
		unsafe { vkQueueSubmit(self, submits.len() as _, submits.as_ptr(), fence) }
	}

	pub fn wait_idle(self) -> VkResult {
		unsafe { vkQueueWaitIdle(self) }
	}

	pub fn bind_sparse(self, bindInfo : &[BindSparseInfo], fence : Fence) -> VkResult {
		unsafe { vkQueueBindSparse(self, bindInfo.len() as _, bindInfo.as_ptr(), fence) }
	}

	pub fn present_khr(self, pPresentInfo : &PresentInfoKHR) -> VkResult {
		unsafe { vkQueuePresentKHR(self, pPresentInfo) }
	}

	pub fn begin_debug_utils_label_ext(self, pLabelInfo : &DebugUtilsLabelEXT) -> Self {
		unsafe { vkQueueBeginDebugUtilsLabelEXT(self, pLabelInfo) };
		self
	}

	pub fn end_debug_utils_label_ext(self) -> Self {
		unsafe { vkQueueEndDebugUtilsLabelEXT(self) };
		self
	}

	pub fn insert_debug_utils_label_ext(self, pLabelInfo : &DebugUtilsLabelEXT) -> Self {
		unsafe { vkQueueInsertDebugUtilsLabelEXT(self, pLabelInfo) };
		self
	}

	pub fn get_checkpoint_data_nv(self) -> Vec<CheckpointDataNV> {
		let mut count = 0;
		unsafe { vkGetQueueCheckpointDataNV(self, &mut count, null_mut()) };
		let mut checkpointData = vec![CheckpointDataNV::new(); count as _ ];
		unsafe { vkGetQueueCheckpointDataNV(self, &mut count, checkpointData.as_mut_ptr()) };
		checkpointData
	}

	pub fn set_performance_configuration_intel(self, configuration : PerformanceConfigurationINTEL) -> VkResult {
		unsafe { vkQueueSetPerformanceConfigurationINTEL(self, configuration) }
	}

}

impl Device {

	pub fn get_proc_addr(self, pName : &u8) -> Option<extern "C" fn()> {
		unsafe { vkGetDeviceProcAddr(self, pName) }
	}

	pub fn destroy(self) -> Self {
		unsafe { vkDestroyDevice(self, null()) };
		self
	}

	pub fn get_queue(self, queueFamilyIndex : u32, queueIndex : u32) -> Queue {
		let mut queue = Queue(0);
		unsafe { vkGetDeviceQueue(self, queueFamilyIndex, queueIndex, &mut queue) };
		queue
	}

	pub fn wait_idle(self) -> VkResult {
		unsafe { vkDeviceWaitIdle(self) }
	}

	pub fn allocate_memory(self, pAllocateInfo : &MemoryAllocateInfo) -> Result<DeviceMemory, VkResult> {
		let mut memory = DeviceMemory(0);
		match unsafe { vkAllocateMemory(self, pAllocateInfo, null(), &mut memory) } {
			VkResult::SUCCESS => Ok(memory),
			err => Err(err)
		}
	}

	pub fn free_memory(self, memory : DeviceMemory) -> Self {
		unsafe { vkFreeMemory(self, memory, null()) };
		self
	}

	pub fn map_memory(self, memory : DeviceMemory, offset : DeviceSize, size : DeviceSize, flags : MemoryMapFlags) -> Result<*mut c_void, VkResult> {
		let mut data = null_mut();
		match unsafe { vkMapMemory(self, memory, offset, size, flags, &mut data) } {
			VkResult::SUCCESS => Ok(data),
			err => Err(err)
		}
	}

	pub fn unmap_memory(self, memory : DeviceMemory) -> Self {
		unsafe { vkUnmapMemory(self, memory) };
		self
	}

	pub fn flush_mapped_memory_ranges(self, memoryRanges : &[MappedMemoryRange]) -> VkResult {
		unsafe { vkFlushMappedMemoryRanges(self, memoryRanges.len() as _, memoryRanges.as_ptr()) }
	}

	pub fn invalidate_mapped_memory_ranges(self, memoryRanges : &[MappedMemoryRange]) -> VkResult {
		unsafe { vkInvalidateMappedMemoryRanges(self, memoryRanges.len() as _, memoryRanges.as_ptr()) }
	}

	pub fn get_memory_commitment(self, memory : DeviceMemory) -> DeviceSize {
		let mut committedMemoryInBytes = <_>::default();
		unsafe { vkGetDeviceMemoryCommitment(self, memory, &mut committedMemoryInBytes) };
		committedMemoryInBytes
	}

	pub fn bind_buffer_memory(self, buffer : Buffer, memory : DeviceMemory, memoryOffset : DeviceSize) -> VkResult {
		unsafe { vkBindBufferMemory(self, buffer, memory, memoryOffset) }
	}

	pub fn bind_image_memory(self, image : Image, memory : DeviceMemory, memoryOffset : DeviceSize) -> VkResult {
		unsafe { vkBindImageMemory(self, image, memory, memoryOffset) }
	}

	pub fn get_buffer_memory_requirements(self, buffer : Buffer) -> MemoryRequirements {
		let mut memoryRequirements = MemoryRequirements::new();
		unsafe { vkGetBufferMemoryRequirements(self, buffer, &mut memoryRequirements) };
		memoryRequirements
	}

	pub fn get_image_memory_requirements(self, image : Image) -> MemoryRequirements {
		let mut memoryRequirements = MemoryRequirements::new();
		unsafe { vkGetImageMemoryRequirements(self, image, &mut memoryRequirements) };
		memoryRequirements
	}

	pub fn get_image_sparse_memory_requirements(self, image : Image) -> Vec<SparseImageMemoryRequirements> {
		let mut count = 0;
		unsafe { vkGetImageSparseMemoryRequirements(self, image, &mut count, null_mut()) };
		let mut sparseMemoryRequirements = vec![SparseImageMemoryRequirements::new(); count as _ ];
		unsafe { vkGetImageSparseMemoryRequirements(self, image, &mut count, sparseMemoryRequirements.as_mut_ptr()) };
		sparseMemoryRequirements
	}

	pub fn create_fence(self, pCreateInfo : &FenceCreateInfo) -> Result<Fence, VkResult> {
		let mut fence = Fence(0);
		match unsafe { vkCreateFence(self, pCreateInfo, null(), &mut fence) } {
			VkResult::SUCCESS => Ok(fence),
			err => Err(err)
		}
	}

	pub fn destroy_fence(self, fence : Fence) -> Self {
		unsafe { vkDestroyFence(self, fence, null()) };
		self
	}

	pub fn reset_fences(self, fences : &[Fence]) -> VkResult {
		unsafe { vkResetFences(self, fences.len() as _, fences.as_ptr()) }
	}

	pub fn get_fence_status(self, fence : Fence) -> VkResult {
		unsafe { vkGetFenceStatus(self, fence) }
	}

	pub fn wait_for_fences(self, fences : &[Fence], waitAll : Bool32, timeout : u64) -> VkResult {
		unsafe { vkWaitForFences(self, fences.len() as _, fences.as_ptr(), waitAll, timeout) }
	}

	pub fn create_semaphore(self, pCreateInfo : &SemaphoreCreateInfo) -> Result<Semaphore, VkResult> {
		let mut semaphore = Semaphore(0);
		match unsafe { vkCreateSemaphore(self, pCreateInfo, null(), &mut semaphore) } {
			VkResult::SUCCESS => Ok(semaphore),
			err => Err(err)
		}
	}

	pub fn destroy_semaphore(self, semaphore : Semaphore) -> Self {
		unsafe { vkDestroySemaphore(self, semaphore, null()) };
		self
	}

	pub fn create_event(self, pCreateInfo : &EventCreateInfo) -> Result<Event, VkResult> {
		let mut event = Event(0);
		match unsafe { vkCreateEvent(self, pCreateInfo, null(), &mut event) } {
			VkResult::SUCCESS => Ok(event),
			err => Err(err)
		}
	}

	pub fn destroy_event(self, event : Event) -> Self {
		unsafe { vkDestroyEvent(self, event, null()) };
		self
	}

	pub fn get_event_status(self, event : Event) -> VkResult {
		unsafe { vkGetEventStatus(self, event) }
	}

	pub fn set_event(self, event : Event) -> VkResult {
		unsafe { vkSetEvent(self, event) }
	}

	pub fn reset_event(self, event : Event) -> VkResult {
		unsafe { vkResetEvent(self, event) }
	}

	pub fn create_query_pool(self, pCreateInfo : &QueryPoolCreateInfo) -> Result<QueryPool, VkResult> {
		let mut queryPool = QueryPool(0);
		match unsafe { vkCreateQueryPool(self, pCreateInfo, null(), &mut queryPool) } {
			VkResult::SUCCESS => Ok(queryPool),
			err => Err(err)
		}
	}

	pub fn destroy_query_pool(self, queryPool : QueryPool) -> Self {
		unsafe { vkDestroyQueryPool(self, queryPool, null()) };
		self
	}

	pub fn get_query_pool_results(self, queryPool : QueryPool, firstQuery : u32, queryCount : u32, dataSize : usize, pData : *mut c_void, stride : DeviceSize, flags : QueryResultFlags) -> VkResult {
		unsafe { vkGetQueryPoolResults(self, queryPool, firstQuery, queryCount, dataSize, pData, stride, flags) }
	}

	pub fn create_buffer(self, pCreateInfo : &BufferCreateInfo) -> Result<Buffer, VkResult> {
		let mut buffer = Buffer(0);
		match unsafe { vkCreateBuffer(self, pCreateInfo, null(), &mut buffer) } {
			VkResult::SUCCESS => Ok(buffer),
			err => Err(err)
		}
	}

	pub fn destroy_buffer(self, buffer : Buffer) -> Self {
		unsafe { vkDestroyBuffer(self, buffer, null()) };
		self
	}

	pub fn create_buffer_view(self, pCreateInfo : &BufferViewCreateInfo) -> Result<BufferView, VkResult> {
		let mut view = BufferView(0);
		match unsafe { vkCreateBufferView(self, pCreateInfo, null(), &mut view) } {
			VkResult::SUCCESS => Ok(view),
			err => Err(err)
		}
	}

	pub fn destroy_buffer_view(self, bufferView : BufferView) -> Self {
		unsafe { vkDestroyBufferView(self, bufferView, null()) };
		self
	}

	pub fn create_image(self, pCreateInfo : &ImageCreateInfo) -> Result<Image, VkResult> {
		let mut image = Image(0);
		match unsafe { vkCreateImage(self, pCreateInfo, null(), &mut image) } {
			VkResult::SUCCESS => Ok(image),
			err => Err(err)
		}
	}

	pub fn destroy_image(self, image : Image) -> Self {
		unsafe { vkDestroyImage(self, image, null()) };
		self
	}

	pub fn get_image_subresource_layout(self, image : Image, pSubresource : &ImageSubresource) -> SubresourceLayout {
		let mut layout = SubresourceLayout::new();
		unsafe { vkGetImageSubresourceLayout(self, image, pSubresource, &mut layout) };
		layout
	}

	pub fn create_image_view(self, pCreateInfo : &ImageViewCreateInfo) -> Result<ImageView, VkResult> {
		let mut view = ImageView(0);
		match unsafe { vkCreateImageView(self, pCreateInfo, null(), &mut view) } {
			VkResult::SUCCESS => Ok(view),
			err => Err(err)
		}
	}

	pub fn destroy_image_view(self, imageView : ImageView) -> Self {
		unsafe { vkDestroyImageView(self, imageView, null()) };
		self
	}

	pub fn create_shader_module(self, pCreateInfo : &ShaderModuleCreateInfo) -> Result<ShaderModule, VkResult> {
		let mut shaderModule = ShaderModule(0);
		match unsafe { vkCreateShaderModule(self, pCreateInfo, null(), &mut shaderModule) } {
			VkResult::SUCCESS => Ok(shaderModule),
			err => Err(err)
		}
	}

	pub fn destroy_shader_module(self, shaderModule : ShaderModule) -> Self {
		unsafe { vkDestroyShaderModule(self, shaderModule, null()) };
		self
	}

	pub fn create_pipeline_cache(self, pCreateInfo : &PipelineCacheCreateInfo) -> Result<PipelineCache, VkResult> {
		let mut pipelineCache = PipelineCache(0);
		match unsafe { vkCreatePipelineCache(self, pCreateInfo, null(), &mut pipelineCache) } {
			VkResult::SUCCESS => Ok(pipelineCache),
			err => Err(err)
		}
	}

	pub fn destroy_pipeline_cache(self, pipelineCache : PipelineCache) -> Self {
		unsafe { vkDestroyPipelineCache(self, pipelineCache, null()) };
		self
	}

	pub fn get_pipeline_cache_data(self, pipelineCache : PipelineCache, pDataSize : *mut usize, pData : *mut c_void) -> VkResult {
		unsafe { vkGetPipelineCacheData(self, pipelineCache, pDataSize, pData) }
	}

	pub fn merge_pipeline_caches(self, dstCache : PipelineCache, srcCaches : &[PipelineCache]) -> VkResult {
		unsafe { vkMergePipelineCaches(self, dstCache, srcCaches.len() as _, srcCaches.as_ptr()) }
	}

	pub fn create_graphics_pipelines(self, pipelineCache : PipelineCache, createInfos : &[GraphicsPipelineCreateInfo], pPipelines : *mut Pipeline) -> VkResult {
		unsafe { vkCreateGraphicsPipelines(self, pipelineCache, createInfos.len() as _, createInfos.as_ptr(), null(), pPipelines) }
	}

	pub fn create_compute_pipelines(self, pipelineCache : PipelineCache, createInfos : &[ComputePipelineCreateInfo], pPipelines : *mut Pipeline) -> VkResult {
		unsafe { vkCreateComputePipelines(self, pipelineCache, createInfos.len() as _, createInfos.as_ptr(), null(), pPipelines) }
	}

	pub fn destroy_pipeline(self, pipeline : Pipeline) -> Self {
		unsafe { vkDestroyPipeline(self, pipeline, null()) };
		self
	}

	pub fn create_pipeline_layout(self, pCreateInfo : &PipelineLayoutCreateInfo) -> Result<PipelineLayout, VkResult> {
		let mut pipelineLayout = PipelineLayout(0);
		match unsafe { vkCreatePipelineLayout(self, pCreateInfo, null(), &mut pipelineLayout) } {
			VkResult::SUCCESS => Ok(pipelineLayout),
			err => Err(err)
		}
	}

	pub fn destroy_pipeline_layout(self, pipelineLayout : PipelineLayout) -> Self {
		unsafe { vkDestroyPipelineLayout(self, pipelineLayout, null()) };
		self
	}

	pub fn create_sampler(self, pCreateInfo : &SamplerCreateInfo) -> Result<Sampler, VkResult> {
		let mut sampler = Sampler(0);
		match unsafe { vkCreateSampler(self, pCreateInfo, null(), &mut sampler) } {
			VkResult::SUCCESS => Ok(sampler),
			err => Err(err)
		}
	}

	pub fn destroy_sampler(self, sampler : Sampler) -> Self {
		unsafe { vkDestroySampler(self, sampler, null()) };
		self
	}

	pub fn create_descriptor_set_layout(self, pCreateInfo : &DescriptorSetLayoutCreateInfo) -> Result<DescriptorSetLayout, VkResult> {
		let mut setLayout = DescriptorSetLayout(0);
		match unsafe { vkCreateDescriptorSetLayout(self, pCreateInfo, null(), &mut setLayout) } {
			VkResult::SUCCESS => Ok(setLayout),
			err => Err(err)
		}
	}

	pub fn destroy_descriptor_set_layout(self, descriptorSetLayout : DescriptorSetLayout) -> Self {
		unsafe { vkDestroyDescriptorSetLayout(self, descriptorSetLayout, null()) };
		self
	}

	pub fn create_descriptor_pool(self, pCreateInfo : &DescriptorPoolCreateInfo) -> Result<DescriptorPool, VkResult> {
		let mut descriptorPool = DescriptorPool(0);
		match unsafe { vkCreateDescriptorPool(self, pCreateInfo, null(), &mut descriptorPool) } {
			VkResult::SUCCESS => Ok(descriptorPool),
			err => Err(err)
		}
	}

	pub fn destroy_descriptor_pool(self, descriptorPool : DescriptorPool) -> Self {
		unsafe { vkDestroyDescriptorPool(self, descriptorPool, null()) };
		self
	}

	pub fn reset_descriptor_pool(self, descriptorPool : DescriptorPool, flags : DescriptorPoolResetFlags) -> VkResult {
		unsafe { vkResetDescriptorPool(self, descriptorPool, flags) }
	}

	pub fn allocate_descriptor_sets(self, pAllocateInfo : &DescriptorSetAllocateInfo) -> Result<DescriptorSet, VkResult> {
		let mut descriptorSets = DescriptorSet(0);
		match unsafe { vkAllocateDescriptorSets(self, pAllocateInfo, &mut descriptorSets) } {
			VkResult::SUCCESS => Ok(descriptorSets),
			err => Err(err)
		}
	}

	pub fn free_descriptor_sets(self, descriptorPool : DescriptorPool, descriptorSets : &[DescriptorSet]) -> VkResult {
		unsafe { vkFreeDescriptorSets(self, descriptorPool, descriptorSets.len() as _, descriptorSets.as_ptr()) }
	}

	pub fn update_descriptor_sets(self, descriptorWrites : &[WriteDescriptorSet], descriptorCopies : &[CopyDescriptorSet]) -> Self {
		unsafe { vkUpdateDescriptorSets(self, descriptorWrites.len() as _, descriptorWrites.as_ptr(), descriptorCopies.len() as _, descriptorCopies.as_ptr()) };
		self
	}

	pub fn create_framebuffer(self, pCreateInfo : &FramebufferCreateInfo) -> Result<Framebuffer, VkResult> {
		let mut framebuffer = Framebuffer(0);
		match unsafe { vkCreateFramebuffer(self, pCreateInfo, null(), &mut framebuffer) } {
			VkResult::SUCCESS => Ok(framebuffer),
			err => Err(err)
		}
	}

	pub fn destroy_framebuffer(self, framebuffer : Framebuffer) -> Self {
		unsafe { vkDestroyFramebuffer(self, framebuffer, null()) };
		self
	}

	pub fn create_render_pass(self, pCreateInfo : &RenderPassCreateInfo) -> Result<RenderPass, VkResult> {
		let mut renderPass = RenderPass(0);
		match unsafe { vkCreateRenderPass(self, pCreateInfo, null(), &mut renderPass) } {
			VkResult::SUCCESS => Ok(renderPass),
			err => Err(err)
		}
	}

	pub fn destroy_render_pass(self, renderPass : RenderPass) -> Self {
		unsafe { vkDestroyRenderPass(self, renderPass, null()) };
		self
	}

	pub fn get_render_area_granularity(self, renderPass : RenderPass) -> Extent2D {
		let mut granularity = Extent2D::new();
		unsafe { vkGetRenderAreaGranularity(self, renderPass, &mut granularity) };
		granularity
	}

	pub fn create_command_pool(self, pCreateInfo : &CommandPoolCreateInfo) -> Result<CommandPool, VkResult> {
		let mut commandPool = CommandPool(0);
		match unsafe { vkCreateCommandPool(self, pCreateInfo, null(), &mut commandPool) } {
			VkResult::SUCCESS => Ok(commandPool),
			err => Err(err)
		}
	}

	pub fn destroy_command_pool(self, commandPool : CommandPool) -> Self {
		unsafe { vkDestroyCommandPool(self, commandPool, null()) };
		self
	}

	pub fn reset_command_pool(self, commandPool : CommandPool, flags : CommandPoolResetFlags) -> VkResult {
		unsafe { vkResetCommandPool(self, commandPool, flags) }
	}

	pub fn allocate_command_buffers(self, pAllocateInfo : &CommandBufferAllocateInfo) -> Result<Vec<CommandBuffer>, VkResult> {
		let mut commandBuffers = vec![CommandBuffer(0); pAllocateInfo.command_buffer_count as _];
		match unsafe { vkAllocateCommandBuffers(self, pAllocateInfo, commandBuffers.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(commandBuffers),
			err => Err(err)
		}
	}

	pub fn free_command_buffers(self, commandPool : CommandPool, commandBuffers : &[CommandBuffer]) -> Self {
		unsafe { vkFreeCommandBuffers(self, commandPool, commandBuffers.len() as _, commandBuffers.as_ptr()) };
		self
	}

	pub fn bind_buffer_memory_2(self, bindInfos : &[BindBufferMemoryInfo]) -> VkResult {
		unsafe { vkBindBufferMemory2(self, bindInfos.len() as _, bindInfos.as_ptr()) }
	}

	pub fn bind_image_memory_2(self, bindInfos : &[BindImageMemoryInfo]) -> VkResult {
		unsafe { vkBindImageMemory2(self, bindInfos.len() as _, bindInfos.as_ptr()) }
	}

	pub fn get_group_peer_memory_features(self, heapIndex : u32, localDeviceIndex : u32, remoteDeviceIndex : u32) -> PeerMemoryFeatureFlags {
		let mut peerMemoryFeatures = <_>::default();
		unsafe { vkGetDeviceGroupPeerMemoryFeatures(self, heapIndex, localDeviceIndex, remoteDeviceIndex, &mut peerMemoryFeatures) };
		peerMemoryFeatures
	}

	pub fn get_image_memory_requirements_2(self, pInfo : &ImageMemoryRequirementsInfo2) -> MemoryRequirements2 {
		let mut memoryRequirements = MemoryRequirements2::new();
		unsafe { vkGetImageMemoryRequirements2(self, pInfo, &mut memoryRequirements) };
		memoryRequirements
	}

	pub fn get_buffer_memory_requirements_2(self, pInfo : &BufferMemoryRequirementsInfo2) -> MemoryRequirements2 {
		let mut memoryRequirements = MemoryRequirements2::new();
		unsafe { vkGetBufferMemoryRequirements2(self, pInfo, &mut memoryRequirements) };
		memoryRequirements
	}

	pub fn get_image_sparse_memory_requirements_2(self, pInfo : *const ImageSparseMemoryRequirementsInfo2) -> Vec<SparseImageMemoryRequirements2> {
		let mut count = 0;
		unsafe { vkGetImageSparseMemoryRequirements2(self, pInfo, &mut count, null_mut()) };
		let mut sparseMemoryRequirements = vec![SparseImageMemoryRequirements2::new(); count as _ ];
		unsafe { vkGetImageSparseMemoryRequirements2(self, pInfo, &mut count, sparseMemoryRequirements.as_mut_ptr()) };
		sparseMemoryRequirements
	}

	pub fn trim_command_pool(self, commandPool : CommandPool, flags : CommandPoolTrimFlags) -> Self {
		unsafe { vkTrimCommandPool(self, commandPool, flags) };
		self
	}

	pub fn get_queue_2(self, pQueueInfo : &DeviceQueueInfo2) -> Queue {
		let mut queue = Queue(0);
		unsafe { vkGetDeviceQueue2(self, pQueueInfo, &mut queue) };
		queue
	}

	pub fn create_sampler_ycbcr_conversion(self, pCreateInfo : &SamplerYcbcrConversionCreateInfo) -> Result<SamplerYcbcrConversion, VkResult> {
		let mut ycbcrConversion = SamplerYcbcrConversion(0);
		match unsafe { vkCreateSamplerYcbcrConversion(self, pCreateInfo, null(), &mut ycbcrConversion) } {
			VkResult::SUCCESS => Ok(ycbcrConversion),
			err => Err(err)
		}
	}

	pub fn destroy_sampler_ycbcr_conversion(self, ycbcrConversion : SamplerYcbcrConversion) -> Self {
		unsafe { vkDestroySamplerYcbcrConversion(self, ycbcrConversion, null()) };
		self
	}

	pub fn create_descriptor_update_template(self, pCreateInfo : &DescriptorUpdateTemplateCreateInfo) -> Result<DescriptorUpdateTemplate, VkResult> {
		let mut descriptorUpdateTemplate = DescriptorUpdateTemplate(0);
		match unsafe { vkCreateDescriptorUpdateTemplate(self, pCreateInfo, null(), &mut descriptorUpdateTemplate) } {
			VkResult::SUCCESS => Ok(descriptorUpdateTemplate),
			err => Err(err)
		}
	}

	pub fn destroy_descriptor_update_template(self, descriptorUpdateTemplate : DescriptorUpdateTemplate) -> Self {
		unsafe { vkDestroyDescriptorUpdateTemplate(self, descriptorUpdateTemplate, null()) };
		self
	}

	pub fn update_descriptor_set_with_template(self, descriptorSet : DescriptorSet, descriptorUpdateTemplate : DescriptorUpdateTemplate, pData : *const c_void) -> Self {
		unsafe { vkUpdateDescriptorSetWithTemplate(self, descriptorSet, descriptorUpdateTemplate, pData) };
		self
	}

	pub fn get_descriptor_set_layout_support(self, pCreateInfo : &DescriptorSetLayoutCreateInfo) -> DescriptorSetLayoutSupport {
		let mut support = DescriptorSetLayoutSupport::new();
		unsafe { vkGetDescriptorSetLayoutSupport(self, pCreateInfo, &mut support) };
		support
	}

	pub fn create_render_pass_2(self, pCreateInfo : &RenderPassCreateInfo2) -> Result<RenderPass, VkResult> {
		let mut renderPass = RenderPass(0);
		match unsafe { vkCreateRenderPass2(self, pCreateInfo, null(), &mut renderPass) } {
			VkResult::SUCCESS => Ok(renderPass),
			err => Err(err)
		}
	}

	pub fn reset_query_pool(self, queryPool : QueryPool, firstQuery : u32, queryCount : u32) -> Self {
		unsafe { vkResetQueryPool(self, queryPool, firstQuery, queryCount) };
		self
	}

	pub fn get_semaphore_counter_value(self, semaphore : Semaphore) -> Result<u64, VkResult> {
		let mut value = <_>::default();
		match unsafe { vkGetSemaphoreCounterValue(self, semaphore, &mut value) } {
			VkResult::SUCCESS => Ok(value),
			err => Err(err)
		}
	}

	pub fn wait_semaphores(self, pWaitInfo : &SemaphoreWaitInfo, timeout : u64) -> VkResult {
		unsafe { vkWaitSemaphores(self, pWaitInfo, timeout) }
	}

	pub fn signal_semaphore(self, pSignalInfo : &SemaphoreSignalInfo) -> VkResult {
		unsafe { vkSignalSemaphore(self, pSignalInfo) }
	}

	pub fn get_buffer_address(self, pInfo : &BufferDeviceAddressInfo) -> DeviceAddress {
		unsafe { vkGetBufferDeviceAddress(self, pInfo) }
	}

	pub fn get_buffer_opaque_capture_address(self, pInfo : &BufferDeviceAddressInfo) -> u64 {
		unsafe { vkGetBufferOpaqueCaptureAddress(self, pInfo) }
	}

	pub fn get_memory_opaque_capture_address(self, pInfo : &DeviceMemoryOpaqueCaptureAddressInfo) -> u64 {
		unsafe { vkGetDeviceMemoryOpaqueCaptureAddress(self, pInfo) }
	}

	pub fn create_swapchain_khr(self, pCreateInfo : &SwapchainCreateInfoKHR) -> Result<SwapchainKHR, VkResult> {
		let mut swapchain = SwapchainKHR(0);
		match unsafe { vkCreateSwapchainKHR(self, pCreateInfo, null(), &mut swapchain) } {
			VkResult::SUCCESS => Ok(swapchain),
			err => Err(err)
		}
	}

	pub fn destroy_swapchain_khr(self, swapchain : SwapchainKHR) -> Self {
		unsafe { vkDestroySwapchainKHR(self, swapchain, null()) };
		self
	}

	pub fn get_swapchain_images_khr(self, swapchain : SwapchainKHR) -> Result<Vec<Image>, VkResult> {
		let mut count = 0;
		unsafe { vkGetSwapchainImagesKHR(self, swapchain, &mut count, null_mut()) };
		let mut swapchainImages = vec![Image(0); count as _ ];
		match unsafe { vkGetSwapchainImagesKHR(self, swapchain, &mut count, swapchainImages.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(swapchainImages),
			err => Err(err)
		}
	}

	pub fn acquire_next_image_khr(self, swapchain : SwapchainKHR, timeout : u64, semaphore : Semaphore, fence : Fence) -> Result<u32, VkResult> {
		let mut imageIndex = <_>::default();
		match unsafe { vkAcquireNextImageKHR(self, swapchain, timeout, semaphore, fence, &mut imageIndex) } {
			VkResult::SUCCESS => Ok(imageIndex),
			err => Err(err)
		}
	}

	pub fn get_group_present_capabilities_khr(self) -> Result<DeviceGroupPresentCapabilitiesKHR, VkResult> {
		let mut deviceGroupPresentCapabilities = DeviceGroupPresentCapabilitiesKHR::new();
		match unsafe { vkGetDeviceGroupPresentCapabilitiesKHR(self, &mut deviceGroupPresentCapabilities) } {
			VkResult::SUCCESS => Ok(deviceGroupPresentCapabilities),
			err => Err(err)
		}
	}

	pub fn get_group_surface_present_modes_khr(self, surface : SurfaceKHR) -> Result<DeviceGroupPresentModeFlagsKHR, VkResult> {
		let mut modes = <_>::default();
		match unsafe { vkGetDeviceGroupSurfacePresentModesKHR(self, surface, &mut modes) } {
			VkResult::SUCCESS => Ok(modes),
			err => Err(err)
		}
	}

	pub fn acquire_next_image_2_khr(self, pAcquireInfo : &AcquireNextImageInfoKHR) -> Result<u32, VkResult> {
		let mut imageIndex = <_>::default();
		match unsafe { vkAcquireNextImage2KHR(self, pAcquireInfo, &mut imageIndex) } {
			VkResult::SUCCESS => Ok(imageIndex),
			err => Err(err)
		}
	}

	pub fn create_shared_swapchains_khr(self, createInfos : &[SwapchainCreateInfoKHR], pSwapchains : *mut SwapchainKHR) -> VkResult {
		unsafe { vkCreateSharedSwapchainsKHR(self, createInfos.len() as _, createInfos.as_ptr(), null(), pSwapchains) }
	}

	pub fn get_group_peer_memory_features_khr(self, heapIndex : u32, localDeviceIndex : u32, remoteDeviceIndex : u32) -> PeerMemoryFeatureFlags {
		let mut peerMemoryFeatures = <_>::default();
		unsafe { vkGetDeviceGroupPeerMemoryFeaturesKHR(self, heapIndex, localDeviceIndex, remoteDeviceIndex, &mut peerMemoryFeatures) };
		peerMemoryFeatures
	}

	pub fn trim_command_pool_khr(self, commandPool : CommandPool, flags : CommandPoolTrimFlags) -> Self {
		unsafe { vkTrimCommandPoolKHR(self, commandPool, flags) };
		self
	}

	pub fn get_memory_fd_khr(self, pGetFdInfo : &MemoryGetFdInfoKHR) -> Result<i32, VkResult> {
		let mut fd = <_>::default();
		match unsafe { vkGetMemoryFdKHR(self, pGetFdInfo, &mut fd) } {
			VkResult::SUCCESS => Ok(fd),
			err => Err(err)
		}
	}

	pub fn get_memory_fd_properties_khr(self, handleType : ExternalMemoryHandleTypeFlags, fd : i32) -> Result<MemoryFdPropertiesKHR, VkResult> {
		let mut memoryFdProperties = MemoryFdPropertiesKHR::new();
		match unsafe { vkGetMemoryFdPropertiesKHR(self, handleType, fd, &mut memoryFdProperties) } {
			VkResult::SUCCESS => Ok(memoryFdProperties),
			err => Err(err)
		}
	}

	pub fn import_semaphore_fd_khr(self, pImportSemaphoreFdInfo : &ImportSemaphoreFdInfoKHR) -> VkResult {
		unsafe { vkImportSemaphoreFdKHR(self, pImportSemaphoreFdInfo) }
	}

	pub fn get_semaphore_fd_khr(self, pGetFdInfo : &SemaphoreGetFdInfoKHR) -> Result<i32, VkResult> {
		let mut fd = <_>::default();
		match unsafe { vkGetSemaphoreFdKHR(self, pGetFdInfo, &mut fd) } {
			VkResult::SUCCESS => Ok(fd),
			err => Err(err)
		}
	}

	pub fn create_descriptor_update_template_khr(self, pCreateInfo : &DescriptorUpdateTemplateCreateInfo) -> Result<DescriptorUpdateTemplate, VkResult> {
		let mut descriptorUpdateTemplate = DescriptorUpdateTemplate(0);
		match unsafe { vkCreateDescriptorUpdateTemplateKHR(self, pCreateInfo, null(), &mut descriptorUpdateTemplate) } {
			VkResult::SUCCESS => Ok(descriptorUpdateTemplate),
			err => Err(err)
		}
	}

	pub fn destroy_descriptor_update_template_khr(self, descriptorUpdateTemplate : DescriptorUpdateTemplate) -> Self {
		unsafe { vkDestroyDescriptorUpdateTemplateKHR(self, descriptorUpdateTemplate, null()) };
		self
	}

	pub fn update_descriptor_set_with_template_khr(self, descriptorSet : DescriptorSet, descriptorUpdateTemplate : DescriptorUpdateTemplate, pData : *const c_void) -> Self {
		unsafe { vkUpdateDescriptorSetWithTemplateKHR(self, descriptorSet, descriptorUpdateTemplate, pData) };
		self
	}

	pub fn create_render_pass_2_khr(self, pCreateInfo : &RenderPassCreateInfo2) -> Result<RenderPass, VkResult> {
		let mut renderPass = RenderPass(0);
		match unsafe { vkCreateRenderPass2KHR(self, pCreateInfo, null(), &mut renderPass) } {
			VkResult::SUCCESS => Ok(renderPass),
			err => Err(err)
		}
	}

	pub fn get_swapchain_status_khr(self, swapchain : SwapchainKHR) -> VkResult {
		unsafe { vkGetSwapchainStatusKHR(self, swapchain) }
	}

	pub fn import_fence_fd_khr(self, pImportFenceFdInfo : &ImportFenceFdInfoKHR) -> VkResult {
		unsafe { vkImportFenceFdKHR(self, pImportFenceFdInfo) }
	}

	pub fn get_fence_fd_khr(self, pGetFdInfo : &FenceGetFdInfoKHR) -> Result<i32, VkResult> {
		let mut fd = <_>::default();
		match unsafe { vkGetFenceFdKHR(self, pGetFdInfo, &mut fd) } {
			VkResult::SUCCESS => Ok(fd),
			err => Err(err)
		}
	}

	pub fn acquire_profiling_lock_khr(self, pInfo : &AcquireProfilingLockInfoKHR) -> VkResult {
		unsafe { vkAcquireProfilingLockKHR(self, pInfo) }
	}

	pub fn release_profiling_lock_khr(self) -> Self {
		unsafe { vkReleaseProfilingLockKHR(self) };
		self
	}

	pub fn get_image_memory_requirements_2_khr(self, pInfo : &ImageMemoryRequirementsInfo2) -> MemoryRequirements2 {
		let mut memoryRequirements = MemoryRequirements2::new();
		unsafe { vkGetImageMemoryRequirements2KHR(self, pInfo, &mut memoryRequirements) };
		memoryRequirements
	}

	pub fn get_buffer_memory_requirements_2_khr(self, pInfo : &BufferMemoryRequirementsInfo2) -> MemoryRequirements2 {
		let mut memoryRequirements = MemoryRequirements2::new();
		unsafe { vkGetBufferMemoryRequirements2KHR(self, pInfo, &mut memoryRequirements) };
		memoryRequirements
	}

	pub fn get_image_sparse_memory_requirements_2_khr(self, pInfo : *const ImageSparseMemoryRequirementsInfo2) -> Vec<SparseImageMemoryRequirements2> {
		let mut count = 0;
		unsafe { vkGetImageSparseMemoryRequirements2KHR(self, pInfo, &mut count, null_mut()) };
		let mut sparseMemoryRequirements = vec![SparseImageMemoryRequirements2::new(); count as _ ];
		unsafe { vkGetImageSparseMemoryRequirements2KHR(self, pInfo, &mut count, sparseMemoryRequirements.as_mut_ptr()) };
		sparseMemoryRequirements
	}

	pub fn create_sampler_ycbcr_conversion_khr(self, pCreateInfo : &SamplerYcbcrConversionCreateInfo) -> Result<SamplerYcbcrConversion, VkResult> {
		let mut ycbcrConversion = SamplerYcbcrConversion(0);
		match unsafe { vkCreateSamplerYcbcrConversionKHR(self, pCreateInfo, null(), &mut ycbcrConversion) } {
			VkResult::SUCCESS => Ok(ycbcrConversion),
			err => Err(err)
		}
	}

	pub fn destroy_sampler_ycbcr_conversion_khr(self, ycbcrConversion : SamplerYcbcrConversion) -> Self {
		unsafe { vkDestroySamplerYcbcrConversionKHR(self, ycbcrConversion, null()) };
		self
	}

	pub fn bind_buffer_memory_2_khr(self, bindInfos : &[BindBufferMemoryInfo]) -> VkResult {
		unsafe { vkBindBufferMemory2KHR(self, bindInfos.len() as _, bindInfos.as_ptr()) }
	}

	pub fn bind_image_memory_2_khr(self, bindInfos : &[BindImageMemoryInfo]) -> VkResult {
		unsafe { vkBindImageMemory2KHR(self, bindInfos.len() as _, bindInfos.as_ptr()) }
	}

	pub fn get_descriptor_set_layout_support_khr(self, pCreateInfo : &DescriptorSetLayoutCreateInfo) -> DescriptorSetLayoutSupport {
		let mut support = DescriptorSetLayoutSupport::new();
		unsafe { vkGetDescriptorSetLayoutSupportKHR(self, pCreateInfo, &mut support) };
		support
	}

	pub fn get_semaphore_counter_value_khr(self, semaphore : Semaphore) -> Result<u64, VkResult> {
		let mut value = <_>::default();
		match unsafe { vkGetSemaphoreCounterValueKHR(self, semaphore, &mut value) } {
			VkResult::SUCCESS => Ok(value),
			err => Err(err)
		}
	}

	pub fn wait_semaphores_khr(self, pWaitInfo : &SemaphoreWaitInfo, timeout : u64) -> VkResult {
		unsafe { vkWaitSemaphoresKHR(self, pWaitInfo, timeout) }
	}

	pub fn signal_semaphore_khr(self, pSignalInfo : &SemaphoreSignalInfo) -> VkResult {
		unsafe { vkSignalSemaphoreKHR(self, pSignalInfo) }
	}

	pub fn get_buffer_address_khr(self, pInfo : &BufferDeviceAddressInfo) -> DeviceAddress {
		unsafe { vkGetBufferDeviceAddressKHR(self, pInfo) }
	}

	pub fn get_buffer_opaque_capture_address_khr(self, pInfo : &BufferDeviceAddressInfo) -> u64 {
		unsafe { vkGetBufferOpaqueCaptureAddressKHR(self, pInfo) }
	}

	pub fn get_memory_opaque_capture_address_khr(self, pInfo : &DeviceMemoryOpaqueCaptureAddressInfo) -> u64 {
		unsafe { vkGetDeviceMemoryOpaqueCaptureAddressKHR(self, pInfo) }
	}

	pub fn get_pipeline_executable_properties_khr(self, pPipelineInfo : *const PipelineInfoKHR) -> Result<Vec<PipelineExecutablePropertiesKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPipelineExecutablePropertiesKHR(self, pPipelineInfo, &mut count, null_mut()) };
		let mut properties = vec![PipelineExecutablePropertiesKHR::new(); count as _ ];
		match unsafe { vkGetPipelineExecutablePropertiesKHR(self, pPipelineInfo, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_pipeline_executable_statistics_khr(self, pExecutableInfo : *const PipelineExecutableInfoKHR) -> Result<Vec<PipelineExecutableStatisticKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPipelineExecutableStatisticsKHR(self, pExecutableInfo, &mut count, null_mut()) };
		let mut statistics = vec![PipelineExecutableStatisticKHR::new(); count as _ ];
		match unsafe { vkGetPipelineExecutableStatisticsKHR(self, pExecutableInfo, &mut count, statistics.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(statistics),
			err => Err(err)
		}
	}

	pub fn get_pipeline_executable_internal_representations_khr(self, pExecutableInfo : *const PipelineExecutableInfoKHR) -> Result<Vec<PipelineExecutableInternalRepresentationKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPipelineExecutableInternalRepresentationsKHR(self, pExecutableInfo, &mut count, null_mut()) };
		let mut internalRepresentations = vec![PipelineExecutableInternalRepresentationKHR::new(); count as _ ];
		match unsafe { vkGetPipelineExecutableInternalRepresentationsKHR(self, pExecutableInfo, &mut count, internalRepresentations.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(internalRepresentations),
			err => Err(err)
		}
	}

	pub fn debug_marker_set_object_tag_ext(self, pTagInfo : &DebugMarkerObjectTagInfoEXT) -> VkResult {
		unsafe { vkDebugMarkerSetObjectTagEXT(self, pTagInfo) }
	}

	pub fn debug_marker_set_object_name_ext(self, pNameInfo : &DebugMarkerObjectNameInfoEXT) -> VkResult {
		unsafe { vkDebugMarkerSetObjectNameEXT(self, pNameInfo) }
	}

	pub fn get_image_view_handle_nvx(self, pInfo : &ImageViewHandleInfoNVX) -> u32 {
		unsafe { vkGetImageViewHandleNVX(self, pInfo) }
	}

	pub fn get_image_view_address_nvx(self, imageView : ImageView) -> Result<ImageViewAddressPropertiesNVX, VkResult> {
		let mut properties = ImageViewAddressPropertiesNVX::new();
		match unsafe { vkGetImageViewAddressNVX(self, imageView, &mut properties) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_shader_info_amd(self, pipeline : Pipeline, shaderStage : ShaderStageFlags, infoType : ShaderInfoTypeAMD, pInfoSize : *mut usize, pInfo : *mut c_void) -> VkResult {
		unsafe { vkGetShaderInfoAMD(self, pipeline, shaderStage, infoType, pInfoSize, pInfo) }
	}

	pub fn display_power_control_ext(self, display : DisplayKHR, pDisplayPowerInfo : &DisplayPowerInfoEXT) -> VkResult {
		unsafe { vkDisplayPowerControlEXT(self, display, pDisplayPowerInfo) }
	}

	pub fn register_event_ext(self, pDeviceEventInfo : &DeviceEventInfoEXT) -> Result<Fence, VkResult> {
		let mut fence = Fence(0);
		match unsafe { vkRegisterDeviceEventEXT(self, pDeviceEventInfo, null(), &mut fence) } {
			VkResult::SUCCESS => Ok(fence),
			err => Err(err)
		}
	}

	pub fn register_display_event_ext(self, display : DisplayKHR, pDisplayEventInfo : &DisplayEventInfoEXT) -> Result<Fence, VkResult> {
		let mut fence = Fence(0);
		match unsafe { vkRegisterDisplayEventEXT(self, display, pDisplayEventInfo, null(), &mut fence) } {
			VkResult::SUCCESS => Ok(fence),
			err => Err(err)
		}
	}

	pub fn get_swapchain_counter_ext(self, swapchain : SwapchainKHR, counter : SurfaceCounterFlagsEXT) -> Result<u64, VkResult> {
		let mut counterValue = <_>::default();
		match unsafe { vkGetSwapchainCounterEXT(self, swapchain, counter, &mut counterValue) } {
			VkResult::SUCCESS => Ok(counterValue),
			err => Err(err)
		}
	}

	pub fn get_refresh_cycle_duration_google(self, swapchain : SwapchainKHR) -> Result<RefreshCycleDurationGOOGLE, VkResult> {
		let mut displayTimingProperties = RefreshCycleDurationGOOGLE::new();
		match unsafe { vkGetRefreshCycleDurationGOOGLE(self, swapchain, &mut displayTimingProperties) } {
			VkResult::SUCCESS => Ok(displayTimingProperties),
			err => Err(err)
		}
	}

	pub fn get_past_presentation_timing_google(self, swapchain : SwapchainKHR) -> Result<Vec<PastPresentationTimingGOOGLE>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPastPresentationTimingGOOGLE(self, swapchain, &mut count, null_mut()) };
		let mut presentationTimings = vec![PastPresentationTimingGOOGLE::new(); count as _ ];
		match unsafe { vkGetPastPresentationTimingGOOGLE(self, swapchain, &mut count, presentationTimings.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(presentationTimings),
			err => Err(err)
		}
	}

	pub fn set_hdr_metadata_ext(self, swapchainCount : u32, pSwapchains : &SwapchainKHR, pMetadata : &HdrMetadataEXT) -> Self {
		unsafe { vkSetHdrMetadataEXT(self, swapchainCount, pSwapchains, pMetadata) };
		self
	}

	pub fn set_debug_utils_object_name_ext(self, pNameInfo : &DebugUtilsObjectNameInfoEXT) -> VkResult {
		unsafe { vkSetDebugUtilsObjectNameEXT(self, pNameInfo) }
	}

	pub fn set_debug_utils_object_tag_ext(self, pTagInfo : &DebugUtilsObjectTagInfoEXT) -> VkResult {
		unsafe { vkSetDebugUtilsObjectTagEXT(self, pTagInfo) }
	}

	pub fn get_image_drm_format_modifier_properties_ext(self, image : Image) -> Result<ImageDrmFormatModifierPropertiesEXT, VkResult> {
		let mut properties = ImageDrmFormatModifierPropertiesEXT::new();
		match unsafe { vkGetImageDrmFormatModifierPropertiesEXT(self, image, &mut properties) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn create_validation_cache_ext(self, pCreateInfo : &ValidationCacheCreateInfoEXT) -> Result<ValidationCacheEXT, VkResult> {
		let mut validationCache = ValidationCacheEXT(0);
		match unsafe { vkCreateValidationCacheEXT(self, pCreateInfo, null(), &mut validationCache) } {
			VkResult::SUCCESS => Ok(validationCache),
			err => Err(err)
		}
	}

	pub fn destroy_validation_cache_ext(self, validationCache : ValidationCacheEXT) -> Self {
		unsafe { vkDestroyValidationCacheEXT(self, validationCache, null()) };
		self
	}

	pub fn merge_validation_caches_ext(self, dstCache : ValidationCacheEXT, srcCaches : &[ValidationCacheEXT]) -> VkResult {
		unsafe { vkMergeValidationCachesEXT(self, dstCache, srcCaches.len() as _, srcCaches.as_ptr()) }
	}

	pub fn get_validation_cache_data_ext(self, validationCache : ValidationCacheEXT, pDataSize : *mut usize, pData : *mut c_void) -> VkResult {
		unsafe { vkGetValidationCacheDataEXT(self, validationCache, pDataSize, pData) }
	}

	pub fn create_acceleration_structure_nv(self, pCreateInfo : &AccelerationStructureCreateInfoNV) -> Result<AccelerationStructureNV, VkResult> {
		let mut accelerationStructure = AccelerationStructureKHR(0);
		match unsafe { vkCreateAccelerationStructureNV(self, pCreateInfo, null(), &mut accelerationStructure) } {
			VkResult::SUCCESS => Ok(accelerationStructure),
			err => Err(err)
		}
	}

	pub fn destroy_acceleration_structure_khr(self, accelerationStructure : AccelerationStructureKHR) -> Self {
		unsafe { vkDestroyAccelerationStructureKHR(self, accelerationStructure, null()) };
		self
	}

	pub fn destroy_acceleration_structure_nv(self, accelerationStructure : AccelerationStructureKHR) -> Self {
		unsafe { vkDestroyAccelerationStructureNV(self, accelerationStructure, null()) };
		self
	}

	pub fn get_acceleration_structure_memory_requirements_nv(self, pInfo : &AccelerationStructureMemoryRequirementsInfoNV) -> MemoryRequirements2KHR {
		let mut memoryRequirements = MemoryRequirements2::new();
		unsafe { vkGetAccelerationStructureMemoryRequirementsNV(self, pInfo, &mut memoryRequirements) };
		memoryRequirements
	}

	pub fn bind_acceleration_structure_memory_khr(self, bindInfos : &[BindAccelerationStructureMemoryInfoKHR]) -> VkResult {
		unsafe { vkBindAccelerationStructureMemoryKHR(self, bindInfos.len() as _, bindInfos.as_ptr()) }
	}

	pub fn bind_acceleration_structure_memory_nv(self, bindInfos : &[BindAccelerationStructureMemoryInfoKHR]) -> VkResult {
		unsafe { vkBindAccelerationStructureMemoryNV(self, bindInfos.len() as _, bindInfos.as_ptr()) }
	}

	pub fn create_ray_tracing_pipelines_nv(self, pipelineCache : PipelineCache, createInfos : &[RayTracingPipelineCreateInfoNV], pPipelines : *mut Pipeline) -> VkResult {
		unsafe { vkCreateRayTracingPipelinesNV(self, pipelineCache, createInfos.len() as _, createInfos.as_ptr(), null(), pPipelines) }
	}

	pub fn get_ray_tracing_shader_group_handles_khr(self, pipeline : Pipeline, firstGroup : u32, groupCount : u32, dataSize : usize, pData : *mut c_void) -> VkResult {
		unsafe { vkGetRayTracingShaderGroupHandlesKHR(self, pipeline, firstGroup, groupCount, dataSize, pData) }
	}

	pub fn get_ray_tracing_shader_group_handles_nv(self, pipeline : Pipeline, firstGroup : u32, groupCount : u32, dataSize : usize, pData : *mut c_void) -> VkResult {
		unsafe { vkGetRayTracingShaderGroupHandlesNV(self, pipeline, firstGroup, groupCount, dataSize, pData) }
	}

	pub fn get_acceleration_structure_handle_nv(self, accelerationStructure : AccelerationStructureKHR, dataSize : usize, pData : *mut c_void) -> VkResult {
		unsafe { vkGetAccelerationStructureHandleNV(self, accelerationStructure, dataSize, pData) }
	}

	pub fn compile_deferred_nv(self, pipeline : Pipeline, shader : u32) -> VkResult {
		unsafe { vkCompileDeferredNV(self, pipeline, shader) }
	}

	pub fn get_memory_host_pointer_properties_ext(self, handleType : ExternalMemoryHandleTypeFlags, pHostPointer : *const c_void) -> Result<MemoryHostPointerPropertiesEXT, VkResult> {
		let mut memoryHostPointerProperties = MemoryHostPointerPropertiesEXT::new();
		match unsafe { vkGetMemoryHostPointerPropertiesEXT(self, handleType, pHostPointer, &mut memoryHostPointerProperties) } {
			VkResult::SUCCESS => Ok(memoryHostPointerProperties),
			err => Err(err)
		}
	}

	pub fn get_calibrated_timestamps_ext(self, timestampInfos : &[CalibratedTimestampInfoEXT], pTimestamps : *mut u64, pMaxDeviation : *mut u64) -> VkResult {
		unsafe { vkGetCalibratedTimestampsEXT(self, timestampInfos.len() as _, timestampInfos.as_ptr(), pTimestamps, pMaxDeviation) }
	}

	pub fn initialize_performance_api_intel(self, pInitializeInfo : &InitializePerformanceApiInfoINTEL) -> VkResult {
		unsafe { vkInitializePerformanceApiINTEL(self, pInitializeInfo) }
	}

	pub fn uninitialize_performance_api_intel(self) -> Self {
		unsafe { vkUninitializePerformanceApiINTEL(self) };
		self
	}

	pub fn acquire_performance_configuration_intel(self, pAcquireInfo : &PerformanceConfigurationAcquireInfoINTEL) -> Result<PerformanceConfigurationINTEL, VkResult> {
		let mut configuration = PerformanceConfigurationINTEL(0);
		match unsafe { vkAcquirePerformanceConfigurationINTEL(self, pAcquireInfo, &mut configuration) } {
			VkResult::SUCCESS => Ok(configuration),
			err => Err(err)
		}
	}

	pub fn release_performance_configuration_intel(self, configuration : PerformanceConfigurationINTEL) -> VkResult {
		unsafe { vkReleasePerformanceConfigurationINTEL(self, configuration) }
	}

	pub fn get_performance_parameter_intel(self, parameter : PerformanceParameterTypeINTEL) -> Result<PerformanceValueINTEL, VkResult> {
		let mut value = PerformanceValueINTEL::new();
		match unsafe { vkGetPerformanceParameterINTEL(self, parameter, &mut value) } {
			VkResult::SUCCESS => Ok(value),
			err => Err(err)
		}
	}

	pub fn set_local_dimming_amd(self, swapChain : SwapchainKHR, localDimmingEnable : Bool32) -> Self {
		unsafe { vkSetLocalDimmingAMD(self, swapChain, localDimmingEnable) };
		self
	}

	pub fn get_buffer_address_ext(self, pInfo : &BufferDeviceAddressInfo) -> DeviceAddress {
		unsafe { vkGetBufferDeviceAddressEXT(self, pInfo) }
	}

	pub fn reset_query_pool_ext(self, queryPool : QueryPool, firstQuery : u32, queryCount : u32) -> Self {
		unsafe { vkResetQueryPoolEXT(self, queryPool, firstQuery, queryCount) };
		self
	}

	pub fn get_generated_commands_memory_requirements_nv(self, pInfo : &GeneratedCommandsMemoryRequirementsInfoNV) -> MemoryRequirements2 {
		let mut memoryRequirements = MemoryRequirements2::new();
		unsafe { vkGetGeneratedCommandsMemoryRequirementsNV(self, pInfo, &mut memoryRequirements) };
		memoryRequirements
	}

	pub fn create_indirect_commands_layout_nv(self, pCreateInfo : &IndirectCommandsLayoutCreateInfoNV) -> Result<IndirectCommandsLayoutNV, VkResult> {
		let mut indirectCommandsLayout = IndirectCommandsLayoutNV(0);
		match unsafe { vkCreateIndirectCommandsLayoutNV(self, pCreateInfo, null(), &mut indirectCommandsLayout) } {
			VkResult::SUCCESS => Ok(indirectCommandsLayout),
			err => Err(err)
		}
	}

	pub fn destroy_indirect_commands_layout_nv(self, indirectCommandsLayout : IndirectCommandsLayoutNV) -> Self {
		unsafe { vkDestroyIndirectCommandsLayoutNV(self, indirectCommandsLayout, null()) };
		self
	}

	pub fn create_private_data_slot_ext(self, pCreateInfo : &PrivateDataSlotCreateInfoEXT) -> Result<PrivateDataSlotEXT, VkResult> {
		let mut privateDataSlot = PrivateDataSlotEXT(0);
		match unsafe { vkCreatePrivateDataSlotEXT(self, pCreateInfo, null(), &mut privateDataSlot) } {
			VkResult::SUCCESS => Ok(privateDataSlot),
			err => Err(err)
		}
	}

	pub fn destroy_private_data_slot_ext(self, privateDataSlot : PrivateDataSlotEXT) -> Self {
		unsafe { vkDestroyPrivateDataSlotEXT(self, privateDataSlot, null()) };
		self
	}

	pub fn set_private_data_ext(self, objectType : ObjectType, objectHandle : u64, privateDataSlot : PrivateDataSlotEXT, data : u64) -> VkResult {
		unsafe { vkSetPrivateDataEXT(self, objectType, objectHandle, privateDataSlot, data) }
	}

	pub fn get_private_data_ext(self, objectType : ObjectType, objectHandle : u64, privateDataSlot : PrivateDataSlotEXT) -> u64 {
		let mut data = <_>::default();
		unsafe { vkGetPrivateDataEXT(self, objectType, objectHandle, privateDataSlot, &mut data) };
		data
	}

	pub fn get_android_hardware_buffer_properties_android(self, buffer : &AHardwareBuffer, pProperties : *mut AndroidHardwareBufferPropertiesANDROID) -> VkResult {
		unsafe { vkGetAndroidHardwareBufferPropertiesANDROID(self, buffer, pProperties) }
	}

	pub fn get_memory_android_hardware_buffer_android(self, pInfo : &MemoryGetAndroidHardwareBufferInfoANDROID, pBuffer : *mut *mut AHardwareBuffer) -> VkResult {
		unsafe { vkGetMemoryAndroidHardwareBufferANDROID(self, pInfo, pBuffer) }
	}

	pub fn get_memory_win_32_handle_khr(self, pGetWin32HandleInfo : &MemoryGetWin32HandleInfoKHR) -> Result<HANDLE, VkResult> {
		let mut handle = <_>::default();
		match unsafe { vkGetMemoryWin32HandleKHR(self, pGetWin32HandleInfo, &mut handle) } {
			VkResult::SUCCESS => Ok(handle),
			err => Err(err)
		}
	}

	pub fn get_memory_win_32_handle_properties_khr(self, handleType : ExternalMemoryHandleTypeFlags, handle : HANDLE) -> Result<MemoryWin32HandlePropertiesKHR, VkResult> {
		let mut memoryWin32HandleProperties = MemoryWin32HandlePropertiesKHR::new();
		match unsafe { vkGetMemoryWin32HandlePropertiesKHR(self, handleType, handle, &mut memoryWin32HandleProperties) } {
			VkResult::SUCCESS => Ok(memoryWin32HandleProperties),
			err => Err(err)
		}
	}

	pub fn import_semaphore_win_32_handle_khr(self, pImportSemaphoreWin32HandleInfo : &ImportSemaphoreWin32HandleInfoKHR) -> VkResult {
		unsafe { vkImportSemaphoreWin32HandleKHR(self, pImportSemaphoreWin32HandleInfo) }
	}

	pub fn get_semaphore_win_32_handle_khr(self, pGetWin32HandleInfo : &SemaphoreGetWin32HandleInfoKHR) -> Result<HANDLE, VkResult> {
		let mut handle = <_>::default();
		match unsafe { vkGetSemaphoreWin32HandleKHR(self, pGetWin32HandleInfo, &mut handle) } {
			VkResult::SUCCESS => Ok(handle),
			err => Err(err)
		}
	}

	pub fn import_fence_win_32_handle_khr(self, pImportFenceWin32HandleInfo : &ImportFenceWin32HandleInfoKHR) -> VkResult {
		unsafe { vkImportFenceWin32HandleKHR(self, pImportFenceWin32HandleInfo) }
	}

	pub fn get_fence_win_32_handle_khr(self, pGetWin32HandleInfo : &FenceGetWin32HandleInfoKHR) -> Result<HANDLE, VkResult> {
		let mut handle = <_>::default();
		match unsafe { vkGetFenceWin32HandleKHR(self, pGetWin32HandleInfo, &mut handle) } {
			VkResult::SUCCESS => Ok(handle),
			err => Err(err)
		}
	}

	pub fn get_memory_win_32_handle_nv(self, memory : DeviceMemory, handleType : ExternalMemoryHandleTypeFlagsNV) -> Result<HANDLE, VkResult> {
		let mut handle = <_>::default();
		match unsafe { vkGetMemoryWin32HandleNV(self, memory, handleType, &mut handle) } {
			VkResult::SUCCESS => Ok(handle),
			err => Err(err)
		}
	}

	pub fn acquire_full_screen_exclusive_mode_ext(self, swapchain : SwapchainKHR) -> VkResult {
		unsafe { vkAcquireFullScreenExclusiveModeEXT(self, swapchain) }
	}

	pub fn release_full_screen_exclusive_mode_ext(self, swapchain : SwapchainKHR) -> VkResult {
		unsafe { vkReleaseFullScreenExclusiveModeEXT(self, swapchain) }
	}

	pub fn get_group_surface_present_modes_2_ext(self, pSurfaceInfo : &PhysicalDeviceSurfaceInfo2KHR) -> Result<DeviceGroupPresentModeFlagsKHR, VkResult> {
		let mut modes = <_>::default();
		match unsafe { vkGetDeviceGroupSurfacePresentModes2EXT(self, pSurfaceInfo, &mut modes) } {
			VkResult::SUCCESS => Ok(modes),
			err => Err(err)
		}
	}

}

impl PhysicalDevice {

	pub fn get_features(self) -> PhysicalDeviceFeatures {
		let mut features = PhysicalDeviceFeatures::new();
		unsafe { vkGetPhysicalDeviceFeatures(self, &mut features) };
		features
	}

	pub fn get_format_properties(self, format : Format) -> FormatProperties {
		let mut formatProperties = FormatProperties::new();
		unsafe { vkGetPhysicalDeviceFormatProperties(self, format, &mut formatProperties) };
		formatProperties
	}

	pub fn get_image_format_properties(self, format : Format, r#type : ImageType, tiling : ImageTiling, usage : ImageUsageFlags, flags : ImageCreateFlags) -> Result<ImageFormatProperties, VkResult> {
		let mut imageFormatProperties = ImageFormatProperties::new();
		match unsafe { vkGetPhysicalDeviceImageFormatProperties(self, format, r#type, tiling, usage, flags, &mut imageFormatProperties) } {
			VkResult::SUCCESS => Ok(imageFormatProperties),
			err => Err(err)
		}
	}

	pub fn get_properties(self) -> PhysicalDeviceProperties {
		let mut properties = PhysicalDeviceProperties::new();
		unsafe { vkGetPhysicalDeviceProperties(self, &mut properties) };
		properties
	}

	pub fn get_queue_family_properties(self) -> Vec<QueueFamilyProperties> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(self, &mut count, null_mut()) };
		let mut queueFamilyProperties = vec![QueueFamilyProperties::new(); count as _ ];
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(self, &mut count, queueFamilyProperties.as_mut_ptr()) };
		queueFamilyProperties
	}

	pub fn get_memory_properties(self) -> PhysicalDeviceMemoryProperties {
		let mut memoryProperties = PhysicalDeviceMemoryProperties::new();
		unsafe { vkGetPhysicalDeviceMemoryProperties(self, &mut memoryProperties) };
		memoryProperties
	}

	pub fn enumerate_extension_properties(self, pLayerName : *const u8) -> Result<Vec<ExtensionProperties>, VkResult> {
		let mut count = 0;
		unsafe { vkEnumerateDeviceExtensionProperties(self, pLayerName, &mut count, null_mut()) };
		let mut properties = vec![ExtensionProperties::new(); count as _ ];
		match unsafe { vkEnumerateDeviceExtensionProperties(self, pLayerName, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn enumerate_layer_properties(self) -> Result<Vec<LayerProperties>, VkResult> {
		let mut count = 0;
		unsafe { vkEnumerateDeviceLayerProperties(self, &mut count, null_mut()) };
		let mut properties = vec![LayerProperties::new(); count as _ ];
		match unsafe { vkEnumerateDeviceLayerProperties(self, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_sparse_image_format_properties(self, format : Format, r#type : ImageType, samples : SampleCountFlags, usage : ImageUsageFlags, tiling : ImageTiling) -> Vec<SparseImageFormatProperties> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceSparseImageFormatProperties(self, format, r#type, samples, usage, tiling, &mut count, null_mut()) };
		let mut properties = vec![SparseImageFormatProperties::new(); count as _ ];
		unsafe { vkGetPhysicalDeviceSparseImageFormatProperties(self, format, r#type, samples, usage, tiling, &mut count, properties.as_mut_ptr()) };
		properties
	}

	pub fn get_features_2(self) -> PhysicalDeviceFeatures2 {
		let mut features = PhysicalDeviceFeatures2::new();
		unsafe { vkGetPhysicalDeviceFeatures2(self, &mut features) };
		features
	}

	pub fn get_properties_2(self) -> PhysicalDeviceProperties2 {
		let mut properties = PhysicalDeviceProperties2::new();
		unsafe { vkGetPhysicalDeviceProperties2(self, &mut properties) };
		properties
	}

	pub fn get_format_properties_2(self, format : Format) -> FormatProperties2 {
		let mut formatProperties = FormatProperties2::new();
		unsafe { vkGetPhysicalDeviceFormatProperties2(self, format, &mut formatProperties) };
		formatProperties
	}

	pub fn get_image_format_properties_2(self, pImageFormatInfo : &PhysicalDeviceImageFormatInfo2) -> Result<ImageFormatProperties2, VkResult> {
		let mut imageFormatProperties = ImageFormatProperties2::new();
		match unsafe { vkGetPhysicalDeviceImageFormatProperties2(self, pImageFormatInfo, &mut imageFormatProperties) } {
			VkResult::SUCCESS => Ok(imageFormatProperties),
			err => Err(err)
		}
	}

	pub fn get_queue_family_properties_2(self) -> Vec<QueueFamilyProperties2> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties2(self, &mut count, null_mut()) };
		let mut queueFamilyProperties = vec![QueueFamilyProperties2::new(); count as _ ];
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties2(self, &mut count, queueFamilyProperties.as_mut_ptr()) };
		queueFamilyProperties
	}

	pub fn get_memory_properties_2(self) -> PhysicalDeviceMemoryProperties2 {
		let mut memoryProperties = PhysicalDeviceMemoryProperties2::new();
		unsafe { vkGetPhysicalDeviceMemoryProperties2(self, &mut memoryProperties) };
		memoryProperties
	}

	pub fn get_sparse_image_format_properties_2(self, pFormatInfo : *const PhysicalDeviceSparseImageFormatInfo2) -> Vec<SparseImageFormatProperties2> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceSparseImageFormatProperties2(self, pFormatInfo, &mut count, null_mut()) };
		let mut properties = vec![SparseImageFormatProperties2::new(); count as _ ];
		unsafe { vkGetPhysicalDeviceSparseImageFormatProperties2(self, pFormatInfo, &mut count, properties.as_mut_ptr()) };
		properties
	}

	pub fn get_external_buffer_properties(self, pExternalBufferInfo : &PhysicalDeviceExternalBufferInfo) -> ExternalBufferProperties {
		let mut externalBufferProperties = ExternalBufferProperties::new();
		unsafe { vkGetPhysicalDeviceExternalBufferProperties(self, pExternalBufferInfo, &mut externalBufferProperties) };
		externalBufferProperties
	}

	pub fn get_external_fence_properties(self, pExternalFenceInfo : &PhysicalDeviceExternalFenceInfo) -> ExternalFenceProperties {
		let mut externalFenceProperties = ExternalFenceProperties::new();
		unsafe { vkGetPhysicalDeviceExternalFenceProperties(self, pExternalFenceInfo, &mut externalFenceProperties) };
		externalFenceProperties
	}

	pub fn get_external_semaphore_properties(self, pExternalSemaphoreInfo : &PhysicalDeviceExternalSemaphoreInfo) -> ExternalSemaphoreProperties {
		let mut externalSemaphoreProperties = ExternalSemaphoreProperties::new();
		unsafe { vkGetPhysicalDeviceExternalSemaphoreProperties(self, pExternalSemaphoreInfo, &mut externalSemaphoreProperties) };
		externalSemaphoreProperties
	}

	pub fn get_surface_support_khr(self, queueFamilyIndex : u32, surface : SurfaceKHR) -> Result<Bool32, VkResult> {
		let mut supported = <_>::default();
		match unsafe { vkGetPhysicalDeviceSurfaceSupportKHR(self, queueFamilyIndex, surface, &mut supported) } {
			VkResult::SUCCESS => Ok(supported),
			err => Err(err)
		}
	}

	pub fn get_surface_capabilities_khr(self, surface : SurfaceKHR) -> Result<SurfaceCapabilitiesKHR, VkResult> {
		let mut surfaceCapabilities = SurfaceCapabilitiesKHR::new();
		match unsafe { vkGetPhysicalDeviceSurfaceCapabilitiesKHR(self, surface, &mut surfaceCapabilities) } {
			VkResult::SUCCESS => Ok(surfaceCapabilities),
			err => Err(err)
		}
	}

	pub fn get_surface_formats_khr(self, surface : SurfaceKHR) -> Result<Vec<SurfaceFormatKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(self, surface, &mut count, null_mut()) };
		let mut surfaceFormats = vec![SurfaceFormatKHR::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(self, surface, &mut count, surfaceFormats.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(surfaceFormats),
			err => Err(err)
		}
	}

	pub fn get_surface_present_modes_khr(self, surface : SurfaceKHR) -> Result<Vec<PresentModeKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(self, surface, &mut count, null_mut()) };
		let mut presentModes = vec![PresentModeKHR::default(); count as _ ];
		match unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(self, surface, &mut count, presentModes.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(presentModes),
			err => Err(err)
		}
	}

	pub fn get_present_rectangles_khr(self, surface : SurfaceKHR) -> Result<Vec<Rect2D>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDevicePresentRectanglesKHR(self, surface, &mut count, null_mut()) };
		let mut rects = vec![Rect2D::new(); count as _ ];
		match unsafe { vkGetPhysicalDevicePresentRectanglesKHR(self, surface, &mut count, rects.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(rects),
			err => Err(err)
		}
	}

	pub fn get_display_properties_khr(self) -> Result<Vec<DisplayPropertiesKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceDisplayPropertiesKHR(self, &mut count, null_mut()) };
		let mut properties = vec![DisplayPropertiesKHR::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceDisplayPropertiesKHR(self, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_display_plane_properties_khr(self) -> Result<Vec<DisplayPlanePropertiesKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceDisplayPlanePropertiesKHR(self, &mut count, null_mut()) };
		let mut properties = vec![DisplayPlanePropertiesKHR::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceDisplayPlanePropertiesKHR(self, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_display_plane_supported_displays_khr(self, planeIndex : u32) -> Result<Vec<DisplayKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetDisplayPlaneSupportedDisplaysKHR(self, planeIndex, &mut count, null_mut()) };
		let mut displays = vec![DisplayKHR(0); count as _ ];
		match unsafe { vkGetDisplayPlaneSupportedDisplaysKHR(self, planeIndex, &mut count, displays.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(displays),
			err => Err(err)
		}
	}

	pub fn get_display_mode_properties_khr(self, display : DisplayKHR) -> Result<Vec<DisplayModePropertiesKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetDisplayModePropertiesKHR(self, display, &mut count, null_mut()) };
		let mut properties = vec![DisplayModePropertiesKHR::new(); count as _ ];
		match unsafe { vkGetDisplayModePropertiesKHR(self, display, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn create_display_mode_khr(self, display : DisplayKHR, pCreateInfo : &DisplayModeCreateInfoKHR) -> Result<DisplayModeKHR, VkResult> {
		let mut mode = DisplayModeKHR(0);
		match unsafe { vkCreateDisplayModeKHR(self, display, pCreateInfo, null(), &mut mode) } {
			VkResult::SUCCESS => Ok(mode),
			err => Err(err)
		}
	}

	pub fn get_display_plane_capabilities_khr(self, mode : DisplayModeKHR, planeIndex : u32) -> Result<DisplayPlaneCapabilitiesKHR, VkResult> {
		let mut capabilities = DisplayPlaneCapabilitiesKHR::new();
		match unsafe { vkGetDisplayPlaneCapabilitiesKHR(self, mode, planeIndex, &mut capabilities) } {
			VkResult::SUCCESS => Ok(capabilities),
			err => Err(err)
		}
	}

	pub fn get_features_2_khr(self) -> PhysicalDeviceFeatures2 {
		let mut features = PhysicalDeviceFeatures2::new();
		unsafe { vkGetPhysicalDeviceFeatures2KHR(self, &mut features) };
		features
	}

	pub fn get_properties_2_khr(self) -> PhysicalDeviceProperties2 {
		let mut properties = PhysicalDeviceProperties2::new();
		unsafe { vkGetPhysicalDeviceProperties2KHR(self, &mut properties) };
		properties
	}

	pub fn get_format_properties_2_khr(self, format : Format) -> FormatProperties2 {
		let mut formatProperties = FormatProperties2::new();
		unsafe { vkGetPhysicalDeviceFormatProperties2KHR(self, format, &mut formatProperties) };
		formatProperties
	}

	pub fn get_image_format_properties_2_khr(self, pImageFormatInfo : &PhysicalDeviceImageFormatInfo2) -> Result<ImageFormatProperties2, VkResult> {
		let mut imageFormatProperties = ImageFormatProperties2::new();
		match unsafe { vkGetPhysicalDeviceImageFormatProperties2KHR(self, pImageFormatInfo, &mut imageFormatProperties) } {
			VkResult::SUCCESS => Ok(imageFormatProperties),
			err => Err(err)
		}
	}

	pub fn get_queue_family_properties_2_khr(self) -> Vec<QueueFamilyProperties2> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties2KHR(self, &mut count, null_mut()) };
		let mut queueFamilyProperties = vec![QueueFamilyProperties2::new(); count as _ ];
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties2KHR(self, &mut count, queueFamilyProperties.as_mut_ptr()) };
		queueFamilyProperties
	}

	pub fn get_memory_properties_2_khr(self) -> PhysicalDeviceMemoryProperties2 {
		let mut memoryProperties = PhysicalDeviceMemoryProperties2::new();
		unsafe { vkGetPhysicalDeviceMemoryProperties2KHR(self, &mut memoryProperties) };
		memoryProperties
	}

	pub fn get_sparse_image_format_properties_2_khr(self, pFormatInfo : *const PhysicalDeviceSparseImageFormatInfo2) -> Vec<SparseImageFormatProperties2> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceSparseImageFormatProperties2KHR(self, pFormatInfo, &mut count, null_mut()) };
		let mut properties = vec![SparseImageFormatProperties2::new(); count as _ ];
		unsafe { vkGetPhysicalDeviceSparseImageFormatProperties2KHR(self, pFormatInfo, &mut count, properties.as_mut_ptr()) };
		properties
	}

	pub fn get_external_buffer_properties_khr(self, pExternalBufferInfo : &PhysicalDeviceExternalBufferInfo) -> ExternalBufferProperties {
		let mut externalBufferProperties = ExternalBufferProperties::new();
		unsafe { vkGetPhysicalDeviceExternalBufferPropertiesKHR(self, pExternalBufferInfo, &mut externalBufferProperties) };
		externalBufferProperties
	}

	pub fn get_external_semaphore_properties_khr(self, pExternalSemaphoreInfo : &PhysicalDeviceExternalSemaphoreInfo) -> ExternalSemaphoreProperties {
		let mut externalSemaphoreProperties = ExternalSemaphoreProperties::new();
		unsafe { vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(self, pExternalSemaphoreInfo, &mut externalSemaphoreProperties) };
		externalSemaphoreProperties
	}

	pub fn get_external_fence_properties_khr(self, pExternalFenceInfo : &PhysicalDeviceExternalFenceInfo) -> ExternalFenceProperties {
		let mut externalFenceProperties = ExternalFenceProperties::new();
		unsafe { vkGetPhysicalDeviceExternalFencePropertiesKHR(self, pExternalFenceInfo, &mut externalFenceProperties) };
		externalFenceProperties
	}

	pub fn enumerate_queue_family_performance_query_counters_khr(self, queueFamilyIndex : u32, pCounterCount : *mut u32, pCounters : *mut PerformanceCounterKHR, pCounterDescriptions : *mut PerformanceCounterDescriptionKHR) -> VkResult {
		unsafe { vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(self, queueFamilyIndex, pCounterCount, pCounters, pCounterDescriptions) }
	}

	pub fn get_queue_family_performance_query_passes_khr(self, pPerformanceQueryCreateInfo : &QueryPoolPerformanceCreateInfoKHR) -> u32 {
		let mut numPasses = <_>::default();
		unsafe { vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(self, pPerformanceQueryCreateInfo, &mut numPasses) };
		numPasses
	}

	pub fn get_surface_capabilities_2_khr(self, pSurfaceInfo : &PhysicalDeviceSurfaceInfo2KHR) -> Result<SurfaceCapabilities2KHR, VkResult> {
		let mut surfaceCapabilities = SurfaceCapabilities2KHR::new();
		match unsafe { vkGetPhysicalDeviceSurfaceCapabilities2KHR(self, pSurfaceInfo, &mut surfaceCapabilities) } {
			VkResult::SUCCESS => Ok(surfaceCapabilities),
			err => Err(err)
		}
	}

	pub fn get_surface_formats_2_khr(self, pSurfaceInfo : *const PhysicalDeviceSurfaceInfo2KHR) -> Result<Vec<SurfaceFormat2KHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceSurfaceFormats2KHR(self, pSurfaceInfo, &mut count, null_mut()) };
		let mut surfaceFormats = vec![SurfaceFormat2KHR::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceSurfaceFormats2KHR(self, pSurfaceInfo, &mut count, surfaceFormats.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(surfaceFormats),
			err => Err(err)
		}
	}

	pub fn get_display_properties_2_khr(self) -> Result<Vec<DisplayProperties2KHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceDisplayProperties2KHR(self, &mut count, null_mut()) };
		let mut properties = vec![DisplayProperties2KHR::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceDisplayProperties2KHR(self, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_display_plane_properties_2_khr(self) -> Result<Vec<DisplayPlaneProperties2KHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceDisplayPlaneProperties2KHR(self, &mut count, null_mut()) };
		let mut properties = vec![DisplayPlaneProperties2KHR::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceDisplayPlaneProperties2KHR(self, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_display_mode_properties_2_khr(self, display : DisplayKHR) -> Result<Vec<DisplayModeProperties2KHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetDisplayModeProperties2KHR(self, display, &mut count, null_mut()) };
		let mut properties = vec![DisplayModeProperties2KHR::new(); count as _ ];
		match unsafe { vkGetDisplayModeProperties2KHR(self, display, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_display_plane_capabilities_2_khr(self, pDisplayPlaneInfo : &DisplayPlaneInfo2KHR) -> Result<DisplayPlaneCapabilities2KHR, VkResult> {
		let mut capabilities = DisplayPlaneCapabilities2KHR::new();
		match unsafe { vkGetDisplayPlaneCapabilities2KHR(self, pDisplayPlaneInfo, &mut capabilities) } {
			VkResult::SUCCESS => Ok(capabilities),
			err => Err(err)
		}
	}

	pub fn get_external_image_format_properties_nv(self, format : Format, r#type : ImageType, tiling : ImageTiling, usage : ImageUsageFlags, flags : ImageCreateFlags, externalHandleType : ExternalMemoryHandleTypeFlagsNV) -> Result<ExternalImageFormatPropertiesNV, VkResult> {
		let mut externalImageFormatProperties = ExternalImageFormatPropertiesNV::new();
		match unsafe { vkGetPhysicalDeviceExternalImageFormatPropertiesNV(self, format, r#type, tiling, usage, flags, externalHandleType, &mut externalImageFormatProperties) } {
			VkResult::SUCCESS => Ok(externalImageFormatProperties),
			err => Err(err)
		}
	}

	pub fn release_display_ext(self, display : DisplayKHR) -> VkResult {
		unsafe { vkReleaseDisplayEXT(self, display) }
	}

	pub fn get_surface_capabilities_2_ext(self, surface : SurfaceKHR) -> Result<SurfaceCapabilities2EXT, VkResult> {
		let mut surfaceCapabilities = SurfaceCapabilities2EXT::new();
		match unsafe { vkGetPhysicalDeviceSurfaceCapabilities2EXT(self, surface, &mut surfaceCapabilities) } {
			VkResult::SUCCESS => Ok(surfaceCapabilities),
			err => Err(err)
		}
	}

	pub fn get_multisample_properties_ext(self, samples : SampleCountFlags) -> MultisamplePropertiesEXT {
		let mut multisampleProperties = MultisamplePropertiesEXT::new();
		unsafe { vkGetPhysicalDeviceMultisamplePropertiesEXT(self, samples, &mut multisampleProperties) };
		multisampleProperties
	}

	pub fn get_calibrateable_time_domains_ext(self) -> Result<Vec<TimeDomainEXT>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(self, &mut count, null_mut()) };
		let mut timeDomains = vec![TimeDomainEXT::default(); count as _ ];
		match unsafe { vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(self, &mut count, timeDomains.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(timeDomains),
			err => Err(err)
		}
	}

	pub fn get_tool_properties_ext(self) -> Result<Vec<PhysicalDeviceToolPropertiesEXT>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceToolPropertiesEXT(self, &mut count, null_mut()) };
		let mut toolProperties = vec![PhysicalDeviceToolPropertiesEXT::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceToolPropertiesEXT(self, &mut count, toolProperties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(toolProperties),
			err => Err(err)
		}
	}

	pub fn get_cooperative_matrix_properties_nv(self) -> Result<Vec<CooperativeMatrixPropertiesNV>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(self, &mut count, null_mut()) };
		let mut properties = vec![CooperativeMatrixPropertiesNV::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(self, &mut count, properties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(properties),
			err => Err(err)
		}
	}

	pub fn get_supported_framebuffer_mixed_samples_combinations_nv(self) -> Result<Vec<FramebufferMixedSamplesCombinationNV>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(self, &mut count, null_mut()) };
		let mut combinations = vec![FramebufferMixedSamplesCombinationNV::new(); count as _ ];
		match unsafe { vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(self, &mut count, combinations.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(combinations),
			err => Err(err)
		}
	}

	pub fn get_win_32_presentation_support_khr(self, queueFamilyIndex : u32) -> Bool32 {
		unsafe { vkGetPhysicalDeviceWin32PresentationSupportKHR(self, queueFamilyIndex) }
	}

	pub fn get_surface_present_modes_2_ext(self, pSurfaceInfo : *const PhysicalDeviceSurfaceInfo2KHR) -> Result<Vec<PresentModeKHR>, VkResult> {
		let mut count = 0;
		unsafe { vkGetPhysicalDeviceSurfacePresentModes2EXT(self, pSurfaceInfo, &mut count, null_mut()) };
		let mut presentModes = vec![PresentModeKHR::default(); count as _ ];
		match unsafe { vkGetPhysicalDeviceSurfacePresentModes2EXT(self, pSurfaceInfo, &mut count, presentModes.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(presentModes),
			err => Err(err)
		}
	}

	pub fn get_wayland_presentation_support_khr(self, queueFamilyIndex : u32, display : *mut wl_display) -> Bool32 {
		unsafe { vkGetPhysicalDeviceWaylandPresentationSupportKHR(self, queueFamilyIndex, display) }
	}

	pub fn get_xcb_presentation_support_khr(self, queueFamilyIndex : u32, connection : *mut xcb_connection_t, visual_id : xcb_visualid_t) -> Bool32 {
		unsafe { vkGetPhysicalDeviceXcbPresentationSupportKHR(self, queueFamilyIndex, connection, visual_id) }
	}

	pub fn get_xlib_presentation_support_khr(self, queueFamilyIndex : u32, dpy : *mut Display, visualID : VisualID) -> Bool32 {
		unsafe { vkGetPhysicalDeviceXlibPresentationSupportKHR(self, queueFamilyIndex, dpy, visualID) }
	}

	pub fn acquire_xlib_display_ext(self, dpy : *mut Display, display : DisplayKHR) -> VkResult {
		unsafe { vkAcquireXlibDisplayEXT(self, dpy, display) }
	}

	pub fn get_rand_routput_display_ext(self, dpy : *mut Display, rrOutput : RROutput, pDisplay : *mut DisplayKHR) -> VkResult {
		unsafe { vkGetRandROutputDisplayEXT(self, dpy, rrOutput, pDisplay) }
	}

}

impl Instance {

	pub fn destroy(self) -> Self {
		unsafe { vkDestroyInstance(self, null()) };
		self
	}

	pub fn enumerate_physical_devices(self) -> Result<Vec<PhysicalDevice>, VkResult> {
		let mut count = 0;
		unsafe { vkEnumeratePhysicalDevices(self, &mut count, null_mut()) };
		let mut physicalDevices = vec![PhysicalDevice(0); count as _ ];
		match unsafe { vkEnumeratePhysicalDevices(self, &mut count, physicalDevices.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(physicalDevices),
			err => Err(err)
		}
	}

	pub fn get_proc_addr(self, pName : &u8) -> Option<extern "C" fn()> {
		unsafe { vkGetInstanceProcAddr(self, pName) }
	}

	pub fn enumerate_physical_device_groups(self) -> Result<Vec<PhysicalDeviceGroupProperties>, VkResult> {
		let mut count = 0;
		unsafe { vkEnumeratePhysicalDeviceGroups(self, &mut count, null_mut()) };
		let mut physicalDeviceGroupProperties = vec![PhysicalDeviceGroupProperties::new(); count as _ ];
		match unsafe { vkEnumeratePhysicalDeviceGroups(self, &mut count, physicalDeviceGroupProperties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(physicalDeviceGroupProperties),
			err => Err(err)
		}
	}

	pub fn destroy_surface_khr(self, surface : SurfaceKHR) -> Self {
		unsafe { vkDestroySurfaceKHR(self, surface, null()) };
		self
	}

	pub fn create_display_plane_surface_khr(self, pCreateInfo : &DisplaySurfaceCreateInfoKHR) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateDisplayPlaneSurfaceKHR(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn enumerate_physical_device_groups_khr(self) -> Result<Vec<PhysicalDeviceGroupProperties>, VkResult> {
		let mut count = 0;
		unsafe { vkEnumeratePhysicalDeviceGroupsKHR(self, &mut count, null_mut()) };
		let mut physicalDeviceGroupProperties = vec![PhysicalDeviceGroupProperties::new(); count as _ ];
		match unsafe { vkEnumeratePhysicalDeviceGroupsKHR(self, &mut count, physicalDeviceGroupProperties.as_mut_ptr()) } {
			VkResult::SUCCESS => Ok(physicalDeviceGroupProperties),
			err => Err(err)
		}
	}

	pub fn create_debug_report_callback_ext(self, pCreateInfo : &DebugReportCallbackCreateInfoEXT) -> Result<DebugReportCallbackEXT, VkResult> {
		let mut callback = DebugReportCallbackEXT(0);
		match unsafe { vkCreateDebugReportCallbackEXT(self, pCreateInfo, null(), &mut callback) } {
			VkResult::SUCCESS => Ok(callback),
			err => Err(err)
		}
	}

	pub fn destroy_debug_report_callback_ext(self, callback : DebugReportCallbackEXT) -> Self {
		unsafe { vkDestroyDebugReportCallbackEXT(self, callback, null()) };
		self
	}

	pub fn debug_report_message_ext(self, flags : DebugReportFlagsEXT, objectType : DebugReportObjectTypeEXT, object : u64, location : usize, messageCode : i32, pLayerPrefix : &u8, pMessage : &u8) -> Self {
		unsafe { vkDebugReportMessageEXT(self, flags, objectType, object, location, messageCode, pLayerPrefix, pMessage) };
		self
	}

	pub fn create_debug_utils_messenger_ext(self, pCreateInfo : &DebugUtilsMessengerCreateInfoEXT) -> Result<DebugUtilsMessengerEXT, VkResult> {
		let mut messenger = DebugUtilsMessengerEXT(0);
		match unsafe { vkCreateDebugUtilsMessengerEXT(self, pCreateInfo, null(), &mut messenger) } {
			VkResult::SUCCESS => Ok(messenger),
			err => Err(err)
		}
	}

	pub fn destroy_debug_utils_messenger_ext(self, messenger : DebugUtilsMessengerEXT) -> Self {
		unsafe { vkDestroyDebugUtilsMessengerEXT(self, messenger, null()) };
		self
	}

	pub fn submit_debug_utils_message_ext(self, messageSeverity : DebugUtilsMessageSeverityFlagsEXT, messageTypes : DebugUtilsMessageTypeFlagsEXT, pCallbackData : &DebugUtilsMessengerCallbackDataEXT) -> Self {
		unsafe { vkSubmitDebugUtilsMessageEXT(self, messageSeverity, messageTypes, pCallbackData) };
		self
	}

	pub fn create_headless_surface_ext(self, pCreateInfo : &HeadlessSurfaceCreateInfoEXT) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateHeadlessSurfaceEXT(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn create_android_surface_khr(self, pCreateInfo : &AndroidSurfaceCreateInfoKHR) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateAndroidSurfaceKHR(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn create_win_32_surface_khr(self, pCreateInfo : &Win32SurfaceCreateInfoKHR) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateWin32SurfaceKHR(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn create_wayland_surface_khr(self, pCreateInfo : &WaylandSurfaceCreateInfoKHR) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateWaylandSurfaceKHR(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn create_xcb_surface_khr(self, pCreateInfo : &XcbSurfaceCreateInfoKHR) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateXcbSurfaceKHR(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn create_xlib_surface_khr(self, pCreateInfo : &XlibSurfaceCreateInfoKHR) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateXlibSurfaceKHR(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn create_iossurface_mvk(self, pCreateInfo : &IOSSurfaceCreateInfoMVK) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateIOSSurfaceMVK(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn create_mac_ossurface_mvk(self, pCreateInfo : &MacOSSurfaceCreateInfoMVK) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateMacOSSurfaceMVK(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

	pub fn create_metal_surface_ext(self, pCreateInfo : &MetalSurfaceCreateInfoEXT) -> Result<SurfaceKHR, VkResult> {
		let mut surface = SurfaceKHR(0);
		match unsafe { vkCreateMetalSurfaceEXT(self, pCreateInfo, null(), &mut surface) } {
			VkResult::SUCCESS => Ok(surface),
			err => Err(err)
		}
	}

}


extern "C" {
    fn init();
    fn init_instance(instance: Instance);
    fn init_device(device: Device);
}

impl PhysicalDevice {
    pub fn create(self, pCreateInfo: &DeviceCreateInfo) -> Result<Device, VkResult> {
        unsafe {
            let mut device = Device(0);
            match vkCreateDevice(self, pCreateInfo, null(), &mut device) {
                VkResult::SUCCESS => {
                    init_device(device);
                    Ok(device)
                }
                err => Err(err),
            }
        }
    }
}

pub fn create_instance(pCreateInfo: &InstanceCreateInfo) -> Result<Instance, VkResult> {
    unsafe {
        init();
        let mut instance = Instance(0);
        match vkCreateInstance(pCreateInfo, null(), &mut instance) {
            VkResult::SUCCESS => {
                init_instance(instance);
                Ok(instance)
            }
            err => Err(err),
        }
    }
}

pub fn enumerate_instance_extension_properties(
    pLayerName: *const u8,
) -> Result<Vec<ExtensionProperties>, VkResult> {
    unsafe {
        init();
        let mut count = 0;
        vkEnumerateInstanceExtensionProperties(pLayerName, &mut count, null_mut());
        let mut properties = vec![ExtensionProperties::new(); count as _];
        match vkEnumerateInstanceExtensionProperties(
            pLayerName,
            &mut count,
            properties.as_mut_ptr(),
        ) {
            VkResult::SUCCESS => Ok(properties),
            err => Err(err),
        }
    }
}

pub fn enumerate_instance_layer_properties() -> Result<Vec<LayerProperties>, VkResult> {
    unsafe {
        init();
        let mut count = 0;
        vkEnumerateInstanceLayerProperties(&mut count, null_mut());
        let mut properties = vec![LayerProperties::new(); count as _];
        match vkEnumerateInstanceLayerProperties(&mut count, properties.as_mut_ptr()) {
            VkResult::SUCCESS => Ok(properties),
            err => Err(err),
        }
    }
}

pub fn enumerate_instance_version() -> Result<u32, VkResult> {
    unsafe {
        init();
        let mut apiVersion = <_>::default();
        match vkEnumerateInstanceVersion(&mut apiVersion) {
            VkResult::SUCCESS => Ok(apiVersion),
            err => Err(err),
        }
    }
}
