# Terminal Clock

A simple, aesthetic, and resource-efficient terminal clock written in Rust.

![Terminal Clock Screenshot](https://raw.githubusercontent.com/placeholder/screenshot.png)

## Features

- **Customizable Colors:** Choose from a wide range of foreground and background colors.
- **Minimal Resource Usage:** Designed to consume negligible CPU by sleeping precisely until the next update.
- **Aesthetic Design:** Uses a custom block font for a clean, modern look.
- **Fullscreen Mode:** Automatically centers and adapts to terminal resizing.
- **Flexible Formats:**
    - 12-hour (AM/PM) or 24-hour (Military) time.
    - Show/hide seconds.
    - Optional blinking colon.
    - Optional date display.

## Installation

Ensure you have Rust installed. Clone the repository and run:

```bash
cargo install --path .
```

## Usage

Run the clock with default settings (Green on Black, 24h):

```bash
terminal-clock
```

### Options

```bash
terminal-clock [OPTIONS]
```

| Option | Description | Default |
|--------|-------------|---------|
| `-c, --color <COLOR>` | Foreground color (red, green, blue, yellow, cyan, magenta, white, black) | green |
| `-B, --bg-color <COLOR>` | Background color | black |
| `-s, --scale <INT>` | Scale multiplier (1, 2, 3...) | 1 |
| `-f, --fullscreen` | Automatically scale to fit the terminal | false |
| `-r, --rainbow` | Rainbow animation mode | false |
| `-p, --pulse` | Pulsing animation mode | false |
| `-t, --12h` | Use 12-hour format with AM/PM | false (24h) |
| `-S, --no-seconds` | Hide seconds | false (show) |
| `-b, --blink` | Blink the colon separator | false |
| `-d, --date` | Show date below the clock | false |
| `-h, --help` | Print help | |

### Examples

**Large Rainbow clock (12h):**
```bash
terminal-clock --scale 2 --rainbow --12h
```

**Pulsing Blue clock with date:**
```bash
terminal-clock --color blue --pulse --date
```

**Red clock with date:**
```bash
terminal-clock --color red --date
```

**Minimalist 12-hour clock (no seconds, blinking colon):**
```bash
terminal-clock --no-military --no-seconds --blink --color cyan
```

**High contrast (Black on White):**
```bash
terminal-clock --color black --bg-color white
```

## Keyboard Controls

- `q`, `Esc`, or `Ctrl+C`: Quit the application.

## Development

This project uses `crossterm` for cross-platform terminal manipulation and `chrono` for time handling. It avoids heavy TUI frameworks to keep the binary small and efficient.

To run locally during development:
```bash
cargo run -- --color blue --blink
```
