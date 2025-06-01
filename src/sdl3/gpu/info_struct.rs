//! Types which hold data but don't own any GPU-resources.
//! There are no calls to SDL here.
//!

use std::marker::PhantomData;

use sys::gpu::{
    SDL_GPUBlendFactor, SDL_GPUBlendOp, SDL_GPUBufferBinding, SDL_GPUBufferLocation,
    SDL_GPUBufferRegion, SDL_GPUColorTargetBlendState, SDL_GPUColorTargetDescription,
    SDL_GPUColorTargetInfo, SDL_GPUCompareOp, SDL_GPUCullMode, SDL_GPUDepthStencilState,
    SDL_GPUDepthStencilTargetInfo, SDL_GPUFillMode, SDL_GPUFilter, SDL_GPUFrontFace,
    SDL_GPUGraphicsPipelineTargetInfo, SDL_GPUIndirectDispatchCommand, SDL_GPURasterizerState,
    SDL_GPUSampleCount, SDL_GPUSamplerAddressMode, SDL_GPUSamplerCreateInfo,
    SDL_GPUSamplerMipmapMode, SDL_GPUStencilOp, SDL_GPUStencilOpState,
    SDL_GPUStorageBufferReadWriteBinding, SDL_GPUStorageTextureReadWriteBinding,
    SDL_GPUTextureCreateInfo, SDL_GPUTextureLocation, SDL_GPUTextureRegion,
    SDL_GPUTextureSamplerBinding, SDL_GPUTextureTransferInfo, SDL_GPUTextureType,
    SDL_GPUTransferBufferLocation, SDL_GPUVertexAttribute, SDL_GPUVertexBufferDescription,
    SDL_GPUVertexInputRate, SDL_GPUVertexInputState,
};

use crate::pixels::Color;

use super::{
    BlendFactor, BlendOp, Buffer, ColorComponentFlags, CompareOp, CullMode, FillMode, Filter,
    FrontFace, LoadOp, SampleCount, Sampler, SamplerAddressMode, SamplerMipmapMode, StencilOp,
    StoreOp, Texture, TextureFormat, TextureType, TextureUsage, TransferBuffer,
    VertexElementFormat, VertexInputRate,
};

/// A structure specifying the parameters of a depth-stencil target used by a render pass.
///
/// # Remarks
///
/// The [`load_op`](Self::with_load_op) field determines what is done with the depth contents of the texture at the beginning of the render pass.
///
/// - [`LoadOp::LOAD`]: Loads the depth values currently in the texture.
/// - [`LoadOp::CLEAR`]: Clears the texture to a single depth.
/// - [`LoadOp::DONT_CARE`]: The driver will do whatever it wants with the memory. This is a good option if you know that every single pixel will be touched in the render pass.
///
/// The [`store_op`](Self::with_store_op) field determines what is done with the depth results of the render pass.
///
/// - [`StoreOp::STORE`]: Stores the depth results in the texture.
/// - [`StoreOp::DONT_CARE`]: The driver will do whatever it wants with the depth results. This is often a good option for depth/stencil textures that don't need to be reused again.
///
/// The [`stencil_load_op`](Self::with_stencil_load_op) field determines what is done with the stencil contents of the texture at the beginning of the render pass.
///
/// - [`LoadOp::LOAD`]: Loads the stencil values currently in the texture.
/// - [`LoadOp::CLEAR`]: Clears the stencil values to a single value.
/// - [`LoadOp::DONT_CARE`]: The driver will do whatever it wants with the memory. This is a good option if you know that every single pixel will be touched in the render pass.
///
/// The [`stencil_store_op`](Self::with_stencil_store_op) field determines what is done with the stencil results of the render pass.
///
/// - [`StoreOp::STORE`]: Stores the stencil results in the texture.
/// - [`StoreOp::DONT_CARE`]: The driver will do whatever it wants with the stencil results. This is often a good option for depth/stencil textures that don't need to be reused again.
///
/// Note that depth/stencil targets do not support multisample resolves.
#[repr(transparent)]
#[derive(Default)]
pub struct DepthStencilTargetInfo<'a> {
    inner: SDL_GPUDepthStencilTargetInfo,
    _marker: PhantomData<&'a Texture>,
}
impl<'a> DepthStencilTargetInfo<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The texture that will be used as the depth stencil target by the render pass.
    pub fn with_texture(mut self, texture: &'a Texture) -> Self {
        self.inner.texture = texture.ll();
        self
    }

    /// The value to clear the depth component to at the beginning of the render pass. Ignored if [`LoadOp::CLEAR`] is not used.
    pub fn with_clear_depth(mut self, clear_depth: f32) -> Self {
        self.inner.clear_depth = clear_depth;
        self
    }

    /// What is done with the depth contents at the beginning of the render pass.
    pub fn with_load_op(mut self, load_op: LoadOp) -> Self {
        self.inner.load_op = load_op;
        self
    }

    /// What is done with the depth results of the render pass.
    pub fn with_store_op(mut self, store_op: StoreOp) -> Self {
        self.inner.store_op = store_op;
        self
    }

    /// What is done with the stencil contents at the beginning of the render pass.
    pub fn with_stencil_load_op(mut self, stencil_load_op: LoadOp) -> Self {
        self.inner.stencil_load_op = stencil_load_op;
        self
    }

    /// What is done with the stencil results of the render pass
    pub fn with_stencil_store_op(mut self, stencil_store_op: StoreOp) -> Self {
        self.inner.stencil_store_op = stencil_store_op;
        self
    }

    /// `true` cycles the texture if the texture is bound and any load ops are not [`LoadOp::LOAD`]
    pub fn with_cycle(mut self, cycle: bool) -> Self {
        self.inner.cycle = cycle;
        self
    }

    /// The value to clear the stencil component to at the beginning of the render pass. Ignored if [`LoadOp::CLEAR`] is not used.
    pub fn with_clear_stencil(mut self, clear_stencil: u8) -> Self {
        self.inner.clear_stencil = clear_stencil;
        self
    }
}

/// A structure specifying the parameters of a color target used by a render pass.
///
/// # Remarks
///
/// The [`load_op`](Self::with_load_op) field determines what is done with the texture at the beginning of the render pass.
///
/// - [`LoadOp::LOAD`]: Loads the data currently in the texture. Not recommended for multisample textures as it requires significant memory bandwidth.
/// - [`LoadOp::CLEAR`]: Clears the texture to a single color.
/// - [`LoadOp::DONT_CARE`]: The driver will do whatever it wants with the texture memory. This is a good option if you know that every single pixel will be touched in the render pass.
///
/// The [`store_op`](Self::with_store_op) field determines what is done with the color results of the render pass.
///
/// - [`StoreOp::STORE`]: Stores the results of the render pass in the texture. Not recommended for multisample textures as it requires significant memory bandwidth.
/// - [`StoreOp::DONT_CARE`]: The driver will do whatever it wants with the texture memory. This is often a good option for depth/stencil textures.
/// - [`StoreOp::RESOLVE`]: Resolves a multisample texture into resolve_texture, which must have a sample count of 1. Then the driver may discard the multisample texture memory. This is the most performant method of resolving a multisample target.
/// - [`StoreOp::RESOLVE_AND_STORE`]: Resolves a multisample texture into the resolve_texture, which must have a sample count of 1. Then the driver stores the multisample texture's contents. Not recommended as it requires significant memory bandwidth.
#[repr(transparent)]
#[derive(Default)]
pub struct ColorTargetInfo<'a> {
    inner: SDL_GPUColorTargetInfo,
    _marker: PhantomData<&'a Texture>,
}
impl<'a> ColorTargetInfo<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The texture that will be used as a color target by a render pass.
    pub fn with_texture(mut self, texture: &'a Texture) -> Self {
        self.inner.texture = texture.ll();
        self
    }

    /// The mip level to use as a color target.
    pub fn with_mip_level(mut self, mip_level: u32) -> Self {
        self.inner.mip_level = mip_level;
        self
    }

    /// The layer index or depth plane to use as a color target. This value is treated as a layer index on 2D array and cube textures, and as a depth plane on 3D textures.
    pub fn with_layer_or_depth_plane(mut self, layer_or_depth_plane: u32) -> Self {
        self.inner.layer_or_depth_plane = layer_or_depth_plane;
        self
    }

    /// The color to clear the color target to at the start of the render pass. Ignored if [`LoadOp::CLEAR`] is not used.
    pub fn with_clear_color(mut self, clear_color: Color) -> Self {
        self.inner.clear_color.r = (clear_color.r as f32) / 255.0;
        self.inner.clear_color.g = (clear_color.g as f32) / 255.0;
        self.inner.clear_color.b = (clear_color.b as f32) / 255.0;
        self.inner.clear_color.a = (clear_color.a as f32) / 255.0;
        self
    }

    /// What is done with the contents of the color target at the beginning of the render pass.
    pub fn with_load_op(mut self, load_op: LoadOp) -> Self {
        self.inner.load_op = load_op;
        self
    }

    /// What is done with the results of the render pass.
    pub fn with_store_op(mut self, store_op: StoreOp) -> Self {
        self.inner.store_op = store_op;
        self
    }

    /// The texture that will receive the results of a multisample resolve operation. Ignored if [`StoreOp::RESOLVE`] or [`StoreOp::RESOLVE_AND_STORE`] is not used.
    pub fn with_resolve_texture(mut self, resolve_texture: &'a Texture) -> Self {
        self.inner.resolve_texture = resolve_texture.ll();
        self
    }

    /// The mip level of the resolve texture to use for the resolve operation. Ignored if [`StoreOp::RESOLVE`] or [`StoreOp::RESOLVE_AND_STORE`] is not used.
    pub fn with_resolve_mip_level(mut self, resolve_mip_level: u32) -> Self {
        self.inner.resolve_mip_level = resolve_mip_level;
        self
    }

    /// The layer index of the resolve texture to use for the resolve operation. Ignored if [`StoreOp::RESOLVE`] or [`StoreOp::RESOLVE_AND_STORE`] is not used.
    pub fn with_resolve_layer(mut self, resolve_layer: u32) -> Self {
        self.inner.resolve_layer = resolve_layer;
        self
    }

    /// `true` cycles the texture if the texture is bound and load_op is not [`LoadOp::LOAD`]
    pub fn with_cycle(mut self, cycle: bool) -> Self {
        self.inner.cycle = cycle;
        self
    }

    /// `true` cycles the resolve texture if the resolve texture is bound. Ignored if [`StoreOp::RESOLVE`] or [`StoreOp::RESOLVE_AND_STORE`] is not used.
    pub fn with_cycle_resolve_texture(mut self, cycle_resolve_texture: bool) -> Self {
        self.inner.cycle_resolve_texture = cycle_resolve_texture;
        self
    }
}

/// A structure specifying the parameters of a texture.
///
/// # Remarks
///
/// [Usage flags](TextureUsage) can be bitwise OR'd together for combinations of usages using the [`BitOr`](::core::ops::BitOr) trait. Note that certain usage combinations are invalid, for example [`TextureUsage::SAMPLER`] and [`TextureUsage::GRAPHICS_STORAGE_READ`].
#[repr(transparent)]
#[derive(Default)]
pub struct TextureCreateInfo {
    pub(super) inner: SDL_GPUTextureCreateInfo,
}
impl TextureCreateInfo {
    pub fn new() -> Self {
        Default::default()
    }

    /// The base dimensionality of the texture.
    pub fn with_type(mut self, r#type: TextureType) -> Self {
        self.inner.r#type = SDL_GPUTextureType(r#type as i32);
        self
    }

    /// The pixel format of the texture.
    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.inner.format = format;
        self
    }

    /// How the texture is intended to be used by the client.
    pub fn with_usage(mut self, usage: TextureUsage) -> Self {
        self.inner.usage = usage.0;
        self
    }

    /// The width of the texture.
    pub fn with_width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }

    /// The height of the texture.
    pub fn with_height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }

    /// The layer count or depth of the texture. This value is treated as a layer count on 2D array textures, and as a depth value on 3D textures.
    pub fn with_layer_count_or_depth(mut self, layer_count_or_depth: u32) -> Self {
        self.inner.layer_count_or_depth = layer_count_or_depth;
        self
    }

    /// The number of mip levels in the texture.
    pub fn with_num_levels(mut self, num_levels: u32) -> Self {
        self.inner.num_levels = num_levels;
        self
    }

    /// The number of samples per texel. Only applies if the texture is used as a render target.
    pub fn with_sample_count(mut self, sample_count: SampleCount) -> Self {
        self.inner.sample_count = SDL_GPUSampleCount(sample_count as i32);
        self
    }
}

/// A structure specifying the parameters of a sampler.
///
/// # Remarks
///
/// Note that [`mip_lod_bias`](Self::with_mip_lod_bias) is a no-op for the Metal driver. For Metal, LOD bias must be applied via shader instead.
#[repr(transparent)]
#[derive(Default)]
pub struct SamplerCreateInfo {
    pub(super) inner: SDL_GPUSamplerCreateInfo,
}
impl SamplerCreateInfo {
    pub fn new() -> Self {
        Default::default()
    }

    /// The minification filter to apply to lookups.
    pub fn with_min_filter(mut self, filter: Filter) -> Self {
        self.inner.min_filter = SDL_GPUFilter(filter as i32);
        self
    }

    /// The magnification filter to apply to lookups.
    pub fn with_mag_filter(mut self, filter: Filter) -> Self {
        self.inner.mag_filter = SDL_GPUFilter(filter as i32);
        self
    }

    /// The mipmap filter to apply to lookups.
    pub fn with_mipmap_mode(mut self, mode: SamplerMipmapMode) -> Self {
        self.inner.mipmap_mode = SDL_GPUSamplerMipmapMode(mode as i32);
        self
    }

    /// The addressing mode for U coordinates outside [0, 1).
    pub fn with_address_mode_u(mut self, mode: SamplerAddressMode) -> Self {
        self.inner.address_mode_u = SDL_GPUSamplerAddressMode(mode as i32);
        self
    }

    /// The addressing mode for V coordinates outside [0, 1).
    pub fn with_address_mode_v(mut self, mode: SamplerAddressMode) -> Self {
        self.inner.address_mode_v = SDL_GPUSamplerAddressMode(mode as i32);
        self
    }

    /// The addressing mode for W coordinates outside [0, 1).
    pub fn with_address_mode_w(mut self, mode: SamplerAddressMode) -> Self {
        self.inner.address_mode_w = SDL_GPUSamplerAddressMode(mode as i32);
        self
    }

    /// The bias to be added to mipmap LOD calculation.
    pub fn with_mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.inner.mip_lod_bias = mip_lod_bias;
        self
    }

    /// The anisotropy value clamp used by the sampler. If enable_anisotropy is false, this is ignored.
    pub fn with_max_anisotropy(mut self, max_anisotropy: f32) -> Self {
        self.inner.max_anisotropy = max_anisotropy;
        self
    }

    /// The comparison operator to apply to fetched data before filtering.
    pub fn with_compare_op(mut self, compare_op: CompareOp) -> Self {
        self.inner.compare_op = SDL_GPUCompareOp(compare_op as i32);
        self
    }

    /// Clamps the minimum of the computed LOD value.
    pub fn with_min_lod(mut self, min_lod: f32) -> Self {
        self.inner.min_lod = min_lod;
        self
    }

    /// Clamps the maximum of the computed LOD value.
    pub fn with_max_lod(mut self, max_lod: f32) -> Self {
        self.inner.max_lod = max_lod;
        self
    }

    /// True to enable anisotropic filtering.
    pub fn with_enable_anisotropy(mut self, enable_anisotropy: bool) -> Self {
        self.inner.enable_anisotropy = enable_anisotropy;
        self
    }

    /// True to enable comparison against a reference value during lookups.
    pub fn with_enable_compare(mut self, enable_compare: bool) -> Self {
        self.inner.enable_compare = enable_compare;
        self
    }
}

/// A structure specifying a region of a texture.
///
/// # Remarks
///
/// Used when transferring data to or from a texture.
#[repr(transparent)]
#[derive(Default)]
pub struct TextureRegion<'a> {
    pub(super) inner: SDL_GPUTextureRegion,
    _marker: PhantomData<&'a Texture>,
}
impl<'a> TextureRegion<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The texture used in the copy operation.
    pub fn with_texture(mut self, texture: &'a Texture) -> Self {
        self.inner.texture = texture.ll();
        self
    }

    /// The mip level index to transfer.
    pub fn with_mip_level(mut self, mip_level: u32) -> Self {
        self.inner.mip_level = mip_level;
        self
    }

    /// The layer index to transfer.
    pub fn with_layer(mut self, layer: u32) -> Self {
        self.inner.layer = layer;
        self
    }

    /// The left offset of the region.
    pub fn with_x(mut self, x: u32) -> Self {
        self.inner.x = x;
        self
    }

    /// The top offset of the region.
    pub fn with_y(mut self, y: u32) -> Self {
        self.inner.y = y;
        self
    }

    /// The front offset of the region.
    pub fn with_z(mut self, z: u32) -> Self {
        self.inner.z = z;
        self
    }

    /// The width of the region.
    pub fn with_width(mut self, width: u32) -> Self {
        self.inner.w = width;
        self
    }

    /// The height of the region.
    pub fn with_height(mut self, height: u32) -> Self {
        self.inner.h = height;
        self
    }

    /// The depth of the region.
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.inner.d = depth;
        self
    }
}

/// A structure specifying parameters related to transferring data to or from a texture.
///
/// # Remarks
///
/// If either of [`pixels_per_row`](Self::with_pixels_per_row) or [`rows_per_layer`](Self::with_rows_per_layer) is zero, then width and height of passed [`TextureRegion`] to [`CopyPass::upload_to_gpu_texture`](super::CopyPass::upload_to_gpu_texture) or [`CopyPass::download_from_gpu_texture`](super::CopyPass::download_from_gpu_texture) are used as default values respectively and data is considered to be tightly packed.
///
/// WARNING: Direct3D 12 requires texture data row pitch to be 256 byte aligned, and offsets to be aligned to 512 bytes. If they are not, SDL will make a temporary copy of the data that is properly aligned, but this adds overhead to the transfer process. Apps can avoid this by aligning their data appropriately, or using a different GPU backend than Direct3D 12.
#[repr(transparent)]
#[derive(Default)]
pub struct TextureTransferInfo<'a> {
    pub(super) inner: SDL_GPUTextureTransferInfo,
    _marker: PhantomData<&'a TransferBuffer>,
}
impl<'a> TextureTransferInfo<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The transfer buffer used in the transfer operation.
    pub fn with_transfer_buffer(mut self, buffer: &'a TransferBuffer) -> Self {
        self.inner.transfer_buffer = buffer.ll();
        self
    }

    /// The starting byte of the image data in the transfer buffer.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }

    /// The number of pixels from one row to the next.
    pub fn with_pixels_per_row(mut self, pixels_per_row: u32) -> Self {
        self.inner.pixels_per_row = pixels_per_row;
        self
    }

    /// The number of rows from one layer/depth-slice to the next.
    pub fn with_rows_per_layer(mut self, rows_per_layer: u32) -> Self {
        self.inner.rows_per_layer = rows_per_layer;
        self
    }
}

/// A structure specifying parameters in a buffer binding call.
#[repr(transparent)]
#[derive(Default)]
pub struct BufferBinding<'a> {
    pub(super) inner: SDL_GPUBufferBinding,
    _marker: PhantomData<&'a Buffer>,
}
impl<'a> BufferBinding<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The buffer to bind. Must have been created with [`BufferUsage::VERTEX`](super::BufferUsage::VERTEX) for [`RenderPass::bind_vertex_buffers`](super::RenderPass::bind_vertex_buffers), or [`BufferUsage::INDEX`](super::BufferUsage::INDEX) for [`RenderPass::bind_index_buffer`](super::RenderPass::bind_index_buffer).
    pub fn with_buffer(mut self, buffer: &'a Buffer) -> Self {
        self.inner.buffer = buffer.ll();
        self
    }

    /// The starting byte of the data to bind in the buffer.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
}

/// A structure specifying a location in a transfer buffer.
///
/// # Remarks
///
/// Used when transferring buffer data to or from a transfer buffer.
#[repr(transparent)]
#[derive(Default)]
pub struct TransferBufferLocation<'a> {
    pub(super) inner: SDL_GPUTransferBufferLocation,
    pub(super) _marker: PhantomData<&'a TransferBuffer>,
}
impl<'a> TransferBufferLocation<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The transfer buffer used in the transfer operation.
    pub fn with_transfer_buffer(mut self, transfer_buffer: &'a TransferBuffer) -> Self {
        self.inner.transfer_buffer = transfer_buffer.ll();
        self
    }

    /// The starting byte of the buffer data in the transfer buffer.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
}

/// A structure specifying a location in a buffer.
#[repr(transparent)]
#[derive(Default)]
pub struct BufferLocation<'a> {
    pub(super) inner: SDL_GPUBufferLocation,
    pub(super) _marker: PhantomData<&'a Buffer>,
}
impl<'a> BufferLocation<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The buffer.
    pub fn with_buffer(mut self, buffer: &'a Buffer) -> Self {
        self.inner.buffer = buffer.ll();
        self
    }

    /// The starting byte within the buffer.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
}

/// A structure specifying a region of a buffer.
#[repr(transparent)]
#[derive(Default)]
pub struct BufferRegion<'a> {
    pub(super) inner: SDL_GPUBufferRegion,
    pub(super) _marker: PhantomData<&'a Buffer>,
}
impl<'a> BufferRegion<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The buffer.
    pub fn with_buffer(mut self, buffer: &'a Buffer) -> Self {
        self.inner.buffer = buffer.ll();
        self
    }

    /// The starting byte within the buffer.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }

    /// The size in bytes of the region.
    pub fn with_size(mut self, size: u32) -> Self {
        self.inner.size = size;
        self
    }
}

/// A structure specifying the parameters of vertex buffers used in a graphics pipeline.
///
/// # Remarks
///
/// When you call [`RenderPass::bind_vertex_buffers`](super::RenderPass::bind_vertex_buffers), you specify the binding slots of the vertex buffers. For example if you called [`RenderPass::bind_vertex_buffers`](super::RenderPass::bind_vertex_buffers) with a first_slot of 2 and num_bindings of 3, the binding slots 2, 3, 4 would be used by the vertex buffers you pass in.
///
/// Vertex attributes are linked to buffers via the buffer_slot field of [`VertexAttribute`]. For example, if an attribute has a buffer_slot of 0, then that attribute belongs to the vertex buffer bound at slot 0.
#[repr(transparent)]
#[derive(Clone, Default)]
pub struct VertexBufferDescription {
    inner: SDL_GPUVertexBufferDescription,
}
impl VertexBufferDescription {
    pub fn new() -> Self {
        Default::default()
    }

    /// The binding slot of the vertex buffer.
    pub fn with_slot(mut self, slot: u32) -> Self {
        self.inner.slot = slot;
        self
    }

    /// The byte pitch between consecutive elements of the vertex buffer.
    pub fn with_pitch(mut self, pitch: u32) -> Self {
        self.inner.pitch = pitch;
        self
    }

    /// Whether attribute addressing is a function of the vertex index or instance index.
    pub fn with_input_rate(mut self, input_rate: VertexInputRate) -> Self {
        self.inner.input_rate = SDL_GPUVertexInputRate(input_rate as i32);
        self
    }

    // /// Reserved for future use. Must be set to 0.
    // pub fn with_instance_step_rate(mut self, value: u32) -> Self {
    //     self.inner.instance_step_rate = value;
    //     self
    // }
}

/// A structure specifying the parameters of a graphics pipeline vertex input state.
#[repr(transparent)]
#[derive(Default)]
pub struct VertexInputState<'a> {
    pub(super) inner: SDL_GPUVertexInputState,
    _marker: PhantomData<(&'a [VertexBufferDescription], &'a [VertexAttribute])>,
}
impl<'a> VertexInputState<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// A slice of vertex buffer descriptions.
    pub fn with_vertex_buffer_descriptions(
        mut self,
        vertex_buffer_descriptions: &'a [VertexBufferDescription],
    ) -> Self {
        self.inner.vertex_buffer_descriptions =
            vertex_buffer_descriptions.as_ptr() as *const SDL_GPUVertexBufferDescription;
        self.inner.num_vertex_buffers = vertex_buffer_descriptions.len() as u32;
        self
    }

    /// A slice of vertex attribute descriptions.
    pub fn with_vertex_attributes(mut self, vertex_attributes: &'a [VertexAttribute]) -> Self {
        self.inner.vertex_attributes = vertex_attributes.as_ptr() as *const SDL_GPUVertexAttribute;
        self.inner.num_vertex_attributes = vertex_attributes.len() as u32;
        self
    }
}

/// A structure specifying the parameters of the graphics pipeline rasterizer state.
///
/// # Remarks
///
/// Note that [`FillMode::Line`] is not supported on many Android devices. For those devices, the fill mode will automatically fall back to [`FillMode::Fill`].
///
/// Also note that the D3D12 driver will enable depth clamping even if enable_depth_clip is true. If you need this clamp+clip behavior, consider enabling depth clip and then manually clamping depth in your fragment shaders on Metal and Vulkan.
#[repr(transparent)]
#[derive(Default)]
pub struct RasterizerState {
    pub(super) inner: SDL_GPURasterizerState,
}
impl RasterizerState {
    pub fn new() -> Self {
        Default::default()
    }

    /// Whether polygons will be filled in or drawn as lines.
    pub fn with_fill_mode(mut self, fill_mode: FillMode) -> Self {
        self.inner.fill_mode = SDL_GPUFillMode(fill_mode as i32);
        self
    }

    /// The facing direction in which triangles will be culled.
    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.inner.cull_mode = SDL_GPUCullMode(cull_mode as i32);
        self
    }

    /// The vertex winding that will cause a triangle to be determined as front-facing.
    pub fn with_front_face(mut self, front_face: FrontFace) -> Self {
        self.inner.front_face = SDL_GPUFrontFace(front_face as i32);
        self
    }

    /// A scalar factor controlling the depth value added to each fragment.
    pub fn with_depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
        self.inner.depth_bias_constant_factor = depth_bias_constant_factor;
        self
    }

    /// The maximum depth bias of a fragment.
    pub fn with_depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.inner.depth_bias_clamp = depth_bias_clamp;
        self
    }

    /// A scalar factor applied to a fragment's slope in depth calculations.
    pub fn with_depth_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
        self.inner.depth_bias_slope_factor = depth_bias_slope_factor;
        self
    }

    /// True to bias fragment depth values.
    pub fn with_enable_depth_bias(mut self, enable_depth_bias: bool) -> Self {
        self.inner.enable_depth_bias = enable_depth_bias;
        self
    }

    /// True to enable depth clip, false to enable depth clamp.
    pub fn with_enable_depth_clip(mut self, enable_depth_clip: bool) -> Self {
        self.inner.enable_depth_clip = enable_depth_clip;
        self
    }
}

/// A structure specifying the stencil operation state of a graphics pipeline.
#[repr(transparent)]
#[derive(Default)]
pub struct StencilOpState {
    pub(super) inner: SDL_GPUStencilOpState,
}
impl StencilOpState {
    pub fn new() -> Self {
        Default::default()
    }

    /// The comparison operator used in the stencil test.
    pub fn with_compare_op(mut self, compare_op: CompareOp) -> Self {
        self.inner.compare_op = SDL_GPUCompareOp(compare_op as i32);
        self
    }

    /// The action performed on samples that fail the stencil test.
    pub fn with_fail_op(mut self, fail_op: StencilOp) -> Self {
        self.inner.fail_op = SDL_GPUStencilOp(fail_op as i32);
        self
    }

    /// The action performed on samples that pass the depth and stencil tests.
    pub fn with_pass_op(mut self, pass_op: StencilOp) -> Self {
        self.inner.pass_op = SDL_GPUStencilOp(pass_op as i32);
        self
    }

    /// The action performed on samples that pass the stencil test and fail the depth test.
    pub fn with_depth_fail_op(mut self, depth_fail_op: StencilOp) -> Self {
        self.inner.depth_fail_op = SDL_GPUStencilOp(depth_fail_op as i32);
        self
    }
}

/// A structure specifying the parameters of the graphics pipeline depth stencil state.
#[repr(transparent)]
#[derive(Default)]
pub struct DepthStencilState {
    pub(super) inner: SDL_GPUDepthStencilState,
}
impl DepthStencilState {
    pub fn new() -> Self {
        Default::default()
    }

    /// The comparison operator used for depth testing.
    pub fn with_compare_op(mut self, compare_op: CompareOp) -> Self {
        self.inner.compare_op = SDL_GPUCompareOp(compare_op as i32);
        self
    }

    /// The stencil op state for back-facing triangles.
    pub fn with_back_stencil_state(mut self, back_stencil_state: StencilOpState) -> Self {
        self.inner.back_stencil_state = back_stencil_state.inner;
        self
    }

    /// The stencil op state for front-facing triangles.
    pub fn with_front_stencil_state(mut self, front_stencil_state: StencilOpState) -> Self {
        self.inner.front_stencil_state = front_stencil_state.inner;
        self
    }

    /// Selects the bits of the stencil values participating in the stencil test.
    pub fn with_compare_mask(mut self, compare_mask: u8) -> Self {
        self.inner.compare_mask = compare_mask;
        self
    }

    /// Selects the bits of the stencil values updated by the stencil test.
    pub fn with_write_mask(mut self, write_mask: u8) -> Self {
        self.inner.write_mask = write_mask;
        self
    }

    /// True enables the depth test.
    pub fn with_enable_depth_test(mut self, enable_depth_test: bool) -> Self {
        self.inner.enable_depth_test = enable_depth_test;
        self
    }

    /// True enables depth writes.
    pub fn with_enable_depth_write(mut self, enable_depth_write: bool) -> Self {
        self.inner.enable_depth_write = enable_depth_write;
        self
    }

    /// True enables the stencil test.
    pub fn with_enable_stencil_test(mut self, enable_stencil_test: bool) -> Self {
        self.inner.enable_stencil_test = enable_stencil_test;
        self
    }
}

/// A structure specifying the descriptions of render targets used in a graphics pipeline.
#[repr(transparent)]
#[derive(Default)]
pub struct GraphicsPipelineTargetInfo<'a> {
    pub(super) inner: SDL_GPUGraphicsPipelineTargetInfo,
    _marker: PhantomData<&'a [ColorTargetDescription]>,
}
impl<'a> GraphicsPipelineTargetInfo<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// A slice of color target descriptions.
    pub fn with_color_target_descriptions(
        mut self,
        color_target_descriptions: &'a [ColorTargetDescription],
    ) -> Self {
        self.inner.color_target_descriptions =
            color_target_descriptions.as_ptr() as *const SDL_GPUColorTargetDescription;
        self.inner.num_color_targets = color_target_descriptions.len() as u32;
        self
    }

    /// The pixel format of the depth-stencil target. Ignored if has_depth_stencil_target is false.
    pub fn with_depth_stencil_format(mut self, depth_stencil_format: TextureFormat) -> Self {
        self.inner.depth_stencil_format = depth_stencil_format;
        self
    }

    /// true specifies that the pipeline uses a depth-stencil target.
    pub fn with_has_depth_stencil_target(mut self, has_depth_stencil_target: bool) -> Self {
        self.inner.has_depth_stencil_target = has_depth_stencil_target;
        self
    }
}

/// A structure specifying a vertex attribute.
///
/// # Remarks
///
/// All vertex attribute locations provided to a [`VertexInputState`] must be unique.
#[repr(transparent)]
#[derive(Clone, Default)]
pub struct VertexAttribute {
    inner: SDL_GPUVertexAttribute,
}
impl VertexAttribute {
    pub fn new() -> Self {
        Default::default()
    }

    /// The shader input location index.
    pub fn with_location(mut self, location: u32) -> Self {
        self.inner.location = location;
        self
    }

    /// The binding slot of the associated vertex buffer.
    pub fn with_buffer_slot(mut self, buffer_slot: u32) -> Self {
        self.inner.buffer_slot = buffer_slot;
        self
    }

    /// The size and type of the attribute data.
    pub fn with_format(mut self, format: VertexElementFormat) -> Self {
        self.inner.format = unsafe { std::mem::transmute(format as u32) };
        self
    }

    /// The byte offset of this attribute relative to the start of the vertex element.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
}

/// A structure specifying the blend state of a color target.
#[repr(transparent)]
#[derive(Default)]
pub struct ColorTargetBlendState {
    inner: SDL_GPUColorTargetBlendState,
}
impl ColorTargetBlendState {
    pub fn new() -> Self {
        Self::default()
    }

    /// The value to be multiplied by the source RGB value.
    pub fn with_src_color_blendfactor(mut self, src_color_blendfactor: BlendFactor) -> Self {
        self.inner.src_color_blendfactor = SDL_GPUBlendFactor(src_color_blendfactor as i32);
        self
    }

    /// The value to be multiplied by the destination RGB value.
    pub fn with_dst_color_blendfactor(mut self, dst_color_blendfactor: BlendFactor) -> Self {
        self.inner.dst_color_blendfactor = SDL_GPUBlendFactor(dst_color_blendfactor as i32);
        self
    }

    /// The blend operation for the RGB components.
    pub fn with_color_blend_op(mut self, color_blend_op: BlendOp) -> Self {
        self.inner.color_blend_op = SDL_GPUBlendOp(color_blend_op as i32);
        self
    }

    /// The value to be multiplied by the source alpha.
    pub fn with_src_alpha_blendfactor(mut self, src_alpha_blendfactor: BlendFactor) -> Self {
        self.inner.src_alpha_blendfactor = SDL_GPUBlendFactor(src_alpha_blendfactor as i32);
        self
    }

    /// The value to be multiplied by the destination alpha.
    pub fn with_dst_alpha_blendfactor(mut self, dst_alpha_blendfactor: BlendFactor) -> Self {
        self.inner.dst_alpha_blendfactor = SDL_GPUBlendFactor(dst_alpha_blendfactor as i32);
        self
    }

    /// The blend operation for the alpha component.
    pub fn with_alpha_blend_op(mut self, alpha_blend_op: BlendOp) -> Self {
        self.inner.alpha_blend_op = SDL_GPUBlendOp(alpha_blend_op as i32);
        self
    }

    /// A bitmask specifying which of the RGBA components are enabled for writing. Writes to all channels if enable_color_write_mask is false.
    pub fn with_color_write_mask(mut self, color_write_mask: ColorComponentFlags) -> Self {
        self.inner.color_write_mask = color_write_mask.0;
        self
    }

    /// Whether blending is enabled for the color target.
    pub fn with_enable_blend(mut self, enable_blend: bool) -> Self {
        self.inner.enable_blend = enable_blend;
        self
    }

    /// Whether the color write mask is enabled.
    pub fn with_enable_color_write_mask(mut self, enable_color_write_mask: bool) -> Self {
        self.inner.enable_color_write_mask = enable_color_write_mask;
        self
    }
}

/// A structure specifying the parameters of color targets used in a graphics pipeline.
#[repr(transparent)]
#[derive(Default, Copy, Clone)]
pub struct ColorTargetDescription {
    inner: SDL_GPUColorTargetDescription,
}
impl ColorTargetDescription {
    pub fn new() -> Self {
        Self::default()
    }

    /// The pixel format of the texture to be used as a color target.
    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.inner.format = format;
        self
    }

    /// The blend state to be used for the color target.
    pub fn with_blend_state(mut self, blend_state: ColorTargetBlendState) -> Self {
        self.inner.blend_state = blend_state.inner;
        self
    }
}

/// A structure specifying parameters in a sampler binding call.
#[repr(transparent)]
#[derive(Default)]
pub struct TextureSamplerBinding<'a> {
    pub(crate) inner: SDL_GPUTextureSamplerBinding,
    _marker: PhantomData<(&'a Texture, &'a Sampler)>,
}
impl<'a> TextureSamplerBinding<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The texture to bind. Must have been created with [`TextureUsage::SAMPLER`].
    pub fn with_texture(mut self, texture: &'a Texture) -> Self {
        self.inner.texture = texture.ll();
        self
    }

    /// The sampler to bind.
    pub fn with_sampler(mut self, sampler: &'a Sampler) -> Self {
        self.inner.sampler = sampler.ll();
        self
    }
}

/// A structure specifying parameters related to binding textures in a compute pass.
#[repr(transparent)]
#[derive(Default)]
pub struct StorageTextureReadWriteBinding<'a> {
    pub(crate) inner: SDL_GPUStorageTextureReadWriteBinding,
    pub(crate) _marker: PhantomData<&'a Texture>,
}
impl<'a> StorageTextureReadWriteBinding<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The texture to bind. Must have been created with [`TextureUsage::COMPUTE_STORAGE_WRITE`] or [`TextureUsage::COMPUTE_STORAGE_SIMULTANEOUS_READ_WRITE`].
    pub fn with_texture(mut self, texture: &'a Texture) -> Self {
        self.inner.texture = texture.ll();
        self
    }

    /// The mip level index to bind.
    pub fn with_mip_level(mut self, mip_level: u32) -> Self {
        self.inner.mip_level = mip_level;
        self
    }

    /// The layer index to bind.
    pub fn with_layer(mut self, layer: u32) -> Self {
        self.inner.layer = layer;
        self
    }

    /// `true` cycles the texture if it is already bound.
    pub fn with_cycle(mut self, cycle: bool) -> Self {
        self.inner.cycle = cycle;
        self
    }
}

/// A structure specifying parameters related to binding buffers in a compute pass.
#[repr(transparent)]
#[derive(Default)]
pub struct StorageBufferReadWriteBinding<'a> {
    pub(crate) inner: SDL_GPUStorageBufferReadWriteBinding,
    pub(crate) _marker: PhantomData<&'a Buffer>,
}
impl<'a> StorageBufferReadWriteBinding<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The buffer to bind. Must have been created with [`BufferUsage::COMPUTE_STORAGE_WRITE`](super::BufferUsage::COMPUTE_STORAGE_WRITE).
    pub fn with_buffer(mut self, buffer: &'a Buffer) -> Self {
        self.inner.buffer = buffer.ll();
        self
    }

    /// `true` cycles the buffer if it is already bound.
    pub fn with_cycle(mut self, cycle: bool) -> Self {
        self.inner.cycle = cycle;
        self
    }
}

/// A structure specifying the parameters of an indexed dispatch command.
pub type IndirectDispatchCommand = SDL_GPUIndirectDispatchCommand;

/// A structure specifying a location in a texture.
#[repr(transparent)]
#[derive(Default)]
pub struct TextureLocation<'a> {
    pub(crate) inner: SDL_GPUTextureLocation,
    pub(crate) _marker: PhantomData<&'a Texture>,
}
impl<'a> TextureLocation<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// The texture used in the copy operation.
    pub fn with_texture(mut self, texture: &'a Texture) -> Self {
        self.inner.texture = texture.ll();
        self
    }

    /// The mip level index of the location.
    pub fn with_mip_level(mut self, mip_level: u32) -> Self {
        self.inner.mip_level = mip_level;
        self
    }

    /// The layer index of the location.
    pub fn with_layer(mut self, layer: u32) -> Self {
        self.inner.layer = layer;
        self
    }

    /// The left offset of the location.
    pub fn with_x(mut self, x: u32) -> Self {
        self.inner.x = x;
        self
    }

    /// The top offset of the location.
    pub fn with_y(mut self, y: u32) -> Self {
        self.inner.y = y;
        self
    }

    /// The front offset of the location.
    pub fn with_z(mut self, z: u32) -> Self {
        self.inner.z = z;
        self
    }
}
