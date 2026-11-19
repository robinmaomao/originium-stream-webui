# Security policy

## Reporting a vulnerability

Please open a private security advisory in this GitHub repository
(Security → Advisories → Report a vulnerability). Do not open a public issue.

## Threat model

Originium Stream is a local desktop orchestrator:

- Hosts are managed over HTTPS with HTTP Basic authentication.
- Certificates are self-signed; the first connection records a SHA-256
  fingerprint (TOFU) and subsequent connections must match.
- Credentials are stored in Windows Credential Manager, never in plain text.
- The Tauri capability set is minimal; no arbitrary shell execution is exposed
  to the webview.
- Components are downloaded over HTTPS and verified against SHA-256 before
  extraction; archive extraction rejects path traversal.

## Known limitations

- `origin_web_ui_allowed` on the Sunshine host controls which source IPs may
  access the Web UI. Remote WAN management requires explicit configuration.
- Component forks are GPL-3.0; distributing binaries requires publishing the
  corresponding source.
