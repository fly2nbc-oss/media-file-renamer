use std::path::Path;
use std::process::Command;

pub fn convert_heic_to_jpg(heic_path: &Path, jpg_path: &Path, quality: u8) -> Result<(), String> {
    let output = Command::new("heif-convert")
        .args([
            "-q",
            &quality.to_string(),
            &heic_path.to_string_lossy(),
            &jpg_path.to_string_lossy(),
        ])
        .output()
        .map_err(|e| {
            format!(
                "heif-convert not found: {}. Install with: sudo pacman -S libheif",
                e
            )
        })?;

    if !output.status.success() {
        return Err(format!(
            "HEIC conversion failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn is_heif_convert_available() -> bool {
    Command::new("heif-convert")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
