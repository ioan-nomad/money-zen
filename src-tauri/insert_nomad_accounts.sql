-- Insert 12 N-OMAD Accounts with owner field
-- Date: October 8, 2025

-- Ioan Accounts (3)
INSERT INTO accounts (id, name, account_type, balance, currency, owner, created_at, updated_at) VALUES
('ac1-ioan-bt', 'BT Ioan', 'bank', 0.0, 'RON', 'Ioan', datetime('now'), datetime('now')),
('ac2-ioan-revolut', 'Revolut Ioan', 'bank', 0.0, 'RON', 'Ioan', datetime('now'), datetime('now')),
('ac3-ioan-wise', 'Wise Ioan', 'bank', 0.0, 'EUR', 'Ioan', datetime('now'), datetime('now'));

-- Nico RON Accounts (3)
INSERT INTO accounts (id, name, account_type, balance, currency, owner, created_at, updated_at) VALUES
('ac4-nico-bt-curent', 'BT Cont Curent Nico', 'bank', 0.0, 'RON', 'Nico', datetime('now'), datetime('now')),
('ac5-nico-bt-anph', 'BT Cont ANPH Nico', 'bank', 0.0, 'RON', 'Nico', datetime('now'), datetime('now')),
('ac6-nico-bt-economii', 'BT Cont Economii Nico', 'bank', 0.0, 'RON', 'Nico', datetime('now'), datetime('now'));

-- Nico EUR Accounts (2)
INSERT INTO accounts (id, name, account_type, balance, currency, owner, created_at, updated_at) VALUES
('ac7-nico-bt-eur', 'BT Cont Curent Euro Nico', 'bank', 0.0, 'EUR', 'Nico', datetime('now'), datetime('now')),
('ac8-nico-bt-economii-eur', 'BT Cont Economii Euro Nico', 'bank', 0.0, 'EUR', 'Nico', datetime('now'), datetime('now'));

-- Firmă Account (1)
INSERT INTO accounts (id, name, account_type, balance, currency, owner, created_at, updated_at) VALUES
('ac9-firma-nico', 'Cont Firma Nico', 'bank', 0.0, 'RON', 'Firmă Nico', datetime('now'), datetime('now'));

-- Cash Accounts (3)
INSERT INTO accounts (id, name, account_type, balance, currency, owner, created_at, updated_at) VALUES
('ac10-cash-ioan', 'Cash Ioan', 'cash', 0.0, 'RON', 'Ioan', datetime('now'), datetime('now')),
('ac11-cash-nico', 'Cash Nico', 'cash', 0.0, 'RON', 'Nico', datetime('now'), datetime('now')),
('ac12-cash-comun', 'Cash Comun', 'cash', 0.0, 'RON', 'Comun', datetime('now'), datetime('now'));

-- Verify insertion
SELECT COUNT(*) as total_accounts FROM accounts;
SELECT name, owner, currency FROM accounts ORDER BY owner, name;