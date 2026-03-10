# GitHub einrichten – Schritt für Schritt

Diese Anleitung führt dich durch die Einrichtung von GitHub und den automatischen Build (Windows + Linux) mit GitHub Actions.

---

## Schritt 1: GitHub-Konto

1. Falls du noch kein Konto hast: Gehe zu **https://github.com** und klicke auf **Sign up**.
2. E-Mail bestätigen und Konto anlegen.
3. Einloggen.

---

## Schritt 2: Neues Repository anlegen

1. Oben rechts auf **+** klicken → **New repository**.
2. **Repository name:** z.B. `media-file-renamer` (oder ein anderer Name).
3. **Description:** optional, z.B. „Batch-Umbenennen von Fotos/Videos nach EXIF-Datum“.
4. **Public** auswählen (oder **Private**, wenn nur du Zugriff haben sollst).
5. **Add a README file** und **Add .gitignore** **nicht** ankreuzen (Projekt existiert schon lokal).
6. Auf **Create repository** klicken.
7. Die angezeigte **Repository-URL** merken, z.B.:
   - HTTPS: `https://github.com/DEIN-BENUTZERNAME/media-file-renamer.git`
   - SSH: `git@github.com:DEIN-BENUTZERNAME/media-file-renamer.git`

---

## Schritt 3: Lokales Projekt vorbereiten

Im Projektordner (dort, wo `package.json` liegt):

### 3a) Prüfen, ob Git initialisiert ist

```bash
cd "/home/rh/Media File Renamer"
git status
```

- Wenn **„not a git repository“** erscheint → Git initialisieren:
  ```bash
  git init
  ```
- Wenn `git status` funktioniert → weiter zu 3b.

### 3b) Remote hinzufügen

Ersetze `DEIN-BENUTZERNAME` und ggf. den Repo-Namen durch deine Werte:

```bash
git remote add origin https://github.com/DEIN-BENUTZERNAME/media-file-renamer.git
```

(Für SSH stattdessen: `git@github.com:DEIN-BENUTZERNAME/media-file-renamer.git`)

### 3c) Alles committen (falls noch nicht geschehen)

```bash
git add .
git status
git commit -m "Add GitHub Actions build (Windows + Linux)"
```

Falls schon Commits existieren, reicht ein Commit nur für die neuen Dateien:

```bash
git add .github/
git add SETUP_GITHUB.md
git commit -m "Add GitHub Actions and setup guide"
```

### 3d) Branch-Name prüfen

```bash
git branch
```

- Wenn **main** oder **master** angezeigt wird → gut.
- Wenn ein anderer Name (z.B. `develop`) steht und du den Build auf dem Hauptbranch willst:
  ```bash
  git branch -M main
  ```

---

## Schritt 4: Ersten Push zu GitHub

```bash
git push -u origin main
```

(Falls dein Hauptbranch `master` heißt: `git push -u origin master`.)

- Beim ersten Mal kann ein Login verlangt werden:
  - **HTTPS:** Benutzername + Passwort. Statt Passwort einen **Personal Access Token (PAT)** verwenden (siehe Schritt 5).
  - **SSH:** Funktioniert, wenn du bereits einen SSH-Key bei GitHub hinterlegt hast.

---

## Schritt 5: Personal Access Token (nur bei HTTPS + Login-Problemen)

Falls `git push` nach Benutzername/Passwort fragt:

1. GitHub → rechts oben **Profilbild** → **Settings**.
2. Links unten **Developer settings** → **Personal access tokens** → **Tokens (classic)**.
3. **Generate new token (classic)**.
4. **Note:** z.B. `Media File Renamer`.
5. **Expiration:** z.B. 90 Tage oder „No expiration“.
6. Unter **Scopes** mindestens **repo** ankreuzen.
7. **Generate token** klicken.
8. Den angezeigten Token **einmalig** kopieren und sicher aufbewahren.
9. Beim nächsten `git push` als Passwort diesen Token eingeben (Benutzername = dein GitHub-Benutzername).

---

## Schritt 6: GitHub Actions ausführen

1. Im Browser zu deinem Repository gehen:  
   `https://github.com/DEIN-BENUTZERNAME/media-file-renamer`
2. Oben auf den Tab **Actions** klicken.
3. Links sollte der Workflow **Build** erscheinen.
   - Bei einem frischen Push wird er oft automatisch gestartet.
   - Sonst: **Run workflow** (rechts) → Branch z.B. **main** wählen → **Run workflow**.
4. Die Jobs **build-windows** und **build-linux** erscheinen und laufen nacheinander bzw. parallel (je nach Konfiguration). Das kann **10–20 Minuten** dauern.
5. Wenn beide grün (Haken) sind, war der Build erfolgreich.

---

## Schritt 7: Build-Artefakte herunterladen

1. In **Actions** den letzten **erfolgreichen** Run anklicken (grüner Haken).
2. Unten auf der Seite siehst du **Artifacts**:
   - **windows-msi** – Windows-Installer (`.msi`)
   - **windows-nsis** – optionaler Windows-Installer (`.exe`)
   - **linux-appimage** – Linux AppImage
   - **linux-deb** – Linux `.deb`-Paket
3. Auf den gewünschten Namen klicken → die Datei wird als ZIP heruntergeladen.
4. ZIP entpacken und die enthaltene `.msi` (Windows) bzw. `.AppImage`/`.deb` (Linux) verwenden.

---

## Schritt 8: Spätere Änderungen pushen

Nach Code-Änderungen:

```bash
cd "/home/rh/Media File Renamer"
git add .
git commit -m "Kurze Beschreibung der Änderung"
git push
```

Der Workflow **Build** startet bei jedem Push auf `main`/`master` (und bei Pull Requests) automatisch neu.

---

## Häufige Probleme

| Problem | Lösung |
|--------|--------|
| **„Icons not found“ / Build bricht ab** | Unter `src-tauri/icons/` die in `tauri.conf.json` angegebenen Icon-Dateien ablegen. Oder lokal einmal `npm run tauri icon` ausführen und die erzeugten Icons committen. |
| **Push wird abgelehnt (Permission denied)** | Bei HTTPS: Personal Access Token als Passwort verwenden (Schritt 5). Bei SSH: SSH-Key in GitHub unter Settings → SSH and GPG keys eintragen. |
| **Workflow startet nicht** | Prüfen, ob die Datei `.github/workflows/build.yml` im Repo existiert und auf dem gepushten Branch liegt (`git add .github && git status`). |
| **Branch heißt nicht main/master** | In `.github/workflows/build.yml` bei `branches` deinen Branch-Namen eintragen oder `git branch -M main` und erneut pushen. |

---

## Kurzüberblick

1. Repository auf GitHub anlegen (ohne README/.gitignore).
2. `git remote add origin <URL>` und `git push -u origin main` (bzw. `master`).
3. Unter **Actions** den Lauf abwarten und bei Erfolg die gewünschten Artefakte herunterladen.

Bei Fragen oder Fehlermeldungen: die genaue Meldung und den Schritt notieren, dann kann gezielt weitergeholfen werden.
