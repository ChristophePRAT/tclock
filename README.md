# tclock

A simple, aesthetic, and resource-efficient terminal clock written in Rust.

## Features

- **Proportional Design:** Digits are visually balanced while maintaining a stable layout.
- **Auto-Scaling:** Fullscreen mode dynamically adjusts the size with safety margins.
- **Animations:** Rainbow and Pulse modes for dynamic visuals.
- **Zero Configuration:** Automatically matches your terminal's background by default.
- **Resource Efficient:** Near-zero CPU usage when idle.

## Installation

### Prerequisites

You must have [Rust](https://www.rust-lang.org/tools/install) and `cargo` installed.

### Install from Source

Clone the repository and run:

```bash
git clone https://github.com/your-username/tclock.git
cd tclock
cargo install --path .
```

After installation, the `tclock` binary will be available in your `$PATH`.

## Usage

```bash
tclock [OPTIONS]
```

### Options

| Short | Long | Description | Default |
|-------|------|-------------|---------|
| `-c` | `--color` | Foreground color (red, green, blue...) | `green` |
| `-B` | `--bg-color` | Background color (reset for terminal default) | `reset` |
| `-s` | `--scale` | Scale multiplier (1, 2, 3...) | `1` |
| `-f` | `--fullscreen` | Automatically scale to fit the terminal | `false` |
| `-r` | `--rainbow` | Rainbow animation mode | `false` |
| `-p` | `--pulse` | Pulsing animation mode | `false` |
| `-t` | `--12h` | Use 12-hour format with AM/PM | `false` |
| `-S` | `--no-seconds` | Hide seconds | `false` |
| `-b` | `--blink` | Blink the colon separator | `false` |
| `-d` | `--date` | Show date below the clock | `false` |

### Keyboard Controls

- `q`, `Esc`, or `Ctrl+C`: Quit the application.

## Development

Run locally:
```bash
cargo run -- --fullscreen --rainbow --date
```

Run tests:
```bash
cargo test
```

## License

Distributed under the MIT License. See `LICENSE` for more information.
