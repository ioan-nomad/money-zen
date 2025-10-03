# 🎯 MASTER PLAN - MoneyZen
> The Big Picture - Documentul de referință pentru tot proiectul
> Created: October 2025
> Owner: Ioan
> Status: Planning Phase

## 1. VIZIUNE & MISIUNE

### Viziune
Creăm cea mai rapidă și intuitivă aplicație desktop de management financiar personal,
unde fiecare feature este gândit pentru SIMPLITATE și VITEZĂ.

### Misiune  
- 🎯 Sub 3 clickuri pentru orice acțiune comună
- ⚡ Sub 100ms response time pentru orice operațiune
- 🎨 Interfață modernă care nu obosește ochii
- 🔒 Date 100% locale, 100% private
- 📱 Pregătită pentru expansiune mobile (future-proof)

### Anti-Goals (Ce NU vrem)
- ❌ Feature creep - mai bine 10 features perfecte decât 100 mediocre
- ❌ Dependență de cloud/internet
- ❌ Interfață complexă care necesită training
- ❌ Performanță sub-optimă "merge și așa"
- ❌ Cod neclar pe care nu-l poate menține AI-ul

## 2. USER PERSONA

**Ioan - Power User Pragmatic**
- Zero răbdare pentru lag sau bugs
- Vrea să vadă instant unde se duc banii
- Import rapid din PDF-uri bancare
- Rapoarte clare pentru decizii rapide
- Dark mode obligatoriu
- Keyboard shortcuts pentru tot

## 3. CORE FEATURES (MVP)

### 🏠 Dashboard Principal
- Sold total (toate conturile)
- Grafic income vs expenses (ultima lună)
- Top 5 cheltuieli recente
- Quick stats (cheltuit azi/săptămână/lună)

### 💳 Gestiune Tranzacții
- Add/Edit/Delete cu form simplu
- Categorii predefinite smart
- Tags opționale pentru detalii
- Duplicate detection

### 🏦 Conturi Multiple
- Cont principal, economii, cash
- Transfer între conturi
- Reconciliere cu extrasul bancar
- Arhivare conturi vechi

### 📊 Vizualizări
- Pie chart categorii
- Line chart trend lunar
- Bar chart comparație luni
- Heatmap calendar cheltuieli

### 📥 Import/Export
- CSV import (toate băncile)
- PDF parser pentru extrase
- JSON backup complet
- Excel export pentru rapoarte

## 4. TECH STACK FINAL
```yaml
FRONTEND:
  Framework: Svelte 5 (latest runes)
  UI: DaisyUI v4 + TailwindCSS v4
  Charts: Chart.js v4
  State: Svelte stores + TanStack Query
  
BACKEND:
  Runtime: Tauri 2.0 (Rust)
  Database: SQLite
  ORM: Drizzle
  
TOOLING:
  Build: Vite 6
  Package Manager: pnpm (faster)
  Linting: BiomeJS
  Git: Conventional Commits

## 5. FOLDER STRUCTURE
```
money-zen/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── db/
│   │   └── commands/
│   └── tauri.conf.json
│
├── src/                # Svelte frontend
│   ├── routes/
│   ├── lib/
│   │   ├── components/
│   │   ├── stores/
│   │   └── utils/
│   └── app.css
│
├── docs/               # Documentație
└── tests/              # Testing
```

## 6. DEVELOPMENT PHASES

### Phase 0: Foundation ✅ [CURRENT]
- ✅ Structură documentație
- ⏳ Setup Tauri + Svelte 5
- ⏳ Git init + first commit
- ⏳ Development environment

### Phase 1: Backend Core [Week 1]
- ⏳ SQLite setup
- ⏳ Schema design
- ⏳ Basic CRUD
- ⏳ Error handling

### Phase 2: Frontend Base [Week 2]
- ⏳ Layout & routing
- ⏳ DaisyUI setup
- ⏳ Forms & validation
- ⏳ Dark mode

### Phase 3: Core Features [Week 3-4]
- ⏳ Dashboard
- ⏳ Transactions CRUD
- ⏳ Accounts management
- ⏳ Basic charts

### Phase 4: Import/Export [Week 5]
- ⏳ CSV parser
- ⏳ PDF reader
- ⏳ Backup system
- ⏳ Reports

### Phase 5: Polish [Week 6]
- ⏳ Performance
- ⏳ Animations
- ⏳ Keyboard shortcuts
- ⏳ Testing

## 7. SUCCESS CRITERIA
- ✅ Boot time < 2 secunde
- ✅ CRUD operations < 50ms
- ✅ Bundle size < 10MB
- ✅ Memory usage < 100MB
- ✅ 0 critical bugs în producție

## 8. CONVENTIONS

### Git Commits
- `feat: add transaction form`
- `fix: correct balance calculation`
- `docs: update API documentation`
- `style: format code`
- `refactor: simplify store logic`
- `test: add unit tests`
- `chore: update dependencies`

### Code Style
- Max 20 lines per function
- Descriptive names (not x, y, z)
- Comments în română OK
- Error messages în engleză

## 9. RISC & MITIGARE

| Risc | Impact | Mitigare |
|------|--------|----------|
| Scope creep | High | Feature freeze după MVP |
| SQLite limits | Medium | Pagination + indexing |
| Tauri bugs | Low | Stable version only |
| Design paralysis | Medium | DaisyUI defaults |

## 10. NEXT IMMEDIATE STEPS
- ✅ Create documentation structure
- ⏳ Setup Tauri + Svelte 5 project
- ⏳ Initialize Git repository
- ⏳ Configure development environment
- ⏳ First "Hello World" window

---
**Last Updated:** October 3, 2025  
**Next Review:** After Phase 0 completion
