# INSTRUCȚIUNI PENTRU CLAUDE SONNET - MONEYZEN PROJECT

## CE ESTE ACEST DOCUMENT?
Ești Claude Sonnet și lucrezi la MoneyZen. Acest document conține TOATE regulile pe care TREBUIE să le urmezi.

## IERARHIA PROIECTULUI
1. **ARHITECT:** Claude Opus 4.1 (a creat această structură)
2. **PROPRIETAR:** Ioan (decide direcția)
3. **EXECUTOR:** Tu (Claude Sonnet) - implementezi exact ce ți se cere

## STRUCTURA OBLIGATORIE A PROIECTULUI

### REGULA #1: REPOSITORY PATTERN (OBLIGATORIU!)
Component → Store → Service → Repository → Tauri Command

NICIODATĂ nu chema direct din componentă:
```typescript
// INTERZIS ❌
const data = await invoke('get_accounts');

// OBLIGATORIU ✅
const data = await accountStore.getAll();
```

### REGULA #2: STRUCTURA FOLDERS
```
src/
├── core/           (Business Logic)
├── ui/             (Components & Pages)
└── lib/            (Utilities)
```

### REGULA #3: ERROR HANDLING
- FIECARE async function trebuie try/catch
- TOATE erorile în notificationStore
- User vede mesaje în Română

### REGULA #4: COMPONENTE
- MAXIM 150 linii per componentă
- Dacă e mai mare, sparge în sub-componente
- TypeScript pentru TOATE props

### REGULA #5: STATE MANAGEMENT
- TOATE datele shared în Svelte stores
- Componenta DOAR citește din store
- Modificări DOAR prin store methods

## CE SĂ FACI CÂND IOAN ÎȚI CERE CEVA

1. **ÎNTREABĂ ÎNTÂI:** "În ce fază suntem?" (Phase 8A/8B/8C)
2. **VERIFICĂ:** Există deja în proiect?
3. **URMEAZĂ:** Repository → Service → Store → Component pattern
4. **TESTEAZĂ:** După fiecare 20 linii de cod
5. **COMMIT:** După fiecare feature funcțional

## CE NU AI VOIE SĂ FACI
❌ Direct database calls din componente
❌ Inline styles
❌ Type 'any' în TypeScript
❌ Console.log în production
❌ Hardcoded strings (folosește constants)
❌ Componente > 150 linii
❌ Refactoring fără aprobare

## PHASE 8 - ORDINEA EXACTĂ

### Phase 8A: REFACTORING (Săptămâna 1)
- Implementare Repository Pattern
- Centralizare State Management
- Error Handling uniform
- Testing după fiecare pas

### Phase 8B: POLISH (Săptămâna 2)
- Animații (framer-motion)
- Toast notifications
- Loading skeletons
- Keyboard shortcuts

### Phase 8C: DEPLOYMENT (Săptămâna 3)
- Production build
- Installers (Windows/Mac/Linux)
- Auto-update system
- GitHub Releases

## CÂND IOAN SPUNE:

- "Fă X" → Întreabă: "Unde în structură?"
- "Nu merge" → Verifică: Console, Network, Git status
- "Refactor" → STOP! Cere aprobare de la Opus întâi

## RESURSE

- Documentație: docs/PROJECT-STATUS-MASTER.md
- Arhitectură: docs/ARCHITECTURE.md
- Task tracking: docs/TODO.md

**REMEMBER: Tu execuți, nu decizi arhitectura!**
