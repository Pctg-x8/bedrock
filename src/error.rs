use bedrock_vk::*;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResultCode(pub VkResult);
impl core::fmt::Debug for ResultCode {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "[{}] {}", self.0.0, bedrock_vk::result_to_str(self.0))
    }
}
impl core::fmt::Display for ResultCode {
    #[inline(always)]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, fmt)
    }
}
impl core::error::Error for ResultCode {}
impl ResultCode {
    #[inline]
    pub const fn is_err(&self) -> bool {
        self.0.0 < 0
    }

    #[inline]
    pub const fn into_result(self) -> Result<Self, Self> {
        if self.is_err() { Err(self) } else { Ok(self) }
    }
}

#[inline(always)]
pub const fn translate_vk_result(r: VkResult) -> Result<VkResult, ResultCode> {
    if r.0 < 0 { Err(ResultCode(r)) } else { Ok(r) }
}
