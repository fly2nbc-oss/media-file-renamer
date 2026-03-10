# Media File Renamer

A desktop app to batch-rename photos and videos using EXIF (or file) dates. Built with **Tauri v2** (Rust + TypeScript), it runs on **Linux** and **Windows**.

---

## Features

- **Batch rename** by date: add files or folders via **Add** or **Drag & Drop**; choose a naming format; preview and rename in one go.
- **Three naming formats:**
  - `YYYY_MM_DD__hhmmss` (e.g. `2024_03_15__143052`) — default
  - `YYMMDD_hhmmss` (e.g. `240315_143052`)
  - `YYMMDD_originalname` (e.g. `240315_IMG_1234`)
- **Date from EXIF** for images (JPEG, TIFF, HEIF, etc.) and from **video metadata** (MP4/MOV). Fallback to file modification time.
- **Time offset:** correct wrong camera time (e.g. timezone) with a seconds field or expanded Years / Months / Days / Hours / Minutes / Seconds. Offset is applied to filenames, file timestamps, and EXIF (when `exiftool` is available).
- **Live preview** of the new names before renaming; names that actually change are highlighted in blue.
- **Optional backup:** create a `backup_YYYYMMDD_HHMMSS` folder and copy originals before renaming.
- **HEIC → JPG:** optional conversion (90% quality) via `heif-convert`; EXIF is preserved; original HEIC is removed after success.
- **Progress** overlay and **error log** panel after a run.
- **Undo** the last rename (one level only).
- **Light/Dark** UI follows system theme.

---

## Supported File Types

| Images | Videos |
|--------|--------|
| JPG, JPEG, PNG, HEIC, HEIF, TIFF, TIF, WEBP, GIF, BMP | MP4, MOV, AVI, MKV |

---

## Installation

### Option A: Download built artifacts (GitHub Actions)

1. Open your repo on GitHub → **Actions**.
2. Pick the latest successful run.
3. Under **Artifacts**, download what you need:

| Artifact | Content | Use on |
|----------|---------|--------|
| **media-file-renamer-windows-exe** | Standalone `.exe` (no installer) | Windows (requires [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) if missing) |
| **media-file-renamer-windows-msi** | MSI installer | Windows |
| **media-file-renamer-windows-nsis** | NSIS installer (single `.exe` setup) | Windows |
| **media-file-renamer-linux-standalone** | Standalone binary | Linux |
| **media-file-renamer-linux-deb** | `.deb` package | Debian / Ubuntu / compatible |
| **media-file-renamer-linux-appimage** | `.AppImage` | Most Linux distros |

Unzip the artifact and run the executable or installer. Artifact contents use filesystem-friendly names (e.g. `media-file-renamer_0.1.0_amd64.deb`, `media-file-renamer_0.1.0_amd64.AppImage`, `media-file-renamer.exe`).

### Option B: Build from source (Linux)

**Prerequisites**

- [Node.js](https://nodejs.org/) (v22 LTS recommended) and npm
- [Rust](https://rustup.rs/) (stable)
- System libraries (examples for Arch/Manjaro and Debian/Ubuntu):

**Arch / Manjaro:**

```bash
sudo pacman -S webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg patchelf
```

**Debian / Ubuntu:**

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libappindicator3-dev librsvg2-dev patchelf
```

**Optional (for full functionality):**

- **HEIC → JPG:** `libheif` (e.g. `sudo pacman -S libheif` or `sudo apt install libheif-dev`)
- **EXIF writing** (when using time offset): `perl-image-exiftool` (e.g. `sudo pacman -S perl-image-exiftool` or `sudo apt install libimage-exiftool-perl`)

**Build**

```bash
git clone <your-repo-url>
cd media-file-renamer
npm install
npm run tauri:build
```

Outputs (under `src-tauri/target/release/` and `.../bundle/`):

- Binary: `media-file-renamer`
- **.deb:** `bundle/deb/media-file-renamer_0.1.0_amd64.deb`
- **.rpm:** `bundle/rpm/media-file-renamer-0.1.0-1.x86_64.rpm`
- **.AppImage:** `bundle/appimage/media-file-renamer_0.1.0_amd64.AppImage`

Bundle filenames use the product name `media-file-renamer` (no spaces) for filesystem compatibility. The build uses `NO_STRIP=1` so AppImage succeeds on modern distros.

### Option C: Build from source (Windows)

Install [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/), and [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). WebView2 is usually already present on Windows 11. Then:

```bash
npm install
npm run tauri build
```

Artifacts are under `src-tauri\target\release\` and `...\bundle\` (e.g. `media-file-renamer_0.1.0_x64_en-US.msi`, `media-file-renamer.exe`).

---

## Usage

1. **Start** the app.
2. **Add files:** click **Add** (file picker) or drag files/folders onto the window. Folders are scanned recursively.
3. **Format:** choose a naming format from the dropdown.
4. **Offset (optional):** enter seconds (e.g. `-7200` for −2 hours) or expand and set Years / Months / Days / Hours / Minutes / Seconds. The list preview updates automatically.
5. **Options:** enable **Backup** and/or **HEIC→JPG** if needed.
6. **Rename:** click **Rename N Files**. Confirm if asked (e.g. for large batches). Use **Undo Last** if you need to revert the last run.

**Date source badges in the table:**

- **EXIF** — date from image/video metadata.
- **File** — date from file system (no EXIF).
- **None** — no date; file is skipped on rename.

---

## Project structure

```
media-file-renamer/
├── src/                    # Frontend (Vanilla TypeScript)
│   ├── main.ts             # App init, events, UI, Tauri invoke
│   ├── types.ts            # Shared TypeScript types
│   └── styles.css          # Layout, theme (light/dark)
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Tauri app & plugins
│   │   ├── commands.rs     # Tauri commands (scan, preview, rename, undo)
│   │   ├── models.rs       # Data structures
│   │   ├── exif_handler.rs # EXIF read; video date; exiftool write
│   │   ├── heic_converter.rs # HEIC → JPG via heif-convert
│   │   ├── renamer.rs      # Name formats, offset, duplicate handling
│   │   ├── backup.rs       # Backup folder creation
│   │   └── undo.rs         # Undo log save/restore
│   ├── Cargo.toml
│   └── tauri.conf.json
├── index.html
├── package.json
└── README.md
```

---

## Tech stack

- **App:** Tauri v2 (Rust + webview)
- **Frontend:** Vanilla TypeScript, Vite, CSS (no framework)
- **Backend:** Rust — `kamadak-exif`, `chrono`, `filetime`, `walkdir`; optional system tools: `exiftool`, `heif-convert`

---

## Development

```bash
npm install
npm run tauri dev
```

Runs the app with hot-reload (Vite on port 1420). Edit `src/*` and `src-tauri/src/*` as needed.

---

## License

See repository license file (if present).
