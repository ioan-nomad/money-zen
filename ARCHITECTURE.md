## DATABASE ARCHITECTURE UPDATE (October 8, 2025)

### Database Separation
MoneyZen now uses a completely separate database from N-OMAD:

**Location:**
- MoneyZen: `%LOCALAPPDATA%/io.moneyzen.app/money-zen.db`
- N-OMAD: `%LOCALAPPDATA%/MoneyZen/money-zen.db`

**Identifier Change:**
```json
// tauri.conf.json
{
  "identifier": "io.moneyzen.app"  // Changed from com.moneyzen.app
}
```

### Critical Fixes Implemented

#### 1. Database Path Resolution (main.rs)
**Problem:** Hardcoded paths using environment variables
**Solution:** Use Tauri's app.path() API

```rust
// BEFORE (hardcoded):
let db_path = PathBuf::from(env::var("LOCALAPPDATA").unwrap())
    .join("MoneyZen")
    .join("money-zen.db");

// AFTER (Tauri API):
use tauri::Manager;  // Added import
let app_data_dir = app.path().app_local_data_dir()
    .map_err(|e| format!("Failed to get app data dir: {}", e))?;
let db_path = app_data_dir.join("money-zen.db");
```

#### 2. SQLite DateTime Parsing (database.rs)
**Problem:** Backend panicked with ParseError(TooShort) on created_at field
**Root Cause:** SQLite stores datetime as "2025-10-08 08:00:39", but code expected RFC3339 format

```rust
// BEFORE (crashed):
created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
    .unwrap()
    .with_timezone(&Utc),

// AFTER (works with SQLite format):
created_at: {
    let datetime_str: String = row.get("created_at");
    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .and_utc()
},
```

### Categories Implementation
32 N-OMAD Categories successfully loaded with proper UUID format:

**Income (8 categories):**
- Salariu, Freelance, Investiții, Bonusuri, Cadouri Primite, Dividende, Vânzări, Altele (Venit)

**Expense (24 categories):**
- ATM Cash, Abonamente, Achiziții Diverse, Alimente, Altele, Asigurări, Concediu/Vacanță, Consumabile Casă, Donații, Economii, Electrocasanice, Electronice, Facturi & Utilități, Îmbrăcăminte, Întreținere Auto, Întreținere Casă, Mașină (Combustibil), Mobilier & Decorațiuni, Petreceri & Evenimente, Restaurant & Cafenea, Sănătate & Fitness, Taxe & Impozite, Transport Public, Educație

**Database Schema:**
```sql
CREATE TABLE categories (
    id TEXT PRIMARY KEY,           -- UUID format
    name TEXT NOT NULL,
    icon TEXT,
    color TEXT,
    category_type TEXT NOT NULL,   -- 'income' or 'expense'
    created_at TEXT NOT NULL       -- SQLite datetime format
);
```

### Git Commits
- `94a992a` - feat: change identifier to io.moneyzen.app - separate from N-OMAD
- `8b72082` - fix: SQLite datetime parsing + database path fixes

### Testing Verification
- ✅ Backend compiles without errors
- ✅ No ParseError panics
- ✅ 32 categories load in frontend dropdown
- ✅ Frontend-Backend connection stable
- ✅ Database persistence working
- ✅ Separate from N-OMAD (no conflicts)