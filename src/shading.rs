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
	pub const ALL_GRAPHICS: Self = ShaderStage(VK_SHADER_STAGE_ALL_GRAPHICS_BIT);
	/// A combination of bits used as shorthand to specify all shader stages supported by the device,
	/// including all additional stages which are introduced by extensions
	pub const ALL: Self = ShaderStage(VK_SHADER_STAGE_ALL_BIT);
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
