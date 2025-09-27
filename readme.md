# 🕐 ASCII Digital Clock

A modern and customizable digital clock for the terminal, written in Rust. Displays the current time using ASCII characters with multiple styles, colors, and configuration options.

## ✨ Features

- 🎨 **Multiple styles**: ASCII blocks, digital, and minimalist
- 🌈 **Customizable colors**: Red, green, blue, yellow, magenta, cyan, and white
- ⏰ **Time formats**: 12h/24h with optional seconds display
- 📅 **Date information**: Day, month, year, and weekday
- 🌍 **Timezone**: Shows current timezone
- 🖥️ **Clean terminal**: Hides cursor during execution
- ⚡ **Performance**: Smooth updates every second
- 🛠️ **Easy to use**: Intuitive command-line interface

## 📦 Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (included with Rust)

### Building
```bash
# Clone the repository
git clone https://github.com/yousseffdil/ascii-clock.git
cd ascii-clock

# Build the project
cargo build --release

# Run
cargo run
```

### Dependencies
Add this to your `Cargo.toml`:

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
ctrlc = "3.4"
```

## 🚀 Usage

### Basic usage
```bash
# Simple clock
cargo run

# Show seconds
cargo run -- -s

# Show date
cargo run -- -d

# 12-hour format
cargo run -- --12h
```

### Command options

| Option | Description |
|--------|-------------|
| `-s`, `--seconds` | Show seconds |
| `-d`, `--date` | Show date and weekday |
| `--12h` | 12-hour format with AM/PM |
| `--digital` | Digital style with borders |
| `--minimal` | Minimalist style |
| `--red` | Red color |
| `--green` | Green color |
| `--yellow` | Yellow color |
| `--blue` | Blue color |
| `--magenta` | Magenta color |
| `--cyan` | Cyan color |
| `-h`, `--help` | Show help |

### Examples

```bash
# Clock with seconds, date and green color
cargo run -- -s -d --green

# 12-hour format in blue
cargo run -- --12h --blue

# Minimalist style in cyan
cargo run -- --minimal --cyan

# Digital style with date in yellow
cargo run -- --digital -d --yellow
```

## 🎨 Available styles

### Block Style (default)
```
███ ███   ███ ███
█ █ █ █   █ █ █  
███ ███   ███ ███
█ █   █ ● █ █   █
███   █   ███ ███
```

### Digital Style
```
┌─────────────────────┐
│      14:30:25       │
└─────────────────────┘
```

### Minimal Style
```
14:30:25
```

## 🌈 Available colors

- 🔴 Red (`--red`)
- 🟢 Green (`--green`)
- 🟡 Yellow (`--yellow`)
- 🔵 Blue (`--blue`)
- 🟣 Magenta (`--magenta`)
- 🔵 Cyan (`--cyan`)
- ⚪ White (default)

## 📸 Screenshots

### Clock with date and seconds
```
27/09/2025

███ █   █   ███ ███   ███ █
█ █ █   █ ● █   █  ● █ █ █
███ ███ ████ ███ ███   ███ █
  █   █   █ ●   █   █ ● █ █ █
███   █   █   ███ ███   ███ █

Saturday
Timezone: CEST
```

### Digital style with color
```
27/09/2025

┌─────────────────────┐
│      14:30:25       │
└─────────────────────┘

Saturday
Timezone: CEST
```

## 🛠️ Development

### Project structure
```
src/
├── main.rs          # Main logic and configuration
├── display.rs       # Rendering functions (optional)
└── config.rs        # Configuration handling (optional)
```

### Running in development mode
```bash
# Run with debug logs
RUST_LOG=debug cargo run

# Run tests
cargo test

# Check code
cargo clippy

# Format code
cargo fmt
```

## 🚀 Future features

- [ ] 🔔 Alarm system
- [ ] 🌍 Multiple timezones
- [ ] 🎨 Customizable themes from file
- [ ] ✨ Transition animations
- [ ] ⏱️ Stopwatch/timer mode
- [ ] 💾 Persistent configuration
- [ ] 🔔 System notifications
- [ ] 🌤️ Weather information integration
- [ ] 📱 Screensaver mode
- [ ] 🎵 Customizable sounds

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Contributing guidelines

- Follow Rust conventions (`cargo fmt`, `cargo clippy`)
- Add tests for new features
- Update documentation if necessary
- Maintain performance and simplicity

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

Your Name - [@your_username](https://github.com/yousseffdil)

## 🙏 Acknowledgments

- [Chrono](https://crates.io/crates/chrono) - Date and time handling
- [ctrlc](https://crates.io/crates/ctrlc) - System signal handling
- Rust community for excellent tools and documentation

## 📊 Project stats

![GitHub stars](https://img.shields.io/github/stars/yousseffdil/ascii-clock?style=social)
![GitHub forks](https://img.shields.io/github/forks/yousseffdil/ascii-clock?style=social)
![GitHub issues](https://img.shields.io/github/issues/yousseffdil/ascii-clock)
![GitHub license](https://img.shields.io/github/license/yousseffdil/ascii-clock)

---

⭐ If you like this project, give it a star on GitHub!

## 🐛 Bug reports

Found a bug? Report it on [GitHub Issues](https://github.com/yousseffdil/ascii-clock/issues)!