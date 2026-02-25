use chrono::{Local, Timelike};
use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use palette::{FromColor, Hsl, Srgb};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

mod font;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple, aesthetic terminal clock", long_about = None)]
struct Args {
    /// Foreground color
    #[arg(short, long, default_value = "green")]
    color: String,

    /// Background color
    #[arg(short = 'B', long, default_value = "black")]
    bg_color: String,

    /// Scale multiplier. Overridden by --fullscreen.
    #[arg(short, long, default_value = "1")]
    scale: u16,

    /// Automatically scale the clock to fit the terminal window
    #[arg(short = 'f', long)]
    fullscreen: bool,

    /// Rainbow animation mode
    #[arg(short = 'r', long)]
    rainbow: bool,

    /// Pulsing animation mode
    #[arg(short = 'p', long)]
    pulse: bool,

    /// Use 12-hour format with AM/PM
    #[arg(short = 't', long = "12h")]
    use_12h: bool,

    /// Hide seconds
    #[arg(short = 'S', long)]
    no_seconds: bool,

    /// Blink the colon separator
    #[arg(short, long)]
    blink: bool,

    /// Show date below the clock
    #[arg(short = 'd', long)]
    date: bool,
}

fn parse_color(color: &str) -> Color {
    match color.to_lowercase().as_str() {
        "red" => Color::Red, "green" => Color::Green, "blue" => Color::Blue,
        "yellow" => Color::Yellow, "cyan" => Color::Cyan, "magenta" => Color::Magenta,
        "white" => Color::White, "black" => Color::Black, "grey" | "gray" => Color::Grey,
        "dark_grey" | "dark_gray" => Color::DarkGrey, _ => Color::White,
    }
}

fn get_animated_color(args: &Args, start_time: Instant, base_color: Color) -> Color {
    let elapsed = start_time.elapsed().as_secs_f32();
    if args.rainbow {
        let hue = (elapsed * 72.0) % 360.0;
        let srgb = Srgb::from_color(Hsl::new(hue, 1.0, 0.5));
        Color::Rgb { r: (srgb.red * 255.0) as u8, g: (srgb.green * 255.0) as u8, b: (srgb.blue * 255.0) as u8 }
    } else if args.pulse {
        let intensity = (elapsed * std::f32::consts::PI).sin().abs() * 0.7 + 0.3;
        let (r, g, b) = match base_color {
            Color::Red => (255, 0, 0), Color::Green => (0, 255, 0), Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0), Color::Cyan => (0, 255, 255), Color::Magenta => (255, 0, 255),
            Color::White => (255, 255, 255), Color::Grey => (128, 128, 128), _ => (255, 255, 255),
        };
        Color::Rgb { r: (r as f32 * intensity) as u8, g: (g as f32 * intensity) as u8, b: (b as f32 * intensity) as u8 }
    } else {
        base_color
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let base_fg_color = parse_color(&args.color);
    let bg_color = parse_color(&args.bg_color);
    
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide, SetBackgroundColor(bg_color), Clear(ClearType::All))?;

    let (mut term_width, mut term_height) = terminal::size()?;
    let start_instant = Instant::now();
    
    loop {
        let current_time = Local::now();
        let hour = if !args.use_12h { current_time.hour() } else {
            let h = current_time.hour12().1; if h == 0 { 12 } else { h }
        };
        
        let nanos = current_time.nanosecond();
        let sep = if !args.blink || nanos < 500_000_000 { ":" } else { " " };

        let time_str = {
            let base = if args.no_seconds { format!("{:02}{}{:02}", hour, sep, current_time.minute()) }
            else { format!("{:02}{}{:02}{}{:02}", hour, sep, current_time.minute(), sep, current_time.second()) };
            if args.use_12h { format!("{}{}", base, if current_time.hour12().0 { " PM" } else { " AM" }) } else { base }
        };

        // Static Scale calculation (always assumes max width per character type)
        let num_chars = time_str.chars().count() as u16;
        let char_spacing = 1;
        let max_base_width: u16 = time_str.chars().map(|c| font::get_slot_width(c)).sum::<u16>() + num_chars.saturating_sub(1) * char_spacing;

        let scale = if args.fullscreen {
            let margin_w = (term_width as f32 * 0.9) as u16;
            let margin_h = (term_height as f32 * 0.8) as u16;
            let scale_w = margin_w / max_base_width;
            let scale_h = margin_h.saturating_sub(if args.date { 3 } else { 0 }) / font::HEIGHT;
            scale_w.min(scale_h).max(1)
        } else {
            args.scale.max(1)
        };

        // Actual width for centering
        let scaled_char_spacing = scale;
        let scaled_total_width = max_base_width * scale ; 

        let clock_height = font::HEIGHT * scale;
        let date_padding = 2;
        let total_block_height = if args.date { clock_height + date_padding + 1 } else { clock_height };

        let start_x = term_width.saturating_sub(scaled_total_width) / 2;
        let start_y = term_height.saturating_sub(total_block_height) / 2;

        let fg_color = get_animated_color(&args, start_instant, base_fg_color);

        // Rendering Clock
        stdout.queue(SetBackgroundColor(bg_color))?;
        for row in 0..font::HEIGHT as usize {
            let mut row_buf = String::with_capacity(scaled_total_width as usize);
            for (i, ch) in time_str.chars().enumerate() {
                let slot_width = font::get_slot_width(ch);
                let pattern = font::get_digit(ch);
                let row_pattern = pattern[row];
                let ink_width = font::get_width(ch);
                
                // Calculate padding inside the slot to center the proportional ink
                let left_pad = (slot_width - ink_width) / 2;
                let right_pad = slot_width - ink_width - left_pad;

                for _ in 0..(left_pad * scale) { row_buf.push(' '); }
                for bit in row_pattern.chars() {
                    for _ in 0..scale { row_buf.push(bit); }
                }
                for _ in 0..(right_pad * scale) { row_buf.push(' '); }
                
                if i < (num_chars - 1) as usize {
                    for _ in 0..scaled_char_spacing { row_buf.push(' '); }
                }
            }

            for s_row in 0..scale {
                stdout.queue(cursor::MoveTo(start_x, start_y + (row as u16 * scale) + s_row))?;
                stdout.queue(SetForegroundColor(fg_color))?;
                stdout.queue(Print(&row_buf))?;
            }
        }

        if args.date {
            let date_str = current_time.format("%A, %B %d, %Y").to_string();
            let date_x = term_width.saturating_sub(date_str.chars().count() as u16) / 2;
            let date_y = start_y + clock_height + date_padding;
            stdout.queue(cursor::MoveTo(0, date_y))?;
            stdout.queue(terminal::Clear(ClearType::CurrentLine))?;
            stdout.queue(cursor::MoveTo(date_x, date_y))?;
            stdout.queue(SetForegroundColor(fg_color))?;
            stdout.queue(Print(date_str))?;
        }

        stdout.flush()?;

        let sleep_duration = if args.rainbow || args.pulse { Duration::from_millis(33) }
        else if args.blink {
            let next_tick = if nanos < 500_000_000 { 500_000_000 } else { 1_000_000_000 };
            Duration::from_nanos((next_tick - nanos) as u64)
        } else {
            Duration::from_nanos((1_000_000_000 - nanos) as u64)
        };

        if event::poll(sleep_duration)? {
             match event::read()? {
                Event::Key(key) => { if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => break,
                        _ => {}
                    }
                }}
                Event::Resize(w, h) => { term_width = w; term_height = h;
                    stdout.queue(SetBackgroundColor(bg_color))?;
                    stdout.queue(Clear(ClearType::All))?;
                    stdout.flush()?;
                }
                _ => {}
            }
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
