# TaskX - Terminal Task Manager

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Version](https://img.shields.io/badge/version-0.1.0-green)
![Platform](https://img.shields.io/badge/platform-macOS%20|%20Linux-lightgrey)
![Open Source](https://img.shields.io/badge/Open%20Source-%E2%9D%A4-red)

TaskX is a powerful, keyboard-driven terminal task manager built with Rust that helps you organize your tasks in a beautiful Kanban board right in your terminal!

<p align="center">
  <img src="https://share.cleanshot.com/tVb0RSfJ" alt="TaskX Screenshot" width="100%"/>
</p>

## ✨ Features

- 📋 Intuitive Kanban board view for your tasks
- 🚀 Lightning-fast performance with native Rust implementation
- ⌨️ 100% keyboard-driven interface - no mouse needed!
- 🏷️ Prioritize tasks with different priority levels
- 📝 Detailed task view with descriptions and metadata
- 🔄 Multiple view modes: Kanban, List, and Detail
- 💾 Local file storage for your tasks
- 🌈 Color-coded tasks by priority and status

## 📦 Installation

### Prerequisites

- Rust and Cargo (1.70+)

### macOS / Linux

```bash
# Clone the repository
git clone https://github.com/rogalio/taskx.git
cd taskx

# Build and install
cargo install --path .

# Run TaskX
taskx
```

## 🚀 Usage

TaskX is designed to be used entirely with the keyboard. Here are the main commands:

### Navigation

| Key         | Action                        |
| ----------- | ----------------------------- |
| `↑` / `k`   | Move selection up             |
| `↓` / `j`   | Move selection down           |
| `←` / `h`   | Previous column (Kanban view) |
| `→` / `l`   | Next column (Kanban view)     |
| `Enter`     | Toggle task details           |
| `Tab`       | Next tab                      |
| `Shift+Tab` | Previous tab                  |

### Task Management

| Key       | Action                    |
| --------- | ------------------------- |
| `n`       | Create new task           |
| `Space`   | Move task to next status  |
| `e` / `i` | Edit task (coming soon)   |
| `d`       | Delete task (coming soon) |

### View Options

| Key | Action                                |
| --- | ------------------------------------- |
| `v` | Toggle view mode (Kanban/List/Detail) |
| `f` | Toggle filter mode                    |
| `1` | Filter Todo tasks                     |
| `2` | Filter In Progress tasks              |
| `3` | Filter Done tasks                     |

### General

| Key | Action      |
| --- | ----------- |
| `?` | Toggle help |
| `q` | Quit        |

## 🔜 Roadmap

TaskX is under active development! Here's what's coming:

- [ ] Task editing and deletion
- [ ] Due dates and reminders
- [ ] Task filtering by multiple criteria
- [ ] Persistent storage improvements
- [ ] Task statistics and progress charts
- [ ] Custom keyboard shortcuts
- [ ] Export/import functionality
- [ ] Theming options

## 🤝 Contributing

TaskX is open-source and we welcome contributions! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgements

- [Ratatui](https://github.com/ratatui-org/ratatui) - The TUI library powering TaskX
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library

---

<p align="center">Made with ❤️ in Rust</p>
