# ARCHITECTURE - MoneyZen
> Last Updated: October 5, 2025 (18:15)

## PHASE 3: PRODUCTION UI - 100% COMPLETE

**Latest Commit:** 67927e8
**Status:** All features implemented and tested
**Development Time:** ~7 hours (October 5, 2025)

### 3.1 Reusable Components (6 total)
- AccountCard.svelte (901 bytes)
- TransactionItem.svelte (768 bytes)
- CategoryBadge.svelte (515 bytes)
- AddTransactionForm.svelte (2,483 bytes)
- TransactionList.svelte (2,447 bytes)
- AccountList.svelte (1,270 bytes)

### 3.2 Dashboard Page
- Dashboard.svelte (2,451 bytes)
- Two-column layout
- Total balance calculation
- Recent transactions (5 most recent)
- Account overview cards

### 3.3 Transactions Page
- Transactions.svelte (866 bytes)
- TransactionList with 4-way filtering
- Search by description
- Filter by account, category, type
- Empty state handling

### 3.4 Accounts Page
- Accounts.svelte (2,710 bytes)
- Full CRUD operations
- Edit modal with form validation
- Delete confirmation with CASCADE warning
- AccountList integration
- Error handling and auto-refresh

### 3.5 Analytics Page
- Analytics.svelte (4.5K)
- Summary cards (income, expense, net balance)
- Top 5 spending categories with progress bars
- PDF export functionality (jsPDF + autoTable)
- Real-time calculations from transaction data

### 3.6 Navigation System
- 6-tab navigation (Dashboard, Transactions, Accounts, Analytics, Welcome, Test)
- Enum-based state management
- TypeScript type safety
- Clean conditional rendering

### 3.7 Backend Integration
- update_account() - Rust function with TypeScript wrapper
- delete_account() - CASCADE deletion of transactions
- All CRUD operations complete
- Database.updateAccount() and Database.deleteAccount()

## FILE STRUCTURE (Current)
money-zen/
src/
App.svelte (6-tab navigation)
DatabaseTest.svelte
lib/
Dashboard.svelte (2,451 bytes)
Transactions.svelte (866 bytes)
Accounts.svelte (2,710 bytes)
Analytics.svelte (4.5K)
database.ts (with update/delete wrappers)
utils.ts
components/
AccountCard.svelte (901 bytes)
TransactionItem.svelte (768 bytes)
CategoryBadge.svelte (515 bytes)
AddTransactionForm.svelte (2,483 bytes)
TransactionList.svelte (2,447 bytes)
AccountList.svelte (1,270 bytes)
src-tauri/
src/
main.rs (update_account, delete_account commands)
database.rs (full CRUD implementation)
docs/
ARCHITECTURE.md (this file)
TODO.md
package.json (jsPDF 3.0.3, jspdf-autotable 5.0.2)

## TECH STACK
- Frontend: Svelte 4.2.20 + TypeScript
- Backend: Rust (Tauri 2.0) + SQLite
- UI: TailwindCSS 3.4.18 + DaisyUI 5.1.27
- PDF: jsPDF 3.0.3 + autoTable
- Build: 429 modules, 8.03s

---

## PHASE 4: IMPORT/EXPORT & DATA MANAGEMENT

**Status:** Planning (October 6, 2025)
**Estimated Time:** 6-8 hours
**Goal:** Enable data import/export and backup functionality

### 4.1 SQLite Backup System (Priority 1 - 1h)
**What:** Manual backup and restore of the SQLite database file
**Why:** Data protection - users need ability to backup their financial data
**How:** Direct file copy using Tauri File API

**Features:**
- Backup button: Copy `money-zen.db` → `Documents/MoneyZen Backups/backup-YYYY-MM-DD.db`
- Restore button: Select backup file → copy to active database location
- Backup history list with dates
- Auto-backup reminder (optional)

**Tech:**
- Tauri File API for file operations
- Rust backend for safe file copying
- No data transformation needed (raw SQLite file)

**Location:** Settings page or Database Test tab

---

### 4.2 XLSX Import (Priority 2 - Started)

**Status:** In Progress (October 6, 2025)
**Goal:** Import bank transactions from Excel files

**What:** Parse Excel files and bulk import transactions
**Why:** Eliminate manual entry - import hundreds of transactions in seconds
**How:** SheetJS parsing → column mapping → validation → batch insert

**User Flow:**
1. User uploads .xlsx file (drag & drop or file picker)
2. System parses file and extracts columns
3. User maps columns: Date → transaction date, Amount → amount, etc.
4. System validates and shows preview (first 10 rows)
5. User confirms → batch insert to SQLite
6. Success message with count (e.g., "Imported 487 transactions")

**Technical Implementation:**

**Backend (Rust):**
- batch_insert_transactions(transactions: Vec<Transaction>) command
- SQLite transaction for atomic insert
- Auto-update account balances
- Duplicate detection (date + amount + description)

**Frontend (Svelte):**
- Import.svelte page with file upload
- SheetJS for parsing Excel files
- ColumnMapper component for field mapping
- Preview table showing first 10 rows
- Validation with error messages
- Progress indicator during import

**Data Transformations:**
- Date parsing: DD/MM/YYYY, DD.MM.YYYY, YYYY-MM-DD
- Amount parsing: "1.234,56", "-500", "500.00"
- Type detection: negative = expense, positive = income
- Description cleanup: trim whitespace, normalize encoding

**Edge Cases:**
- Empty rows → skip
- Invalid dates → error with row number
- Invalid amounts → error with row number
- Duplicate transactions → skip with warning
- Missing required fields → error before import

**Files:**
- src/lib/Import.svelte (main page)
- src/lib/components/ColumnMapper.svelte (mapping UI)
- src-tauri/src/main.rs (batch_insert_transactions command)

---

### 4.3 Advanced PDF Reports (Priority 3 - 2h)
**What:** Enhanced PDF export with filtering options
**Why:** Users need specific reports (monthly, per account, per category)
**How:** Extend existing jsPDF Analytics with query filters

**Report Types:**
- **Monthly Report:** All transactions for selected month
- **Account Report:** All transactions for specific account
- **Category Report:** All spending in specific category
- **Date Range Report:** Custom date range

**Features:**
- Dropdown selector for report type
- Date pickers for range
- Account/category dropdowns
- Preview before export
- Professional formatting (header, footer, page numbers)
- Summary statistics at top

**Tech:**
- jsPDF + autoTable (already integrated)
- SQLite queries with WHERE clauses
- Chart.js for embedded charts (optional)
- Same PDF generation as Analytics.svelte

**Location:** Analytics page - "Export Options" dropdown

---

### 4.4 Data Migration Tools (Phase 5 candidate)
**What:** Import from other finance apps (YNAB, Wallet, etc.)
**Why:** Help users migrate without losing historical data
**Status:** Lower priority - postponed to Phase 5

---

## PHASE 4 IMPLEMENTATION ORDER

1. **SQLite Backup** (Day 1 morning - 1h)
   - Fast win
   - Critical for data safety
   - Simple implementation

2. **XLSX Import** (Day 1 afternoon - 3-4h)
   - Most complex feature
   - Highest user value
   - Core functionality

3. **PDF Reports** (Day 2 - 2h)
   - Builds on existing Analytics
   - Polish feature
   - User delight

**Total Estimated:** 6-8 hours of focused work

---
**Phase 3 Complete:** October 5, 2025
**Author:** Ioan + Claude Code
**Next:** Phase 4 implementation
