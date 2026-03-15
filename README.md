Hier ist eine polierte Version deiner `README.md`. Ich habe sie so strukturiert, dass sie visuell ansprechender ist, klare Handlungsaufforderungen (Call-to-Action) enthält und die technischen Details übersichtlich darstellt.

Du kannst diesen Block direkt kopieren und deine alte `README.md` damit ersetzen.

---

# 📸 Media File Renamer

> **Batch-Umbenennung von Fotos und Videos basierend auf EXIF-Daten – schnell, sicher und plattformübergreifend.**

**Media File Renamer** ist eine Desktop-Anwendung, die Ordnung in deine Mediensammlung bringt. Durch das Auslesen von Metadaten (EXIF/Video-Tags) werden Dateien präzise nach ihrem Aufnahmedatum benannt. Entwickelt mit **Tauri v2** (Rust + TypeScript) für maximale Performance und eine geringe Dateigröße.

---

## ✨ Features

* **Intelligente Umbenennung:** Verarbeitet Dateien oder ganze Ordner via **Drag & Drop**.
* **Drei flexible Formate:**
* `YYYY_MM_DD__hhmmss` (Standard)
* `YYMMDD_hhmmss`
* `YYMMDD_originalname`


* **Metadaten-Power:** Nutzt EXIF-Daten für Bilder und Video-Metadaten (MP4/MOV). Fallback auf das Dateidatum, falls keine Metadaten vorhanden sind.
* **Zeitkorrektur (Offset):** Korrigiere falsche Kamerazeiten (z. B. Zeitzonen) sekundengenau oder über ein komfortables Menü für Jahre/Monate/Tage.
* **HEIC zu JPG Konvertierung:** Automatische Umwandlung inklusive EXIF-Erhalt und 90% Qualitätsstufe.
* **Sicherheit zuerst:** Optionale Backups vor dem Umbenennen und eine **Undo-Funktion**, um den letzten Schritt rückgängig zu machen.
* **Modernes UI:** Unterstützt nativ Light- und Dark-Mode basierend auf deinen Systemeinstellungen.

---

## 📂 Unterstützte Formate

| Kategorie | Formate |
| --- | --- |
| **Bilder** | JPG, JPEG, PNG, HEIC, HEIF, TIFF, TIF, WEBP, GIF, BMP |
| **Videos** | MP4, MOV, AVI, MKV |

---

## 🚀 Installation

### Windows

Lade den bevorzugten Installer aus den [Releases](https://github.com/fly2nbc-oss/media-file-renamer/releases) herunter:

* **Standalone:** `media-file-renamer-windows-exe` (keine Installation nötig)
* **Installer:** `.msi` oder `.exe` (NSIS)

### Linux

Wähle das passende Paket für deine Distribution:

* **AppImage:** Läuft auf fast allen Distros.
* **Debian/Ubuntu:** `.deb` Paket.
* **Standalone:** Binärdatei für den direkten Start.

---

## 🛠 Entwicklung & Build

Falls du die App selbst bauen möchtest, benötigst du **Node.js (v22+)** und **Rust (stable)**.

1. **Repository klonen:**
```bash
git clone https://github.com/fly2nbc-oss/media-file-renamer.git
cd media-file-renamer

```


2. **Abhängigkeiten installieren:**
```bash
npm install

```


3. **Entwicklungsmodus:**
```bash
npm run tauri dev

```


4. **Produktions-Build:**
```bash
npm run tauri build

```



> **Hinweis für Linux-User:** Stelle sicher, dass `libwebkit2gtk-4.1-dev` und `libheif-dev` installiert sind.

---

## 🏗 Projektstruktur

* `/src`: Frontend (Vanilla TypeScript & Vite).
* `/src-tauri`: Rust Backend (Logik für EXIF, Konvertierung und Dateisystem).
* `exif_handler.rs`: Metadaten-Extraktion.
* `heic_converter.rs`: HEIC zu JPG Logik.
* `renamer.rs`: Kernlogik der Umbenennung.



---

## 📄 Lizenz

Lizenziert unter der **Apache License, Version 2.0**.

---

**Möchtest du, dass ich dir auch noch ein paar "Social Preview" Bilder oder ein Logo-Design-Konzept erstelle, um die Seite optisch abzurunden?**
