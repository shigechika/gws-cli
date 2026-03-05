---
"@googleworkspace/cli": patch
---

fix: use ~/.config/gws on all platforms for consistent config path

Previously used `dirs::config_dir()` which resolves to different paths per OS
(e.g. ~/Library/Application Support/gws on macOS, %APPDATA%\gws on Windows),
contradicting the documented ~/.config/gws/ path. Now uses ~/.config/gws/
everywhere with a fallback to the legacy OS-specific path for existing installs.
