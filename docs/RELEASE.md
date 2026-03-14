# Programm auf GitHub veröffentlichen

## 1. Repository sichtbar machen (falls privat)

- Auf GitHub: **Settings** → **General** → **Danger Zone** → **Change repository visibility** → **Make public**.

Damit ist der Quellcode und die **Actions**-Seite für alle erreichbar. Nutzer können dann unter **Actions** die neueste erfolgreiche Run wählen und die **Artifacts** (Windows EXE/MSI/NSIS, Linux standalone/deb/AppImage) herunterladen – siehe README „Option A“.

---

## 2. Version festlegen und Tag setzen

Vor dem ersten Release die Version in allen Stellen auf den gewünschten Stand bringen (z.B. `1.0.0` oder `1.1.0`):

- `package.json` → `version`
- `src-tauri/Cargo.toml` → `version`
- `src-tauri/tauri.conf.json` → `version`

Optional: `npm run version:patch` / `version:minor` / `version:major` nutzen (siehe `docs/versioning.md`).

Dann Tag erstellen und pushen:

```bash
git tag v1.0.0
git push origin v1.0.0
```

---

## 3. GitHub Release erstellen (empfohlen)

**Manuell:**

1. Auf GitHub: **Releases** → **Create a new release**.
2. **Choose a tag:** den gerade gepushten Tag wählen (z.B. `v1.0.0`).
3. **Release title:** z.B. `v1.0.0` oder „Media File Renamer 1.0.0“.
4. **Describe:** Kurzbeschreibung oder Changelog (z.B. aus `CHANGELOG.md`).
5. Nach dem nächsten erfolgreichen **Build** (auf `main`/`master`): Bei der letzten Run unter **Actions** die **Artifacts** herunterladen.
6. Im Release-Formular bei **Attach binaries** die entpackten Dateien (z.B. `.exe`, `.msi`, `.deb`, `.AppImage`) hochladen.
7. **Publish release** klicken.

**Automatisch (optional):**  
Ein eigener Workflow kann bei Push eines Tags (z.B. `v*`) laufen, die CI-Artefakte sammeln und ein GitHub Release mit angehängten Binaries erstellen. Dafür braucht der Workflow ein Token mit `contents: write` (z.B. `GITHUB_TOKEN` reicht für das eigene Repo).

---

## Kurz-Checkliste

- [ ] Repo auf **public** gestellt (wenn gewünscht).
- [ ] Version in `package.json`, `Cargo.toml`, `tauri.conf.json` angepasst.
- [ ] Änderungen committet und nach `main`/`master` gepusht.
- [ ] Build in **Actions** erfolgreich durchgelaufen.
- [ ] Tag gesetzt und gepusht: `git tag vX.Y.Z && git push origin vX.Y.Z`.
- [ ] **Release** auf GitHub erstellt und Binaries aus den letzten Artifacts angehängt.

Danach ist das Programm „veröffentlicht“: Quellcode und Builds sind einsehbar, Nutzer können über **Releases** oder **Actions** → Artifacts die fertigen Dateien herunterladen.
