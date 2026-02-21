# URL Dispatcher

A cross-platform (Windows + Linux) URL dispatcher that can be registered as the default browser. When a URL is opened, it presents a popup with configurable actions instead of directly opening a browser.

## Features

- **Set as default browser** on Windows (registry) and Linux (xdg-mime / .desktop)
- **Configurable actions** via a graphical settings UI:
  - **Copy to Clipboard** — copy the URL
  - **Append to File** — append the URL (with timestamp) to a text file
  - **Open in Browser** — launch any browser with custom command-line arguments (e.g. `--incognito`, `--new-window`)
- **Keyboard shortcuts** — press 1–9 to quickly select an action, Esc to cancel
- **Portable config** — settings stored in a single JSON file

## Screenshots

When a URL is dispatched:

```
┌───────────────────────────────────────┐
│ URL Dispatcher                        │
├───────────────────────────────────────┤
│ URL:                                  │
│ https://example.com/some/page         │
│───────────────────────────────────────│
│ [1] Copy to Clipboard                 │
│ [2] Append to File                    │
│ [3] Open in Firefox                   │
│ [4] Open in Chrome (Incognito)        │
│───────────────────────────────────────│
│ [Settings]                   [Cancel] │
└───────────────────────────────────────┘
```

## Installation

### Download

Pre-built binaries are available on the [Releases](https://github.com/aidev666888/url-dispatcher/releases) page:

- `url-dispatcher-linux-x86_64` — Linux (x86_64)
- `url-dispatcher-windows-x86_64.exe` — Windows (x86_64)

### Build from Source

Requirements: [Rust](https://rustup.rs/) toolchain.

**Linux** (also needs system dependencies):

```bash
# Ubuntu/Debian
sudo apt-get install -y \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev libgtk-3-dev

cargo build --release
# Binary at: target/release/url-dispatcher
```

**Windows:**

```powershell
cargo build --release
# Binary at: target\release\url-dispatcher.exe
```

## Usage

### Run Settings UI

Launch without arguments to open the settings window:

```bash
./url-dispatcher
```

From here you can:
- Add, edit, delete, and reorder actions
- Configure browser executables and command-line arguments
- Set the file path for "Append to File" actions
- Register/unregister as the default browser

### Register as Default Browser

Click **"Register as Default Browser"** in the settings UI.

**Linux:** This creates a `.desktop` file and runs `xdg-mime default` for `http` and `https` schemes. It takes effect immediately on most desktop environments.

**Windows:** This writes the necessary registry entries under `HKEY_CURRENT_USER`. After registering, go to **Settings > Apps > Default apps > Web browser** and select **URL Dispatcher**.

### Dispatch a URL

Once registered as the default browser, clicking any link in another application will launch:

```bash
url-dispatcher "https://example.com"
```

A popup appears with your configured actions. Pick one, or press Esc to cancel.

## Configuration

Config file location:
- **Linux:** `~/.config/url-dispatcher/config.json`
- **Windows:** `%APPDATA%\URLDispatcher\config.json`

### Example config

```json
{
  "version": 1,
  "actions": [
    {
      "type": "CopyToClipboard",
      "id": "...",
      "name": "Copy to Clipboard",
      "enabled": true
    },
    {
      "type": "AppendToFile",
      "id": "...",
      "name": "Save URL",
      "enabled": true
    },
    {
      "type": "OpenInBrowser",
      "id": "...",
      "name": "Firefox",
      "enabled": true,
      "executable_path": "/usr/bin/firefox",
      "args": ["{URL}"]
    },
    {
      "type": "OpenInBrowser",
      "id": "...",
      "name": "Chrome (Incognito)",
      "enabled": true,
      "executable_path": "/usr/bin/google-chrome",
      "args": ["--incognito", "{URL}"]
    }
  ],
  "append_file_path": "/home/user/urls.txt"
}
```

The `{URL}` placeholder in `args` is replaced with the actual URL at dispatch time.

## License

MIT
