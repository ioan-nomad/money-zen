# 💰 MoneyZen

> Modern, fast, and intuitive desktop finance management app built with Tauri and Svelte 5

[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-blue)](https://tauri.app)
[![Svelte 5](https://img.shields.io/badge/Svelte-5-orange)](https://svelte.dev)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

## 🚀 Features

- **⚡ Lightning Fast** - Native performance with Rust backend
- **🔒 Privacy First** - All data stored locally, no cloud dependency
- **🎨 Modern UI** - Beautiful dark/light themes with DaisyUI
- **📊 Smart Analytics** - Visualize your spending patterns
- **📥 Bank Import** - Parse PDF statements and CSV files
- **💼 Multi-Account** - Manage multiple accounts seamlessly

## 🛠️ Tech Stack

- **Frontend:** Svelte 5 + TypeScript
- **Backend:** Tauri 2.0 (Rust)
- **Database:** SQLite
- **UI:** DaisyUI + TailwindCSS
- **Charts:** Chart.js
- **Build:** Vite

## 📦 Installation

### Prerequisites
- Node.js 20+
- Rust 1.75+
- pnpm (recommended)

### Development Setup
```bash
# Clone repository
git clone https://github.com/ioan-nomad/money-zen.git
cd money-zen

# Install dependencies
pnpm install

# Run in development
pnpm tauri dev

# Build the app
pnpm tauri build

# Output will be in src-tauri/target/release
```

### Production Build
```bash
# Build optimized version
pnpm tauri build

# Outputs:
# - Windows: src-tauri/target/release/bundle/msi/
# - macOS: src-tauri/target/release/bundle/dmg/
# - Linux: src-tauri/target/release/bundle/deb/
```

## 🏗️ Project Structure

```
money-zen/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── db/
│   │   └── commands/
│   └── tauri.conf.json
├── src/                # Svelte frontend
│   ├── routes/
│   ├── lib/
│   │   ├── components/
│   │   ├── stores/
│   │   └── utils/
│   └── app.css
├── docs/               # Documentation
└── tests/              # Testing
```

## 📚 Documentation

- [Master Plan](docs/MASTER_PLAN.md) - Complete project vision and roadmap
- [Architecture](docs/ARCHITECTURE.md) - Technical architecture details
- [Features](docs/FEATURES.md) - Detailed feature specifications
- [Development Log](docs/DAILY_LOG.md) - Daily progress tracking

## 🎯 Current Status

**Phase 0: Foundation** ✅
- [x] Documentation structure
- [ ] Tauri + Svelte setup
- [ ] Git repository initialization
- [ ] Development environment

## 🤝 Contributing

1. Check the [Master Plan](docs/MASTER_PLAN.md) for project direction
2. Follow conventional commit format
3. Keep functions under 20 lines
4. Test your changes thoroughly

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

## 🔗 Links

- [Tauri Documentation](https://tauri.app)
- [Svelte 5 Guide](https://svelte.dev)
- [DaisyUI Components](https://daisyui.com)

---

**Made with ❤️ for personal finance management**
