# Components

Originium Stream installs portable components instead of requiring separate
installers. The manifest is [`components.lock.json`](../components.lock.json).

## Schema

```json
{
  "components": [
    {
      "id": "sunshine",
      "variant": "upstream | originium",
      "version": "2026.1119-org.1",
      "upstream": "v2026.914.233613",
      "url": "https://github.com/robinmaomao/originium-sunshine/releases/download/v2026.1119-org.1/Sunshine-Originium-x64.zip",
      "sha256": "…",
      "entry": "sunshine.exe",
      "capabilities": ["session-events"]
    }
  ]
}
```

| Field | Meaning |
|---|---|
| `id` | `sunshine` or `moonlight` |
| `variant` | `upstream` (official) or `originium` (fork build) |
| `version` | Installed version reported in the UI |
| `upstream` | Upstream tag this build is based on |
| `url` | ZIP download URL (HTTPS) |
| `sha256` | Expected archive hash; empty string skips verification (not recommended) |
| `entry` | Executable inside the archive |
| `capabilities` | Feature flags negotiated at runtime |

## Install flow

1. Download the archive (progress events on `component://progress`).
2. Verify SHA-256.
3. Extract into a staging directory, rejecting absolute paths and `..`.
4. Locate the entry file (root or single top-level folder).
5. Atomically swap the target directory, keeping `.old` until success.
6. Write `component.json` metadata next to the executable.

## Capabilities

| Capability | Used for | Fallback |
|---|---|---|
| `control-pipe` | Named-pipe control channel: `session.state`, `session.stop` (start remains CLI-driven) | CLI + exit codes |
| `embed-parent` | Native child-window embedding | Borderless fullscreen + HUD |
| `session-events` | `GET /api/session` status polling (Originium Sunshine fork) | Paired-client list + log polling |
| `no-webui` | Embedded upstream WebUI removed | N/A |

## Local overrides

Set a custom executable path in **Settings → Components**, or place a build under
`%APPDATA%/dev.originium.stream/components/<id>/`. Overrides take precedence over
bundled installations and are validated by filename.

## Update and removal safety

- Sunshine updates are blocked while its Windows service is running or starting.
- Moonlight updates/removal are blocked while a session is launching, streaming,
  or stopping.
- The Sunshine bundle cannot be removed while its service is still registered.
  Use **Uninstall service** on the This PC or Settings page first; the
  component then becomes removable.
