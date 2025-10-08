# MoneyZen - Current Project Context
**Last Updated:** October 8, 2025
**Status:** ✅ Database separation complete, app fully functional

## Quick Start for New Conversations

### What MoneyZen Is
A modern Tauri desktop finance tracking application using:
- **Backend:** Rust + Tauri 2.0 + SQLite
- **Frontend:** Svelte 5 + TailwindCSS + DaisyUI
- **Database:** SQLite with local storage

### Current Working State
**App Status:** ✅ FULLY FUNCTIONAL
- Backend: Stable, no crashes
- Frontend: Connected, displays all features
- Database: Independent, separate from N-OMAD
- Categories: 32 N-OMAD categories loaded

**Running the app:**
```bash
npm run tauri dev
```

## Recent Major Changes (Oct 8, 2025)

### 1. Database Separation
- **Old:** Shared database with N-OMAD at `%LOCALAPPDATA%/MoneyZen/`
- **New:** Independent database at `%LOCALAPPDATA%/io.moneyzen.app/`
- **Reason:** Prevented category conflicts between apps

### 2. Critical Fixes
- Fixed SQLite datetime parsing (created_at field)
- Replaced hardcoded paths with Tauri app.path() API
- Added `use tauri::Manager` import for path resolution

### 3. Categories Implementation
- 32 N-OMAD categories with proper UUIDs
- 8 income + 24 expense categories
- All categories visible in frontend dropdown

## Key Files to Know

### Backend (Rust):
- `src-tauri/src/main.rs` - Main entry, database initialization, Tauri commands
- `src-tauri/src/database.rs` - Database operations, queries, schema
- `src-tauri/tauri.conf.json` - App configuration, identifier: "io.moneyzen.app"

### Frontend (Svelte):
- `src/lib/stores/database.ts` - Database connection & queries
- `src/lib/components/` - Svelte components
- `src/routes/` - App pages/routes

### Documentation:
- `ARCHITECTURE.md` - Full technical architecture & recent fixes
- `TODO.md` - Task tracking, completed items
- `COMMANDS.md` - Useful development commands

## Database Schema
**Tables:**
- `accounts` - User financial accounts
- `categories` - Transaction categories (income/expense)
- `transactions` - Financial transactions

**Important:** Categories use UUID format for IDs, datetime uses SQLite format (YYYY-MM-DD HH:MM:SS)

## Common Development Commands
```bash
# Run dev server
npm run tauri dev

# Build for production
npm run tauri build

# Run tests
npm test

# Check Git status
git status
git log --oneline -5
```

## Known Working Features
✅ Backend-Frontend communication via Tauri IPC
✅ Database queries (get_categories, get_accounts, etc.)
✅ Category dropdown with 32 categories
✅ SQLite datetime parsing
✅ Dynamic database path resolution
✅ Separate database from N-OMAD

## Development Philosophy
- **"Turtle vs Rabbit"** - Slow, methodical, no mistakes
- Max 20 lines per change
- Test after every modification
- Git commit after each working feature
- Document everything

## Next Potential Tasks
- [ ] Implement transaction creation/editing
- [ ] Add account management UI
- [ ] Dashboard with statistics
- [ ] Data visualization (charts/graphs)
- [ ] Backup/restore functionality
- [ ] Export to CSV/Excel

## Important Notes for AI Assistants
- User has ZERO coding experience - explain architecture, not code
- User coordinates via Claude Code - give exact commands
- Always verify changes visually after implementation
- Never refactor working code without explicit permission
- Performance > Features - must be FAST
- Modern UI only - Ferrari mindset, not Dacia

## Git Repository
- **Remote:** https://github.com/ioan-nomad/money-zen.git
- **Branch:** master
- **Latest commits:**
  - `8b72082` - fix: SQLite datetime parsing + database path fixes
  - `94a992a` - feat: change identifier to io.moneyzen.app

## Troubleshooting Reference

### If backend panics:
- Check SQLite datetime format in queries
- Verify all paths use app.path() API, not hardcoded
- Ensure `use tauri::Manager` is imported

### If categories don't load:
- Verify database exists at `%LOCALAPPDATA%/io.moneyzen.app/`
- Check categories have proper UUID format
- Inspect browser console (F12) for frontend errors

### If database conflicts:
- Confirm identifier in tauri.conf.json is "io.moneyzen.app"
- Each app should have separate database folder