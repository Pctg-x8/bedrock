//! Vulkan Shading(Shader/Pipeline)

use vk::*;

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

/// Opaque handle to a shader module object
pub struct ShaderModule(VkShaderModule, ::Device);
/// Opaque handle to a pipeline cache object
pub struct PipelineCache(VkPipelineCache, ::Device);

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{
	for ShaderModule[vkDestroyShaderModule], PipelineCache[vkDestroyPipelineCache]
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
			codeSize: buffer.len() as _, pCode: buffer.as_ptr() as *const _, .. Default::default()
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
	pub fn from_file<FilePath: AsRef<OsStr> + ?Sized>(device: &::Device, path: &FilePath) -> Result<Self, Box<::std::error::Error>>
	{
		let bin = ::std::fs::File::open(path).and_then(|mut fp| { let v = Vec::new(); fp.read_to_end(&mut v).map(|_| v) })?;
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
			initialDataSize: initial.len() as _, pInitialData: initial.as_ptr(), .. Default::default()
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
		let mut b = Vec::with_capacity(n as _); unsafe { b.set_len(n as _) };
		unsafe { vkGetPipelineCacheData(self.1.native_ptr(), self.0, &mut n, b.as_mut_ptr()) }.into_result()
			.map(|_| b)
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
