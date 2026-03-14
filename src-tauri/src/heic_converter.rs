use std::io::Cursor;
use std::path::Path;

use image::codecs::jpeg::JpegEncoder;
use image::ImageEncoder;
use libheif_rs::{ColorSpace, HeifContext, ItemId, LibHeif, RgbChroma};

pub fn convert_heic_to_jpg(heic_path: &Path, jpg_path: &Path, quality: u8) -> Result<(), String> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file(
        heic_path
            .to_str()
            .ok_or_else(|| "Invalid HEIC file path".to_string())?,
    )
    .map_err(|e| format!("Failed to read HEIC file: {}", e))?;

    let handle = ctx
        .primary_image_handle()
        .map_err(|e| format!("Failed to get image handle: {}", e))?;

    let exif_data = extract_exif(&handle);

    let width = handle.width();
    let height = handle.height();

    let decoded = lib_heif
        .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)
        .map_err(|e| format!("Failed to decode HEIC image: {}", e))?;

    let planes = decoded.planes();
    let interleaved = planes
        .interleaved
        .ok_or_else(|| "No interleaved RGB plane in decoded image".to_string())?;

    // Stride may be larger than width*3 due to alignment padding
    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height as usize {
        let row_start = y * interleaved.stride;
        let row_end = row_start + (width as usize * 3);
        rgb_data.extend_from_slice(&interleaved.data[row_start..row_end]);
    }

    let mut jpeg_buf = Cursor::new(Vec::new());
    let encoder = JpegEncoder::new_with_quality(&mut jpeg_buf, quality);
    encoder
        .write_image(
            &rgb_data,
            width,
            height,
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| format!("Failed to encode JPEG: {}", e))?;

    let mut jpeg_data = jpeg_buf.into_inner();

    if let Some(exif_raw) = exif_data {
        // libheif EXIF blocks have a 4-byte Tiff header offset prefix
        let tiff_data = if exif_raw.len() > 4 {
            &exif_raw[4..]
        } else {
            &exif_raw
        };
        if !tiff_data.is_empty() {
            jpeg_data = inject_exif_into_jpeg(&jpeg_data, tiff_data)?;
        }
    }

    std::fs::write(jpg_path, &jpeg_data)
        .map_err(|e| format!("Failed to write JPEG file: {}", e))?;

    Ok(())
}

fn extract_exif(handle: &libheif_rs::ImageHandle) -> Option<Vec<u8>> {
    let mut meta_ids: Vec<ItemId> = vec![0; 1];
    let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
    if count > 0 {
        handle.metadata(meta_ids[0]).ok()
    } else {
        None
    }
}

/// Insert an APP1/Exif segment right after the JPEG SOI marker.
fn inject_exif_into_jpeg(jpeg_data: &[u8], exif_tiff_data: &[u8]) -> Result<Vec<u8>, String> {
    if jpeg_data.len() < 2 || jpeg_data[0] != 0xFF || jpeg_data[1] != 0xD8 {
        return Err("Invalid JPEG data".to_string());
    }

    let exif_header = b"Exif\x00\x00";
    let segment_size = 2 + exif_header.len() + exif_tiff_data.len();
    if segment_size > 0xFFFF {
        return Err("EXIF data too large for a single JPEG APP1 segment".to_string());
    }

    let mut out = Vec::with_capacity(jpeg_data.len() + 2 + segment_size);
    out.extend_from_slice(&jpeg_data[..2]);
    out.push(0xFF);
    out.push(0xE1);
    out.push((segment_size >> 8) as u8);
    out.push((segment_size & 0xFF) as u8);
    out.extend_from_slice(exif_header);
    out.extend_from_slice(exif_tiff_data);
    out.extend_from_slice(&jpeg_data[2..]);

    Ok(out)
}

pub fn is_heif_convert_available() -> bool {
    true
}
