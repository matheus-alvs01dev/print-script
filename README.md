# print-satty

A small Rust wrapper around [`print-satty.sh`](./print-satty.sh) — a convenient screenshot workflow for Wayland. The script is embedded into the binary, so after installing you can simply run `print-satty` without any extra files.

## What it does

Running `print-satty` executes the embedded script, which will:

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

```bash
cargo install --git https://github.com/matheus-alvs01dev/print-script.git
```

### From a local clone

```bash
git clone https://github.com/matheus-alvs01dev/print-script.git
cd print-script
cargo install --path .
```

## Usage

After installation, just run:

```bash
print-satty
```

- Drag to select the area you want to capture.
- `satty` will open in fullscreen with the crop tool active.
- Save inside `satty` (`Ctrl+S` or the save button).
- The final image is copied to your clipboard and saved to `~/Pictures/Screenshots/screenshot_YYYYMMDD_HHMMSS.png`.

## Project Structure

```
.
├── Cargo.toml
├── print-satty.sh      # Embedded bash script (bundled into the binary)
├── src/
│   └── main.rs         # Rust wrapper that executes the embedded script
└── target/
```

## License

MIT
