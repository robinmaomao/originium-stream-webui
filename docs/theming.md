# Theming

Themes are persisted in `config.json` (`theme`) and mirrored to `theme.json` for
portability. The schema is versioned.

## Schema v1

```json
{
  "version": 1,
  "name": "Originium",
  "preset": "originium",
  "colors": {
    "primary": "#7C4DFF",
    "secondary": "#00E5FF",
    "surface": "#121218",
    "background": "#0B0B0F",
    "error": "#FF5370",
    "success": "#4ADE80",
    "info": "#38BDF8",
    "warning": "#FBBF24"
  },
  "typography": { "fontFamily": "Inter, 'Noto Sans SC', sans-serif", "scale": 1.0 },
  "background": { "mode": "cover", "opacity": 0.35, "blur": 12, "image": "backgrounds/aurora.webp" },
  "effects": { "rounded": 14, "shadows": true, "motion": true }
}
```

## How it is applied

- Colors become CSS custom properties (`--originium-*`) and are also pushed into
  Vuetify's `originium` theme.
- `typography.fontFamily` becomes `--originium-font`; `scale` becomes
  `--originium-scale`.
- Imported fonts are registered via generated `@font-face` rules using
  `asset:`/data URLs; supported formats: woff2, woff, ttf, otf (≤ 20 MB).
- Background images are copied into the app data directory and rendered in a
  fixed, blurred, opacity-controlled layer (≤ 30 MB; png/jpg/webp/gif).
- `effects.motion = false` disables animations and transitions.
- Light themes are detected from background luminance and switch Vuetify to
  light mode automatically.
- `settings.acrylic` (default `true`) enables the native Windows Acrylic
  backdrop through Tauri's `set_effects`; the translucent CSS shell remains as
  a fallback on unsupported systems.
- The custom title bar follows the same light/dark tokens and exposes a
  one-click theme toggle.

## Presets

| ID | Description |
|---|---|
| `originium` | Default dark, violet/cyan |
| `midnight` | Deep blue, teal accent |
| `light` | Aurora light |

Presets are defined in `apps/desktop/src-tauri/src/commands/theme.rs` and mirrored
in the onboarding/settings UI.

## Import / export

- **Export** downloads the current theme as JSON.
- **Import** reads JSON from the clipboard, validates colors, sizes and asset
  existence, then rolls back on failure.
