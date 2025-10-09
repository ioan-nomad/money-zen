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
    pub owner: Option<String>,
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
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
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

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
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

        // Create tags table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tags (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                color TEXT NOT NULL DEFAULT '#8B5CF6',
                icon TEXT NOT NULL DEFAULT '🏷️',
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        // Create transaction-tags junction table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transaction_tags (
                transaction_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY (transaction_id, tag_id),
                FOREIGN KEY (transaction_id) REFERENCES transactions (id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        // Insert default categories if none exist
        self.insert_default_categories().await?;
        // Run migrations
        self.run_migrations().await?;
        // Insert N-OMAD accounts if none exist
        self.insert_nomad_accounts().await?;

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
            owner: Some("Ioan".to_string()),
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
                owner: row.get("owner"),
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
                updated_at: {
                    let datetime_str: String = row.get("updated_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
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
            owner: row.get("owner"),
            created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            updated_at: {
                    let datetime_str: String = row.get("updated_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
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
        tag_ids: Option<Vec<String>>,
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

        // Add tags if provided
        if let Some(tag_ids) = tag_ids {
            if !tag_ids.is_empty() {
                self.add_tags_to_transaction(id.clone(), tag_ids).await?;
            }
        }

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

    pub async fn update_transaction(
        &self,
        id: String,
        account_id: String,
        category_id: String,
        amount: f64,
        description: String,
        transaction_type: String,
        date: DateTime<Utc>,
    ) -> Result<Transaction, sqlx::Error> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE transactions
            SET account_id = ?, category_id = ?, amount = ?, description = ?,
                transaction_type = ?, date = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&account_id)
        .bind(&category_id)
        .bind(amount)
        .bind(&description)
        .bind(&transaction_type)
        .bind(date.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(&id)
        .execute(&self.pool)
        .await?;

        // Return the updated transaction
        let row = sqlx::query("SELECT * FROM transactions WHERE id = ?")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Transaction {
            id: row.get("id"),
            account_id: row.get("account_id"),
            category_id: row.get("category_id"),
            amount: row.get("amount"),
            description: row.get("description"),
            transaction_type: row.get("transaction_type"),
            date: DateTime::parse_from_rfc3339(&row.get::<String, _>("date")).unwrap().with_timezone(&Utc),
            created_at: {
                let datetime_str: String = row.get("created_at");
                chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_else(|_| chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc())
                    .and_utc()
            },
            updated_at: {
                let datetime_str: String = row.get("updated_at");
                chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_else(|_| chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc())
                    .and_utc()
            },
        })
    }

    pub async fn delete_transaction(
        &self,
        id: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM transactions WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_multiple_transactions(
        &self,
        transaction_ids: Vec<String>,
    ) -> Result<usize, sqlx::Error> {
        let mut deleted_count: usize = 0;

        for transaction_id in transaction_ids {
            // Delete transaction (cascade deletes transaction_tags automatically)
            let result = sqlx::query("DELETE FROM transactions WHERE id = ?")
                .bind(&transaction_id)
                .execute(&self.pool)
                .await?;

            if result.rows_affected() > 0 {
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
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
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
                updated_at: {
                    let datetime_str: String = row.get("updated_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(transactions)
    }

    pub async fn transaction_exists(&self, date: &str, amount: f64, description: &str) -> Result<bool, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id FROM transactions WHERE date = ? AND amount = ? AND description = ?"
        )
        .bind(date)
        .bind(amount)
        .bind(description)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.is_some())
    }

    pub async fn get_transactions_by_month(&self, year: i32, month: i32) -> Result<Vec<Transaction>, sqlx::Error> {
        let start_date = format!("{:04}-{:02}-01T00:00:00Z", year, month);
        let end_date = if month == 12 {
            format!("{:04}-01-01T00:00:00Z", year + 1)
        } else {
            format!("{:04}-{:02}-01T00:00:00Z", year, month + 1)
        };

        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE date >= ? AND date < ? ORDER BY date DESC, created_at DESC"
        )
        .bind(&start_date)
        .bind(&end_date)
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
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
                updated_at: {
                    let datetime_str: String = row.get("updated_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(transactions)
    }

    pub async fn get_transactions_by_account(&self, account_id: String) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE account_id = ? ORDER BY date DESC, created_at DESC"
        )
        .bind(&account_id)
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
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
                updated_at: {
                    let datetime_str: String = row.get("updated_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(transactions)
    }

    pub async fn get_transactions_by_category(&self, category_id: String) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE category_id = ? ORDER BY date DESC, created_at DESC"
        )
        .bind(&category_id)
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
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
                updated_at: {
                    let datetime_str: String = row.get("updated_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(transactions)
    }

    pub async fn get_transactions_by_date_range(&self, start_date: String, end_date: String) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE date >= ? AND date <= ? ORDER BY date DESC, created_at DESC"
        )
        .bind(&start_date)
        .bind(&end_date)
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
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
                updated_at: {
                    let datetime_str: String = row.get("updated_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(transactions)
    }

    // Category operations
    pub async fn create_category(
        &self,
        name: String,
        icon: String,
        category_type: String,
        color: String,
    ) -> Result<Category, sqlx::Error> {
        // Check if category name already exists for this type
        let existing = sqlx::query(
            "SELECT id FROM categories WHERE name = ? AND category_type = ?"
        )
        .bind(&name)
        .bind(&category_type)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(sqlx::Error::RowNotFound);
        }

        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO categories (id, name, color, icon, category_type, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&name)
        .bind(&color)
        .bind(&icon)
        .bind(&category_type)
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(Category {
            id,
            name,
            color,
            icon,
            category_type,
            created_at: now,
        })
    }

    pub async fn update_category(
        &self,
        id: String,
        name: String,
        icon: String,
        category_type: String,
        color: String,
    ) -> Result<Category, sqlx::Error> {
        // Check if category name already exists for this type (excluding current category)
        let existing = sqlx::query(
            "SELECT id FROM categories WHERE name = ? AND category_type = ? AND id != ?"
        )
        .bind(&name)
        .bind(&category_type)
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query(
            "UPDATE categories SET name = ?, color = ?, icon = ?, category_type = ? WHERE id = ?"
        )
        .bind(&name)
        .bind(&color)
        .bind(&icon)
        .bind(&category_type)
        .bind(&id)
        .execute(&self.pool)
        .await?;

        // Return the updated category
        let row = sqlx::query("SELECT * FROM categories WHERE id = ?")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Category {
            id: row.get("id"),
            name: row.get("name"),
            color: row.get("color"),
            icon: row.get("icon"),
            category_type: row.get("category_type"),
            created_at: {
                let datetime_str: String = row.get("created_at");
                chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap())
                    .and_utc()
            },
        })
    }

    pub async fn delete_category(&self, id: String) -> Result<(), sqlx::Error> {
        // Check if category is being used in any transactions
        let transaction_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM transactions WHERE category_id = ?"
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await?;

        if transaction_count > 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

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
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(categories)
    }

    // Tag CRUD operations
    pub async fn create_tag(
        &self,
        name: String,
        icon: String,
        color: String,
    ) -> Result<Tag, sqlx::Error> {
        // Check if tag name already exists
        let existing = sqlx::query(
            "SELECT id FROM tags WHERE name = ?"
        )
        .bind(&name)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(sqlx::Error::RowNotFound);
        }

        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO tags (id, name, color, icon, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&name)
        .bind(&color)
        .bind(&icon)
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(Tag {
            id,
            name,
            color,
            icon,
            created_at: now,
        })
    }

    pub async fn update_tag(
        &self,
        id: String,
        name: String,
        icon: String,
        color: String,
    ) -> Result<Tag, sqlx::Error> {
        // Check if tag name already exists (excluding current tag)
        let existing = sqlx::query(
            "SELECT id FROM tags WHERE name = ? AND id != ?"
        )
        .bind(&name)
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query(
            r#"
            UPDATE tags
            SET name = ?, color = ?, icon = ?
            WHERE id = ?
            "#
        )
        .bind(&name)
        .bind(&color)
        .bind(&icon)
        .bind(&id)
        .execute(&self.pool)
        .await?;

        let updated_at = Utc::now();

        Ok(Tag {
            id,
            name,
            color,
            icon,
            created_at: updated_at, // Note: we don't track updated_at for tags, so using current time
        })
    }

    pub async fn delete_tag(&self, id: String) -> Result<(), sqlx::Error> {
        // Check if tag is being used in any transactions
        let transaction_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM transaction_tags WHERE tag_id = ?"
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await?;

        if transaction_count > 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query("DELETE FROM tags WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_tags(&self) -> Result<Vec<Tag>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM tags ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        let tags = rows.into_iter().map(|row| {
            Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                icon: row.get("icon"),
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(tags)
    }

    // Transaction-Tag relationship functions
    pub async fn add_tags_to_transaction(&self, transaction_id: String, tag_ids: Vec<String>) -> Result<(), sqlx::Error> {
        let now = Utc::now();

        for tag_id in tag_ids {
            sqlx::query(
                r#"
                INSERT OR IGNORE INTO transaction_tags (transaction_id, tag_id, created_at)
                VALUES (?, ?, ?)
                "#
            )
            .bind(&transaction_id)
            .bind(&tag_id)
            .bind(now.to_rfc3339())
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn remove_tags_from_transaction(&self, transaction_id: String, tag_ids: Vec<String>) -> Result<(), sqlx::Error> {
        for tag_id in tag_ids {
            sqlx::query(
                "DELETE FROM transaction_tags WHERE transaction_id = ? AND tag_id = ?"
            )
            .bind(&transaction_id)
            .bind(&tag_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn get_transaction_tags(&self, transaction_id: String) -> Result<Vec<Tag>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT t.* FROM tags t
            INNER JOIN transaction_tags tt ON t.id = tt.tag_id
            WHERE tt.transaction_id = ?
            ORDER BY t.name
            "#
        )
        .bind(&transaction_id)
        .fetch_all(&self.pool)
        .await?;

        let tags = rows.into_iter().map(|row| {
            Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                icon: row.get("icon"),
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(tags)
    }

    pub async fn get_transactions_by_tag(&self, tag_id: String) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT tr.* FROM transactions tr
            INNER JOIN transaction_tags tt ON tr.id = tt.transaction_id
            WHERE tt.tag_id = ?
            ORDER BY tr.date DESC, tr.created_at DESC
            "#
        )
        .bind(&tag_id)
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
                date: {
                    let date_str: String = row.get("date");
                    chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
                created_at: {
                    let datetime_str: String = row.get("created_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
                updated_at: {
                    let datetime_str: String = row.get("updated_at");
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()).and_utc()
                },
            }
        }).collect();

        Ok(transactions)
    }

    // Migration functions
    async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        self.add_owner_column_to_accounts().await?;
        Ok(())
    }

    async fn add_owner_column_to_accounts(&self) -> Result<(), sqlx::Error> {
        // Check if owner column already exists
        let table_info = sqlx::query("PRAGMA table_info(accounts)")
            .fetch_all(&self.pool)
            .await?;

        let has_owner_column = table_info.iter().any(|row| {
            let column_name: String = row.get("name");
            column_name == "owner"
        });

        if !has_owner_column {
            // Add owner column
            sqlx::query("ALTER TABLE accounts ADD COLUMN owner TEXT")
                .execute(&self.pool)
                .await?;

            // Set default value for existing accounts
            sqlx::query("UPDATE accounts SET owner = 'Ioan' WHERE owner IS NULL")
                .execute(&self.pool)
                .await?;

            println!("✅ Added owner column to accounts table");
        }

        Ok(())
    }
    pub async fn insert_nomad_accounts(&self) -> Result<(), sqlx::Error> {
        // Check if accounts already exist
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM accounts")
            .fetch_one(&self.pool)
            .await?;

        if count > 0 {
            println!("⚠️  Accounts already exist, skipping N-OMAD account insertion");
            return Ok(());
        }

        let accounts = vec![
            // Ioan Accounts (3)
            ("ac1-ioan-bt", "BT Ioan", "bank", "RON", "Ioan"),
            ("ac2-ioan-revolut", "Revolut Ioan", "bank", "RON", "Ioan"),
            ("ac3-ioan-wise", "Wise Ioan", "bank", "EUR", "Ioan"),

            // Nico RON Accounts (3)
            ("ac4-nico-bt-curent", "BT Cont Curent Nico", "bank", "RON", "Nico"),
            ("ac5-nico-bt-anph", "BT Cont ANPH Nico", "bank", "RON", "Nico"),
            ("ac6-nico-bt-economii", "BT Cont Economii Nico", "bank", "RON", "Nico"),

            // Nico EUR Accounts (2)
            ("ac7-nico-bt-eur", "BT Cont Curent Euro Nico", "bank", "EUR", "Nico"),
            ("ac8-nico-bt-economii-eur", "BT Cont Economii Euro Nico", "bank", "EUR", "Nico"),

            // Firmă Account (1)
            ("ac9-firma-nico", "Cont Firma Nico", "bank", "RON", "Firmă Nico"),

            // Cash Accounts (3)
            ("ac10-cash-ioan", "Cash Ioan", "cash", "RON", "Ioan"),
            ("ac11-cash-nico", "Cash Nico", "cash", "RON", "Nico"),
            ("ac12-cash-comun", "Cash Comun", "cash", "RON", "Comun"),
        ];

        for (id, name, account_type, currency, owner) in accounts {
            let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

            sqlx::query(
                r#"
                INSERT INTO accounts (id, name, account_type, balance, currency, owner, created_at, updated_at)
                VALUES (?, ?, ?, 0.0, ?, ?, ?, ?)
                "#
            )
            .bind(id)
            .bind(name)
            .bind(account_type)
            .bind(currency)
            .bind(owner)
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        }

        println!("✅ Inserted 12 N-OMAD accounts successfully");
        Ok(())
    }
}

