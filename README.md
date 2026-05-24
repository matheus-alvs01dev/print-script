# print-script

A small Rust helper that installs [`print-satty.sh`](./print-satty.sh) — a convenient screenshot workflow for Wayland — into your Cargo `bin` directory.

## What it does

Running `print-script` copies the bundled `print-satty.sh` script next to the binary and makes it executable. After installation, you can run the script to:

1. **Select a region** with [`slurp`](https://github.com/emersion/slurp).
2. **Capture it** with [`grim`](https://git.sr.ht/~emersion/grim).
3. **Annotate / crop** the screenshot with [`satty`](https://github.com/sigmaSd/satt).
4. **Copy to clipboard** with [`wl-clipboard`](https://github.com/bugaevc/wl-clipboard) and save to `~/Pictures/Screenshots`.

## Dependencies

Make sure the following tools are installed and available in your `$PATH`:

| Tool         | Purpose                          |
|--------------|----------------------------------|
| `grim`       | Screenshot utility for Wayland   |
| `slurp`      | Region selection                 |
| `satty`      | Screenshot annotation / crop     |
| `wl-clipboard` | Clipboard integration (`wl-copy`) |

> **Note:** This workflow is designed for **Wayland** compositors.

## Installation

### Without cloning (recommended)

You can install directly from the repository using `cargo install`:

```bash
cargo install --git https://github.com/matheus-alvs01dev/print-script.git
```

### From a local clone

```bash
# Clone the repository
git clone https://github.com/matheus-alvs01dev/print-script.git
cd print-script

# Build and install the helper (and the script) to cargo bin
cargo install --path .
```

After installation, run the installer once:

```bash
print-script
```

This will emit a message showing the path where `print-satty.sh` was installed.

## Usage

After installation, simply run:

```bash
print-satty.sh
```

- Drag to select the area you want to capture.
- `satty` will open in fullscreen with the crop tool active.
- Save inside `satty` (`Ctrl+S` or the save button).
- The final image is copied to your clipboard and saved to `~/Pictures/Screenshots/screenshot_YYYYMMDD_HHMMSS.png`.

## Project Structure

```
.
├── Cargo.toml
├── print-satty.sh      # The actual screenshot script (embedded into the binary)
├── src/
│   └── main.rs         # Rust installer
└── target/
```

## License

MIT
