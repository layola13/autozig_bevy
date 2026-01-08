use autozig_image::{
    AddressMode, Color, Extent3d, FilterMode, Image, ImageLoader, SamplerDescriptor,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsage,
};

#[test]
fn test_image_creation() {
    let image = Image::new(100, 100, TextureFormat::Rgba8).expect("Failed to create image");
    assert_eq!(image.width(), 100);
    assert_eq!(image.height(), 100);
    assert_eq!(image.format(), TextureFormat::Rgba8);
}

#[test]
fn test_image_from_raw_data() {
    let width = 10;
    let height = 10;
    let format = TextureFormat::Rgba8;
    let data_size = (width * height * 4) as usize;
    let data = vec![255u8; data_size];

    let image = Image::from_raw_data(&data, width, height, format)
        .expect("Failed to create image from raw data");

    assert_eq!(image.width(), width);
    assert_eq!(image.height(), height);
    assert_eq!(image.format(), format);
    assert_eq!(image.data().len(), data_size);
}

#[test]
fn test_texture_format_bytes_per_pixel() {
    assert_eq!(TextureFormat::R8.bytes_per_pixel(), 1);
    assert_eq!(TextureFormat::Rg8.bytes_per_pixel(), 2);
    assert_eq!(TextureFormat::Rgba8.bytes_per_pixel(), 4);
    assert_eq!(TextureFormat::Rgba16Float.bytes_per_pixel(), 8);
    assert_eq!(TextureFormat::Rgba32Float.bytes_per_pixel(), 16);
}

#[test]
fn test_texture_format_component_count() {
    assert_eq!(TextureFormat::R8.component_count(), 1);
    assert_eq!(TextureFormat::Rg8.component_count(), 2);
    assert_eq!(TextureFormat::Rgba8.component_count(), 4);
    assert_eq!(TextureFormat::Rgba16Float.component_count(), 4);
    assert_eq!(TextureFormat::Rgba32Float.component_count(), 4);
}

#[test]
fn test_get_set_pixel() {
    let mut image = Image::new(10, 10, TextureFormat::Rgba8).expect("Failed to create image");

    let test_color = Color {
        r: 1.0,
        g: 0.5,
        b: 0.25,
        a: 1.0,
    };

    image.set_pixel(5, 5, test_color);
    let retrieved = image.get_pixel(5, 5);

    // 允许小误差（由于浮点到整数的转换）
    assert!((retrieved.r - test_color.r).abs() < 0.01);
    assert!((retrieved.g - test_color.g).abs() < 0.01);
    assert!((retrieved.b - test_color.b).abs() < 0.01);
    assert!((retrieved.a - test_color.a).abs() < 0.01);
}

#[test]
fn test_solid_color_image() {
    let color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    let image = Image::solid_color(10, 10, color).expect("Failed to create solid color image");

    assert_eq!(image.width(), 10);
    assert_eq!(image.height(), 10);

    let pixel = image.get_pixel(5, 5);
    assert!((pixel.r - 1.0).abs() < 0.01);
    assert!((pixel.g - 0.0).abs() < 0.01);
    assert!((pixel.b - 0.0).abs() < 0.01);
}

#[test]
fn test_image_resize() {
    let original = Image::new(100, 100, TextureFormat::Rgba8).expect("Failed to create image");
    let resized = original
        .resize(50, 50)
        .expect("Failed to resize image");

    assert_eq!(resized.width(), 50);
    assert_eq!(resized.height(), 50);
    assert_eq!(resized.format(), TextureFormat::Rgba8);
}

#[test]
fn test_image_crop() {
    let original = Image::new(100, 100, TextureFormat::Rgba8).expect("Failed to create image");
    let cropped = original
        .crop(10, 10, 50, 50)
        .expect("Failed to crop image");

    assert_eq!(cropped.width(), 50);
    assert_eq!(cropped.height(), 50);
    assert_eq!(cropped.format(), TextureFormat::Rgba8);
}

#[test]
fn test_image_flip() {
    let mut image = Image::new(10, 10, TextureFormat::Rgba8).expect("Failed to create image");

    let top_color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    let bottom_color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    image.set_pixel(5, 0, top_color);
    image.set_pixel(5, 9, bottom_color);

    // 垂直翻转
    image.flip_vertical();

    let flipped_top = image.get_pixel(5, 0);
    let flipped_bottom = image.get_pixel(5, 9);

    assert!((flipped_top.g - 1.0).abs() < 0.01);
    assert!((flipped_bottom.r - 1.0).abs() < 0.01);
}

#[test]
fn test_image_flip_horizontal() {
    let mut image = Image::new(10, 10, TextureFormat::Rgba8).expect("Failed to create image");

    let left_color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    let right_color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    image.set_pixel(0, 5, left_color);
    image.set_pixel(9, 5, right_color);

    // 水平翻转
    image.flip_horizontal();

    let flipped_left = image.get_pixel(0, 5);
    let flipped_right = image.get_pixel(9, 5);

    assert!((flipped_left.b - 1.0).abs() < 0.01);
    assert!((flipped_right.r - 1.0).abs() < 0.01);
}

#[test]
fn test_format_conversion() {
    let original = Image::new(10, 10, TextureFormat::Rgba8).expect("Failed to create image");

    // 设置一些像素
    let mut_original = Image::new(10, 10, TextureFormat::Rgba8).expect("Failed to create image");
    let test_color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    
    // 注意：这里我们创建一个新的可变图像来测试
    let mut temp_image = Image::new(10, 10, TextureFormat::Rgba8).expect("Failed to create image");
    temp_image.set_pixel(5, 5, test_color);

    // 转换格式（实际上是复制，因为格式相同）
    let converted = temp_image
        .convert_format(TextureFormat::Rgba8)
        .expect("Failed to convert format");

    assert_eq!(converted.width(), 10);
    assert_eq!(converted.height(), 10);
    assert_eq!(converted.format(), TextureFormat::Rgba8);
}

#[test]
fn test_texture_descriptor() {
    let desc = TextureDescriptor::default_2d(1024, 768);

    assert_eq!(desc.size.width, 1024);
    assert_eq!(desc.size.height, 768);
    assert_eq!(desc.size.depth_or_array_layers, 1);
    assert_eq!(desc.mip_level_count, 1);
    assert_eq!(desc.sample_count, 1);
    assert_eq!(desc.dimension, TextureDimension::D2);
    assert_eq!(desc.format, TextureFormat::Rgba8);
}

#[test]
fn test_texture_descriptor_with_mip_levels() {
    let desc = TextureDescriptor::default_2d(1024, 768).with_mip_levels(5);

    assert_eq!(desc.mip_level_count, 5);
}

#[test]
fn test_texture_descriptor_render_target() {
    let desc = TextureDescriptor::render_target_2d(800, 600, TextureFormat::Rgba8);

    assert_eq!(desc.size.width, 800);
    assert_eq!(desc.size.height, 600);
    assert_eq!(desc.format, TextureFormat::Rgba8);
    assert!(desc.usage.render_attachment);
}

#[test]
fn test_extent3d() {
    let extent_2d = Extent3d::new_2d(640, 480);
    assert_eq!(extent_2d.width, 640);
    assert_eq!(extent_2d.height, 480);
    assert_eq!(extent_2d.depth_or_array_layers, 1);

    let extent_3d = Extent3d::new_3d(640, 480, 10);
    assert_eq!(extent_3d.width, 640);
    assert_eq!(extent_3d.height, 480);
    assert_eq!(extent_3d.depth_or_array_layers, 10);
}

#[test]
fn test_texture_usage() {
    let usage = TextureUsage::default();
    assert!(usage.texture_binding);

    let render_target = TextureUsage::render_target();
    assert!(render_target.render_attachment);
    assert!(render_target.texture_binding);
}

#[test]
fn test_sampler_descriptor() {
    let sampler = SamplerDescriptor::default();
    assert_eq!(sampler.address_mode_u, AddressMode::ClampToEdge);
    assert_eq!(sampler.address_mode_v, AddressMode::ClampToEdge);
    assert_eq!(sampler.address_mode_w, AddressMode::ClampToEdge);
    assert_eq!(sampler.mag_filter, FilterMode::Linear);
    assert_eq!(sampler.min_filter, FilterMode::Linear);
    assert_eq!(sampler.mipmap_filter, FilterMode::Linear);
}

#[test]
fn test_sampler_descriptor_nearest() {
    let sampler = SamplerDescriptor::nearest();
    assert_eq!(sampler.mag_filter, FilterMode::Nearest);
    assert_eq!(sampler.min_filter, FilterMode::Nearest);
}

#[test]
fn test_sampler_descriptor_repeat() {
    let sampler = SamplerDescriptor::repeat();
    assert_eq!(sampler.address_mode_u, AddressMode::Repeat);
    assert_eq!(sampler.address_mode_v, AddressMode::Repeat);
    assert_eq!(sampler.address_mode_w, AddressMode::Repeat);
}

#[test]
fn test_sampler_descriptor_with_filter() {
    let sampler = SamplerDescriptor::default().with_filter(FilterMode::Nearest);
    assert_eq!(sampler.mag_filter, FilterMode::Nearest);
    assert_eq!(sampler.min_filter, FilterMode::Nearest);
    assert_eq!(sampler.mipmap_filter, FilterMode::Nearest);
}

#[test]
fn test_image_asset_integration() {
    use autozig_asset::Asset;

    // 验证 Image 实现了 Asset trait
    let type_uuid = Image::type_uuid();
    assert_ne!(type_uuid, 0);

    // 创建图像
    let image = Image::new(32, 32, TextureFormat::Rgba8).expect("Failed to create image");
    assert_eq!(image.width(), 32);
    assert_eq!(image.height(), 32);
}

#[test]
fn test_image_loader() {
    let loader = ImageLoader::new();
    let extensions = loader.extensions();

    assert!(extensions.contains(&"png"));
    assert!(extensions.contains(&"jpg"));
    assert!(extensions.contains(&"jpeg"));
    assert!(extensions.contains(&"bmp"));
}

#[test]
fn test_image_loader_load_from_rgba8() {
    let loader = ImageLoader::new();
    let width = 4;
    let height = 4;
    let data = vec![255u8; (width * height * 4) as usize];

    let image = loader
        .load_from_rgba8(&data, width, height)
        .expect("Failed to load image from rgba8 data");

    assert_eq!(image.width(), width);
    assert_eq!(image.height(), height);
    assert_eq!(image.format(), TextureFormat::Rgba8);
}