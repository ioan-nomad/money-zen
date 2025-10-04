# 🛠️ COMMANDS - MoneyZen Reference
> Quick Reference for All Useful Commands
> Last Updated: October 4, 2025

## 📦 PACKAGE MANAGEMENT

### Install Dependencies
```bash
# Install all dependencies
pnpm install

# Install specific package
pnpm add <package-name>

# Install dev dependency
pnpm add -D <package-name>

# Update all packages
pnpm update

# Remove package
pnpm remove <package-name>
```

### Clean Install
```bash
# Remove node_modules and reinstall
rm -rf node_modules pnpm-lock.yaml
pnpm install
```

## 🚀 DEVELOPMENT

### Run Development Server
```bash
# Start Tauri dev server (hot reload)
pnpm tauri dev

# Start only frontend (without Tauri)
pnpm dev
```

### Build Production
```bash
# Build optimized production version
pnpm tauri build

# Output locations:
# Windows: src-tauri/target/release/bundle/msi/
# macOS: src-tauri/target/release/bundle/dmg/
# Linux: src-tauri/target/release/bundle/deb/
```

### Type Checking
```bash
# Check TypeScript types
pnpm check

# Check types in watch mode
pnpm check --watch
```

## 🦀 RUST/CARGO COMMANDS

### Build Backend
```bash
# Build Rust backend only
cd src-tauri
cargo build

# Build release version
cargo build --release

# Check for errors without building
cargo check
```

### Clean Build Artifacts
```bash
cd src-tauri
cargo clean

# Clean and rebuild from scratch
cargo clean && cargo build
```

### Update Rust Dependencies
```bash
cd src-tauri
cargo update
```

## 🗄️ DATABASE COMMANDS

### Database Location
```bash
# Windows location
%LOCALAPPDATA%\MoneyZen\money-zen.db

# Full path
C:\Users\<username>\AppData\Local\MoneyZen\money-zen.db
```

### View Database
```bash
# Using SQLite CLI (if installed)
sqlite3 "%LOCALAPPDATA%\MoneyZen\money-zen.db"

# Common SQLite commands:
.tables          # List all tables
.schema          # Show schema
SELECT * FROM accounts;
SELECT * FROM categories;
SELECT * FROM transactions;
.quit            # Exit
```

### Backup Database
```bash
# Create backup
copy "%LOCALAPPDATA%\MoneyZen\money-zen.db" "backup-$(date +%Y%m%d).db"

# Restore from backup
copy "backup-20251004.db" "%LOCALAPPDATA%\MoneyZen\money-zen.db"
```

### Reset Database
```bash
# Delete database (app will recreate on next launch)
rm "%LOCALAPPDATA%\MoneyZen\money-zen.db"
```

## 📝 GIT COMMANDS

### Basic Workflow
```bash
# Check status
git status

# Stage all changes
git add .

# Commit with conventional format
git commit -m "feat: add new feature"
git commit -m "fix: correct bug"
git commit -m "docs: update documentation"

# Push to remote
git push

# Pull latest changes
git pull
```

### Commit Message Formats
```bash
feat:     # New feature
fix:      # Bug fix
docs:     # Documentation changes
style:    # Code style changes (formatting)
refactor: # Code refactoring
test:     # Adding/updating tests
chore:    # Maintenance tasks
```

### Branch Management
```bash
# Create new branch
git checkout -b feature/new-feature

# Switch branches
git checkout main

# List all branches
git branch -a

# Delete branch
git branch -d feature/old-feature
```

### View History
```bash
# View commit log
git log

# View last 5 commits
git log --oneline -5

# View changes in last commit
git show

# View file history
git log --follow <file>
```

## 🔍 PROJECT INSPECTION

### Check Installed Versions
```bash
# Node.js version
node --version

# pnpm version
pnpm --version

# Rust version
rustc --version

# Cargo version
cargo --version

# Tauri CLI version
pnpm tauri --version
```

### View Project Structure
```bash
# List all files (Windows)
tree /F /A

# List only directories
tree /A

# Alternative (Git Bash)
find . -type f | head -20
```

### Check File Sizes
```bash
# List files with sizes
ls -lah src/
ls -lah src-tauri/src/
```

### View Package Info
```bash
# View all installed packages
cat package.json

# View Rust dependencies
cat src-tauri/Cargo.toml

# Check for outdated packages
pnpm outdated
```

## 🧹 CLEANUP COMMANDS

### Clean Build Artifacts
```bash
# Remove frontend build
rm -rf dist/

# Remove Tauri build artifacts
cd src-tauri
cargo clean
cd ..

# Remove node_modules
rm -rf node_modules/
```

### Full Project Reset
```bash
# Nuclear option - clean everything
rm -rf node_modules/ pnpm-lock.yaml dist/
cd src-tauri && cargo clean && cd ..
pnpm install
```

## 🐛 DEBUGGING

### View Console Logs
```bash
# Frontend logs appear in Tauri dev console
# Backend Rust logs appear in terminal running 'pnpm tauri dev'

# Add debug prints in Rust:
println!("Debug: {:?}", variable);
eprintln!("Error: {}", error);
```

### Check for Compilation Errors
```bash
# TypeScript errors
pnpm check

# Rust errors
cd src-tauri && cargo check
```

### Verbose Build
```bash
# Build with verbose output
pnpm tauri build --verbose
```

## 📚 QUICK REFERENCE

### Most Used Commands (Daily)
```bash
pnpm tauri dev           # Start development
git status               # Check changes
git add . && git commit  # Save work
git push                 # Backup to remote
```

### Emergency Commands
```bash
# App won't start?
rm -rf node_modules/ && pnpm install

# Database corrupted?
rm "%LOCALAPPDATA%\MoneyZen\money-zen.db"

# Build stuck?
cd src-tauri && cargo clean && cd .. && pnpm tauri dev
```

## 🎯 NEXT STEPS COMMANDS

### When Ready for Phase 2
```bash
# Example: Installing TailwindCSS (DO NOT RUN YET)
pnpm add -D tailwindcss@latest postcss autoprefixer
npx tailwindcss init -p

# Example: Installing DaisyUI (DO NOT RUN YET)
pnpm add -D daisyui@latest
```

**Note:** Always test after running ANY command. If something breaks, check git status and revert if needed.