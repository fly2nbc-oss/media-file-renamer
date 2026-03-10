use chrono::NaiveDateTime;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::process::Command;

use crate::models::{is_video_extension, DateSource};

pub fn read_date_from_file(path: &Path) -> (Option<NaiveDateTime>, DateSource) {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if is_video_extension(&ext) {
        if let Some(dt) = read_video_creation_date(path) {
            return (Some(dt), DateSource::Exif);
        }
    } else if let Some(dt) = read_exif_date(path) {
        return (Some(dt), DateSource::Exif);
    }

    match read_file_modified_date(path) {
        Some(dt) => (Some(dt), DateSource::FileSystem),
        None => (None, DateSource::None),
    }
}

fn read_exif_date(path: &Path) -> Option<NaiveDateTime> {
    let file = File::open(path).ok()?;
    let mut buf_reader = BufReader::new(&file);
    let exif = exif::Reader::new()
        .read_from_container(&mut buf_reader)
        .ok()?;

    let tags = [
        exif::Tag::DateTimeOriginal,
        exif::Tag::DateTimeDigitized,
        exif::Tag::DateTime,
    ];

    for tag in &tags {
        if let Some(field) = exif.get_field(*tag, exif::In::PRIMARY) {
            if let Some(dt) = parse_exif_datetime_field(field) {
                return Some(dt);
            }
        }
    }

    None
}

fn parse_exif_datetime_field(field: &exif::Field) -> Option<NaiveDateTime> {
    match &field.value {
        exif::Value::Ascii(ref vec) if !vec.is_empty() => {
            let s = std::str::from_utf8(&vec[0]).ok()?;
            let trimmed = s.trim().trim_end_matches('\0');
            if trimmed.is_empty() {
                return None;
            }
            NaiveDateTime::parse_from_str(trimmed, "%Y:%m:%d %H:%M:%S")
                .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S"))
                .ok()
        }
        _ => None,
    }
}

fn read_file_modified_date(path: &Path) -> Option<NaiveDateTime> {
    let metadata = std::fs::metadata(path).ok()?;
    let modified = metadata.modified().ok()?;
    let dt: chrono::DateTime<chrono::Utc> = modified.into();
    Some(dt.naive_local())
}

fn read_video_creation_date(path: &Path) -> Option<NaiveDateTime> {
    let file = File::open(path).ok()?;
    let file_size = file.metadata().ok()?.len();
    let mut reader = BufReader::new(file);

    let moov = find_atom(&mut reader, b"moov", 0, file_size)?;
    let mvhd = find_atom(&mut reader, b"mvhd", moov.data_start, moov.end)?;

    reader.seek(SeekFrom::Start(mvhd.data_start)).ok()?;
    let mut version_buf = [0u8; 4];
    reader.read_exact(&mut version_buf).ok()?;
    let version = version_buf[0];

    let creation_time: u64 = if version == 0 {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).ok()?;
        u32::from_be_bytes(buf) as u64
    } else {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf).ok()?;
        u64::from_be_bytes(buf)
    };

    if creation_time == 0 {
        return None;
    }

    // Mac epoch (1904-01-01) to Unix epoch offset
    let unix_ts = creation_time as i64 - 2_082_844_800;
    chrono::DateTime::from_timestamp(unix_ts, 0).map(|dt| dt.naive_utc())
}

struct AtomInfo {
    data_start: u64,
    end: u64,
}

fn find_atom(reader: &mut BufReader<File>, target: &[u8; 4], start: u64, end: u64) -> Option<AtomInfo> {
    let mut pos = start;
    while pos + 8 <= end {
        reader.seek(SeekFrom::Start(pos)).ok()?;

        let mut size_buf = [0u8; 4];
        reader.read_exact(&mut size_buf).ok()?;
        let mut atom_size = u32::from_be_bytes(size_buf) as u64;

        let mut type_buf = [0u8; 4];
        reader.read_exact(&mut type_buf).ok()?;

        if atom_size == 1 {
            let mut ext_buf = [0u8; 8];
            reader.read_exact(&mut ext_buf).ok()?;
            atom_size = u64::from_be_bytes(ext_buf);
        } else if atom_size == 0 {
            atom_size = end - pos;
        }

        if atom_size < 8 {
            break;
        }

        if &type_buf == target {
            return Some(AtomInfo {
                data_start: pos + 8,
                end: pos + atom_size,
            });
        }

        pos += atom_size;
    }
    None
}

pub fn write_exif_dates(path: &Path, datetime: &NaiveDateTime) -> Result<(), String> {
    let date_str = datetime.format("%Y:%m:%d %H:%M:%S").to_string();
    let output = Command::new("exiftool")
        .args([
            "-overwrite_original",
            &format!("-DateTimeOriginal={}", date_str),
            &format!("-CreateDate={}", date_str),
            &format!("-ModifyDate={}", date_str),
        ])
        .arg(path.as_os_str())
        .output()
        .map_err(|e| format!("exiftool not found: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}

pub fn is_exiftool_available() -> bool {
    Command::new("exiftool")
        .arg("-ver")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
