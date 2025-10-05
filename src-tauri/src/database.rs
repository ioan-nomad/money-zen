use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String,
    pub balance: f64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub category_type: String, // "income" or "expense"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub category_id: String,
    pub amount: f64,
    pub description: String,
    pub transaction_type: String, // "income" or "expense"
    pub date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_path: PathBuf) -> Result<Self, sqlx::Error> {
        if let Some(parent) = database_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| sqlx::Error::Io(e))?;
        }
        let database_url = format!("sqlite://{}?mode=rwc", database_path.display());
        let pool = SqlitePool::connect(&database_url).await?;
        Ok(Database { pool })
    }

    pub async fn init_schema(&self) -> Result<(), sqlx::Error> {
        // Create accounts table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                account_type TEXT NOT NULL,
                balance REAL NOT NULL DEFAULT 0.0,
                currency TEXT NOT NULL DEFAULT 'RON',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        // Create categories table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS categories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                color TEXT NOT NULL DEFAULT '#3B82F6',
                icon TEXT NOT NULL DEFAULT '💰',
                category_type TEXT NOT NULL CHECK (category_type IN ('income', 'expense')),
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        // Create transactions table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                account_id TEXT NOT NULL,
                category_id TEXT NOT NULL,
                amount REAL NOT NULL,
                description TEXT NOT NULL,
                transaction_type TEXT NOT NULL CHECK (transaction_type IN ('income', 'expense')),
                date TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE,
                FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE RESTRICT
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        // Insert default categories if none exist
        self.insert_default_categories().await?;

        Ok(())
    }

    async fn insert_default_categories(&self) -> Result<(), sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories")
            .fetch_one(&self.pool)
            .await?;

        if count == 0 {
            let default_categories = vec![
                ("🍔", "Food & Dining", "#EF4444", "expense"),
                ("🚗", "Transportation", "#F59E0B", "expense"),
                ("🏠", "Home & Utilities", "#10B981", "expense"),
                ("👕", "Shopping", "#8B5CF6", "expense"),
                ("🎬", "Entertainment", "#EC4899", "expense"),
                ("💊", "Healthcare", "#06B6D4", "expense"),
                ("💼", "Salary", "#22C55E", "income"),
                ("💸", "Investment", "#3B82F6", "income"),
                ("🎁", "Gift", "#F97316", "income"),
                ("📚", "Education", "#6366F1", "expense"),
            ];

            for (icon, name, color, category_type) in default_categories {
                let id = Uuid::new_v4().to_string();
                let now = Utc::now();

                sqlx::query(
                    r#"
                    INSERT INTO categories (id, name, color, icon, category_type, created_at)
                    VALUES (?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&id)
                .bind(name)
                .bind(color)
                .bind(icon)
                .bind(category_type)
                .bind(now.to_rfc3339())
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }

    // Account operations
    pub async fn create_account(&self, name: String, account_type: String, currency: String) -> Result<Account, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO accounts (id, name, account_type, balance, currency, created_at, updated_at)
            VALUES (?, ?, ?, 0.0, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&name)
        .bind(&account_type)
        .bind(&currency)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(Account {
            id,
            name,
            account_type,
            balance: 0.0,
            currency,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_accounts(&self) -> Result<Vec<Account>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM accounts ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let accounts = rows.into_iter().map(|row| {
            Account {
                id: row.get("id"),
                name: row.get("name"),
                account_type: row.get("account_type"),
                balance: row.get("balance"),
                currency: row.get("currency"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&Utc),
            }
        }).collect();

        Ok(accounts)
    }

    pub async fn update_account(&self, id: String, name: String, account_type: String, currency: String) -> Result<Account, sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE accounts SET name = ?, account_type = ?, currency = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&name)
        .bind(&account_type)
        .bind(&currency)
        .bind(&now)
        .bind(&id)
        .execute(&self.pool)
        .await?;

        // Return the updated account
        let row = sqlx::query("SELECT * FROM accounts WHERE id = ?")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Account {
            id: row.get("id"),
            name: row.get("name"),
            account_type: row.get("account_type"),
            balance: row.get("balance"),
            currency: row.get("currency"),
            created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&Utc),
        })
    }

    pub async fn delete_account(&self, id: String) -> Result<(), sqlx::Error> {
        // Note: Due to CASCADE constraint, deleting account will also delete all associated transactions
        sqlx::query("DELETE FROM accounts WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // Transaction operations
    pub async fn create_transaction(
        &self,
        account_id: String,
        category_id: String,
        amount: f64,
        description: String,
        transaction_type: String,
        date: DateTime<Utc>,
    ) -> Result<Transaction, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Insert transaction
        sqlx::query(
            r#"
            INSERT INTO transactions (id, account_id, category_id, amount, description, transaction_type, date, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&account_id)
        .bind(&category_id)
        .bind(amount)
        .bind(&description)
        .bind(&transaction_type)
        .bind(date.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        // Update account balance
        let balance_change = if transaction_type == "income" { amount } else { -amount };
        sqlx::query(
            "UPDATE accounts SET balance = balance + ?, updated_at = ? WHERE id = ?"
        )
        .bind(balance_change)
        .bind(now.to_rfc3339())
        .bind(&account_id)
        .execute(&self.pool)
        .await?;

        Ok(Transaction {
            id,
            account_id,
            category_id,
            amount,
            description,
            transaction_type,
            date,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_transactions(&self) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM transactions ORDER BY date DESC, created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let transactions = rows.into_iter().map(|row| {
            Transaction {
                id: row.get("id"),
                account_id: row.get("account_id"),
                category_id: row.get("category_id"),
                amount: row.get("amount"),
                description: row.get("description"),
                transaction_type: row.get("transaction_type"),
                date: DateTime::parse_from_rfc3339(&row.get::<String, _>("date")).unwrap().with_timezone(&Utc),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&Utc),
            }
        }).collect();

        Ok(transactions)
    }

    // Category operations
    pub async fn get_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM categories ORDER BY category_type, name")
            .fetch_all(&self.pool)
            .await?;

        let categories = rows.into_iter().map(|row| {
            Category {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                icon: row.get("icon"),
                category_type: row.get("category_type"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&Utc),
            }
        }).collect();

        Ok(categories)
    }
}