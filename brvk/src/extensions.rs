use crate::*;

// Spreading single value to all dimensions
impl VkExtent2D {
    pub const fn spread1(value: u32) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}
impl VkExtent3D {
    pub const fn spread1(value: u32) -> Self {
        Self {
            width: value,
            height: value,
            depth: value,
        }
    }

    pub const fn new1(width: u32) -> Self {
        Self {
            width,
            height: 1,
            depth: 1,
        }
    }

    pub const fn new2(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            depth: 1,
        }
    }

    pub const fn new(width: u32, height: u32, depth: u32) -> Self {
        Self { width, height, depth }
    }

    pub const fn as_2d_ref(&self) -> &VkExtent2D {
        unsafe { std::mem::transmute(self) }
    }
}
impl VkOffset2D {
    pub const fn spread1(value: i32) -> Self {
        Self { x: value, y: value }
    }
}
impl VkOffset3D {
    pub const fn spread1(value: i32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }

    pub const fn new1(x: i32) -> Self {
        Self { x, y: 0, z: 0 }
    }

    pub const fn new2(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn as_2d_ref(&self) -> &VkOffset2D {
        unsafe { std::mem::transmute(self) }
    }
}

// into conversion to larger dimension //
impl VkExtent2D {
    pub const fn with_depth(self, depth: u32) -> VkExtent3D {
        VkExtent3D {
            width: self.width,
            height: self.height,
            depth,
        }
    }
}
impl VkOffset2D {
    pub const fn with_z(self, z: i32) -> VkOffset3D {
        VkOffset3D {
            x: self.x,
            y: self.y,
            z,
        }
    }
}

// AsRef for self //
impl AsRef<VkExtent3D> for VkExtent3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkExtent2D> for VkExtent2D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkOffset3D> for VkOffset3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkOffset2D> for VkOffset2D {
    fn as_ref(&self) -> &Self {
        self
    }
}

// AsRef Conversion to smaller-dimension
impl AsRef<VkExtent2D> for VkExtent3D {
    fn as_ref(&self) -> &VkExtent2D {
        unsafe { core::mem::transmute(self) }
    }
}
impl AsRef<VkOffset2D> for VkOffset3D {
    fn as_ref(&self) -> &VkOffset2D {
        unsafe { core::mem::transmute(self) }
    }
}

// From conversion to smaller-dimension
impl From<VkOffset3D> for VkOffset2D {
    fn from(value: VkOffset3D) -> Self {
        Self { x: value.x, y: value.y }
    }
}

impl From<VkExtent3D> for VkExtent2D {
    fn from(value: VkExtent3D) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

// Swizzling
impl VkExtent3D {
    pub const fn wh(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.width,
            height: self.height,
        }
    }

    pub const fn wd(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.width,
            height: self.depth,
        }
    }

    pub const fn hw(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.height,
            height: self.width,
        }
    }

    pub const fn hd(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.height,
            height: self.depth,
        }
    }

    pub const fn dw(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.depth,
            height: self.width,
        }
    }

    pub const fn dh(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.depth,
            height: self.height,
        }
    }
}
impl VkOffset3D {
    pub const fn xy(&self) -> VkOffset2D {
        VkOffset2D { x: self.x, y: self.y }
    }

    pub const fn xz(&self) -> VkOffset2D {
        VkOffset2D { x: self.x, y: self.z }
    }

    pub const fn yx(&self) -> VkOffset2D {
        VkOffset2D { x: self.y, y: self.x }
    }

    pub const fn yz(&self) -> VkOffset2D {
        VkOffset2D { x: self.y, y: self.z }
    }

    pub const fn zx(&self) -> VkOffset2D {
        VkOffset2D { x: self.z, y: self.x }
    }

    pub const fn zy(&self) -> VkOffset2D {
        VkOffset2D { x: self.z, y: self.y }
    }
}

/// Utility Constants
impl VkExtent2D {
    pub const ONE: Self = Self::spread1(1);
}
impl VkExtent3D {
    pub const ONE: Self = Self::spread1(1);
}
impl VkOffset2D {
    pub const ZERO: Self = Self::spread1(0);
}
impl VkOffset3D {
    pub const ZERO: Self = Self::spread1(0);
}

/// Viewport and Rect Util Functions
impl VkExtent2D {
    pub const fn into_rect(self, offset: VkOffset2D) -> VkRect2D {
        VkRect2D { offset, extent: self }
    }
}
impl From<VkViewport> for VkRect2D {
    fn from(vp: VkViewport) -> Self {
        VkRect2D {
            offset: VkOffset2D {
                x: vp.x as _,
                y: vp.y as _,
            },
            extent: VkExtent2D {
                width: vp.width as _,
                height: vp.height as _,
            },
        }
    }
}
impl VkRect2D {
    pub const fn make_viewport(&self, depth_range: std::ops::Range<f32>) -> VkViewport {
        VkViewport {
            x: self.offset.x as _,
            y: self.offset.y as _,
            width: self.extent.width as _,
            height: self.extent.height as _,
            minDepth: depth_range.start,
            maxDepth: depth_range.end,
        }
    }
}
impl VkViewport {
    pub const fn from_rect_with_depth_range(rect: &VkRect2D, depth_range: core::ops::Range<f32>) -> Self {
        rect.make_viewport(depth_range)
    }

    pub const fn set_offset(&mut self, offset: &VkOffset2D) -> &mut Self {
        self.x = offset.x as _;
        self.y = offset.y as _;
        self
    }
    pub const fn set_extent(&mut self, extent: &VkExtent2D) -> &mut Self {
        self.width = extent.width as _;
        self.height = extent.height as _;
        self
    }
    pub const fn set_depth_range(&mut self, range: core::ops::Range<f32>) -> &mut Self {
        self.minDepth = range.start;
        self.maxDepth = range.end;
        self
    }
}
