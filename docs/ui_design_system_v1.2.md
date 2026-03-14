# UI Design System
**Version 1.2 – Desktop PC Applications**
*Fokus: Datenvisualisierung / Tabellen · Monitoring / Dashboards · Hell-Modus + Dark Mode + Icon-System*

---

## 1. Designprinzipien

| Prinzip | Beschreibung |
|---|---|
| **Daten zuerst** | Tabellen und Charts sind das primäre UI-Element. Bedienelemente stören nicht. |
| **Technische Klarheit** | Helle oder dunkle Flächen, Slate-Blau als Funktionsfarbe, keine Dekoration. |
| **Stabile Performance** | UI muss bei großen Datensätzen und vielen Widgets ruhig bleiben. |
| **Direkte Rückmeldung** | Status und Fehler werden kompakt und inline kommuniziert. |

---

## 2. Farbsystem

### 2.1 Philosophie

Das Farbsystem folgt drei Schichten:

1. **Neutral-Schicht** – Weiß und Kühlgrau dominieren die Flächen
2. **Funktionsfarbe** – Slate-Blau (`#3B5F8A`) markiert alle aktiven, primären und interaktiven Elemente
3. **Status-Schicht** – Grün / Rot / Orange für Systemzustände, niemals dekorativ

### 2.2 Design Tokens

```css
:root {
  /* ── Flächen ── */
  --bg:             #F2F4F7;   /* App-Hintergrund, kühl-grau */
  --surface:        #FFFFFF;   /* Primärflächen: Cards, Panels, Toolbar */
  --surface-muted:  #F8F9FB;   /* Chips, Status-Badges, ruhige Hilfsflächen */

  /* ── Text ── */
  --text:           #1A2330;   /* Primärer Inhalt */
  --muted:          #52637A;   /* Sekundäre Labels, Metainfo */

  /* ── Linien ── */
  --line:           #D4DCE6;   /* Standard-Rahmen */
  --line-soft:      #E6ECF2;   /* Tabellen-Trennlinien, Chip-Grenzen */

  /* ── Akzent: Slate-Blau ── */
  --accent:         #3B5F8A;   /* Primäraktion, Fokus, Highlight */
  --accent-strong:  #2D4D73;   /* Hover auf Primäraktion */
  --accent-subtle:  #EBF0F7;   /* Aktiv-BG, Hover-Fläche, Fokus-Ring-BG */

  /* ── Status ── */
  --status-ok:      #1A7F5A;   /* Online / Erfolg */
  --status-ok-bg:   #E6F4EE;
  --status-error:   #C0392B;   /* Alarm / Fehler */
  --status-error-bg:#FDECEB;
  --status-warn:    #D4700A;   /* Warnung */
  --status-warn-bg: #FEF3E6;

  /* ── Schatten ── */
  --shadow:         0 10px 28px rgba(26, 35, 48, 0.08);
}
```

### 2.3 Dark Mode Tokens

Der Dark Mode verwendet dieselben Token-Namen. Die Werte werden per `[data-theme="dark"]` oder `@media (prefers-color-scheme: dark)` überschrieben.

```css
[data-theme="dark"] {
  /* ── Flächen ── */
  --bg:             #0F1923;   /* Tiefstes Dunkel – App-Hintergrund */
  --surface:        #1A2535;   /* Primärflächen: Cards, Panels, Toolbar */
  --surface-muted:  #1F2E40;   /* Chips, Status-Badges, Hilfsflächen */

  /* ── Text ── */
  --text:           #E2EAF4;   /* Primärer Inhalt */
  --muted:          #8A9BB0;   /* Sekundäre Labels, Metainfo */

  /* ── Linien ── */
  --line:           #2A3A4E;   /* Standard-Rahmen */
  --line-soft:      #243244;   /* Tabellen-Trennlinien, Chip-Grenzen */

  /* ── Akzent: aufgehelltes Slate-Blau ── */
  --accent:         #5B8EC4;   /* Primäraktion – heller für Lesbarkeit auf Dunkel */
  --accent-strong:  #7AACD8;   /* Hover – noch heller */
  --accent-subtle:  #1E3A52;   /* Aktiv-BG, Hover-Fläche */

  /* ── Status – gesättigter für Dunkel-Kontrast ── */
  --status-ok:      #2ECC8B;
  --status-ok-bg:   #0E3326;
  --status-error:   #E05A4E;
  --status-error-bg:#3D1210;
  --status-warn:    #F0A330;
  --status-warn-bg: #3A2600;

  /* ── Schatten – stärker auf dunklen Flächen ── */
  --shadow:         0 4px 20px rgba(0, 0, 0, 0.35);

  /* ── Tabelle ── */
  --header-bg:      #1E2D3F;
  --row-hover:      #1F3045;
}
```

**Aktivierung per JS:**
```javascript
document.documentElement.setAttribute('data-theme', 'dark');  // Dark
document.documentElement.setAttribute('data-theme', 'light'); // Hell
// oder: document.documentElement.removeAttribute('data-theme');
```

**Aktivierung per Media Query (automatisch):**
```css
@media (prefers-color-scheme: dark) {
  :root { /* Dark Mode Tokens hier einfügen */ }
}
```

### 2.4 Dark Mode – Designregeln

- **Accent aufhellen:** Im Dark Mode ist `--accent` von `#3B5F8A` auf `#5B8EC4` aufgehellt – gleiches Blau, aber ausreichend Kontrast auf dunklen Flächen (WCAG AA ≥ 4.5:1)
- **Status gesättigter:** Statusfarben im Dark Mode sind leuchtender, da sie gegen dunkle BGs bestehen müssen
- **Kein reines Schwarz:** `--bg` ist `#0F1923`, nicht `#000000` – reduziert Halation und ist augenfreundlicher
- **Schatten anders:** Im Dunklen wirken große Schatten nicht mehr – stattdessen subtile `border`-Trennung + kompakterer Schatten
- **Bilder und Charts:** Chart-Primärfarbe bleibt `--accent`, Grid-Linien wechseln auf `--line-soft`
- **Keine separaten Komponenten:** Alle Komponenten funktionieren mit beiden Token-Sets ohne Code-Änderung

### 2.5 Farbpalette – Übersicht (Hell / Dunkel)

| Token | Hell | Dunkel | Rolle |
|---|---|---|---|
| `--bg` | `#F2F4F7` | `#0F1923` | App-Hintergrund |
| `--surface` | `#FFFFFF` | `#1A2535` | Primärflächen |
| `--surface-muted` | `#F8F9FB` | `#1F2E40` | Chips, Hilfsflächen |
| `--text` | `#1A2330` | `#E2EAF4` | Primärer Inhalt |
| `--muted` | `#52637A` | `#8A9BB0` | Sekundäre Labels |
| `--line` | `#D4DCE6` | `#2A3A4E` | Standard-Rahmen |
| `--line-soft` | `#E6ECF2` | `#243244` | Feine Trennlinien |
| `--accent` | `#3B5F8A` | `#5B8EC4` | Primäraktion / Fokus |
| `--accent-strong` | `#2D4D73` | `#7AACD8` | Hover Primär |
| `--accent-subtle` | `#EBF0F7` | `#1E3A52` | Aktiv-BG / Hover-Fläche |
| `--status-ok` | `#1A7F5A` | `#2ECC8B` | Online / Erfolg |
| `--status-error` | `#C0392B` | `#E05A4E` | Alarm / Fehler |
| `--status-warn` | `#D4700A` | `#F0A330` | Warnung |

### 2.6 Farbregeln

- Akzent (`--accent`) nur für **eine** primäre Aktion pro View
- Status-Farben immer mit Icon oder Label kombinieren – nie nur per Farbe
- Keine Farbverläufe auf Bedienelementen
- Hintergrundfarben der Status-Badges (`--status-*-bg`) nicht für Text verwenden

---

## 3. Design Tokens – Typografie, Spacing, Radius

### Typografie

```css
--font-ui:   'Segoe UI', Tahoma, sans-serif;
--font-mono: 'Consolas', 'Courier New', monospace;
```

| Rolle | Größe | Gewicht | Besonderheit |
|---|---|---|---|
| Standard-UI-Text | 13px | 400 | – |
| Feldlabels | 11px | 600 | uppercase, `letter-spacing: 0.05em` |
| Tabellenkopf | 12px | 600 | – |
| KPI-Wert | 28px | 600 | – |
| KPI-Label | 11px | 400 | `--muted` |
| Empty-State-Titel | 22px | 600 | – |
| Log / Rohdaten | 12px | 400 | Monospace |

### Spacing

| Token | Wert | Einsatz |
|---|---|---|
| `--space-xs` | 4px | Icon-Gap, enge Inline-Abstände |
| `--space-sm` | 8px | Gruppen-Innenabstand |
| `--space-md` | 12px | App-Außenabstand, Section-Gap |
| `--space-lg` | 16px | Card-Padding |
| `--space-xl` | 24px | Bereichs-Trennung |

### Radius & Schatten

```css
--radius-sm:   4px;   /* Badges, Status-Dots */
--radius-md:   6px;   /* Buttons, Inputs */
--radius-lg:   8px;   /* Cards, Panels */
--radius-pill: 999px; /* Chips */
```

Schatten nur auf primären Containern: `var(--shadow)`

---

## 4. Layout

### App Shell

```
┌──────────────────────────────────────────┐
│  Topbar (Navigation + globale Aktionen)  │  48–56px
├──────────────────────────────────────────┤
│  Toolbar / Filter-Bar (optional)         │  44px
├──────────────────────────────────────────┤
│  Content Area                            │  flex-grow: 1
│  ┌──────────────────────────────────┐    │
│  │  KPI Row                         │    │
│  ├──────────────────────────────────┤    │
│  │  Chart Area                      │    │
│  ├──────────────────────────────────┤    │
│  │  Table Panel                     │    │
│  └──────────────────────────────────┘    │
├──────────────────────────────────────────┤
│  Meta-Bar / Statusleiste                 │  28px
└──────────────────────────────────────────┘
```

- Außenabstand: `12px` rundum · Abstände zwischen Bereichen: `10px`

---

## 5. Kernkomponenten

### Buttons

| Typ | Hintergrund | Text | Border | Hover |
|---|---|---|---|---|
| **Primary** | `--accent` | white | – | `--accent-strong` |
| **Secondary** | white | `--text` | `--line` | Border + Text → `--accent` |
| **Ghost** | transparent | `--accent` | – | `--accent-subtle` BG |
| **Disabled** | – | – | – | `opacity: 0.4`, kein Pointer |

Mindesthöhe: `38px` · Radius: `--radius-md`

### Status Badges

| Zustand | Text | Hintergrund |
|---|---|---|
| OK / Online | `--status-ok` | `--status-ok-bg` |
| Fehler / Alarm | `--status-error` | `--status-error-bg` |
| Warnung | `--status-warn` | `--status-warn-bg` |
| Neutral | `--muted` | `--surface-muted` |

### Tabelle

- Header: `var(--header-bg)`, 12px/600, sticky
- Zeilen: 13px, Padding `9px 10px`, Hover `var(--row-hover)`
- Trennlinien: `--line-soft`
- Virtualisiert, horizontales + vertikales Scrollen

### KPI Card

- Fläche: `--surface` · Border: `--line` · Radius: `--radius-lg`
- Wert: 28px/600 · Label: 11px, `--muted`
- Trend-Farben: `--status-ok` / `--status-error`

### Chart Container

- Primärdaten: `--accent` (`#3B5F8A`)
- Sekundärdaten: `#5C85B5`
- Grid-Linien: `--line-soft`
- Tooltip: `--surface` + `--shadow`

---

## 6. Mode-Toggle – Implementierungshinweis

Der Wechsel zwischen Hell und Dunkel erfolgt ausschließlich über Token-Überschreibung. Kein doppelter Komponenten-Code.

```css
/* Transition für sanften Wechsel */
*, *::before, *::after {
  transition: background-color 0.2s ease, border-color 0.2s ease, color 0.15s ease;
}
```

Empfohlene Toggle-Position: rechts in der Topbar, Icon-only (Sonne / Mond), Tooltip mit Label.

---

## 7. Icon-System

### 7.1 Stil-Grundsätze

| Eigenschaft | Definition |
|---|---|
| **Stil** | Outline (Kontur), kein Filled/Solid |
| **Strichstärke** | 1.5px auf 24px-Grid · 1.25px auf 16px-Grid |
| **Ecken** | Abgerundet – `stroke-linecap: round` · `stroke-linejoin: round` |
| **Grid** | 24 × 24px (Standard) · 16 × 16px (kompakt/inline) · 20 × 20px (Navigation) |
| **Empfohlene Bibliothek** | [Lucide Icons](https://lucide.dev) – konsistentes Outline-Set, MIT-Lizenz |

### 7.2 Farbregeln

```css
/* Standard – folgt dem Kontext */
.icon              { color: var(--muted); }

/* Auf aktiven / primären Elementen */
.icon-primary      { color: var(--accent); }

/* Auf Primary Buttons (weißer BG des Icons nicht nötig) */
.btn-primary .icon { color: inherit; }   /* erbt #fff vom Button */

/* Status-Icons */
.icon-ok           { color: var(--status-ok); }
.icon-warn         { color: var(--status-warn); }
.icon-error        { color: var(--status-error); }
```

**Regel:** Icons übernehmen die Farbe ihres Text-Kontexts über `currentColor`. Nie hartcodierte Hex-Werte in SVG-Attributen.

```svg
<!-- Korrekt -->
<svg stroke="currentColor" fill="none" ...>

<!-- Falsch -->
<svg stroke="#3B5F8A" fill="none" ...>
```

### 7.3 Größen & Einsatz

| Größe | Grid | Kontext |
|---|---|---|
| **16px** | 16 × 16 | Inline-Text, Tabellenzellen, Badges, Chips |
| **20px** | 20 × 20 | Sidebar-Navigation, Tabs, Toolbar-Buttons |
| **24px** | 24 × 24 | Standard – Topbar, Cards, Formulare |
| **32px** | 32 × 32 | Empty States, große Statusindikatoren |

### 7.4 Icon-Katalog (Kernset)

**Navigation & Struktur**

| Icon (Lucide) | Verwendung |
|---|---|
| `layout-dashboard` | Dashboard |
| `monitor` | Geräte / Geräteübersicht |
| `folder` | Dateiverwaltung |
| `bell` | Alarme / Benachrichtigungen |
| `settings` | Einstellungen |
| `layers` | Komponentenbibliothek |
| `chevron-right` | Untermenü, Breadcrumb |
| `chevron-down` | Aufklappen, Dropdown |

**Aktionen**

| Icon (Lucide) | Verwendung |
|---|---|
| `plus` | Neu anlegen |
| `upload` | Importieren |
| `download` | Exportieren / Herunterladen |
| `copy` | In Zwischenablage kopieren |
| `edit-2` | Bearbeiten |
| `trash-2` | Löschen |
| `refresh-cw` | Aktualisieren / Neuladen |
| `filter` | Filtern |
| `search` | Suchen |
| `x` | Schließen / Entfernen |

**Status & Feedback**

| Icon (Lucide) | Farbe | Verwendung |
|---|---|---|
| `check-circle` | `--status-ok` | Erfolg, Verbunden |
| `alert-triangle` | `--status-warn` | Warnung |
| `alert-circle` | `--status-error` | Fehler, Alarm |
| `info` | `--accent` | Hinweis |
| `wifi-off` | `--status-error` | Verbindungsfehler |
| `loader` | `--muted` | Ladevorgang (rotierend) |

**Daten & Dateien**

| Icon (Lucide) | Verwendung |
|---|---|
| `file-text` | CSV / TXT-Datei |
| `file-spreadsheet` | XLSX-Datei |
| `file-type` | PDF-Datei |
| `database` | Datenbank |
| `bar-chart-2` | Diagramm / Chart |
| `table` | Tabelle |
| `cpu` | Server / Hardware |

**UI-Steuerung**

| Icon (Lucide) | Verwendung |
|---|---|
| `sun` | Hell-Modus aktiv |
| `moon` | Dunkel-Modus aktiv |
| `panel-left` | Sidebar ein-/ausblenden |
| `more-horizontal` | Weitere Aktionen (Kontextmenü) |
| `log-out` | Abmelden |
| `user` | Benutzerprofil |

### 7.5 Verwendungsregeln

**Icon + Label (Normalfall)**
```html
<button class="btn btn-primary">
  <svg class="icon" width="16" height="16">…</svg>
  Gerät hinzufügen
</button>
```

**Icon-only (Toolbar / Topbar)**
- Immer mit `title`-Attribut oder `aria-label` versehen
- Mindestgröße der Touch-/Klickfläche: `32 × 32px`

```html
<button class="icon-btn" title="Exportieren" aria-label="Exportieren">
  <svg class="icon" width="20" height="20">…</svg>
</button>
```

**Status-Dot vs. Status-Icon**

| Situation | Komponente |
|---|---|
| Platzsparend in Tabelle / Liste | Status-Dot (`8px`, rund, Farbe) |
| Mittlere Priorität, Badge-Kontext | Badge mit `●`-Symbol |
| Alarm-Detail, großflächige Anzeige | Status-Icon (`24px`, `alert-circle`) |

### 7.6 Verbotene Verwendungen

- Filled/Solid-Icons im selben UI wie Outline-Icons mischen
- Icons ohne ausreichenden Kontrast auf farbigen Flächen
- Icons kleiner als 12px (Lesbarkeit)
- Animierte Icons außer `loader` (Ladeindikator)
- Icons als einziger Indikator für Status – immer mit Label oder Tooltip kombinieren

---

## 8. Was dieses System nicht abdeckt

- Mobile / Touch-Layouts
- Modale Dialoge & komplexe Overlays
