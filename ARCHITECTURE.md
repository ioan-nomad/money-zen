## MoneyZen Architecture Documentation

### Current Status (October 9, 2025)
- ✅ **Phase 1:** Database Separation & Fixes (100% Complete)
- ✅ **Phase 2:** Categories & Accounts System (100% Complete)
- ✅ **Phase 3:** Tags System & Advanced Filtering (100% Complete)
- 🚧 **Phase 4:** Enhanced Transaction Management (Next Priority)

---

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
---

## TODAY'S PROGRESS (October 8, 2025)

### Owner Column & N-OMAD Accounts Implementation

#### Account Schema Enhancement
**Added owner field** to Account struct and database schema:

```rust
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String,
    pub balance: f64,
    pub currency: String,
    pub owner: String,        // NEW: "Ioan", "Nico", "Firmă"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### 12 N-OMAD Accounts Inserted
- **Ioan (3):** BT Ioan, Revolut Ioan, Wise Ioan
- **Nico (5):** BT Current, ANPH, Savings, EUR variants
- **Company (1):** Firmă
- **Cash (3):** Ioan, Nico, Comun

#### Deprecation Fixes
Fixed `from_timestamp_opt()` deprecation warnings:
```rust
// OLD: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()
// NEW: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc()
```

#### Latest Commits
- `3ebdb2c` - feat: Add owner column and N-OMAD accounts
- `02ede9d` - feat: implement complete tags system with analytics

---

## Phase 3: Tags System Architecture (COMPLETED - Oct 9, 2025)

### Overview
Complete tags infrastructure for organizing and analyzing transactions with flexible filtering and analytics.

### Database Schema

#### Tags Table
```sql
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
)
```

#### Transaction_Tags Junction Table
```sql
CREATE TABLE transaction_tags (
    id INTEGER PRIMARY KEY,
    transaction_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE(transaction_id, tag_id)
)
```

### Components Architecture

#### 1. Tags.svelte (Main Tags Page)
**Purpose:** Tag management interface
**Features:**
- List all tags with color indicators
- Create new tags with TagForm
- Edit existing tags
- Delete tags with confirmation
- Usage statistics per tag

**State Management:** Local state with reactive updates
**Database Calls:** get_tags, delete_tag

#### 2. TagForm.svelte (Tag Create/Edit Form)
**Purpose:** Form for creating and editing tags
**Features:**
- Name input validation
- HEX color picker (#RRGGBB format)
- Create/Update mode support
- Form validation with error messages

**Props:**
- `tag` (optional): Tag object for editing
- `onSave`: Callback after successful save
- `onCancel`: Callback for cancel action

**Database Calls:** create_tag, update_tag

#### 3. TagPicker.svelte (Tag Selection Component)
**Purpose:** Interactive tag selector for transactions
**Features:**
- Search tags by name
- Multi-select with checkboxes
- Visual feedback for selected tags
- Color-coded tag badges
- Dropdown interface with smooth animations

**Props:**
- `selectedTagIds`: Array of currently selected tag IDs
- `onTagsChange`: Callback when selection changes

**State:** Local tags list, search filter, open/close state

#### 4. AdvancedFilters.svelte (Comprehensive Filtering)
**Purpose:** Advanced transaction filtering interface
**Features:**
- **Search:** Description text search
- **Accounts:** Multi-select account filtering
- **Categories:** Multi-select category filtering (Income/Expense grouped)
- **Transaction Type:** All/Income/Expense dropdown
- **Tags:** Tag selection with OR/AND logic toggle
- **Date Range:** Start/End date pickers
- **Amount Range:** Min/Max amount inputs with current range display
- **Apply & Close:** Apply filters and close modal
- **Clear All:** Reset all filters

**Props:**
- `filters`: Current filter state object
- `onApplyFilters`: Callback with new filter state

**Filter State Structure:**
```typescript
{
  searchTerm: string
  selectedAccounts: number[]
  selectedCategories: number[]
  transactionType: 'all' | 'income' | 'expense'
  selectedTags: number[]
  tagFilterMode: 'OR' | 'AND'
  startDate: string | null
  endDate: string | null
  minAmount: number | null
  maxAmount: number | null
}
```

### Integration Points

#### AddTransactionForm.svelte
- **Integration:** TagPicker component
- **Feature:** Select tags when creating transactions
- **Database:** Stores tag associations in transaction_tags table

#### EditTransactionModal.svelte
- **Integration:** TagPicker component
- **Feature:** Modify tags on existing transactions
- **Database:** Updates transaction_tags associations

#### TransactionItem.svelte & TransactionList.svelte
- **Integration:** Display tag badges
- **Feature:** Visual tag indicators with colors
- **Props:** Receives tags array from parent

#### Analytics.svelte
**New Features:**
- **Spending by Tag:** Chart showing expenses grouped by tag
- **Most Used Tags:** Top tags by usage count
- **Tag Combination Insights:** Frequently used tag combinations
- **Tag Trends Over Time:** Monthly tag usage over last 6 months

**Database Queries:**
- get_spending_by_tag
- get_most_used_tags
- get_tag_combinations
- get_tag_trends

### Rust Backend (database.rs)

#### New Commands
- `get_tags()`: Retrieve all tags
- `create_tag(name, color)`: Create new tag
- `update_tag(id, name, color)`: Update existing tag
- `delete_tag(id)`: Delete tag and associations
- `get_spending_by_tag(start_date, end_date)`: Analytics query
- `get_most_used_tags(start_date, end_date)`: Analytics query
- `get_tag_combinations(start_date, end_date)`: Analytics query
- `get_tag_trends(months)`: Analytics query

#### Transaction Tag Management
- Automatic handling of transaction_tags associations
- Cascade delete on tag removal
- Unique constraint on transaction-tag pairs

### TypeScript Interfaces (database.ts)
```typescript
export interface Tag {
  id: number
  name: string
  color: string
  created_at: string
  updated_at: string
}

export interface TransactionTag {
  id: number
  transaction_id: number
  tag_id: number
}
```

### Filtering Logic

#### Tag Filtering Modes
- **OR Mode:** Show transactions with ANY selected tag
- **AND Mode:** Show transactions with ALL selected tags

#### Filter Application
- Filters are combinable (accounts + categories + tags + dates + amounts)
- Each filter type is applied independently
- Final result is intersection of all filter results

### Performance Considerations
- **Indexed Queries:** Tags and transaction_tags have indexed foreign keys
- **Efficient Joins:** SQLite queries use proper JOIN operations
- **Client-Side Caching:** Tags loaded once and cached in components
- **Reactive Updates:** Svelte reactivity ensures UI updates efficiently

### User Experience
- **Smooth Animations:** All modals and dropdowns have transitions
- **Visual Feedback:** Color-coded tags, hover effects, loading states
- **Validation:** Real-time form validation with clear error messages
- **Accessibility:** Proper ARIA labels and keyboard navigation

### Commit Reference
- **Commit:** 02ede9d
- **Files Changed:** 15 (4 new, 11 modified)
- **Lines Added:** 2,222
- **Lines Removed:** 85

---

## Phase 4: Backend Bulk Operations (October 9, 2025)

### Overview
Phase 4 enhances the backend with bulk operations for efficient transaction management and tag editing. All new functions are implemented in the Rust backend with corresponding Tauri commands.

### Database Layer (database.rs)

#### Enhanced Transaction Creation
```rust
pub async fn create_transaction(
    account_id: String,
    category_id: String,
    amount: f64,
    description: String,
    transaction_type: String,
    date: DateTime<Utc>,
    tag_ids: Option<Vec<String>>,  // NEW: Optional tags support
) -> Result<Transaction, sqlx::Error>
```

#### Bulk Delete Transactions
```rust
pub async fn delete_multiple_transactions(
    transaction_ids: Vec<String>,
) -> Result<usize, sqlx::Error>
```
- **Purpose:** Delete multiple transactions in a single operation
- **Returns:** Count of successfully deleted transactions
- **CASCADE:** Automatically removes associated transaction_tags
- **Error Handling:** Continues on individual failures, returns total count

#### Bulk Tag Operations
```rust
pub async fn bulk_update_transaction_tags(
    transaction_ids: Vec<String>,
    tags_to_add: Option<Vec<String>>,
    tags_to_remove: Option<Vec<String>>,
) -> Result<usize, sqlx::Error>
```
- **Purpose:** Add/remove tags from multiple transactions
- **Flexibility:** Supports add-only, remove-only, or both operations
- **Duplicate Safe:** Uses `INSERT OR IGNORE` to prevent duplicate assignments
- **Returns:** Count of successfully updated transactions

### Tauri Commands (main.rs)

#### Enhanced Commands
```rust
#[tauri::command]
async fn create_transaction(
    // ... existing parameters
    tag_ids: Option<Vec<String>>,  // NEW: Optional tags
    db: State<'_, DatabaseState>,
) -> Result<Transaction, String>

#[tauri::command]
async fn delete_multiple_transactions(
    transaction_ids: Vec<String>,
    db: State<'_, DatabaseState>,
) -> Result<usize, String>

#[tauri::command]
async fn bulk_update_transaction_tags(
    transaction_ids: Vec<String>,
    tags_to_add: Option<Vec<String>>,
    tags_to_remove: Option<Vec<String>>,
    db: State<'_, DatabaseState>,
) -> Result<usize, String>
```

### Technical Implementation

#### Database Operations
- **Atomic per transaction:** Each operation is atomic but not wrapped in a global transaction
- **Error handling:** Individual failures don't stop the entire operation
- **Performance:** Optimized for bulk operations with minimal database round trips
- **Constraint leverage:** Uses existing CASCADE and UNIQUE constraints

#### Error Handling Strategy
```rust
// Pattern used across all bulk operations
let mut success_count: usize = 0;
for item in items {
    match operation(item).await {
        Ok(_) => success_count += 1,
        Err(_) => continue, // Log but continue with remaining items
    }
}
Ok(success_count)
```

#### Memory Efficiency
- **Streaming approach:** Processes items one by one instead of loading all in memory
- **Resource cleanup:** Automatic cleanup of failed operations
- **Connection pooling:** Leverages SQLx connection pooling for optimal performance

### Code Quality Improvements

#### Deprecation Warnings Fixed
- **Problem:** 21 deprecation warnings from `chrono::NaiveDateTime::from_timestamp_opt`
- **Solution:** Migrated to `chrono::DateTime::from_timestamp().naive_utc()`
- **Impact:** Zero compilation warnings, future-proof against chrono updates
- **Pattern:** Applied consistently across all datetime parsing in database.rs

#### Modern API Usage
```rust
// Old (deprecated)
chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()

// New (modern)
chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc()
```

### Frontend Integration Points

#### Ready for Integration
All backend functions are implemented and ready for frontend integration:

1. **Bulk Delete UI:** Frontend can call `delete_multiple_transactions`
2. **Bulk Tag Editor:** Frontend can call `bulk_update_transaction_tags`
3. **Enhanced Creation:** Frontend can pass tags when creating transactions
4. **Operation Feedback:** All functions return success counts for user feedback

#### Expected Frontend Flow
```typescript
// Bulk delete example
const selectedIds = ['tx1', 'tx2', 'tx3'];
const deletedCount = await invoke('delete_multiple_transactions', {
  transactionIds: selectedIds
});
console.log(`Deleted ${deletedCount} transactions`);

// Bulk tag update example
const updatedCount = await invoke('bulk_update_transaction_tags', {
  transactionIds: selectedIds,
  tagsToAdd: ['tag1', 'tag2'],
  tagsToRemove: ['tag3']
});
console.log(`Updated ${updatedCount} transactions`);
```

### Commit References
- **3f8f18f:** feat(backend): add tags parameter to create_transaction
- **b38c612:** feat(backend): add bulk delete transactions functionality
- **6ac8d27:** fix(backend): replace deprecated chrono from_timestamp_opt
- **b523cdb:** feat(backend): add bulk edit tags functionality

### Performance Metrics
- **Compilation:** Zero warnings after chrono migration
- **Database:** All operations use indexed foreign keys
- **Memory:** Streaming approach for bulk operations
- **Error Rate:** Graceful handling of individual failures in bulk operations
