-- Clean-up Database Script
-- Șterge tranzacții + categorii vechi MoneyZen

-- Step 1: Șterge TOATE tranzacțiile
DELETE FROM transactions;

-- Step 2: Șterge categoriile vechi MoneyZen (cele fără emoji sau cu emoji vechi)
DELETE FROM categories WHERE name IN (
    'Food & Dining',
    'Transportation', 
    'Home & Utilities',
    'Shopping',
    'Entertainment',
    'Healthcare',
    'Education',
    'Salary',
    'Investment',
    'Gift'
);

-- Step 3: Verificare - numără categoriile rămase
SELECT COUNT(*) as total_categories FROM categories;

-- Step 4: Afișează categoriile rămase
SELECT name, icon, category_type FROM categories ORDER BY category_type, name;
