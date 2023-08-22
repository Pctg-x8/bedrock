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

use cfg_if::cfg_if;
use derives::implements;

use crate::{vk::*, VkHandle, VkHandleMut};

mod memory;
pub use self::memory::*;

mod buffer;
pub use self::buffer::*;

mod image;
pub use self::image::*;

mod sampler;
pub use self::sampler::*;

cfg_if! {
    if #[cfg(feature = "VK_KHR_swapchain")] {
        mod swapchain_image;
        pub use self::swapchain_image::*;
    }
}

/// Common operations for memory bound objects
pub trait MemoryBound: VkHandle {
    /// Returns the memory requirements for specified Vulkan object
    #[implements]
    fn requirements(&self) -> VkMemoryRequirements;

    /// Bind device memory to the object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()>
    where
        Self: VkHandleMut;
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
        Self::IDENTITY
    }
}
impl ComponentMapping {
    pub const IDENTITY: Self = Self::all(ComponentSwizzle::Identity);
    pub const ZERO: Self = Self::all(ComponentSwizzle::Zero);
    pub const ONE: Self = Self::all(ComponentSwizzle::One);

    /// Set same value to all component
    pub const fn all(s: ComponentSwizzle) -> Self {
        ComponentMapping(s, s, s, s)
    }

    /// Set 2 values with repeating
    pub const fn set2(a: ComponentSwizzle, b: ComponentSwizzle) -> Self {
        ComponentMapping(a, b, a, b)
    }
}
