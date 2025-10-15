use image::GenericImageView;

fn set_bytes(to: &mut [u8], from: &[u8], range: std::ops::Range<usize>) {
    for (from_zero_index, i) in range.enumerate() {
        to[i] = from[from_zero_index];
    }
}

fn get_header(width: u32, height: u32) -> Vec<u8> {
    let mut vec = vec![0; 54];
    vec[0] = 66;
    vec[1] = 77;
    let file_size = width * height * 4 + 54;
    set_bytes(&mut vec, &file_size.to_le_bytes(), 2..6);
    set_bytes(&mut vec, &0_u32.to_le_bytes(), 6..10);
    let offset = 54_u32;
    set_bytes(&mut vec, &offset.to_le_bytes(), 10..14);
    let header_size = 40_u32;
    set_bytes(&mut vec, &header_size.to_le_bytes(), 14..18);
    let width_bytes = width.to_le_bytes();
    set_bytes(&mut vec, &width_bytes, 18..22);
    let height_bytes = height.to_le_bytes();
    set_bytes(&mut vec, &height_bytes, 22..26);
    let planes = 1_u16;
    set_bytes(&mut vec, &planes.to_le_bytes(), 26..28);
    let bits_per_pixel = 32_u16;
    set_bytes(&mut vec, &bits_per_pixel.to_le_bytes(), 28..30);
    let compression_type = 0_u32;
    set_bytes(&mut vec, &compression_type.to_le_bytes(), 30..34);
    let compressed_size = 0_u32;
    set_bytes(&mut vec, &compressed_size.to_le_bytes(), 34..38);
    let horizontal_resoultion = 0_u32;
    set_bytes(&mut vec, &horizontal_resoultion.to_le_bytes(), 38..42);
    let vertical_resolution = 0_u32;
    set_bytes(&mut vec, &vertical_resolution.to_le_bytes(), 42..46);
    let actually_used_colors = 0_u32;
    set_bytes(&mut vec, &actually_used_colors.to_le_bytes(), 46..50);
    let number_of_important_colors = 0_u32;
    set_bytes(&mut vec, &number_of_important_colors.to_le_bytes(), 50..54);
    vec
}

pub fn gen_from_img(img: &image::DynamicImage) -> Vec<u8> {
    let img = img.flipv();
    let mut byte_vec = get_header(img.width(), img.height());
    for (_, _, pixel) in img.pixels() {
        let pixel_bytes = pixel.0;
        byte_vec.push(pixel_bytes[2]);
        byte_vec.push(pixel_bytes[1]);
        byte_vec.push(pixel_bytes[0]);
        byte_vec.push(pixel_bytes[3]);
    }
    byte_vec
}