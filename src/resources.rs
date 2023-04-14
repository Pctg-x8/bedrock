//! Vulkan Resources
//!
//! (以下メモ)
//!
//! ## バッファの作成
//!
//! [`BufferDesc`](struct.BufferDesc.html)で作成する
//!
//! ```rust,ignore
//! let buffer = BufferDesc::new(4 * 4 * 3, BufferUsage::VERTEX_BUFFER.transfer_dest()).crete(&device)?;
//! ```
//!
//! `new`から`create`までにメソッドチェーンを用いて以下のようなバッファの詳細を指定できる。
//!
//! - [`sparse_binding_opt`](struct.BufferDesc.html#method.sparse_binding_opt): SparseBinding時の許可される挙動を指定する。デフォルトでは"なし"
//!   - [`BufferSparseBinding::Bound`]でSparseBindingによってメモリにバインドできることを示す
//!   - [`BufferSparseBinding::Residency`]で部分的にメモリにバインドできることを示す
//!   - [`BufferSparseBinding::Aliased`]で、バインド先のメモリ範囲が他のバッファに同時に使われる可能性を示す
//!   - [`BufferSparseBinding::Both`]は`Residency`と`Aliased`の両方を示す
//! - [`sharing_queue_families`](struct.BufferDesc.html#method.sharing_queue_families): 複数のキューでアイテムを共有する際に、共有したいキューファミリの番号を指定する。デフォルトは空(占有)
//!
//! ## イメージの作成
//!
//! [`ImageDesc`](struct.ImageDesc.html)で作成する
//!
//! ```rust,ignore
//! let image = ImageDesc::new(&Extent2D(128, 128), VK_FORMAT_R8G8B8A8_UNORM, ImageUsage::SAMPLED.color_attachment(), ImageLayout::General)
//! 	.create(&device)?;
//! ```
//!
//! [`ImageDesc::new`](struct.ImageDesc.html#method.new)の第一引数に
//!
//! - `Extent1D`を指定すると1Dテクスチャ
//! - `Extent2D`を指定すると2Dテクスチャ
//! - `Extent3D`を指定すると3Dテクスチャ
//!
//! を生成するようになる。
//! `new`から`create`までにメソッドチェーンを用いて以下のようなイメージの詳細を指定できる。
//!
//! - [`sample_counts`](struct.ImageDesc.html#method.sample_counts): イメージの要素ごとのサンプル数を2^nの値(1, 2, 4, 8, 16, 32, 64)で指定する。デフォルトは1。
//!   以下の条件を一つでも満たす場合は1を設定する必要がある。
//!   - 最適タイリング(`VK_IMAGE_TILING_OPTIMAL`)が使われていない(`use_linear_tiling`を併用する場合)
//!   - 2Dテクスチャではない(`new`の第一引数が`Extent2D`でない場合)
//!   - キューブテクスチャである(`flags`に`ImageFlags::CUBE_COMPATIBLE`を指定している場合)
//!   - 指定したフォーマットがカラーアタッチメントもしくは深度/ステンシルアタッチメントとしての利用に対応していない場合
//!     - RGBAフォーマットやDSフォーマットを指定している分には気にする必要はない
//! - [`use_linear_tiling`](struct.ImageDesc.html#method.use_linear_tiling): イメージデータのメモリ上での配列を線形に強制する(デフォルトではデバイス最適な並びを使うようになっている)
//!   - ディスクから読み込んだピクセルデータなどを`map`して流し込む場合はこれが必要
//! - [`array_layers`](struct.ImageDesc.html#method.array_layers): 配列イメージの要素数を指定する。デフォルトは1(配列ではない)
//! - [`mip_levels`](struct.ImageDesc.html#method.mip_levels): ミップマップの最大縮小レベルを指定する。デフォルトは1(ミップマップを使用しない)
//! - [`sharing_queue_families`](struct.ImageDesc.html#method.sharing_queue_families): 複数のキューでアイテムを共有する際に、共有したいキューファミリの番号を指定する。デフォルトは空(占有)
//! - [`flags`](struct.ImageDesc.html#method.flags): [`ImageFlags`](struct.ImageFlags.html)を指定する。デフォルトでは"なし"
//!
//! ## `BufferUsage`の種類
//!
//! [`BufferUsage`](struct.BufferUsage.html)はメソッドチェーンを利用してビットフラグを指定する。メソッド名は定数名をすべて小文字にしたもの。
//!
//! ```rust,ignore
//! BufferUsage::VERTEX_BUFFER.transfer_dest()
//! ```
//!
//! ### 入力/利用形態系
//!
//! - `VERTEX_BUFFER`: **頂点バッファ** として頂点入力時に使用できる
//! - `INDEX_BUFFER`: **インデックスバッファ** として頂点入力時に使用できる
//! - `UNIFORM_BUFFER`: **定数バッファ** としてデスクリプタ入力時に使用できる
//! - `STORAGE_BUFFER`: **ストレージバッファ** としてデスクリプタ入力時に使用できる
//!   - 定数バッファより大容量
//! - `UNIFORM_TEXEL_BUFFER`: 1Dのイメージアイテムとして適用可能な定数バッファとしてデスクリプタ入力時に使用できる
//! - `STORAGE_TEXEL_BUFFER`: 1Dのイメージアイテムとして適用可能なストレージバッファとしてデスクリプタ入力時に使用できる
//! - `INDIRECT_BUFFER`: 間接実行コマンドの **引数バッファ** として使用できる
//!
//! ### 転送系
//!
//! - `TRANSFER_SRC`: 転送コマンドでソースアイテムとして指定可能であることを示す
//! - `TRANSFER_DEST`: 転送コマンドで対象アイテムとして指定可能であることを示す
//!   - *このバッファに対してクリア、値埋めコマンドを適用したい場合もこれを指定する必要がある*
//!
//! ## `ImageUsage`の種類
//!
//! [`ImageUsage`](struct.ImageUsage.html)もメソッドチェーンを利用してビットフラグを指定する。
//!
//! ```rust,ignore
//! ImageUsage::SAMPLED.color_attachment()
//! ```
//!
//! ### シェーダ入力系
//!
//! - `SAMPLED`: シェーダによってサンプル可能であることを示す
//!   - シェーダで **テクスチャ** として使用できるようにする場合はこれ
//! - `INPUT_ATTACHMENT`: シェーダによって入力アタッチメントとしての扱いを受けることができる
//!   - シェーダで入力アタッチメントとして指定したい場合(中間バッファなど)はこれ
//! - `STORAGE`: シェーダのイメージ入力として使用可能であることを示す
//!   - `SAMPLED`との違いは、こちらはサンプラーによるサンプリングを使用できない
//!
//! ### 出力系
//!
//! - `COLOR_ATTACHMENT`: [`crate::framebuffer::Framebuffer`]の出力(カラーもしくはマルチサンプル解決)アイテムとして利用可能であることを示す
//!   - 要するに、 **コマンドで描画した結果を受け取る** ことができる
//!   - プロシージャルテクスチャの作成やオフスクリーンレンダリングの出力として使いたい場合はこれ
//! - `DEPTH_STENCIL_ATTACHMENT`: [`crate::framebuffer::Framebuffer`]での深度/ステンシルバッファとして利用可能であることを示す
//!   - オフスクリーンレンダリングなどで深度バッファが必要な場合はこれ
//!
//! ### 転送系
//!
//! - `TRANSFER_SRC`: 転送コマンドでソースアイテムとして指定可能であることを示す
//!   - このテクスチャが何らかのコピー元になる場合はこれ
//! - `TRANSFER_DEST`: 転送コマンドで対象アイテムとして指定可能であることを示す
//!   - このテクスチャが何らかのコピー先になる場合はこれ
//!   - このテクスチャに対してクリア、値埋めコマンドを適用したい場合はこれ
//!
//! ### その他
//!
//! - `TRANSIENT_ATTACHMENT`: 色、深度/ステンシル、マルチサンプル解決、および入力アイテムとして指定可能であることを示す
//!   - テクスチャが`VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`が指定された[`DeviceMemory`]にバインドされることを想定している
//!   - パス間の中間バッファなどで、一時的に確保される必要があるバッファに指定するとメモリ使用量が少なくて済むかもしれない？
//!

#[cfg(feature = "Implements")]
use crate::vkresolve::{Resolver, ResolverInterface};
use crate::{vk::*, DeviceChild, VkObject, VulkanStructure};
use crate::{AnalogNumRange, CompareOp, VkHandle};
#[cfg(feature = "Implements")]
use std::ops::Range;
use std::ops::{BitOr, BitOrAssign, Deref};

DefineStdDeviceChildObject! {
    /// Opaque handle to a device memory object
    DeviceMemoryObject(VkDeviceMemory, VK_OBJECT_TYPE_DEVICE_MEMORY): DeviceMemory { drop free_memory }
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a buffer object(constructed via [`BufferDesc`])
    BufferObject(VkBuffer, VK_OBJECT_TYPE_BUFFER): Buffer { drop destroy_buffer }
}
impl<Device: crate::Device> MemoryBound for BufferObject<Device> {
    #[cfg(feature = "Implements")]
    fn requirements(&self) -> VkMemoryRequirements {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_buffer_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                p.as_mut_ptr(),
            );

            p.assume_init()
        }
    }

    #[cfg(feature = "Implements")]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()> {
        unsafe {
            Resolver::get()
                .bind_buffer_memory(
                    self.device().native_ptr(),
                    self.native_ptr(),
                    memory.native_ptr(),
                    offset as _,
                )
                .into_result()
                .map(drop)
        }
    }
}

/// Opaque handle to a image object(constructed via `ImageDesc`)
#[derive(VkHandle)]
pub struct ImageObject<Device: crate::Device>(VkImage, Device, VkImageType, VkFormat, VkExtent3D);
impl<Device: crate::Device> VkObject for ImageObject<Device> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_IMAGE;
}
unsafe impl<Device: crate::Device + Sync> Sync for ImageObject<Device> {}
unsafe impl<Device: crate::Device + Send> Send for ImageObject<Device> {}
impl<Device: crate::Device> DeviceChild for ImageObject<Device> {
    type ConcreteDevice = Device;

    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
#[cfg(feature = "Implements")]
impl<Device: crate::Device> Drop for ImageObject<Device> {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_image(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}
impl<Device: crate::Device> Image for ImageObject<Device> {
    fn format(&self) -> VkFormat {
        self.3
    }

    fn size(&self) -> &VkExtent3D {
        &self.4
    }

    fn dimension(&self) -> VkImageViewType {
        match self.2 {
            VK_IMAGE_TYPE_1D => VK_IMAGE_VIEW_TYPE_1D,
            VK_IMAGE_TYPE_2D => VK_IMAGE_VIEW_TYPE_2D,
            VK_IMAGE_TYPE_3D => VK_IMAGE_VIEW_TYPE_3D,
            _ => unreachable!(),
        }
    }
}
impl<Device: crate::Device> MemoryBound for ImageObject<Device> {
    #[cfg(feature = "Implements")]
    fn requirements(&self) -> VkMemoryRequirements {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_image_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                p.as_mut_ptr(),
            );

            p.assume_init()
        }
    }

    #[cfg(feature = "Implements")]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()> {
        unsafe {
            Resolver::get()
                .bind_image_memory(
                    self.device().native_ptr(),
                    self.native_ptr(),
                    memory.native_ptr(),
                    offset as _,
                )
                .into_result()
                .map(drop)
        }
    }
}

/// Opaque handle to a image object, backed by Swapchain.
#[cfg(feature = "VK_KHR_swapchain")]
#[derive(VkHandle)]
pub struct SwapchainImage<Swapchain: crate::Swapchain>(pub(crate) VkImage, pub(crate) Swapchain, pub(crate) VkFormat);
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<Swapchain: crate::Swapchain + Sync> Sync for SwapchainImage<Swapchain> {}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<Swapchain: crate::Swapchain + Send> Send for SwapchainImage<Swapchain> {}
#[cfg(feature = "VK_KHR_swapchain")]
impl<Swapchain: crate::Swapchain> VkObject for SwapchainImage<Swapchain> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_IMAGE;
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<Swapchain: crate::Swapchain> DeviceChild for SwapchainImage<Swapchain> {
    type ConcreteDevice = Swapchain::ConcreteDevice;

    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<Swapchain: crate::Swapchain> Image for SwapchainImage<Swapchain> {
    fn format(&self) -> VkFormat {
        self.2
    }

    fn size(&self) -> &VkExtent3D {
        self.1.size()
    }

    fn dimension(&self) -> VkImageViewType {
        VK_IMAGE_VIEW_TYPE_2D
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<Swapchain: crate::Swapchain> MemoryBound for SwapchainImage<Swapchain> {
    #[cfg(feature = "Implements")]
    fn requirements(&self) -> VkMemoryRequirements {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_image_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                p.as_mut_ptr(),
            );

            p.assume_init()
        }
    }

    #[cfg(feature = "Implements")]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()> {
        unsafe {
            Resolver::get()
                .bind_image_memory(
                    self.device().native_ptr(),
                    self.native_ptr(),
                    memory.native_ptr(),
                    offset as _,
                )
                .into_result()
                .map(drop)
        }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<Swapchain: crate::Swapchain + Clone> SwapchainImage<&'_ Swapchain> {
    /// Clones parent reference
    pub fn clone_parent(self) -> SwapchainImage<Swapchain> {
        let r = SwapchainImage(self.0, self.1.clone(), self.2);
        // disable dropping self.0
        std::mem::forget(self);
        r
    }
}

#[derive(VkHandle)]
/// Opaque handle to a buffer view object
pub struct BufferViewObject<Buffer: crate::Buffer>(VkBufferView, Buffer);
impl<Buffer: crate::Buffer> VkObject for BufferViewObject<Buffer> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_BUFFER_VIEW;
}
unsafe impl<Buffer: crate::Buffer + Sync> Sync for BufferViewObject<Buffer> {}
unsafe impl<Buffer: crate::Buffer + Send> Send for BufferViewObject<Buffer> {}
impl<Buffer: crate::Buffer> DeviceChild for BufferViewObject<Buffer> {
    type ConcreteDevice = Buffer::ConcreteDevice;

    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
#[cfg(feature = "Implements")]
impl<Buffer: crate::Buffer> Drop for BufferViewObject<Buffer> {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_buffer_view(self.1.device().native_ptr(), self.0, std::ptr::null());
        }
    }
}
impl<Buffer: crate::Buffer> BufferView for BufferViewObject<Buffer> {}

/// Opaque handle to a image view object
#[derive(VkHandle)]
pub struct ImageViewObject<Image: crate::Image>(VkImageView, Image);
impl<Image: crate::Image> VkObject for ImageViewObject<Image> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_IMAGE_VIEW;
}
unsafe impl<Image: crate::Image + Sync> Sync for ImageViewObject<Image> {}
unsafe impl<Image: crate::Image + Send> Send for ImageViewObject<Image> {}
impl<Image: crate::Image> DeviceChild for ImageViewObject<Image> {
    type ConcreteDevice = Image::ConcreteDevice;

    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
#[cfg(feature = "Implements")]
impl<Image: crate::Image> Drop for ImageViewObject<Image> {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_image_view(self.1.device().native_ptr(), self.0, std::ptr::null());
        }
    }
}
impl<Image: crate::Image> ImageView for ImageViewObject<Image> {}

impl<Buffer: crate::Buffer> Deref for BufferViewObject<Buffer> {
    type Target = Buffer;

    fn deref(&self) -> &Buffer {
        &self.1
    }
}
impl<Image: crate::Image> Deref for ImageViewObject<Image> {
    type Target = Image;

    fn deref(&self) -> &Image {
        &self.1
    }
}
impl<Image: crate::Image> ImageChild for ImageViewObject<Image> {
    type ConcreteImage = Image;

    fn image(&self) -> &Image {
        &self.1
    }
}

/// Bitmask specifying allowed usage of a buffer
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct BufferUsage(pub VkBufferUsageFlags);
impl BufferUsage {
    /// Specifies that the buffer can be used as the source of a transfer command
    pub const TRANSFER_SRC: Self = BufferUsage(VK_BUFFER_USAGE_TRANSFER_SRC_BIT);
    /// Specifies that the buffer can be used as the destination of a transfer command
    pub const TRANSFER_DEST: Self = BufferUsage(VK_BUFFER_USAGE_TRANSFER_DST_BIT);
    /// Specifies that the buffer can be used to create a `BufferView` suitable for
    /// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`
    pub const UNIFORM_TEXEL_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT);
    /// Specifies that the buffer can be used to create a `BufferView` suitable for
    /// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`
    pub const STORAGE_TEXEL_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT);
    /// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
    /// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
    pub const UNIFORM_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT);
    /// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
    /// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
    pub const STORAGE_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_STORAGE_BUFFER_BIT);
    /// Specifies that the buffer is suitable for passing as the `buffer` parameter to `DrawCommandBuffer::bind_index_buffer`
    pub const INDEX_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_INDEX_BUFFER_BIT);
    /// Specifies that the buffer is suitable for passing as an element of the `buffers` array to `DrawCommandBuffer::bind_vertex_buffers`
    pub const VERTEX_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_VERTEX_BUFFER_BIT);
    /// Specifies that the buffer is suitable for passing as the `buffer` parameter to
    /// `DrawCommandBuffer::draw_indirect`, `DrawCommandBuffer::draw_indexed_indirect`, or `ComputeCommandBuffer::dispatch_indirect`
    pub const INDIRECT_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT);

    /// Specifies that the buffer can be used as the source of a transfer command
    pub fn transfer_src(self) -> Self {
        BufferUsage(self.0 | Self::TRANSFER_SRC.0)
    }
    /// Specifies that the buffer can be used as the destination of a transfer command
    pub fn transfer_dest(self) -> Self {
        BufferUsage(self.0 | Self::TRANSFER_DEST.0)
    }
    /// Specifies that the buffer can be used to create a `BufferView` suitable for
    /// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`
    pub fn uniform_texel_buffer(self) -> Self {
        BufferUsage(self.0 | Self::UNIFORM_TEXEL_BUFFER.0)
    }
    /// Specifies that the buffer can be used to create a `BufferView` suitable for
    /// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`
    pub fn storage_texel_buffer(self) -> Self {
        BufferUsage(self.0 | Self::STORAGE_TEXEL_BUFFER.0)
    }
    /// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
    /// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
    pub fn uniform_buffer(self) -> Self {
        BufferUsage(self.0 | Self::UNIFORM_BUFFER.0)
    }
    /// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
    /// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
    pub fn storage_buffer(self) -> Self {
        BufferUsage(self.0 | Self::STORAGE_BUFFER.0)
    }
    /// Specifies that the buffer is suitable for passing as the `buffer` parameter to `DrawCommandBuffer::bind_index_buffer`
    pub fn index_buffer(self) -> Self {
        BufferUsage(self.0 | Self::INDEX_BUFFER.0)
    }
    /// Specifies that the buffer is suitable for passing as an element of the `buffers` array to `DrawCommandBuffer::bind_vertex_buffers`
    pub fn vertex_buffer(self) -> Self {
        BufferUsage(self.0 | Self::VERTEX_BUFFER.0)
    }
    /// Specifies that the buffer is suitable for passing as the `buffer` parameter to
    /// `DrawCommandBuffer::draw_indirect`, `DrawCommandBuffer::draw_indexed_indirect`, or `ComputeCommandBuffer::dispatch_indirect`
    pub fn indirect_buffer(self) -> Self {
        BufferUsage(self.0 | Self::INDIRECT_BUFFER.0)
    }

    /// Generates a default access type mask
    pub fn default_access_mask(self) -> VkAccessFlags {
        let mut bits = 0;
        if (self.0 & Self::TRANSFER_SRC.0) != 0 {
            bits |= VK_ACCESS_TRANSFER_READ_BIT;
        }
        if (self.0 & Self::TRANSFER_DEST.0) != 0 {
            bits |= VK_ACCESS_TRANSFER_WRITE_BIT;
        }
        if (self.0 & Self::UNIFORM_TEXEL_BUFFER.0) != 0 {
            bits |= VK_ACCESS_UNIFORM_READ_BIT;
        }
        if (self.0 & Self::STORAGE_TEXEL_BUFFER.0) != 0 {
            bits |= VK_ACCESS_UNIFORM_READ_BIT;
        }
        if (self.0 & Self::UNIFORM_BUFFER.0) != 0 {
            bits |= VK_ACCESS_UNIFORM_READ_BIT;
        }
        if (self.0 & Self::STORAGE_BUFFER.0) != 0 {
            bits |= VK_ACCESS_UNIFORM_READ_BIT;
        }
        if (self.0 & Self::INDEX_BUFFER.0) != 0 {
            bits |= VK_ACCESS_INDEX_READ_BIT;
        }
        if (self.0 & Self::VERTEX_BUFFER.0) != 0 {
            bits |= VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT;
        }
        if (self.0 & Self::INDIRECT_BUFFER.0) != 0 {
            bits |= VK_ACCESS_INDIRECT_COMMAND_READ_BIT;
        }
        bits
    }
    /// Determines if flag contains usage of uniform-buffer
    pub fn is_uniform(self) -> bool {
        (self.0 & (Self::UNIFORM_BUFFER.0 | Self::UNIFORM_TEXEL_BUFFER.0)) != 0
    }
    /// Determines if flag contains usage of storage-buffer
    pub fn is_storage(self) -> bool {
        (self.0 & (Self::STORAGE_BUFFER.0 | Self::STORAGE_TEXEL_BUFFER.0)) != 0
    }
}
impl BitOr for BufferUsage {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        BufferUsage(self.0 | other.0)
    }
}
impl BitOrAssign for BufferUsage {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// Bitset specifying additional parameters of a buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum BufferSparseBinding {
    /// No sparse binding features
    None = 0,
    /// the buffer will be backed using sparse memory binding
    Bound = VK_BUFFER_CREATE_SPARSE_BINDING_BIT as _,
    /// the buffer can be partially backed using sparse memory binding.
    Residency = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT | VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT) as _,
    /// the buffer will be backed using sparse memory binding with memory ranges
    /// that might also simultaneously be backing another buffer (or another portion of the same buffer)
    Aliased = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT | VK_BUFFER_CREATE_SPARSE_ALIASED_BIT) as _,
    /// Aliased and Residency
    Both = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT
        | VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT
        | VK_BUFFER_CREATE_SPARSE_ALIASED_BIT) as _,
}
/// Builder structure specifying the parameters of a newly created buffer object
#[repr(transparent)]
#[derive(Clone)]
pub struct BufferDesc(VkBufferCreateInfo);
impl BufferDesc {
    pub fn new(byte_size: usize, usage: BufferUsage) -> Self {
        Self(VkBufferCreateInfo {
            sType: VkBufferCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            size: byte_size as _,
            usage: usage.0,
            sharingMode: VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: std::ptr::null(),
        })
    }

    /// A list of queue families that will access this buffer
    pub fn sharing_queue_families(&mut self, indices: &[u32]) -> &mut Self {
        self.0.sharingMode = if indices.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
        self.0.queueFamilyIndexCount = indices.len() as _;
        self.0.pQueueFamilyIndices = indices.as_ptr();
        self
    }

    /// A bitmask of `BufferSparseBinding` specifying additional parameters of the buffer
    pub fn sparse_binding_opt(&mut self, opt: BufferSparseBinding) -> &mut Self {
        self.0.flags = opt as _;
        self
    }

    /// Create a new buffer object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    pub fn create<Device: crate::Device>(self, device: Device) -> crate::Result<BufferObject<Device>> {
        device.new_buffer(self)
    }
}
impl crate::VulkanStructureProvider for BufferDesc {
    type RootStructure = VkBufferCreateInfo;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut Self::RootStructure) -> &'r mut crate::GenericVulkanStructure {
        *root = self.0.clone();
        root.as_generic_mut()
    }
}

/// Bitmask specifying intended usage of an image
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ImageUsage(pub VkImageUsageFlags);
impl ImageUsage {
    /// The image can be used as the source of a transfer command
    pub const TRANSFER_SRC: Self = ImageUsage(VK_IMAGE_USAGE_TRANSFER_SRC_BIT);
    /// The image can be used as the destination of a transfer command
    pub const TRANSFER_DEST: Self = ImageUsage(VK_IMAGE_USAGE_TRANSFER_DST_BIT);
    /// The image can be used to create `ImageView` suitable for occupying a `DescriptorSet` slot
    /// either of type `DescriptorType::SampledImage` or `DescriptorType::CombinedImageSampler`, and be sampled by a shader
    pub const SAMPLED: Self = ImageUsage(VK_IMAGE_USAGE_SAMPLED_BIT);
    /// The image can be used to create a `ImageView` suitable for occupying a `DescriptorSet` slot of type `DescriptorType::StorageImage`
    pub const STORAGE: Self = ImageUsage(VK_IMAGE_USAGE_STORAGE_BIT);
    /// The image can be used to create a `ImageView` suitable for use as a color or resolve attachment in a `Framebuffer`
    pub const COLOR_ATTACHMENT: Self = ImageUsage(VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT);
    /// The image can be used to create a `ImageView` suitable for use as a depth/stencil attachment in a `Framebuffer`
    pub const DEPTH_STENCIL_ATTACHMENT: Self = ImageUsage(VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT);
    /// The memory bound to this image will have been allocated with the `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
    /// This bit can be set for any image that can be used to create a `ImageView` suitable for use as a color, resolve, depth/stencil,
    /// or input attachment
    pub const TRANSIENT_ATTACHMENT: Self = ImageUsage(VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT);
    /// The image can be used to create a `ImageView` suitable for occupying `DescriptorSet` slot of type `DescriptorType::InputAttachment`;
    /// be read from a shader as an input attachment; and be used as an input attachment in a framebuffer
    pub const INPUT_ATTACHMENT: Self = ImageUsage(VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT);

    /// The image can be used as the source of a transfer command
    pub fn transfer_src(self) -> Self {
        ImageUsage(self.0 | Self::TRANSFER_SRC.0)
    }
    /// The image can be used as the destination of a transfer command
    pub fn transfer_dest(self) -> Self {
        ImageUsage(self.0 | Self::TRANSFER_DEST.0)
    }
    /// The image can be used to create `ImageView` suitable for occupying a `DescriptorSet` slot
    /// either of type `DescriptorType::SampledImage` or `DescriptorType::CombinedImageSampler`, and be sampled by a shader
    pub fn sampled(self) -> Self {
        ImageUsage(self.0 | Self::SAMPLED.0)
    }
    /// The image can be used to create a `ImageView` suitable for occupying a `DescriptorSet` slot of type `DescriptorType::StorageImage`
    pub fn storage(self) -> Self {
        ImageUsage(self.0 | Self::STORAGE.0)
    }
    /// The image can be used to create a `ImageView` suitable for use as a color or resolve attachment in a `Framebuffer`
    pub fn color_attachment(self) -> Self {
        ImageUsage(self.0 | Self::COLOR_ATTACHMENT.0)
    }
    /// The image can be used to create a `ImageView` suitable for use as a depth/stencil attachment in a `Framebuffer`
    pub fn depth_stencil_attachment(self) -> Self {
        ImageUsage(self.0 | Self::DEPTH_STENCIL_ATTACHMENT.0)
    }
    /// The memory bound to this image will have been allocated with the `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
    /// This bit can be set for any image that can be used to create a `ImageView` suitable for use as a color, resolve, depth/stencil,
    /// or input attachment
    pub fn transient_attachment(self) -> Self {
        ImageUsage(self.0 | Self::TRANSIENT_ATTACHMENT.0)
    }
    /// The image can be used to create a `ImageView` suitable for occupying `DescriptorSet` slot of type `DescriptorType::InputAttachment`;
    /// be read from a shader as an input attachment; and be used as an input attachment in a framebuffer
    pub fn input_attachment(self) -> Self {
        ImageUsage(self.0 | Self::INPUT_ATTACHMENT.0)
    }
}
impl BitOr for ImageUsage {
    type Output = ImageUsage;
    fn bitor(self, other: Self) -> Self {
        ImageUsage(self.0 | other.0)
    }
}
impl BitOrAssign for ImageUsage {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// Bitmask specifying additional parameters of an image
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ImageFlags(pub VkImageCreateFlags);
impl ImageFlags {
    /// Empty bits
    pub const EMPTY: Self = ImageFlags(0);
    /// The image will be backed using sparse memory binding
    pub const SPARSE_BINDING: Self = ImageFlags(VK_IMAGE_CREATE_SPARSE_BINDING_BIT);
    /// The image can be partially backed using sparse memory binding. This bit is with `SPARSE_BINDING` implicitly
    pub const SPARSE_RESIDENCY: Self =
        ImageFlags(VK_IMAGE_CREATE_SPARSE_BINDING_BIT | VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT);
    /// The image will be backed using sparse memory binding with memory ranges
    /// that might also simultaneously be backing another image. This bit is with `SPARSE_BINDING` implicitly
    pub const SPARSE_ALIASED: Self =
        ImageFlags(VK_IMAGE_CREATE_SPARSE_BINDING_BIT | VK_IMAGE_CREATE_SPARSE_ALIASED_BIT);
    /// The image can be used to create a `ImageView` with a different format from the image
    pub const MUTABLE_FORMAT: Self = ImageFlags(VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT);
    /// The image can be used to create a `ImageView` of type `ImageViewType::Cube` or `ImageViewType::CubeArray`
    pub const CUBE_COMPATIBLE: Self = ImageFlags(VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT);

    /// The image will be backed using sparse memory binding
    pub fn sparse_binding(self) -> Self {
        ImageFlags(self.0 | Self::SPARSE_BINDING.0)
    }
    /// The image can be partially backed using sparse memory binding. This bit is with `SPARSE_BINDING` implicitly
    pub fn sparse_residency(self) -> Self {
        ImageFlags(self.0 | Self::SPARSE_RESIDENCY.0)
    }
    /// The image will be backed using sparse memory binding with memory ranges
    /// that might also simultaneously be backing another image. This bit is with `SPARSE_BINDING` implicitly
    pub fn sparse_aliased(self) -> Self {
        ImageFlags(self.0 | Self::SPARSE_ALIASED.0)
    }
    /// The image can be used to create a `ImageView` with a different format from the image
    pub fn mutable_format(self) -> Self {
        ImageFlags(self.0 | Self::MUTABLE_FORMAT.0)
    }
    /// The image can be used to create a `ImageView` of type `ImageViewType::Cube` or `ImageViewType::CubeArray`
    pub fn cube_compatible(self) -> Self {
        ImageFlags(self.0 | Self::CUBE_COMPATIBLE.0)
    }
}
impl BitOr for ImageFlags {
    type Output = ImageFlags;
    fn bitor(self, other: Self) -> Self {
        ImageFlags(self.0 | other.0)
    }
}
impl BitOrAssign for ImageFlags {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// Builder structure specifying the parameters of a newly created image object
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ImageDesc<'d>(
    pub(crate) VkImageCreateInfo,
    std::marker::PhantomData<Option<&'d dyn std::any::Any>>,
);
impl<'d> ImageDesc<'d> {
    pub fn new<Size: ImageSize>(size: &Size, format: VkFormat, usage: ImageUsage, initial_layout: ImageLayout) -> Self {
        ImageDesc(
            VkImageCreateInfo {
                sType: VkImageCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                imageType: Size::DIMENSION,
                extent: size.conv(),
                format,
                usage: usage.0,
                mipLevels: 1,
                arrayLayers: 1,
                samples: 1,
                initialLayout: initial_layout as _,
                tiling: VK_IMAGE_TILING_OPTIMAL,
                sharingMode: VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: std::ptr::null(),
            },
            std::marker::PhantomData,
        )
    }

    /// A list of queue families that will access this image,
    /// or an empty list if no queue families can access this image simultaneously
    pub fn sharing_queue_families(&mut self, indices: &[u32]) -> &mut Self {
        self.0.sharingMode = if indices.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
        self.0.queueFamilyIndexCount = indices.len() as _;
        self.0.pQueueFamilyIndices = indices.as_ptr();
        self
    }

    /// The number of sub-data element samples in the image  
    /// bitmask of 1(default), 2, 4, 8, 16, 32, 64
    pub fn sample_counts(&mut self, count_bits: u32) -> &mut Self {
        self.0.samples = count_bits;
        self
    }

    /// Sets the tiling arrangement of the data elements in memory as "linear tiling"  
    /// default: optimal tiling
    pub fn use_linear_tiling(&mut self) -> &mut Self {
        self.0.tiling = VK_IMAGE_TILING_LINEAR;
        self
    }

    /// A bitmask of `ImageFlags`describing additional parameters of the image  
    /// default: none
    pub fn flags(&mut self, opt: ImageFlags) -> &mut Self {
        self.0.flags = opt.0;
        self
    }

    /// The number of layers in the image  
    /// default: 1
    pub fn array_layers(&mut self, layers: u32) -> &mut Self {
        self.0.arrayLayers = layers;
        self
    }

    /// The number of levels of detail available for minified sampling of the image  
    /// default: 1
    pub fn mip_levels(&mut self, levels: u32) -> &mut Self {
        self.0.mipLevels = levels;
        self
    }
}
impl AsRef<VkImageCreateInfo> for ImageDesc<'_> {
    fn as_ref(&self) -> &VkImageCreateInfo {
        &self.0
    }
}

#[cfg(feature = "Implements")]
impl ImageDesc<'_> {
    /// Create an image
    pub fn create<Device: crate::Device>(&self, device: Device) -> crate::Result<ImageObject<Device>> {
        let mut h = VK_NULL_HANDLE as _;
        unsafe { Resolver::get().create_image(device.native_ptr(), &self.0, std::ptr::null(), &mut h) }
            .into_result()
            .map(|_| ImageObject(h, device, self.0.imageType, self.0.format, self.0.extent.clone()))
    }
}

pub trait Image: VkHandle<Handle = VkImage> + DeviceChild {
    /// The pixel format of an image
    fn format(&self) -> VkFormat;

    /// The size of an image
    fn size(&self) -> &VkExtent3D;

    fn dimension(&self) -> VkImageViewType;

    /// Create an image view
    #[cfg(feature = "Implements")]
    fn create_view(
        self,
        format: Option<VkFormat>,
        vtype: Option<VkImageViewType>,
        cmap: &ComponentMapping,
        subresource_range: &ImageSubresourceRange,
    ) -> crate::Result<ImageViewObject<Self>>
    where
        Self: Sized,
    {
        let (format, vtype) = (
            format.unwrap_or_else(|| self.format()),
            vtype.unwrap_or_else(|| self.dimension()),
        );
        let cinfo = VkImageViewCreateInfo {
            sType: VkImageViewCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            image: self.native_ptr(),
            viewType: vtype,
            format,
            components: cmap.clone().into(),
            subresourceRange: subresource_range.0.clone(),
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe { Resolver::get().create_image_view(self.device().native_ptr(), &cinfo, std::ptr::null(), &mut h) }
            .into_result()
            .map(|_| ImageViewObject(h, self))
    }

    /// Retrieve information about an image subresource  
    /// Subresource: (`aspect`, `mipLevel`, `arrayLayer`)
    #[cfg(feature = "Implements")]
    fn image_subresource_layout(
        &self,
        subres_aspect: AspectMask,
        subres_mip_level: u32,
        subres_array_layer: u32,
    ) -> VkSubresourceLayout {
        let subres = VkImageSubresource {
            aspectMask: subres_aspect.0,
            mipLevel: subres_mip_level,
            arrayLayer: subres_array_layer,
        };
        let mut s = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_image_subresource_layout(
                self.device().native_ptr(),
                self.native_ptr(),
                &subres,
                s.as_mut_ptr(),
            );

            s.assume_init()
        }
    }

    /// Query the memory requirements for a sparse image
    #[cfg(feature = "Implements")]
    fn sparse_requirements(&self) -> Vec<VkSparseImageMemoryRequirements> {
        let mut n = 0;
        unsafe {
            Resolver::get().get_image_sparse_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                &mut n,
                std::ptr::null_mut(),
            );
        };
        let mut v = Vec::with_capacity(n as _);
        unsafe {
            v.set_len(n as _);
            Resolver::get().get_image_sparse_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                &mut n,
                v.as_mut_ptr(),
            )
        };

        v
    }
}
DerefContainerBracketImpl!(for Image {
    fn format(&self) -> VkFormat {
        T::format(self)
    }

    fn size(&self) -> &VkExtent3D {
        T::size(self)
    }

    fn dimension(&self) -> VkImageViewType {
        T::dimension(self)
    }
});

pub trait ImageChild {
    type ConcreteImage: crate::Image;

    fn image(&self) -> &Self::ConcreteImage;
}
DerefContainerBracketImpl!(for ImageChild {
    type ConcreteImage = T::ConcreteImage;

    fn image(&self) -> &Self::ConcreteImage { T::image(self) }
});

pub trait Buffer: VkHandle<Handle = VkBuffer> + DeviceChild {
    /// Create a buffer view
    #[cfg(feature = "Implements")]
    fn create_view(self, format: VkFormat, range: Range<u64>) -> crate::Result<BufferViewObject<Self>>
    where
        Self: Sized,
    {
        let cinfo = VkBufferViewCreateInfo {
            sType: VkBufferViewCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            buffer: self.native_ptr(),
            format,
            offset: range.start,
            range: range.end - range.start,
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe { Resolver::get().create_buffer_view(self.device().native_ptr(), &cinfo, std::ptr::null(), &mut h) }
            .into_result()
            .map(|_| BufferViewObject(h, self))
    }
}
DerefContainerBracketImpl!(for Buffer {});

pub trait DeviceMemory: VkHandle<Handle = VkDeviceMemory> + DeviceChild {
    /// Map a memory object into application address space
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_MEMORY_MAP_FAILED`
    #[cfg(feature = "Implements")]
    fn map(&mut self, range: Range<usize>) -> crate::Result<MappedMemoryRange<Self>> {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get()
                .map_memory(
                    self.device().native_ptr(),
                    self.native_ptr(),
                    range.start as _,
                    (range.end - range.start) as _,
                    0,
                    p.as_mut_ptr(),
                )
                .into_result()
                .map(move |_| MappedMemoryRange(range, p.assume_init() as *mut _, self))
        }
    }

    /// Unmap a previously mapped memory object
    /// # Safety
    /// Caller must guarantee that there is no `MappedMemoryRange` alives.  
    /// Accessing the mapped memory after this call has undefined behavior
    #[cfg(feature = "Implements")]
    unsafe fn unmap(&mut self) {
        Resolver::get().unmap_memory(self.device().native_ptr(), self.native_ptr());
    }

    /// Query the current commitment for a `DeviceMemory`
    #[cfg(feature = "Implements")]
    fn commitment_bytes(&self) -> VkDeviceSize {
        let mut b = 0;
        unsafe {
            Resolver::get().get_device_memory_commitment(self.device().native_ptr(), self.native_ptr(), &mut b);
        }

        b
    }
}
DerefContainerBracketImpl!(for DeviceMemory {});

/// Common operations for memory bound objects
pub trait MemoryBound {
    /// Returns the memory requirements for specified Vulkan object
    #[cfg(feature = "Implements")]
    fn requirements(&self) -> VkMemoryRequirements;

    /// Bind device memory to the object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()>;
}

pub trait BufferView: VkHandle<Handle = VkBufferView> + DeviceChild {}
DerefContainerBracketImpl!(for BufferView {});

pub trait ImageView: VkHandle<Handle = VkImageView> + DeviceChild {}
DerefContainerBracketImpl!(for ImageView {});

/// Image Dimension by corresponding extent type
pub trait ImageSize {
    const DIMENSION: VkImageType;

    fn conv(&self) -> VkExtent3D;
}
impl ImageSize for u32 {
    const DIMENSION: VkImageType = VK_IMAGE_TYPE_1D;

    fn conv(&self) -> VkExtent3D {
        VkExtent3D {
            width: *self,
            height: 1,
            depth: 1,
        }
    }
}
impl ImageSize for VkExtent2D {
    const DIMENSION: VkImageType = VK_IMAGE_TYPE_2D;

    fn conv(&self) -> VkExtent3D {
        self.clone().with_depth(1)
    }
}
impl ImageSize for VkExtent3D {
    const DIMENSION: VkImageType = VK_IMAGE_TYPE_3D;

    fn conv(&self) -> VkExtent3D {
        self.clone()
    }
}

/// Specifies the block of mapped memory in a `DeviceMemory`
pub struct MappedMemoryRange<'m, DeviceMemory: crate::DeviceMemory + DeviceChild + ?Sized + 'm>(
    std::ops::Range<usize>,
    *mut u8,
    &'m mut DeviceMemory,
);
#[allow(clippy::mut_from_ref)]
impl<'m, DeviceMemory: crate::DeviceMemory + DeviceChild + ?Sized + 'm> MappedMemoryRange<'m, DeviceMemory> {
    /// Get a reference in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn get<T>(&self, offset: usize) -> &T {
        &*(self.1.add(offset) as *const T)
    }
    /// Get a mutable reference in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn get_mut<T>(&self, offset: usize) -> &mut T {
        &mut *(self.1.add(offset) as *mut T)
    }
    /// Get a slice in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn slice<T>(&self, offset: usize, count: usize) -> &[T] {
        std::slice::from_raw_parts(self.1.add(offset) as *const T, count)
    }
    /// Get a mutable slice in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn slice_mut<T>(&self, offset: usize, count: usize) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.1.add(offset) as *mut T, count)
    }
    /// Clone data from slice at the specified offset in mapped memory.
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn clone_from_slice_at<T: Clone>(&self, offset: usize, src: &[T]) {
        self.slice_mut(offset, src.len()).clone_from_slice(src);
    }
    /// Clone data from slice at the specified offset in mapped memory.
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn clone_at<T: Clone>(&self, offset: usize, src: &T) {
        *self.get_mut(offset) = src.clone();
    }

    #[cfg(feature = "Implements")]
    /// [feature = "Implements"] Unmap region
    pub fn end(self) {
        unsafe {
            self.2.unmap();
        }
    }
}

/// Layouts of image and image subresources
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ImageLayout {
    /// does not support device access
    Undefined = VK_IMAGE_LAYOUT_UNDEFINED as _,
    /// does not support device access. host can be written to this memory immediately
    Preinitialized = VK_IMAGE_LAYOUT_PREINITIALIZED as _,
    /// supports all types of device access
    General = VK_IMAGE_LAYOUT_GENERAL as _,
    /// must only be used as a color or resolve attachment in a `Framebuffer`
    ColorAttachmentOpt = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL as _,
    /// must only be used as a depth/stencil attachment in a `Framebuffer`
    DepthStencilAttachmentOpt = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL as _,
    /// must only be used as a read-only depth/stencil attachment in a `Framebuffer`
    /// and/or as a read-only image in a shader (which can be read as a sampled image,
    /// combined image/sampler and/or input attachment).
    DepthStencilReadOnlyOpt = VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL as _,
    /// must only be used as a read-only image in a shader (which can be read as a sampled image,
    /// combined image/sampler and/or input attachment).
    ShaderReadOnlyOpt = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL as _,
    /// must only be used as a source image of a transfer command
    TransferSrcOpt = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL as _,
    /// must only be used as a destination image of a transfer command
    TransferDestOpt = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL as _,
    /// must only be used for presenting a swapchain image for display
    #[cfg(feature = "VK_KHR_swapchain")]
    PresentSrc = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR as _,
}
impl ImageLayout {
    /// Commonly used access types with the layout
    pub fn default_access_mask(self) -> VkAccessFlags {
        match self {
            Self::Undefined | Self::Preinitialized => 0,
            Self::General => VK_ACCESS_MEMORY_READ_BIT,
            Self::ColorAttachmentOpt => VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            Self::DepthStencilAttachmentOpt => VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
            Self::DepthStencilReadOnlyOpt => VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
            Self::ShaderReadOnlyOpt => VK_ACCESS_SHADER_READ_BIT,
            Self::TransferSrcOpt => VK_ACCESS_TRANSFER_READ_BIT,
            Self::TransferDestOpt => VK_ACCESS_TRANSFER_WRITE_BIT,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::PresentSrc => VK_ACCESS_MEMORY_READ_BIT,
        }
    }
}

/// Specify how a component is swizzled
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentSwizzle {
    /// the component is set to the identity swizzle
    Identity = VK_COMPONENT_SWIZZLE_IDENTITY as _,
    /// the component is set to zero
    Zero = VK_COMPONENT_SWIZZLE_ZERO as _,
    /// the component is set to either 1 or 1.0, depending on whether
    /// the type of the image view format is integer of floating-pointer respectively
    One = VK_COMPONENT_SWIZZLE_ONE as _,
    /// the component is set to the value of the R component of the image
    R = VK_COMPONENT_SWIZZLE_R as _,
    /// the component is set to the value of the G component of the image
    G = VK_COMPONENT_SWIZZLE_G as _,
    /// the component is set to the value of the B component of the image
    B = VK_COMPONENT_SWIZZLE_B as _,
    /// the component is set to the value of the A component of the image
    A = VK_COMPONENT_SWIZZLE_A as _,
}

/// Structure specifying a color component mapping
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentMapping(
    pub ComponentSwizzle,
    pub ComponentSwizzle,
    pub ComponentSwizzle,
    pub ComponentSwizzle,
);
impl Into<VkComponentMapping> for ComponentMapping {
    fn into(self) -> VkComponentMapping {
        VkComponentMapping {
            r: self.0 as _,
            g: self.1 as _,
            b: self.2 as _,
            a: self.3 as _,
        }
    }
}
impl Default for ComponentMapping {
    fn default() -> Self {
        Self::all(ComponentSwizzle::Identity)
    }
}
impl ComponentMapping {
    /// Set same value to all component
    pub const fn all(s: ComponentSwizzle) -> Self {
        ComponentMapping(s, s, s, s)
    }
    /// Set 2 values with repeating
    pub const fn set2(a: ComponentSwizzle, b: ComponentSwizzle) -> Self {
        ComponentMapping(a, b, a, b)
    }
}

/// Bitmask specifying which aspects of an image are included in a view
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(transparent)]
pub struct AspectMask(pub VkImageAspectFlags);
impl AspectMask {
    /// The color aspect
    pub const COLOR: Self = AspectMask(VK_IMAGE_ASPECT_COLOR_BIT);
    /// The depth aspect
    pub const DEPTH: Self = AspectMask(VK_IMAGE_ASPECT_DEPTH_BIT);
    /// The stencil aspect
    pub const STENCIL: Self = AspectMask(VK_IMAGE_ASPECT_STENCIL_BIT);
    /// The metadata aspect, used for sparse sparse resource operations
    pub const METADATA: Self = AspectMask(VK_IMAGE_ASPECT_METADATA_BIT);

    /// The color aspect
    pub fn color(self) -> Self {
        AspectMask(self.0 | Self::COLOR.0)
    }
    /// The depth aspect
    pub fn depth(self) -> Self {
        AspectMask(self.0 | Self::DEPTH.0)
    }
    /// The stencil aspect
    pub fn stencil(self) -> Self {
        AspectMask(self.0 | Self::STENCIL.0)
    }
    /// The metadata aspect, used for sparse sparse resource oeprations
    pub fn metadata(self) -> Self {
        AspectMask(self.0 | Self::METADATA.0)
    }
}
impl BitOr for AspectMask {
    type Output = AspectMask;
    fn bitor(self, other: Self) -> Self {
        AspectMask(self.0 | other.0)
    }
}
impl BitOrAssign for AspectMask {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// Structure specifying a image subresource range
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ImageSubresourceRange(VkImageSubresourceRange);
impl From<VkImageSubresourceRange> for ImageSubresourceRange {
    fn from(v: VkImageSubresourceRange) -> Self {
        ImageSubresourceRange(v)
    }
}
impl Into<VkImageSubresourceRange> for ImageSubresourceRange {
    fn into(self) -> VkImageSubresourceRange {
        self.0
    }
}
impl AsRef<VkImageSubresourceRange> for ImageSubresourceRange {
    fn as_ref(&self) -> &VkImageSubresourceRange {
        &self.0
    }
}
impl ImageSubresourceRange {
    /// Specify color subresource
    pub fn color(mip_levels: impl AnalogNumRange<u32>, array_layers: impl AnalogNumRange<u32>) -> Self {
        VkImageSubresourceRange {
            aspectMask: AspectMask::COLOR.0,
            baseMipLevel: mip_levels.begin(),
            baseArrayLayer: array_layers.begin(),
            levelCount: mip_levels.count(),
            layerCount: array_layers.count(),
        }
        .into()
    }

    /// Specify stencil subresource
    pub fn stencil(mip_levels: impl AnalogNumRange<u32>, array_layers: impl AnalogNumRange<u32>) -> Self {
        VkImageSubresourceRange {
            aspectMask: AspectMask::STENCIL.0,
            baseMipLevel: mip_levels.begin(),
            baseArrayLayer: array_layers.begin(),
            levelCount: mip_levels.count(),
            layerCount: array_layers.count(),
        }
        .into()
    }

    /// Specify depth subresource
    pub fn depth(mip_levels: impl AnalogNumRange<u32>, array_layers: impl AnalogNumRange<u32>) -> Self {
        VkImageSubresourceRange {
            aspectMask: AspectMask::DEPTH.0,
            baseMipLevel: mip_levels.begin(),
            baseArrayLayer: array_layers.begin(),
            levelCount: mip_levels.count(),
            layerCount: array_layers.count(),
        }
        .into()
    }

    /// Specify depth and stencil subresource
    pub fn depth_stencil(mip_levels: impl AnalogNumRange<u32>, array_layers: impl AnalogNumRange<u32>) -> Self {
        VkImageSubresourceRange {
            aspectMask: AspectMask::DEPTH.stencil().0,
            baseMipLevel: mip_levels.begin(),
            baseArrayLayer: array_layers.begin(),
            levelCount: mip_levels.count(),
            layerCount: array_layers.count(),
        }
        .into()
    }
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a sampler object
    SamplerObject(VkSampler, VK_OBJECT_TYPE_SAMPLER): Sampler { drop destroy_sampler }
}

pub trait Sampler: VkHandle<Handle = VkSampler> + DeviceChild {}
DerefContainerBracketImpl!(for Sampler {});

/// Specify behavior of sampling with texture coordinates outside an image
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum AddressingMode {
    /// The repeat wrap mode
    Repeat = VK_SAMPLER_ADDRESS_MODE_REPEAT as _,
    /// The mirrored repeat wrap mode
    MirroredRepeat = VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT as _,
    /// The clamp to edge wrap mode
    ClampToEdge = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE as _,
    /// The clamp to border wrap mode
    ClampToBorder = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER as _,
    /// The mirror clamp to edge wrap mode
    #[cfg(feature = "VK_KHR_mirror_clamp_to_edge")]
    MirrorClampToEdge = VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE as _,
}

/// Specify filter used for texture lookups
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum FilterMode {
    /// Nearest filtering
    Nearest = VK_FILTER_NEAREST as _,
    /// Linear filtering
    Linear = VK_FILTER_LINEAR as _,
}

/// Specify mipmap mode used for texture lookups
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MipmapFilterMode {
    /// Nearest filtering
    Nearest = VK_SAMPLER_MIPMAP_MODE_NEAREST as _,
    /// Linear filtering
    Linear = VK_SAMPLER_MIPMAP_MODE_LINEAR as _,
}

/// Specify border color used for texture lookups
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BorderColor {
    /// A transparent, floating-point format, black color
    TransparentBlackF = VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK as _,
    /// A transparent, integer format, black color
    TransparentBlackI = VK_BORDER_COLOR_INT_TRANSPARENT_BLACK as _,
    /// An opaque, floating-point format, black color
    OpaqueBlackF = VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK as _,
    /// An opaque, integer format, black color
    OpaqueBlackI = VK_BORDER_COLOR_INT_OPAQUE_BLACK as _,
    /// An opaque, floating-point format, white color
    OpaqueWhiteF = VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE as _,
    /// An opaque, integer format, white color
    OpaqueWhiteI = VK_BORDER_COLOR_INT_OPAQUE_WHITE as _,
}

/// Builder object for constructing the sampler object
#[repr(transparent)]
pub struct SamplerBuilder(VkSamplerCreateInfo);
/// A default sampler builder: Linear Filtering, Repeat addressing, no anisotrophy and no lod biases
impl Default for SamplerBuilder {
    fn default() -> Self {
        Self(VkSamplerCreateInfo {
            sType: VkSamplerCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            magFilter: FilterMode::Linear as _,
            minFilter: FilterMode::Linear as _,
            mipmapMode: MipmapFilterMode::Linear as _,
            addressModeU: AddressingMode::Repeat as _,
            addressModeV: AddressingMode::Repeat as _,
            addressModeW: AddressingMode::Repeat as _,
            mipLodBias: 0.0,
            anisotropyEnable: false as _,
            compareEnable: false as _,
            compareOp: CompareOp::Always as _,
            minLod: 0.0,
            maxLod: 0.0,
            borderColor: BorderColor::TransparentBlackF as _,
            unnormalizedCoordinates: false as _,
            maxAnisotropy: 1.0,
        })
    }
}
impl Into<VkSamplerCreateInfo> for SamplerBuilder {
    fn into(self) -> VkSamplerCreateInfo {
        self.0
    }
}
impl SamplerBuilder {
    /// The magnification and the minification filters to apply to lookups.  
    /// Default: Magnification=`FilterMode::Linear`, Minification=`FilterMode::Linear`
    pub fn filter(&mut self, mag: FilterMode, min: FilterMode) -> &mut Self {
        self.0.magFilter = mag as _;
        self.0.minFilter = min as _;
        self
    }

    /// The mipmap filter to apply to lookups.  
    /// Default: `MipmapFilterMode::Linear`
    pub fn mip_filter(&mut self, f: MipmapFilterMode) -> &mut Self {
        self.0.mipmapMode = f as _;
        self
    }

    /// The addressing mode for outside [0..1] range for U, V and W coordinates.  
    /// Default: U=`AddressingMode::Repeat`, V=`AddressinMode::Repeat`, W=`AddressingMode::Repeat`
    pub fn addressing(&mut self, u: AddressingMode, v: AddressingMode, w: AddressingMode) -> &mut Self {
        self.0.addressModeU = u as _;
        self.0.addressModeV = v as _;
        self.0.addressModeW = w as _;
        self
    }

    /// The bias to be added to mipmap LOD calculation and bias provided by image sampling functions in SPIR-V,
    /// as described in the `Level-of-Detail Operation` section in Vulkan Specification.  
    /// Default: 0.0
    pub fn lod_bias(&mut self, bias: f32) -> &mut Self {
        self.0.mipLodBias = bias;
        self
    }

    /// The anisotropy value clamp. Specifying `None` switches off the anisotropic filtering  
    /// Default: `None`
    pub fn max_anisotropy(&mut self, level: Option<f32>) -> &mut Self {
        self.0.anisotropyEnable = level.is_some() as _;
        self.0.maxAnisotropy = level.unwrap_or_default();
        self
    }

    /// The comparison function to apply to fetched data before filtering
    /// as described in the `Depth Compare Operation` section in Vulkan Specification.
    /// Specifying `None` switches off the comparison against a reference value during lookups.  
    /// Default: `None`
    pub fn comparison(&mut self, op: Option<CompareOp>) -> &mut Self {
        self.0.compareEnable = op.is_some() as _;
        self.0.compareOp = op.unwrap_or(CompareOp::Always) as _;
        self
    }

    /// The values used to clamp the computed level-of-detail value,
    /// as described in the `Level-of-Detail Operation` section in Vulkan Specification.  
    /// Default: min_lod=0.0, max_lod=0.0
    /// # Panics
    /// `max_lod` must be greater than or equal to `min_lod`
    pub fn lod_clamp(&mut self, min_lod: f32, max_lod: f32) -> &mut Self {
        assert!(max_lod >= min_lod);
        self.0.minLod = min_lod;
        self.0.maxLod = max_lod;
        self
    }

    /// Whether to use unnormalized or normalized texel coordinates to address texels of the image.  
    /// Default: `false`
    /// # Safety
    /// User must meet the constraints as described in the "Valid Usage" section in the `VkSamplerCreateInfo` manual page
    pub unsafe fn unnormalized_coordinates(&mut self, use_unnormalized: bool) -> &mut Self {
        self.0.unnormalizedCoordinates = use_unnormalized as _;
        self
    }

    /// Create a new sampler object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[cfg(feature = "Implements")]
    pub fn create<Device: crate::Device>(&self, device: Device) -> crate::Result<SamplerObject<Device>> {
        let mut h = VK_NULL_HANDLE as _;
        unsafe { Resolver::get().create_sampler(device.native_ptr(), &self.0, std::ptr::null(), &mut h) }
            .into_result()
            .map(|_| SamplerObject(h, device))
    }
}
