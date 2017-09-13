//! Vulkan Shading(Shader/Pipeline)

use vk::*;
use std::ffi::CString;
#[cfg(feature = "FeImplements")] use VkResultHandler;

/// Bitmask specifying a pipeline stage
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ShaderStage(pub VkShaderStageFlags);
impl ShaderStage
{
	/// Empty bits
	pub const EMPTY: Self = ShaderStage(0);
	/// The vertex stage
	pub const VERTEX: Self = ShaderStage(VK_SHADER_STAGE_VERTEX_BIT);
	/// The tessellation control stage
	pub const TESSELLATION_CONTROL: Self = ShaderStage(VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT);
	/// The tessellation evaluation stage
	pub const TESSELLATION_EVALUATION: Self = ShaderStage(VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT);
	/// The geometry stage
	pub const GEOMETRY: Self = ShaderStage(VK_SHADER_STAGE_GEOMETRY_BIT);
	/// The fragment stage
	pub const FRAGMENT: Self = ShaderStage(VK_SHADER_STAGE_FRAGMENT_BIT);
	/// The compute stage
	pub const COMPUTE: Self = ShaderStage(VK_SHADER_STAGE_COMPUTE_BIT);
	/// A combination of bits used as shorthand to specify all graphics stages defined above (excluding the compute stage)
	pub const ALL_GRAPHICS: Self = ShaderStage(VK_SHADER_STAGE_ALL_GRAPHICS);
	/// A combination of bits used as shorthand to specify all shader stages supported by the device,
	/// including all additional stages which are introduced by extensions
	pub const ALL: Self = ShaderStage(VK_SHADER_STAGE_ALL);
	/// A combination of tessellation control stage and tessellation evaluation stage
	pub const TESSELLATION: Self = ShaderStage(Self::TESSELLATION_CONTROL.0 | Self::TESSELLATION_EVALUATION.0);

	/// The vertex stage
	pub fn vertex(&self) -> Self { ShaderStage(self.0 | Self::VERTEX.0) }
	/// The tessellation control stage
	pub fn tessellation_control(&self) -> Self { ShaderStage(self.0 | Self::TESSELLATION_CONTROL.0) }
	/// The tessellation evaluation stage
	pub fn tessellation_evaluation(&self) -> Self { ShaderStage(self.0 | Self::TESSELLATION_EVALUATION.0) }
	/// The geometry stage
	pub fn geometry(&self) -> Self { ShaderStage(self.0 | Self::GEOMETRY.0) }
	/// The fragment stage
	pub fn fragment(&self) -> Self { ShaderStage(self.0 | Self::FRAGMENT.0) }
	/// The compute stage
	pub fn compute(&self) -> Self { ShaderStage(self.0 | Self::COMPUTE.0) }
	/// A combination of bits used as shorthand to specify all graphics stages defined above (excluding the compute stage)
	pub fn all_graphics(&self) -> Self { ShaderStage(self.0 | Self::ALL_GRAPHICS.0) }
	/// A combination of tessellation control stage and tessellation evaluation stage
	pub fn tessellation(&self) -> Self { ShaderStage(self.0 | Self::TESSELLATION.0) }
}

/// Stencil comparison function
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp
{
	/// The test never passes
	Never = VK_COMPARE_OP_NEVER as _,
	/// The test passes when `Ref < Stencil`
	Less = VK_COMPARE_OP_LESS as _,
	/// The test passes when `Ref == Stencil`
	Equal = VK_COMPARE_OP_EQUAL as _,
	/// The test passes when `Ref <= Stencil`
	LessOrEqual = VK_COMPARE_OP_LESS_OR_EQUAL as _,
	/// The test passes when `Ref > Stencil`
	Greater = VK_COMPARE_OP_GREATER as _,
	/// The test passes when `Ref != Stencil`
	NotEqual = VK_COMPARE_OP_NOT_EQUAL as _,
	/// The test passes when `Ref >= Stencil`
	GreaterOrEqual = VK_COMPARE_OP_GREATER_OR_EQUAL as _,
	/// The test always passes
	Always = VK_COMPARE_OP_ALWAYS as _
}
/// Stencil action function
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StencilOp
{
	/// Keeps the current value
	Keep = VK_STENCIL_OP_KEEP as _,
	/// Sets the value to 0
	Zero = VK_STENCIL_OP_ZERO as _,
	/// Sets the value to `reference`
	Replace = VK_STENCIL_OP_REPLACE as _,
	/// Increments the current value and clamps to the maximum representable unsigned value
	IncrementClamp = VK_STENCIL_OP_INCREMENT_AND_CLAMP as _,
	/// Decrements the current value and clamps to 0
	DecrementClamp = VK_STENCIL_OP_DECREMENT_AND_CLAMP as _,
	/// Bitwise-inverts the current value
	Invert = VK_STENCIL_OP_INVERT as _,
	/// Increments the current value and wraps to 0 when the maximum value would have been exceeded
	IncrementWrap = VK_STENCIL_OP_INCREMENT_AND_WRAP as _,
	/// Decrements the current value and wraps to the maximum possible value when the value would go below 0
	DecrementWrap = VK_STENCIL_OP_DECREMENT_AND_WRAP as _
}
/// Framebuffer logical operations
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp
{
	/// 0
	Clear = VK_LOGIC_OP_CLEAR as _,
	/// source & dest
	And = VK_LOGIC_OP_AND as _,
	/// source & ~dest
	AndReverse = VK_LOGIC_OP_AND_REVERSE as _,
	/// source
	Copy = VK_LOGIC_OP_COPY as _,
	/// ~source & dest
	AndInverted = VK_LOGIC_OP_AND_INVERTED as _,
	/// dest
	NoOp = VK_LOGIC_OP_NO_OP as _,
	/// source ^ dest
	Xor = VK_LOGIC_OP_XOR as _,
	/// source | dest
	Or = VK_LOGIC_OP_OR as _,
	/// ~(source | dest)
	Nor = VK_LOGIC_OP_NOR as _,
	/// ~(source ^ dest)
	Equivalent = VK_LOGIC_OP_EQUIVALENT as _,
	/// ~dest
	Invert = VK_LOGIC_OP_INVERT as _,
	/// source | ~dest
	OrReverse = VK_LOGIC_OP_OR_REVERSE as _,
	/// ~source
	CopyInverted = VK_LOGIC_OP_COPY_INVERTED as _,
	/// ~source | dest
	OrInverted = VK_LOGIC_OP_OR_INVERTED as _,
	/// ~(source & dest)
	Nand = VK_LOGIC_OP_NAND as _,
	/// 1
	Set = VK_LOGIC_OP_SET as _
}
/// Bitmask specifying sets of stencil state for which to update the compare mask
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum StencilFaceMask
{
	/// Only the front set of stencil state
	Front = VK_STENCIL_FACE_FRONT_BIT as _,
	/// Only the back set of stencil state
	Back = VK_STENCIL_FACE_BACK_BIT as _,
	/// Both sets of stencil state
	Both = VK_STENCIL_FRONT_AND_BACK as _
}

/// Opaque handle to a shader module object
pub struct ShaderModule(VkShaderModule, ::Device);
/// Opaque handle to a pipeline cache object
pub struct PipelineCache(VkPipelineCache, ::Device);
/// Opaque handle to a pipeline layout object
pub struct PipelineLayout(pub VkPipelineLayout, ::Device);
/// Opaque handle to a pipeline object
pub struct Pipeline(pub VkPipeline, ::Device);

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{
	for ShaderModule[vkDestroyShaderModule], PipelineCache[vkDestroyPipelineCache], PipelineLayout[vkDestroyPipelineLayout]
}

#[cfg(feature = "FeImplements")]
impl ShaderModule
{
	/// Creates a new shader module object from bytes on the memory
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn from_memory<Buffer: AsRef<[u8]> + ?Sized>(device: &::Device, buffer: &Buffer) -> ::Result<Self>
	{
		let cinfo = VkShaderModuleCreateInfo
		{
			codeSize: buffer.as_ref().len() as _, pCode: buffer.as_ref().as_ptr() as *const _, .. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateShaderModule(device.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }.into_result()
			.map(|_| ShaderModule(h, device.clone()))
	}
	/// Creates a new shader module object from a file
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// IO Errors may be occured when reading file
	pub fn from_file<FilePath: AsRef<::std::path::Path> + ?Sized>(device: &::Device, path: &FilePath) -> Result<Self, Box<::std::error::Error>>
	{
		use ::std::io::prelude::Read;
		let bin = ::std::fs::File::open(path).and_then(|mut fp| { let mut v = Vec::new(); fp.read_to_end(&mut v).map(|_| v) })?;
		Self::from_memory(device, &bin).map_err(From::from)
	}
}
#[cfg(feature = "FeImplements")]
impl PipelineCache
{
	/// Creates a new pipeline cache
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn new<Data: AsRef<[u8]> + ?Sized>(device: &::Device, initial: &Data) -> ::Result<Self>
	{
		let cinfo = VkPipelineCacheCreateInfo
		{
			initialDataSize: initial.as_ref().len() as _, pInitialData: initial.as_ref().as_ptr() as *const _, .. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreatePipelineCache(device.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }.into_result()
			.map(|_| PipelineCache(h, device.clone()))
	}
	/// Get the data store from a pipeline cache
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn data(&self) -> ::Result<Vec<u8>>
	{
		let mut n = 0;
		unsafe { vkGetPipelineCacheData(self.1.native_ptr(), self.0, &mut n, ::std::ptr::null_mut()) }.into_result()?;
		let mut b = Vec::<u8>::with_capacity(n as _); unsafe { b.set_len(n as _) };
		unsafe { vkGetPipelineCacheData(self.1.native_ptr(), self.0, &mut n, b.as_mut_ptr() as *mut _) }.into_result().map(|_| b)
	}
	/// Combine the data stores of pipeline caches into `self`
	/// # Failures
	/// On failure, this command returns
	/// VK_ERROR_OUT_OF_HOST_MEMORY
	/// VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn merge_into(&self, src: &[&PipelineCache]) -> ::Result<()>
	{
		let srcs = src.iter().map(|x| x.0).collect::<Vec<_>>();
		unsafe { vkMergePipelineCaches(self.1.native_ptr(), self.0, srcs.len() as _, srcs.as_ptr()) }.into_result()
	}
}
#[cfg(feature = "FeImplements")]
impl PipelineLayout
{
	/// Creates a new pipeline layout object
	pub fn new(device: &::Device, layouts: &[&::DescriptorSetLayout], push_constants: &[(ShaderStage, ::std::ops::Range<u32>)]) -> ::Result<Self>
	{
		let layouts = layouts.into_iter().map(|x| x.0).collect::<Vec<_>>();
		let push_constants = push_constants.iter().map(|x| VkPushConstantRange { stageFlags: x.0 .0, offset: x.1.start, size: x.1.end - x.1.start })
			.collect::<Vec<_>>();
		let cinfo = VkPipelineLayoutCreateInfo
		{
			setLayoutCount: layouts.len() as _, pSetLayouts: layouts.as_ptr(),
			pushConstantRangeCount: push_constants.len() as _, pPushConstantRanges: push_constants.as_ptr(),
			.. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreatePipelineLayout(device.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }.into_result()
			.map(|_| PipelineLayout(h, device.clone()))
	}
}

pub enum SwitchOrDynamicState<T> { Disabled, Dynamic, Static(T) }
impl<T> SwitchOrDynamicState<T>
{
	fn is_dynamic(&self) -> bool { match self { &SwitchOrDynamicState::Dynamic => true, _ => false } }
	fn is_enabled(&self) -> bool { match self { &SwitchOrDynamicState::Disabled => false, _ => true } }
}
pub use SwitchOrDynamicState::*;
/// Untyped data cell
#[cfg_attr(not(feature = "FeImplements"), allow(dead_code))]
pub struct DynamicDataCell<'d> { size: usize, data: *const (), ph: ::std::marker::PhantomData<&'d ()> }
impl<'d, T> From<&'d T> for DynamicDataCell<'d>
{
	/// Construct borrowing a data
	fn from(d: &'d T) -> Self { DynamicDataCell { size: ::std::mem::size_of::<T>(), data: d as *const T as *const _, ph: ::std::marker::PhantomData } }
}
impl<'d> DynamicDataCell<'d>
{
	/// Construct borrowing a slice
	pub fn from_slice<T>(s: &'d [T]) -> Self
	{
		DynamicDataCell { size: ::std::mem::size_of::<T>() * s.len(), data: s.as_ptr() as *const _, ph: ::std::marker::PhantomData }
	}
}
/// Builder struct to construct a shader stage in a `Pipeline`
#[cfg_attr(not(feature = "FeImplements"), allow(dead_code))]
pub struct PipelineShader<'d>
{
	module: &'d ShaderModule, entry_name: CString, specinfo: Option<(Vec<VkSpecializationMapEntry>, DynamicDataCell<'d>)>
}
/// Whether the state(type of array) is dynamic or static
pub enum DynamicArrayState<T> { Dynamic(usize), Static(Vec<T>) }
/// Build struct holding dynamic state flags
struct DynamicStateFlags
{
	viewport: bool, scissor: bool, line_width: bool, depth_bias: bool, blend_constants: bool, depth_bounds: bool,
	stencil_compare_mask: bool, stencil_write_mask: bool, stencil_reference: bool
}
/// Which is pipeline state to derive from
#[derive(Clone, Copy)]
pub enum BasePipeline<'d>
{
	/// Does not derive
	None,
	/// Derive from a handle to the pipeline state object
	Handle(&'d Pipeline),
	/// Derive from a create info in the `pCreateInfos` parameter
	Index(u32)
}
/// Builder struct to construct a `Pipeline` for graphics operations
pub struct GraphicsPipelineBuilder<'d>
{
	flags: VkPipelineCreateFlags, _layout: &'d PipelineLayout, rp: &'d ::RenderPass, subpass: u32, _base: BasePipeline<'d>,
	vs: Option<PipelineShader<'d>>, tcs: Option<PipelineShader<'d>>, tes: Option<PipelineShader<'d>>,
	gs: Option<PipelineShader<'d>>, fs: Option<PipelineShader<'d>>,
	vi_state: (VkPipelineVertexInputStateCreateInfo, Vec<VkVertexInputBindingDescription>, Vec<VkVertexInputAttributeDescription>),
	ia_state: VkPipelineInputAssemblyStateCreateInfo,
	rasterizer_state: VkPipelineRasterizationStateCreateInfo,
	tess_state: Option<Box<VkPipelineTessellationStateCreateInfo>>,
	viewport_state: Option<(Box<VkPipelineViewportStateCreateInfo>, Vec<VkViewport>, Vec<VkRect2D>)>,
	ms_state: Option<(Box<VkPipelineMultisampleStateCreateInfo>, Vec<VkSampleMask>)>,
	ds_state: Option<Box<VkPipelineDepthStencilStateCreateInfo>>,
	color_blending: Option<(Box<VkPipelineColorBlendStateCreateInfo>, Vec<VkPipelineColorBlendAttachmentState>)>,
	dynamic_state_flags: DynamicStateFlags
}
impl<T> DynamicArrayState<T>
{
	fn count(&self) -> usize { match self { &DynamicArrayState::Dynamic(s) => s, &DynamicArrayState::Static(ref v) => v.len() } }
	fn as_ptr(&self) -> *const T { match self { &DynamicArrayState::Static(ref v) => v.as_ptr(), _ => ::std::ptr::null() } }
	fn is_dynamic(&self) -> bool { match self { &DynamicArrayState::Dynamic(_) => true, _ => false } }
}
impl<'d> PipelineShader<'d>
{
	/// Construct a shader stage in the pipeline
	pub fn new(module: &'d ShaderModule, entry_name: &str, specinfo: Option<(Vec<VkSpecializationMapEntry>, DynamicDataCell<'d>)>) -> Self
	{
		PipelineShader { module, entry_name: CString::new(entry_name).unwrap(), specinfo }
	}
}
impl<'d> GraphicsPipelineBuilder<'d>
{
	// Shading State and Input Configuration //

	pub fn vertex_shader(&mut self, shader: PipelineShader<'d>) -> &mut Self { self.vs = Some(shader); self }
	pub fn tessellation_control_shader(&mut self, shader: PipelineShader<'d>) -> &mut Self { self.tcs = Some(shader); self }
	pub fn tessellation_evaluation_shader(&mut self, shader: PipelineShader<'d>) -> &mut Self { self.tes = Some(shader); self }
	pub fn geometry_shader(&mut self, shader: PipelineShader<'d>) -> &mut Self { self.gs = Some(shader); self }
	pub fn fragment_shader(&mut self, shader: PipelineShader<'d>) -> &mut Self { self.fs = Some(shader); self }
	pub fn vertex_input_state(&mut self, bindings: Vec<VkVertexInputBindingDescription>, attributes: Vec<VkVertexInputAttributeDescription>) -> &mut Self
	{
		self.vi_state.0.vertexBindingDescriptionCount = bindings.len() as _; self.vi_state.0.pVertexBindingDescriptions = bindings.as_ptr();
		self.vi_state.0.vertexAttributeDescriptionCount = attributes.len() as _; self.vi_state.0.pVertexAttributeDescriptions = attributes.as_ptr();
		self.vi_state.1 = bindings; self.vi_state.2 = attributes; self
	}
	pub fn vertex_processing(&mut self, shader: PipelineShader<'d>,
		bindings: Vec<VkVertexInputBindingDescription>, attributes: Vec<VkVertexInputAttributeDescription>) -> &mut Self
	{
		self.vertex_shader(shader).vertex_input_state(bindings, attributes)
	}
	/// The primitive topology and primitiveRestartEnable: controls whether a special vertex index value is treated as restarting the assembly of primitives.
	/// The special index value is either `0xFFFFFFFF` when the index type is equal to `VK_INDEX_TYPE_UINT32`,
	/// or `0xFFFF` when the index type is equal to `VK_INDEX_TYPE_UINT16`
	/// Primitive restart is not allowed for "list" topologies
	pub fn primitive_topology(&mut self, topo: VkPrimitiveTopology, enable_restarting: bool) -> &mut Self
	{
		self.ia_state.topology = topo; self.ia_state.primitiveRestartEnable = enable_restarting as _; self
	}
	/// Number of control points per patch
	pub fn patch_control_point_count(&mut self, count: u32) -> &mut Self
	{
		if self.tess_state.is_none() { self.tess_state = Some(Default::default()); }
		self.tess_state.as_mut().unwrap().patchControlPoints = count; self
	}
	pub fn tessellator_settings(&mut self, control: PipelineShader<'d>, evaluation: PipelineShader<'d>, num_control_points: u32) -> &mut Self
	{
		self.tessellation_control_shader(control).tessellation_evaluation_shader(evaluation).patch_control_point_count(num_control_points)
	}

	// Viewport / Scissor State //

	/// # Safety
	/// Application must guarantee that the number of viewports and scissors are identical
	pub unsafe fn viewports(&mut self, vps: DynamicArrayState<VkViewport>) -> &mut Self
	{
		if self.viewport_state.is_none() { self.viewport_state = Some((Default::default(), Vec::new(), Vec::new())); }
		self.viewport_state.as_mut().unwrap().0.viewportCount = vps.count() as _;
		self.viewport_state.as_mut().unwrap().0.pViewports = vps.as_ptr();
		self.dynamic_state_flags.viewport = vps.is_dynamic();
		if let DynamicArrayState::Static(v) = vps { self.viewport_state.as_mut().unwrap().1 = v; }
		self
	}
	/// # Safety
	/// Application must guarantee that the number of viewports and scissors are identical
	pub unsafe fn scissors(&mut self, scs: DynamicArrayState<VkRect2D>) -> &mut Self
	{
		if self.viewport_state.is_none() { self.viewport_state = Some((Default::default(), Vec::new(), Vec::new())); }
		self.viewport_state.as_mut().unwrap().0.scissorCount = scs.count() as _;
		self.viewport_state.as_mut().unwrap().0.pScissors = scs.as_ptr();
		self.dynamic_state_flags.scissor = scs.is_dynamic();
		if let DynamicArrayState::Static(v) = scs { self.viewport_state.as_mut().unwrap().2 = v; }
		self
	}
	/// Safety way calling `viewports` and `scissors`
	pub fn fixed_viewport_scissors(&mut self, vps: DynamicArrayState<VkViewport>, scissor: DynamicArrayState<VkRect2D>) -> &mut Self
	{
		assert_eq!(vps.count(), scissor.count());
		unsafe { self.viewports(vps).scissors(scissor) }
	}

	// Rasterization State //

	/// Controls whether to clamp the fragment's depth values instead of clipping primitives to the z planes of the frustum,
	/// as described in `Primitive Clipping` in Vulkan Specification
	pub fn depth_clamp_enable(&mut self, enable: bool) -> &mut Self { self.rasterizer_state.depthClampEnable = enable as _; self }
	/// Controls whether primitives are discarded immediately before the rasterization stage
	pub fn rasterizer_discard_enable(&mut self, enable: bool) -> &mut Self { self.rasterizer_state.rasterizerDiscardEnable = enable as _; self }
	/// The triangle rendering mode
	pub fn polygon_mode(&mut self, mode: VkPolygonMode) -> &mut Self { self.rasterizer_state.polygonMode = mode; self }
	/// The triangle facing direction used for primitive culling
	pub fn cull_mode(&mut self, mode: VkCullModeFlags) -> &mut Self { self.rasterizer_state.cullMode = mode; self }
	/// The front-facing triangle orientation to be used for culling
	pub fn front_face(&mut self, face: VkFrontFace) -> &mut Self { self.rasterizer_state.frontFace = face; self }
	/// Specify `None` to disable to bias fragment depth values.
	/// Tuple Member: (`ConstantFactor`, `Clamp`, `SlopeFactor`)
	/// - `ConstantFactor`: A scalar factor controlling the constant depth value added to each fragment
	/// - `Clamp`: The maximum (or minimum) depth bias of a fragment
	/// - `SlopeFactor`: A scalar factor applied to a fragment's slope in depth bias calculations
	pub fn depth_bias(&mut self, opts: SwitchOrDynamicState<(f32, f32, f32)>) -> &mut Self
	{
		self.rasterizer_state.depthBiasEnable = opts.is_enabled() as _;
		self.dynamic_state_flags.depth_bias = opts.is_dynamic();
		if let SwitchOrDynamicState::Static((cf, c, sf)) = opts
		{
			self.rasterizer_state.depthBiasConstantFactor = cf;
			self.rasterizer_state.depthBiasClamp = c;
			self.rasterizer_state.depthBiasSlopeFactor = sf;
		}
		self
	}
	/// The width of rasterized line segments. Specifying `None` means that the `lineWidth` parameter is a dynamic state.
	pub fn line_width(&mut self, width: Option<f32>) -> &mut Self
	{
		self.dynamic_state_flags.line_width = width.is_none() as _;
		self.rasterizer_state.lineWidth = width.unwrap_or(0.0); self
	}

	// Multisample State //

	fn ms_ref(&mut self) -> &mut (Box<VkPipelineMultisampleStateCreateInfo>, Vec<VkSampleMask>)
	{
		if self.ms_state.is_none() { self.ms_state = Some((Default::default(), Vec::new())); }
		self.ms_state.as_mut().unwrap()
	}
	/// Fragment shading executed per-sample(`true`), or per-fragment(`false`), as described in `Sample Shading` in Vulkan Specification
	pub fn shading_per_sample(&mut self, flag: bool) -> &mut Self { self.ms_ref().0.sampleShadingEnable = flag as _; self }
	/// The minimum fraction of sample shading, as described in `Sample Shading` in Vulkan Specification
	pub fn min_sample_shading(&mut self, frac: f32) -> &mut Self { self.ms_ref().0.minSampleShading = frac; self }
	/// Controls whether a temporary coverage value is generated based on the alpha component of the fragment's first color output as
	/// specified in the `Multisample Coverage` section in Vulkan Specification
	pub fn alpha_to_coverage_enable(&mut self, enable: bool) -> &mut Self { self.ms_ref().0.alphaToCoverageEnable = enable as _; self }
	/// Controls whether the alpha component of the fragment's first color output is replaced with one
	/// as described in `Multisample Coverage` section in Vulkan Specification
	pub fn alpha_to_one_enable(&mut self, enable: bool) -> &mut Self { self.ms_ref().0.alphaToOneEnable = enable as _; self }
	/// A sample count bits specifying the number of samples per pixel used in rasterization
	pub fn rasterization_samples(&mut self, bits: u32, sample_masks: Vec<VkSampleMask>) -> &mut Self
	{
		assert!(sample_masks.is_empty() || sample_masks.len() == bits as usize / 32);
		self.ms_ref().0.rasterizationSamples = bits;
		self.ms_ref().1 = sample_masks;
		self.ms_ref().0.pSampleMask = if self.ms_ref().1.is_empty() { ::std::ptr::null() } else { self.ms_ref().1.as_ptr() };
		self
	}

	// Depth / Stencil State //

	/// Clear depth/stencil state
	pub fn clear_depth_stencil_state(&mut self) -> &mut Self { self.ds_state = None; self }
	fn dss_ref(&mut self) -> &mut VkPipelineDepthStencilStateCreateInfo
	{
		if self.ds_state.is_none() { self.ds_state = Some(Box::new(Default::default())); }
		self.ds_state.as_mut().unwrap()
	}
	/// Controls whether depth testing is enabled
	pub fn depth_test_enable(&mut self, enable: bool) -> &mut Self { self.dss_ref().depthTestEnable = enable as _; self }
	/// Controls whether depth writes are enabled, or always disabled
	pub fn depth_write_enable(&mut self, enable: bool) -> &mut Self { self.dss_ref().depthWriteEnable = enable as _; self }
	/// The comparison operator used in the depth test
	pub fn depth_compare_op(&mut self, op: CompareOp) -> &mut Self { self.dss_ref().depthCompareOp = op as _; self }
	/// Controls whether depth testing is enabled, depth writes are enabled, and the comparison operator used in the depth test
	/// Specifying `None` to `compare_to` disables depth testing
	pub fn depth_test_settings(&mut self, compare_op: Option<CompareOp>, write_enable: bool) -> &mut Self
	{
		if let Some(op) = compare_op { self.depth_test_enable(true).depth_compare_op(op) } else { self.depth_test_enable(false) }
			.depth_write_enable(write_enable)
	}
	/// Controls whether depth bounds testing is enabled
	pub fn depth_bounds_test_enable(&mut self, enable: bool) -> &mut Self { self.dss_ref().depthBoundsTestEnable = enable as _; self }
	/// Controls whether stencil testing is enabled
	pub fn stencil_test_enable(&mut self, enable: bool) -> &mut Self { self.dss_ref().stencilTestEnable = enable as _; self }
	/// Control the parameter of the stencil test
	pub fn stencil_control_front(&mut self, compare_op: CompareOp, reference: u32, compare_mask: u32,
		fail_op: StencilOp, pass_op: StencilOp, depth_fail_op: StencilOp, write_mask: u32) -> &mut Self
	{
		self.dynamic_state_flags.stencil_compare_mask = false;
		self.dynamic_state_flags.stencil_write_mask = false;
		self.dynamic_state_flags.stencil_reference = false;
		self.dss_ref().front = VkStencilOpState
		{
			failOp: fail_op as _, passOp: pass_op as _, depthFailOp: depth_fail_op as _, compareOp: compare_op as _,
			compareMask: compare_mask, writeMask: write_mask, reference
		};
		self
	}
	/// Control the parameter of the stencil test
	pub fn stencil_control_back(&mut self, compare_op: CompareOp, reference: u32, compare_mask: u32,
		fail_op: StencilOp, pass_op: StencilOp, depth_fail_op: StencilOp, write_mask: u32) -> &mut Self
	{
		self.dynamic_state_flags.stencil_compare_mask = false;
		self.dynamic_state_flags.stencil_write_mask = false;
		self.dynamic_state_flags.stencil_reference = false;
		self.dss_ref().back = VkStencilOpState
		{
			failOp: fail_op as _, passOp: pass_op as _, depthFailOp: depth_fail_op as _, compareOp: compare_op as _,
			compareMask: compare_mask, writeMask: write_mask, reference
		};
		self
	}
	/// Controls the parameter of the compare mask of the stencil test. Tuple ordering: (front, back).
	/// Specifying `None` means that the parameter is a dynamic state
	pub fn stencil_compare_mask(&mut self, mask: Option<(u32, u32)>) -> &mut Self
	{
		self.dynamic_state_flags.stencil_compare_mask = if let Some((f, b)) = mask
		{
			self.dss_ref().front.compareMask = f;
			self.dss_ref().back.compareMask = b;
			false
		}
		else { true }; self
	}
	/// Controls the parameter of the write mask of the stencil test. Tuple ordering: (front, back)
	/// Specifying `None` means that the parameter is a dynamic state
	pub fn stencil_write_mask(&mut self, mask: Option<(u32, u32)>) -> &mut Self
	{
		self.dynamic_state_flags.stencil_write_mask = if let Some((f, b)) = mask
		{
			self.dss_ref().front.writeMask = f; self.dss_ref().back.writeMask = b;
			false
		}
		else { true }; self
	}
	/// Controls the parameter of the reference of the stencil test. Tuple ordering: (front, back)
	/// Specifying `None` means that the parameter is a dynamic state
	pub fn stencil_reference(&mut self, mask: Option<(u32, u32)>) -> &mut Self
	{
		self.dynamic_state_flags.stencil_reference = if let Some((f, b)) = mask
		{
			self.dss_ref().front.reference = f; self.dss_ref().back.reference = b;
			false
		}
		else { true }; self
	}
	/// The range of values used in the depth bounds test
	pub fn depth_bounds_range(&mut self, bounds: ::std::ops::Range<f32>) -> &mut Self
	{
		self.dss_ref().minDepthBounds = bounds.start; self.dss_ref().maxDepthBounds = bounds.end; self
	}
	/// Control the depth bounds test
	pub fn depth_bounds(&mut self, bounds: SwitchOrDynamicState<::std::ops::Range<f32>>) -> &mut Self
	{
		self.depth_bounds_test_enable(bounds.is_enabled());
		self.dynamic_state_flags.depth_bounds = bounds.is_dynamic();
		if let SwitchOrDynamicState::Static(r) = bounds { self.depth_bounds_range(r) } else { self }
	}

	// Color Blending //

	fn cb_ref(&mut self) -> &mut (Box<VkPipelineColorBlendStateCreateInfo>, Vec<VkPipelineColorBlendAttachmentState>)
	{
		if self.color_blending.is_none() { self.color_blending = Some((Default::default(), Vec::new())) }
		self.color_blending.as_mut().unwrap()
	}
	/// Which logical operation to apply. Specifying `None` disables *Logical Operations*
	pub fn logic_op(&mut self, op: Option<LogicOp>) -> &mut Self
	{
		self.cb_ref().0.logicOpEnable = op.is_some() as _;
		self.cb_ref().0.logicOp = op.unwrap_or(LogicOp::NoOp) as _;
		self
	}
	/// Per target attachment states
	pub fn add_attachment_blend(&mut self, blend: VkPipelineColorBlendAttachmentState) -> &mut Self
	{
		self.cb_ref().1.push(blend); self
	}
	/// Array of four values used as the R, G, B, and A components of the blend constant that are used in blending, depending on the blend factor.
	/// Specifying `None` means that the `blendConstants` parameter is a dynamic state
	pub fn blend_constants(&mut self, values: Option<[f32; 4]>) -> &mut Self
	{
		self.dynamic_state_flags.blend_constants = values.is_none();
		self.cb_ref().0.blendConstants.copy_from_slice(&values.unwrap_or([0.0; 4])); self
	}

	// Misc Configurations //

	/// The base pipeline handle/index to derive from
	pub fn base(&mut self, b: BasePipeline<'d>) -> &mut Self { self._base = b; self }
	/// The description of binding locations used by both the pipeline and descriptor sets used with the pipeline
	pub fn layout(&mut self, l: &'d PipelineLayout) -> &mut Self { self._layout = l; self }
	/// A handle to a render pass object and the index of the subpass where this pipeline will be used
	pub fn render_pass(&mut self, rpo: &'d ::RenderPass, subpass: u32) -> &mut Self
	{
		self.rp = rpo; self.subpass = subpass; self
	}
	/// The created pipeline will or will not be optimized
	/// Disabling optimization of the pipeline may reduce the time taken to create the pipeline
	pub fn enable_optimization(&mut self, opt: bool) -> &mut Self
	{
		if opt { self.flags &= !VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT }
		else { self.flags |= VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT };
		self
	}
	/// The pipeline to be created is allowed to be the parent of a pipeline that will be created in a subsequent creation operation
	pub fn allow_derivatives(&mut self, allow: bool) -> &mut Self
	{
		if allow { self.flags |= VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT }
		else { self.flags &= !VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT };
		self
	}

	// Unsafe Utilities //

	/// Set the `VkPipelineVertexInputStateCreateInfo` structure directly
	/// # Safety
	/// Application must guarantee these constraints:
	/// - The lifetime of the content in the structure is valid for this builder
	/// - The content in the structure is valid
	pub unsafe fn vertex_input_state_create_info(&mut self, state: VkPipelineVertexInputStateCreateInfo) -> &mut Self
	{
		self.vi_state = (state, Vec::new(), Vec::new()); self
	}
	/// Set the `VkPipelineInputAssemblyStateCreateInfo` structure directly
	/// # Safety
	/// Application must guarantee these constraints:
	/// - The lifetime of the content in the structure is valid for this builder
	/// - The content in the structure is valid
	pub unsafe fn input_assembly_state_create_info(&mut self, state: VkPipelineInputAssemblyStateCreateInfo) -> &mut Self
	{
		self.ia_state = state; self
	}
	/// Set the `VkPipelineTessellationStateCreateInfo` structure directly
	/// # Safety
	/// Application must guarantee these constraints:
	/// - The lifetime of the content in the structure is valid for this builder
	/// - The content in the structure is valid
	pub unsafe fn tessellation_state_create_info(&mut self, state: Option<Box<VkPipelineTessellationStateCreateInfo>>) -> &mut Self
	{
		self.tess_state = state; self
	}
	/// Set the `VkPipelineViewportStateCreateInfo` structure directly.
	/// This does not clear any dynamic states
	/// # Safety
	/// Application must guarantee these constraints:
	/// - The lifetime of the content in the structure is valid for this builder
	/// - The content in the structure is valid
	pub unsafe fn viewport_state_create_info(&mut self, state: Option<Box<VkPipelineViewportStateCreateInfo>>) -> &mut Self
	{
		self.viewport_state = state.map(|x| (x, Vec::new(), Vec::new())); self
	}
	/// Set the `VkPipelineRasterizationStateCreateInfo` structure directly.
	/// This does not clear any dynamic states
	/// # Safety
	/// Application must guarantee these constraints:
	/// - The lifetime of the content in the structure is valid for this builder
	/// - The content in the structure is valid
	pub unsafe fn rasterization_state_create_info(&mut self, state: VkPipelineRasterizationStateCreateInfo) -> &mut Self
	{
		self.rasterizer_state = state; self
	}
	/// Set the `VkPipelineMultisampleStateCreateInfo` structure directly.
	/// This does not clear any dynamic states
	/// # Safety
	/// Application must guarantee these constraints:
	/// - The lifetime of the content in the structure is valid for this builder
	/// - The content in the structure is valid
	pub unsafe fn multisample_state_create_info(&mut self, state: Option<Box<VkPipelineMultisampleStateCreateInfo>>) -> &mut Self
	{
		self.ms_state = state.map(|x| (x, Vec::new())); self
	}
	/// Set the `VkPipelineDepthStencilStateCreateInfo` structure directly.
	/// This does not clear any dynamic states
	/// # Safety
	/// Application must guarantee these constraints:
	/// - The lifetime of the content in the structure is valid for this builder
	/// - The content in the structure is valid
	pub unsafe fn depth_stencil_state_create_info(&mut self, state: Option<Box<VkPipelineDepthStencilStateCreateInfo>>) -> &mut Self
	{
		self.ds_state = state; self
	}
	/// Set the `VkPipelineColorBlendStateCreateInfo` structure directly.
	/// This does not clear any dynamic states
	/// # Safety
	/// Application must guarantee these constraints:
	/// - The lifetime of the content in the structure is valid for this builder
	/// - The content in the structure is valid
	pub unsafe fn color_blend_state_info(&mut self, state: Option<Box<VkPipelineColorBlendStateCreateInfo>>) -> &mut Self
	{
		self.color_blending = state.map(|x| (x, Vec::new())); self
	}
}

#[cfg(feature = "FeImplements")]
impl<'d> PipelineShader<'d>
{
	fn createinfo_native(&self, stage: ShaderStage) -> (VkPipelineShaderStageCreateInfo, Option<Box<VkSpecializationInfo>>)
	{
		let specinfo = self.specinfo.as_ref().map(|&(ref m, ref d)| Box::new(VkSpecializationInfo
		{
			mapEntryCount: m.len() as _, pMapEntries: m.as_ptr(), dataSize: d.size as _, pData: d.data as _
		}));
		(VkPipelineShaderStageCreateInfo
		{
			stage: stage.0, module: self.module.0, pName: self.entry_name.as_ptr(),
			pSpecializationInfo: specinfo.as_ref().map(|x| &**x as *const _).unwrap_or(::std::ptr::null()),
			.. Default::default()
		}, specinfo)
	}
}
#[cfg(feature = "FeImplements")]
impl<'d> GraphicsPipelineBuilder<'d>
{
	/// Create a graphics pipeline
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	#[allow(unused_variables)]
	pub fn create(&self, device: &::Device, cache: Option<&PipelineCache>) -> ::Result<Pipeline>
	{
		let (vs, vs_) = self.vs.as_ref().expect("Required the VertexShader in Graphics Pipeline").createinfo_native(ShaderStage::VERTEX);
		let tcs = self.tcs.as_ref().map(|x| x.createinfo_native(ShaderStage::TESSELLATION_CONTROL));
		let tes = self.tes.as_ref().map(|x| x.createinfo_native(ShaderStage::TESSELLATION_EVALUATION));
		let gs = self.gs.as_ref().map(|x| x.createinfo_native(ShaderStage::GEOMETRY));
		let fs = self.fs.as_ref().map(|x| x.createinfo_native(ShaderStage::FRAGMENT));
		let mut stages = vec![vs];
		let tcs_ = if let Some((s, sp)) = tcs { stages.push(s); Some(sp) } else { None };
		let tes_ = if let Some((s, sp)) = tes { stages.push(s); Some(sp) } else { None };
		let gs_ = if let Some((s, sp)) = gs { stages.push(s); Some(sp) } else { None };
		let fs_ = if let Some((s, sp)) = fs { stages.push(s); Some(sp) } else { None };
		let mut dynamic_states = Vec::new();
		if self.dynamic_state_flags.viewport { dynamic_states.push(VK_DYNAMIC_STATE_VIEWPORT); }
		if self.dynamic_state_flags.scissor  { dynamic_states.push(VK_DYNAMIC_STATE_SCISSOR); }
		if self.dynamic_state_flags.line_width { dynamic_states.push(VK_DYNAMIC_STATE_LINE_WIDTH); }
		if self.dynamic_state_flags.depth_bias { dynamic_states.push(VK_DYNAMIC_STATE_DEPTH_BIAS); }
		if self.dynamic_state_flags.blend_constants { dynamic_states.push(VK_DYNAMIC_STATE_BLEND_CONSTANTS); }
		if self.dynamic_state_flags.depth_bounds { dynamic_states.push(VK_DYNAMIC_STATE_DEPTH_BOUNDS); }
		if self.dynamic_state_flags.stencil_compare_mask { dynamic_states.push(VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK); }
		if self.dynamic_state_flags.stencil_write_mask { dynamic_states.push(VK_DYNAMIC_STATE_STENCIL_WRITE_MASK); }
		if self.dynamic_state_flags.stencil_reference { dynamic_states.push(VK_DYNAMIC_STATE_STENCIL_REFERENCE); }
		let ds = if !dynamic_states.is_empty()
		{
			Some(VkPipelineDynamicStateCreateInfo
			{
				dynamicStateCount: dynamic_states.len() as _, pDynamicStates: dynamic_states.as_ptr(), .. Default::default()
			})
		}
		else { None };
		let base = match self._base
		{
			BasePipeline::Handle(ref h) => Some(h.0), BasePipeline::None => None,
			_ => panic!("Deriving from other info in same creation is invalid for single creation of pipeline")
		};
		let flags = self.flags | if base.is_some() { VK_PIPELINE_CREATE_DERIVATIVE_BIT } else { 0 };
		let cinfo = VkGraphicsPipelineCreateInfo
		{
			stageCount: stages.len() as _, pStages: stages.as_ptr(), pVertexInputState: &self.vi_state.0,
			pInputAssemblyState: &self.ia_state, pTessellationState: self.tess_state.as_ref().map(|x| &**x as *const _).unwrap_or(::std::ptr::null()),
			pViewportState: self.viewport_state.as_ref().map(|&(ref x, _, _)| &**x as *const _).unwrap_or(::std::ptr::null()),
			pRasterizationState: &self.rasterizer_state as *const _,
			pMultisampleState: self.ms_state.as_ref().map(|&(ref x, _)| &**x as *const _).unwrap_or(::std::ptr::null()),
			pDepthStencilState: self.ds_state.as_ref().map(|x| &**x as *const _).unwrap_or(::std::ptr::null()),
			pColorBlendState: self.color_blending.as_ref().map(|&(ref x, _)| &**x as *const _).unwrap_or(::std::ptr::null()),
			pDynamicState: ds.as_ref().map(|x| x as *const _).unwrap_or(::std::ptr::null()),
			layout: self._layout.0, renderPass: self.rp.0, subpass: self.subpass,
			basePipelineHandle: if let &BasePipeline::Handle(ref h) = &self._base { h.0 } else { VK_NULL_HANDLE as _ },
			basePipelineIndex: -1, flags, .. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateGraphicsPipelines(device.native_ptr(), cache.map(|x| x.0).unwrap_or(VK_NULL_HANDLE as _),
			1, &cinfo, ::std::ptr::null(), &mut h) }.into_result().map(|_| Pipeline(h, device.clone()))
	}
}

#[cfg(feature = "FeImplements")]
impl ::Device
{
	/// Create graphics pipelines
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn create_graphics_pipelines(&self, builders: &[GraphicsPipelineBuilder], cache: Option<&PipelineCache>) -> ::Result<Vec<Pipeline>>
	{
		let aggregates = builders.iter().map(|x|
		{
			let (vs, vs_) = x.vs.as_ref().expect("Required the VertexShader in Graphics Pipeline").createinfo_native(ShaderStage::VERTEX);
			let tcs = x.tcs.as_ref().map(|x| x.createinfo_native(ShaderStage::TESSELLATION_CONTROL));
			let tes = x.tes.as_ref().map(|x| x.createinfo_native(ShaderStage::TESSELLATION_EVALUATION));
			let gs = x.gs.as_ref().map(|x| x.createinfo_native(ShaderStage::GEOMETRY));
			let fs = x.fs.as_ref().map(|x| x.createinfo_native(ShaderStage::FRAGMENT));
			let mut stages = vec![vs];
			let tcs_ = if let Some((s, sp)) = tcs { stages.push(s); Some(sp) } else { None };
			let tes_ = if let Some((s, sp)) = tes { stages.push(s); Some(sp) } else { None };
			let gs_ = if let Some((s, sp)) = gs { stages.push(s); Some(sp) } else { None };
			let fs_ = if let Some((s, sp)) = fs { stages.push(s); Some(sp) } else { None };
			let mut dynamic_states = Vec::new();
			if x.dynamic_state_flags.viewport { dynamic_states.push(VK_DYNAMIC_STATE_VIEWPORT); }
			if x.dynamic_state_flags.scissor  { dynamic_states.push(VK_DYNAMIC_STATE_SCISSOR); }
			if x.dynamic_state_flags.line_width { dynamic_states.push(VK_DYNAMIC_STATE_LINE_WIDTH); }
			if x.dynamic_state_flags.depth_bias { dynamic_states.push(VK_DYNAMIC_STATE_DEPTH_BIAS); }
			if x.dynamic_state_flags.blend_constants { dynamic_states.push(VK_DYNAMIC_STATE_BLEND_CONSTANTS); }
			if x.dynamic_state_flags.depth_bounds { dynamic_states.push(VK_DYNAMIC_STATE_DEPTH_BOUNDS); }
			if x.dynamic_state_flags.stencil_compare_mask { dynamic_states.push(VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK); }
			if x.dynamic_state_flags.stencil_write_mask { dynamic_states.push(VK_DYNAMIC_STATE_STENCIL_WRITE_MASK); }
			if x.dynamic_state_flags.stencil_reference { dynamic_states.push(VK_DYNAMIC_STATE_STENCIL_REFERENCE); }
			let ds = if !dynamic_states.is_empty()
			{
				Some(VkPipelineDynamicStateCreateInfo
				{
					dynamicStateCount: dynamic_states.len() as _, pDynamicStates: dynamic_states.as_ptr(), .. Default::default()
				})
			}
			else { None };
			(stages, ds, vs_, tcs_, tes_, gs_, fs_, dynamic_states)
		}).collect::<Vec<_>>();
		let cinfos = builders.iter().zip(aggregates.iter()).map(|(b, &(ref stages, ref ds, _, _, _, _, _, _))|
		{
			let (base_handle, base_index) = match b._base
			{
				BasePipeline::Handle(ref h) => (h.0, -1), BasePipeline::None => (VK_NULL_HANDLE as _, -1),
				BasePipeline::Index(x) => (VK_NULL_HANDLE as _, x as i32)
			};
			let flags = b.flags | if base_handle != VK_NULL_HANDLE as _ || base_index >= 0 { VK_PIPELINE_CREATE_DERIVATIVE_BIT } else { 0 };
			VkGraphicsPipelineCreateInfo
			{
				stageCount: stages.len() as _, pStages: stages.as_ptr(), pVertexInputState: &b.vi_state.0,
				pInputAssemblyState: &b.ia_state, pTessellationState: b.tess_state.as_ref().map(|x| &**x as *const _).unwrap_or(::std::ptr::null()),
				pViewportState: b.viewport_state.as_ref().map(|&(ref x, _, _)| &**x as *const _).unwrap_or(::std::ptr::null()),
				pRasterizationState: &b.rasterizer_state as *const _,
				pMultisampleState: b.ms_state.as_ref().map(|&(ref x, _)| &**x as *const _).unwrap_or(::std::ptr::null()),
				pDepthStencilState: b.ds_state.as_ref().map(|x| &**x as *const _).unwrap_or(::std::ptr::null()),
				pColorBlendState: b.color_blending.as_ref().map(|&(ref x, _)| &**x as *const _).unwrap_or(::std::ptr::null()),
				pDynamicState: ds.as_ref().map(|x| x as *const _).unwrap_or(::std::ptr::null()),
				layout: b._layout.0, renderPass: b.rp.0, subpass: b.subpass,
				basePipelineHandle: base_handle, basePipelineIndex: base_index, flags, .. Default::default()
			}
		}).collect::<Vec<_>>();
		let mut hs = vec![VK_NULL_HANDLE as VkPipeline; builders.len()];
		unsafe { vkCreateGraphicsPipelines(self.native_ptr(), cache.map(|x| x.0).unwrap_or(VK_NULL_HANDLE as _),
			cinfos.len() as _, cinfos.as_ptr(), ::std::ptr::null(), hs.as_mut_ptr()) }.into_result()
			.map(|_| hs.into_iter().map(|h| Pipeline(h, self.clone())).collect())
	}
}

/// Bitmask specifying pipeline stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineStageFlags(VkPipelineStageFlags);
impl PipelineStageFlags
{
	/// The stage of the pipeline where any commands are initially received by the queue
	pub const TOP_OF_PIPE: Self = PipelineStageFlags(VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT);
	/// The stage of the pipeline where Draw/DispatchIndirect data structures are consumed
	pub const DRAW_INDIRECT: Self = PipelineStageFlags(VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT);
	/// The stage of the pipeline where vertex and index buffers are consumed
	pub const VERTEX_INPUT: Self = PipelineStageFlags(VK_PIPELINE_STAGE_VERTEX_INPUT_BIT);
	/// The vertex shader stage
	pub const VERTEX_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_VERTEX_SHADER_BIT);
	/// The tessellation control shader stage
	pub const TESSELLATION_CONTROL_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT);
	/// The tessellation evaluation shader stage
	pub const TESSELLATION_EVALUATION_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT);
	/// The geometry shader stage
	pub const GEOMETRY_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT);
	/// The fragment shader stage
	pub const FRAGMENT_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
	/// The stage of the pipeline where early fragment tests (depth and stencil tests before fragment shading) are performed
	pub const EARLY_FRAGMENT_TESTS: Self = PipelineStageFlags(VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT);
	/// The stage of the pipeline where late fragment tests (depth and stencil tests after fragment shading) are performed
	pub const LATE_FRAGMENT_TESTS: Self = PipelineStageFlags(VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT);
	/// The stage of the pipeline after blending where the final color values are output from the pipeline
	pub const COLOR_ATTACHMENT_OUTPUT: Self = PipelineStageFlags(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT);
	/// The execution of copy commands
	pub const TRANSFER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_TRANSFER_BIT);
	/// The execution of a compute shader
	pub const COMPUTE_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT);
	/// The final stage in the pipeline where operations generated by all commands complete execution
	pub const BOTTOM_OF_PIPE: Self = PipelineStageFlags(VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT);
	/// A pseudo-stage indicating execution on the host of reads/writes of device memory
	pub const HOST: Self = PipelineStageFlags(VK_PIPELINE_STAGE_HOST_BIT);
	/// The execution of all graphics pipeline stages
	pub const ALL_GRAPHICS: Self = PipelineStageFlags(VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT);
	/// Equivalent to the logical OR of every other pipeline stage flag that is supported on the quue it is used with
	pub const ALL_COMMANDS: Self = PipelineStageFlags(VK_PIPELINE_STAGE_ALL_COMMANDS_BIT);

	/// The stage of the pipeline where any commands are initially received by the queue
	pub fn top_of_pipe(&self) -> Self { PipelineStageFlags(self.0 | Self::TOP_OF_PIPE.0) }
	/// The stage of the pipeline where Draw/DispatchIndirect data structures are consumed
	pub fn draw_indirect(&self) -> Self { PipelineStageFlags(self.0 | Self::DRAW_INDIRECT.0) }
	/// The stage of the pipeline where vertex and index buffers are consumed
	pub fn vertex_input(&self) -> Self { PipelineStageFlags(self.0 | Self::VERTEX_INPUT.0) }
	/// The vertex shader stage
	pub fn vertex_shader(&self) -> Self { PipelineStageFlags(self.0 | Self::VERTEX_SHADER.0) }
	/// The tessellation control shader stage
	pub fn tessellation_control_shader(&self) -> Self { PipelineStageFlags(self.0 | Self::TESSELLATION_CONTROL_SHADER.0) }
	/// The tessellation evaluation shader stage
	pub fn tessellation_evaluation_shader(&self) -> Self { PipelineStageFlags(self.0 | Self::TESSELLATION_EVALUATION_SHADER.0) }
	/// The geometry shader stage
	pub fn geometry_shader(&self) -> Self { PipelineStageFlags(self.0 | Self::GEOMETRY_SHADER.0) }
	/// The fragment shader stage
	pub fn fragment_shader(&self) -> Self { PipelineStageFlags(self.0 | Self::FRAGMENT_SHADER.0) }
	/// The stage of the pipeline where early fragment tests (depth and stencil tests before fragment shading) are performed
	pub fn early_fragment_tests(&self) -> Self { PipelineStageFlags(self.0 | Self::EARLY_FRAGMENT_TESTS.0) }
	/// The stage of the pipeline where late fragment tests (depth and stencil tests after fragment shading) are performed
	pub fn late_fragment_tests(&self) -> Self { PipelineStageFlags(self.0 | Self::LATE_FRAGMENT_TESTS.0) }
	/// The stage of the pipeline after blending where the final color values are output from the pipeline
	pub fn color_attachment_output(&self) -> Self { PipelineStageFlags(self.0 | Self::COLOR_ATTACHMENT_OUTPUT.0) }
	/// The execution of copy commands
	pub fn transfer(&self) -> Self { PipelineStageFlags(self.0 | Self::TRANSFER.0) }
	/// The execution of a compute shader
	pub fn compute_shader(&self) -> Self { PipelineStageFlags(self.0 | Self::COMPUTE_SHADER.0) }
	/// The final stage in the pipeline where operations generated by all commands complete execution
	pub fn bottom_of_pipe(&self) -> Self { PipelineStageFlags(self.0 | Self::BOTTOM_OF_PIPE.0) }
	/// A pseudo-stage indicating execution on the host of reads/writes of device memory
	pub fn host(&self) -> Self { PipelineStageFlags(self.0 | Self::HOST.0) }
	/// The execution of all graphics pipeline stages
	pub fn all_graphics(&self) -> Self { PipelineStageFlags(self.0 | Self::ALL_GRAPHICS.0) }
	/// Equivalent to the logical OR of every other pipeline stage flag that is supported on the quue it is used with
	pub fn all_commands(&self) -> Self { PipelineStageFlags(self.0 | Self::ALL_COMMANDS.0) }
}
