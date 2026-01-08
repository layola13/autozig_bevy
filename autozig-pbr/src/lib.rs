//! # AutoZig-PBR - PBR 材质系统
//!
//! 实现 Bevy PBR 的简化版本，专注于 WebGPU/WASM 平台
//! 
//! ## 核心功能
//! - PBR 标准材质（金属度/粗糙度工作流）
//! - 纹理支持（基础颜色、法线、金属度/粗糙度、自发光）
//! - SIMD 向量化 PBR 光照计算
//! - 零拷贝纹理采样
//!
//! ## 架构
//! - Rust: 类型安全的材质管理和 API
//! - Zig: SIMD 优化的 PBR 光照计算
//!
//! ## 示例
//! ```rust,no_run
//! use autozig_pbr::StandardMaterial;
//! 
//! let material = StandardMaterial::new()
//!     .with_base_color(1.0, 0.0, 0.0, 1.0)
//!     .with_metallic(0.8)
//!     .with_roughness(0.2)
//!     .with_emissive(0.0, 0.0, 0.0);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use autozig::include_zig;

// ============================================================================
// Zig FFI 绑定 - PBR 材质管理
// ============================================================================

include_zig!("zig/pbr_material.zig", {
    fn pbr_material_create() -> PbrMaterialHandle;
    fn pbr_material_destroy(handle: PbrMaterialHandle);
    fn pbr_material_set_base_color(handle: PbrMaterialHandle, r: f32, g: f32, b: f32, a: f32);
    fn pbr_material_set_metallic(handle: PbrMaterialHandle, metallic: f32);
    fn pbr_material_set_roughness(handle: PbrMaterialHandle, roughness: f32);
    fn pbr_material_set_emissive(handle: PbrMaterialHandle, r: f32, g: f32, b: f32);
    fn pbr_material_get_base_color(handle: PbrMaterialHandle, out: *mut [f32; 4]);
    fn pbr_material_get_metallic(handle: PbrMaterialHandle) -> f32;
    fn pbr_material_get_roughness(handle: PbrMaterialHandle) -> f32;
    fn pbr_material_get_emissive(handle: PbrMaterialHandle, out: *mut [f32; 3]);
});

// ============================================================================
// Zig FFI 绑定 - PBR 纹理系统
// ============================================================================

include_zig!("zig/pbr_texture.zig", {
    fn pbr_material_bind_base_color_texture(handle: PbrMaterialHandle, data: *const u8, width: u32, height: u32) -> bool;
    fn pbr_material_bind_normal_texture(handle: PbrMaterialHandle, data: *const u8, width: u32, height: u32) -> bool;
    fn pbr_material_bind_metallic_roughness_texture(handle: PbrMaterialHandle, data: *const u8, width: u32, height: u32) -> bool;
    fn pbr_material_bind_emissive_texture(handle: PbrMaterialHandle, data: *const u8, width: u32, height: u32) -> bool;
});

// ============================================================================
// Zig FFI 绑定 - PBR 光照计算
// ============================================================================

include_zig!("zig/pbr_lighting.zig", {
    fn pbr_calculate_lighting(
        base_color: *const [f32; 3],
        metallic: f32,
        roughness: f32,
        emissive: *const [f32; 3],
        normal: *const [f32; 3],
        view_dir: *const [f32; 3],
        light_dir: *const [f32; 3],
        light_color: *const [f32; 3],
        light_intensity: f32,
        out_color: *mut [f32; 3]
    );
    
    fn pbr_calculate_lighting_simd(
        positions: *const f32,
        normals: *const f32,
        base_colors: *const f32,
        metallic: f32,
        roughness: f32,
        emissive: *const [f32; 3],
        camera_pos: *const [f32; 3],
        lights: *const LightData,
        light_count: u32,
        ambient: *const [f32; 3],
        out_colors: *mut f32
    );
});

// ============================================================================
// Zig FFI 绑定 - PBR 批量光照计算
// ============================================================================

include_zig!("zig/pbr.zig", {
    fn pbr_lighting_calculate_batch_simd(
        materials: *const PbrMaterialHandle,
        positions: *const f32,
        normals: *const f32,
        view_dirs: *const f32,
        lights: *const LightData,
        num_vertices: u32,
        num_lights: u32,
        out_colors: *mut f32
    );
});

// ============================================================================
// Rust 类型定义
// ============================================================================

/// PBR 材质句柄（不透明指针）
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PbrMaterialHandle(*mut u8);

/// 光源数据结构
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LightData {
    /// 光源位置 [x, y, z]
    pub position: [f32; 3],
    /// 光源方向 [x, y, z]
    pub direction: [f32; 3],
    /// 光源颜色 [r, g, b]
    pub color: [f32; 3],
    /// 光源强度
    pub intensity: f32,
    /// 光源半径
    pub radius: f32,
    /// 填充对齐
    pub _padding: [f32; 3],
}

/// 标准 PBR 材质
/// 
/// 对应 Bevy 的 `StandardMaterial`，简化版本
#[derive(Debug, Clone)]
pub struct StandardMaterial {
    handle: PbrMaterialHandle,
    base_color: [f32; 4],
    metallic: f32,
    roughness: f32,
    emissive: [f32; 3],
    base_color_texture: Option<Vec<u8>>,
    normal_texture: Option<Vec<u8>>,
    metallic_roughness_texture: Option<Vec<u8>>,
    emissive_texture: Option<Vec<u8>>,
}

impl StandardMaterial {
    /// 创建默认材质
    /// 
    /// 默认值：
    /// - base_color: 白色 (1.0, 1.0, 1.0, 1.0)
    /// - metallic: 0.0 (非金属)
    /// - roughness: 0.5 (中等粗糙度)
    /// - emissive: 黑色 (0.0, 0.0, 0.0)
    pub fn new() -> Self {
        let handle = pbr_material_create();
        Self {
            handle,
            base_color: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            emissive: [0.0, 0.0, 0.0],
            base_color_texture: None,
            normal_texture: None,
            metallic_roughness_texture: None,
            emissive_texture: None,
        }
    }
    
    /// 设置基础颜色
    pub fn with_base_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.base_color = [r, g, b, a];
        pbr_material_set_base_color(self.handle, r, g, b, a);
        self
    }
    
    /// 设置金属度 (0.0 = 非金属, 1.0 = 完全金属)
    pub fn with_metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic.clamp(0.0, 1.0);
        pbr_material_set_metallic(self.handle, self.metallic);
        self
    }
    
    /// 设置粗糙度 (0.0 = 光滑镜面, 1.0 = 完全粗糙)
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness.clamp(0.0, 1.0);
        pbr_material_set_roughness(self.handle, self.roughness);
        self
    }
    
    /// 设置自发光颜色
    pub fn with_emissive(mut self, r: f32, g: f32, b: f32) -> Self {
        self.emissive = [r, g, b];
        pbr_material_set_emissive(self.handle, r, g, b);
        self
    }
    
    /// 设置基础颜色纹理
    pub fn with_base_color_texture(mut self, texture: Vec<u8>) -> Self {
        let width = 1u32; // 默认1x1纹理
        let height = 1u32;
        pbr_material_bind_base_color_texture(self.handle, texture.as_ptr(), width, height);
        self.base_color_texture = Some(texture);
        self
    }
    
    /// 设置法线贴图
    pub fn with_normal_texture(mut self, texture: Vec<u8>) -> Self {
        let width = 1u32;
        let height = 1u32;
        pbr_material_bind_normal_texture(self.handle, texture.as_ptr(), width, height);
        self.normal_texture = Some(texture);
        self
    }
    
    /// 设置金属度/粗糙度纹理
    pub fn with_metallic_roughness_texture(mut self, texture: Vec<u8>) -> Self {
        let width = 1u32;
        let height = 1u32;
        pbr_material_bind_metallic_roughness_texture(self.handle, texture.as_ptr(), width, height);
        self.metallic_roughness_texture = Some(texture);
        self
    }
    
    /// 设置自发光纹理
    pub fn with_emissive_texture(mut self, texture: Vec<u8>) -> Self {
        let width = 1u32;
        let height = 1u32;
        pbr_material_bind_emissive_texture(self.handle, texture.as_ptr(), width, height);
        self.emissive_texture = Some(texture);
        self
    }
    
    /// 获取材质句柄
    pub fn handle(&self) -> PbrMaterialHandle {
        self.handle
    }
    
    /// 获取基础颜色
    pub fn base_color(&self) -> [f32; 4] {
        self.base_color
    }
    
    /// 获取金属度
    pub fn metallic(&self) -> f32 {
        self.metallic
    }
    
    /// 获取粗糙度
    pub fn roughness(&self) -> f32 {
        self.roughness
    }
    
    /// 获取自发光颜色
    pub fn emissive(&self) -> [f32; 3] {
        self.emissive
    }
    
    /// 计算单个光源的光照
    pub fn calculate_lighting(
        &self,
        _position: [f32; 3],  // 保留用于未来扩展
        normal: [f32; 3],
        view_dir: [f32; 3],
        light_dir: [f32; 3],
        light_color: [f32; 3],
        light_intensity: f32,
    ) -> [f32; 3] {
        let base_color_rgb = [self.base_color[0], self.base_color[1], self.base_color[2]];
        let mut out_color = [0.0f32; 3];
        pbr_calculate_lighting(
            &base_color_rgb,
            self.metallic,
            self.roughness,
            &self.emissive,
            &normal,
            &view_dir,
            &light_dir,
            &light_color,
            light_intensity,
            &mut out_color,
        );
        out_color
    }
}

impl Default for StandardMaterial {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StandardMaterial {
    fn drop(&mut self) {
        pbr_material_destroy(self.handle);
    }
}


/// PBR 光照计算器
///
/// 提供批量 SIMD 向量化光照计算
pub struct PbrLightingCalculator;

impl PbrLightingCalculator {
    /// 批量计算 PBR 光照（SIMD 优化）
    ///
    /// # 参数
    /// - `materials`: 材质句柄数组
    /// - `positions`: 顶点位置数组 [x, y, z, x, y, z, ...]
    /// - `normals`: 法线数组 [nx, ny, nz, nx, ny, nz, ...]
    /// - `view_dirs`: 视线方向数组 [vx, vy, vz, vx, vy, vz, ...]
    /// - `lights`: 光源数据数组
    ///
    /// # 返回
    /// 输出颜色数组 [r, g, b, r, g, b, ...]
    pub fn calculate_batch_simd(
        materials: &[PbrMaterialHandle],
        positions: &[f32],
        normals: &[f32],
        view_dirs: &[f32],
        lights: &[LightData],
    ) -> Vec<f32> {
        let num_vertices = (positions.len() / 3) as u32;
        let num_lights = lights.len() as u32;
        let mut out_colors = vec![0.0f32; (num_vertices * 3) as usize];
        
        pbr_lighting_calculate_batch_simd(
            materials.as_ptr(),
            positions.as_ptr(),
            normals.as_ptr(),
            view_dirs.as_ptr(),
            lights.as_ptr(),
            num_vertices,
            num_lights,
            out_colors.as_mut_ptr(),
        );
        
        out_colors
    }
}

// ============================================================================
// 公共 API
// ============================================================================

pub mod prelude {
    //! 预导出的常用类型
    pub use crate::{
        StandardMaterial,
        PbrMaterialHandle,
        LightData,
        PbrLightingCalculator,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let material = StandardMaterial::new();
        assert_eq!(material.base_color(), [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(material.metallic(), 0.0);
        assert_eq!(material.roughness(), 0.5);
        assert_eq!(material.emissive(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_material_builder() {
        let material = StandardMaterial::new()
            .with_base_color(1.0, 0.0, 0.0, 1.0)
            .with_metallic(0.8)
            .with_roughness(0.2)
            .with_emissive(0.5, 0.5, 0.5);
        
        assert_eq!(material.base_color(), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(material.metallic(), 0.8);
        assert_eq!(material.roughness(), 0.2);
        assert_eq!(material.emissive(), [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_metallic_clamping() {
        let material = StandardMaterial::new().with_metallic(1.5);
        assert_eq!(material.metallic(), 1.0);
        
        let material = StandardMaterial::new().with_metallic(-0.5);
        assert_eq!(material.metallic(), 0.0);
    }

    #[test]
    fn test_roughness_clamping() {
        let material = StandardMaterial::new().with_roughness(1.5);
        assert_eq!(material.roughness(), 1.0);
        
        let material = StandardMaterial::new().with_roughness(-0.5);
        assert_eq!(material.roughness(), 0.0);
    }
}