# PLAN DE MIGRARE - MoneyZen Refactoring

## CE AVEM ACUM
- 22 componente în src/lib/components/
- 9 pages în src/lib/
- database.ts monolith cu 40+ funcții

## CE VREM SĂ AVEM
- Repository Pattern
- Centralized State Management
- Clean Architecture

## ORDINEA MIGRĂRII (pentru Sonnet)

### SĂPTĂMÂNA 1: Core Infrastructure
1. **Luni:** Creează toate Repositories
2. **Marți:** Creează toate Services  
3. **Miercuri:** Creează toate Stores
4. **Joi:** Migrează primul page (Dashboard)
5. **Vineri:** Testing & Bug fixes

### SĂPTĂMÂNA 2: Migrare Pages
1. Transactions page
2. Accounts page
3. Categories page
4. Tags page
5. Analytics page

### SĂPTĂMÂNA 3: Polish & Deploy
1. Animații
2. Notifications
3. Production build
4. Installers

## REGULI PENTRU SONNET

1. **NU șterge nimic** până nu e înlocuit complet
2. **Testează după fiecare migrare**
3. **Commit după fiecare componentă**
4. **Dacă nu merge, STOP și întreabă**

## COMENZI UTILE
```bash
# Rulează aplicația
pnpm tauri dev

# Verifică ce nu merge
pnpm tauri dev --verbose

# Git status
git status

# Revert la ultima versiune stabilă
git checkout .
```
