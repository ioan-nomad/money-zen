# TODO - MoneyZen Task Tracker
> Last Updated: October 5, 2025 (18:00)

## PHASE 3: PRODUCTION UI ✅ COMPLETE (100%)

**Committed:** 20f4b96

### ACHIEVEMENTS

**6 Reusable Components:**
- AccountCard.svelte (901 bytes)
- TransactionItem.svelte (768 bytes)
- CategoryBadge.svelte (515 bytes)
- AddTransactionForm.svelte (2,483 bytes)
- TransactionList.svelte (2,447 bytes)
- AccountList.svelte (1,270 bytes)

**4 Production Pages:**
- Dashboard.svelte - Overview with accounts + recent transactions
- Transactions.svelte - Full filtering (search, account, category, type)
- Accounts.svelte - CRUD with edit/delete modals (2,710 bytes)
- Analytics.svelte - Summary cards + top 5 categories + PDF export (4.5K)

**Navigation:**
- 6-tab system (Dashboard, Transactions, Accounts, Analytics, Welcome, Test)

**Backend Integration:**
- Full CRUD for accounts (create, read, update, delete)
- Transaction management with auto-balance
- TypeScript wrappers complete

**Analytics Features:**
- Summary cards (income, expense, net balance)
- Top 5 spending categories with progress bars
- PDF export (jsPDF + autoTable)
- Professional financial reports

---

## PHASE 4: IMPORT/EXPORT & DATA MANAGEMENT ⏳ IN PROGRESS (0%)

**Started:** October 6, 2025
**Estimated:** 6-8 hours total
**Status:** Documentation phase

### 4.1 SQLite Backup System (Priority 1 - 1h)

**Backend (Rust):**
- [ ] Create `backup_database()` command in main.rs
- [ ] Create `restore_database()` command in main.rs
- [ ] Create `list_backups()` command to get all backup files
- [ ] Use Tauri fs API for file operations
- [ ] Test backup: verify file copied correctly
- [ ] Test restore: verify data restored correctly

**Frontend (Svelte):**
- [ ] Create BackupManager.svelte component
- [ ] Add "Create Backup" button with timestamp generation
- [ ] Add "Restore from Backup" with file picker
- [ ] Display backup history list with dates
- [ ] Add TypeScript wrappers in database.ts
- [ ] Success/error notifications
- [ ] Visual confirmation before restore

**Testing:**
- [ ] Backup creates valid SQLite file
- [ ] Restore overwrites current database
- [ ] App restarts correctly after restore
- [ ] No data loss or corruption

**Location:** Add to Database Test tab or create Settings page

---

### 4.2 XLSX Import (Priority 2 - 3-4h)

**Phase 1: File Upload & Parsing**
- [ ] Create Import.svelte page
- [ ] Add drag & drop zone for .xlsx files
- [ ] Use Tauri fs API to read file
- [ ] Parse with SheetJS → get rows array
- [ ] Display file info (name, rows count, columns)
- [ ] Extract column headers automatically

**Phase 2: Column Mapping UI**
- [ ] Create ColumnMapper.svelte component
- [ ] Dropdown for each required field:
  - Date column → transaction date
  - Amount column → transaction amount
  - Description column → transaction description
  - Type column (optional) → income/expense
- [ ] Preview mapping with first 5 rows
- [ ] Validate: all required fields mapped

**Phase 3: Data Transformation**
- [ ] Parse dates (handle multiple formats: DD/MM/YYYY, DD.MM.YYYY)
- [ ] Parse amounts (handle: "1.234,56", "-500", "500 RON")
- [ ] Detect income vs expense (negative = expense, positive = income)
- [ ] Auto-categorize based on description keywords
- [ ] Handle missing values gracefully

**Phase 4: Preview & Validation**
- [ ] Show first 10 transformed transactions
- [ ] Display warnings (missing dates, invalid amounts)
- [ ] Allow user to select target account
- [ ] Duplicate detection logic (same date + amount + description)
- [ ] Confirm import button

**Phase 5: Batch Insert**
- [ ] Create `batch_insert_transactions()` Rust command
- [ ] SQLite transaction for atomic insert
- [ ] Update account balances
- [ ] Progress indicator (importing X of Y...)
- [ ] Success summary (imported 487 transactions, 3 duplicates skipped)

**Testing:**
- [ ] Test with real bank statement (BT, BCR, Revolut formats)
- [ ] Test with edge cases (empty rows, invalid dates)
- [ ] Verify balance calculations correct
- [ ] Verify no duplicates created
- [ ] Test rollback on error

**Location:** New "Import" tab in App.svelte navigation

---

### 4.3 Advanced PDF Reports (Priority 3 - 2h)

**Backend Queries:**
- [ ] Add `get_transactions_by_month()` to database.rs
- [ ] Add `get_transactions_by_account()` to database.rs
- [ ] Add `get_transactions_by_category()` to database.rs
- [ ] Add `get_transactions_by_date_range()` to database.rs

**Frontend UI:**
- [ ] Add report type selector to Analytics.svelte
  - Monthly Report
  - Account Report
  - Category Report
  - Date Range Report
- [ ] Add date pickers for range selection
- [ ] Add account dropdown (populated from DB)
- [ ] Add category dropdown (populated from DB)
- [ ] Preview button → show filtered transactions

**PDF Generation:**
- [ ] Extend existing jsPDF code in Analytics.svelte
- [ ] Add report title with filters applied
- [ ] Add summary statistics at top
- [ ] Professional header with logo/app name
- [ ] Footer with page numbers and generation date
- [ ] Table with all filtered transactions
- [ ] Charts for visual analysis (optional)

**Testing:**
- [ ] Generate monthly report → verify correct transactions
- [ ] Generate account report → verify filtering works
- [ ] Test with 500+ transactions → verify performance
- [ ] Verify PDF formatting looks professional

**Location:** Analytics.svelte - dropdown "Export Options"

---

### 4.4 Documentation & Git

- [ ] Update ARCHITECTURE.md with implementation details
- [ ] Update TODO.md after each feature completion
- [ ] Git commit after each working feature:
  - `feat: add SQLite backup system`
  - `feat: add XLSX import functionality`
  - `feat: add advanced PDF reports`
- [ ] Final commit: `docs: Phase 4 complete`

---

**METHODOLOGY REMINDER:**
- Max 20 lines per change → test immediately
- Document BEFORE coding
- Test after EVERY package install
- Git commit after EVERY working feature
- Turtle vs Rabbit - slow and steady wins

---

## PHASE 5: POLISH (Future)

- Performance optimization
- Smooth animations
- Keyboard shortcuts
- Comprehensive testing

---

**PHASE 3 COMPLETED:** October 5, 2025 after ~7 hours of focused development
**Next:** Phase 4 planning
