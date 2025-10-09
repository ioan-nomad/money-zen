// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod migrate_categories;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{State, Manager};
use database::{Database, Account, Transaction, Category, Tag};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

type DatabaseState = Arc<Mutex<Database>>;

#[derive(Debug, Deserialize, Serialize)]
struct ImportTransaction {
    account_id: String,
    category_id: String,
    amount: f64,
    description: String,
    transaction_type: String,
    date: String, // ISO format from frontend
}

#[derive(Debug, Serialize)]
struct ImportResult {
    inserted: usize,
    skipped: usize,
    errors: Vec<String>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn init_database(db: State<'_, DatabaseState>) -> Result<(), String> {
    let db = db.lock().await;
    db.init_schema().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn create_account(
    db: State<'_, DatabaseState>,
    name: String,
    account_type: String,
    currency: String,
) -> Result<Account, String> {
    let db = db.lock().await;
    db.create_account(name, account_type, currency)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_accounts(db: State<'_, DatabaseState>) -> Result<Vec<Account>, String> {
    let db = db.lock().await;
    db.get_accounts().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_account(
    db: State<'_, DatabaseState>,
    id: String,
    name: String,
    account_type: String,
    currency: String,
) -> Result<Account, String> {
    let db = db.lock().await;
    db.update_account(id, name, account_type, currency)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_account(db: State<'_, DatabaseState>, id: String) -> Result<(), String> {
    let db = db.lock().await;
    db.delete_account(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn backup_database(app: tauri::AppHandle) -> Result<String, String> {
    use std::fs;
    use chrono::Local;

    // Get paths using Tauri API
    let app_data_dir = app.path().app_local_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let db_path = app_data_dir.join("money-zen.db");

    // Create backup directory in Documents
    let documents_dir = app.path().document_dir()
        .map_err(|e| format!("Failed to get documents directory: {}", e))?;
    let backup_dir = documents_dir.join("MoneyZen Backups");
    fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    // Generate timestamped filename
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("backup-{}.db", timestamp);
    let backup_path = backup_dir.join(&backup_filename);

    // Copy database file
    fs::copy(&db_path, &backup_path)
        .map_err(|e| format!("Failed to copy database: {}", e))?;

    // Return success with backup path
    Ok(format!("Backup created: {}", backup_path.display()))
}

#[tauri::command]
async fn restore_database(app: tauri::AppHandle, backup_path: String) -> Result<String, String> {
    use std::fs;

    // Get active database path using Tauri API
    let app_data_dir = app.path().app_local_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let db_path = app_data_dir.join("money-zen.db");

    // Verify backup file exists
    let backup = std::path::Path::new(&backup_path);
    if !backup.exists() {
        return Err("Backup file not found".to_string());
    }

    // Copy backup to active database location (overwrite)
    fs::copy(&backup, &db_path)
        .map_err(|e| format!("Failed to restore database: {}", e))?;

    Ok(format!("Database restored from: {}", backup_path))
}

#[tauri::command]
async fn batch_insert_transactions(
    db: State<'_, DatabaseState>,
    transactions: Vec<ImportTransaction>,
) -> Result<ImportResult, String> {
    println!("🔍 batch_insert_transactions called with {} transactions", transactions.len());

    let db = db.lock().await;
    let mut inserted = 0;
    let mut skipped = 0;
    let mut errors = Vec::new();

    for tx in transactions {
        println!("Processing transaction: date={}, amount={}, desc={}",
                 tx.date, tx.amount, tx.description);

        // Parse date - handle YYYY-MM-DD format
        let date = if tx.date.contains('T') {
            // RFC3339 format (with time)
            match DateTime::parse_from_rfc3339(&tx.date) {
                Ok(d) => d.with_timezone(&Utc),
                Err(_) => {
                    println!("❌ Date parsing failed for: {}", tx.date);
                    errors.push(format!("Invalid date: {}", tx.date));
                    continue;
                }
            }
        } else {
            // Simple YYYY-MM-DD format
            match chrono::NaiveDate::parse_from_str(&tx.date, "%Y-%m-%d") {
                Ok(naive_date) => {
                    let datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
                    DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc)
                },
                Err(_) => {
                    println!("❌ Date parsing failed for: {}", tx.date);
                    errors.push(format!("Invalid date: {}", tx.date));
                    continue;
                }
            }
        };

        // Check duplicate (same date, amount, description)
        match db.transaction_exists(&date.to_rfc3339(), tx.amount, &tx.description).await {
            Ok(true) => {
                println!("⏭️ Duplicate found, skipping transaction");
                skipped += 1;
                continue;
            }
            Ok(false) => {
                // Insert new transaction
                match db.create_transaction(
                    tx.account_id,
                    tx.category_id,
                    tx.amount,
                    tx.description,
                    tx.transaction_type,
                    date,
                    None
                ).await {
                    Ok(_) => {
                        inserted += 1;
                        println!("✅ Transaction inserted successfully");
                    },
                    Err(e) => {
                        println!("❌ Failed to insert transaction: {:?}", e);
                        errors.push(format!("Failed to insert: {}", e));
                    },
                }
            }
            Err(e) => {
                errors.push(format!("Database error checking duplicate: {}", e));
                continue;
            }
        }
    }

    Ok(ImportResult {
        inserted,
        skipped,
        errors,
    })
}

#[tauri::command]
async fn create_transaction(
    db: State<'_, DatabaseState>,
    account_id: String,
    category_id: String,
    amount: f64,
    description: String,
    transaction_type: String,
    date: String,
    tag_ids: Option<Vec<String>>,
) -> Result<Transaction, String> {
    let db = db.lock().await;
    let parsed_date = DateTime::parse_from_rfc3339(&date)
        .map_err(|e| format!("Invalid date format: {}", e))?
        .with_timezone(&Utc);

    db.create_transaction(account_id, category_id, amount, description, transaction_type, parsed_date, tag_ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_transaction(
    db: State<'_, DatabaseState>,
    id: String,
    account_id: String,
    category_id: String,
    amount: f64,
    description: String,
    transaction_type: String,
    date: String,
) -> Result<Transaction, String> {
    let db = db.lock().await;
    let parsed_date = DateTime::parse_from_rfc3339(&date)
        .map_err(|e| format!("Invalid date format: {}", e))?
        .with_timezone(&Utc);

    db.update_transaction(id, account_id, category_id, amount, description, transaction_type, parsed_date)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_transaction(
    id: String,
    db: State<'_, DatabaseState>,
) -> Result<(), String> {
    let db = db.lock().await;
    db.delete_transaction(id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_multiple_transactions(
    transaction_ids: Vec<String>,
    db: State<'_, DatabaseState>,
) -> Result<usize, String> {
    let db = db.lock().await;
    db.delete_multiple_transactions(transaction_ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transactions(db: State<'_, DatabaseState>) -> Result<Vec<Transaction>, String> {
    let db = db.lock().await;
    db.get_transactions().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transactions_by_month(
    db: State<'_, DatabaseState>,
    year: i32,
    month: i32,
) -> Result<Vec<Transaction>, String> {
    let db = db.lock().await;
    db.get_transactions_by_month(year, month).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transactions_by_account(
    db: State<'_, DatabaseState>,
    account_id: String,
) -> Result<Vec<Transaction>, String> {
    let db = db.lock().await;
    db.get_transactions_by_account(account_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transactions_by_category(
    db: State<'_, DatabaseState>,
    category_id: String,
) -> Result<Vec<Transaction>, String> {
    let db = db.lock().await;
    db.get_transactions_by_category(category_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transactions_by_date_range(
    db: State<'_, DatabaseState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<Transaction>, String> {
    let db = db.lock().await;
    db.get_transactions_by_date_range(start_date, end_date).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_categories(db: State<'_, DatabaseState>) -> Result<Vec<Category>, String> {
    let db = db.lock().await;
    db.get_categories().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_category(
    db: State<'_, DatabaseState>,
    name: String,
    icon: String,
    category_type: String,
    color: String,
) -> Result<Category, String> {
    let db = db.lock().await;
    db.create_category(name, icon, category_type, color).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_category(
    db: State<'_, DatabaseState>,
    id: String,
    name: String,
    icon: String,
    category_type: String,
    color: String,
) -> Result<Category, String> {
    let db = db.lock().await;
    db.update_category(id, name, icon, category_type, color).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_category(
    db: State<'_, DatabaseState>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    db.delete_category(id).await.map_err(|e| e.to_string())
}

// Tag commands
#[tauri::command]
async fn create_tag(
    db: State<'_, DatabaseState>,
    name: String,
    icon: String,
    color: String,
) -> Result<Tag, String> {
    let db = db.lock().await;
    db.create_tag(name, icon, color).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_tag(
    db: State<'_, DatabaseState>,
    id: String,
    name: String,
    icon: String,
    color: String,
) -> Result<Tag, String> {
    let db = db.lock().await;
    db.update_tag(id, name, icon, color).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_tag(
    db: State<'_, DatabaseState>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    db.delete_tag(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_tags(
    db: State<'_, DatabaseState>,
) -> Result<Vec<Tag>, String> {
    let db = db.lock().await;
    db.get_tags().await.map_err(|e| e.to_string())
}

// Transaction-Tag relationship commands
#[tauri::command]
async fn add_tags_to_transaction(
    db: State<'_, DatabaseState>,
    transaction_id: String,
    tag_ids: Vec<String>,
) -> Result<(), String> {
    let db = db.lock().await;
    db.add_tags_to_transaction(transaction_id, tag_ids).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_tags_from_transaction(
    db: State<'_, DatabaseState>,
    transaction_id: String,
    tag_ids: Vec<String>,
) -> Result<(), String> {
    let db = db.lock().await;
    db.remove_tags_from_transaction(transaction_id, tag_ids).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transaction_tags(
    db: State<'_, DatabaseState>,
    transaction_id: String,
) -> Result<Vec<Tag>, String> {
    let db = db.lock().await;
    db.get_transaction_tags(transaction_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transactions_by_tag(
    db: State<'_, DatabaseState>,
    tag_id: String,
) -> Result<Vec<Transaction>, String> {
    let db = db.lock().await;
    db.get_transactions_by_tag(tag_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn migrate_nomad_categories(
    db: State<'_, DatabaseState>
) -> Result<String, String> {
    let db = db.lock().await;
    migrate_categories::migrate_nomad_categories(db.get_pool()).await
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // Use Tauri's path API to get the correct app data directory
            let app_data_dir = handle.path().app_local_data_dir()
                .expect("Failed to get app data directory");
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            let database_path = app_data_dir.join("money-zen.db");

            // Initialize database in async context
            tauri::async_runtime::spawn(async move {
                let database = Database::new(database_path).await
                    .expect("Failed to connect to database");
                let db_state = Arc::new(Mutex::new(database));

                // Initialize schema
                {
                    let db = db_state.lock().await;
                    if let Err(e) = db.init_schema().await {
                        eprintln!("Failed to initialize database schema: {}", e);
                    }
                }

                // Store the database state in the app
                handle.manage(db_state);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            init_database,
            create_account,
            get_accounts,
            update_account,
            delete_account,
            backup_database,
            restore_database,
            batch_insert_transactions,
            create_transaction,
            update_transaction,
            delete_transaction,
            delete_multiple_transactions,
            get_transactions,
            get_transactions_by_month,
            get_transactions_by_account,
            get_transactions_by_category,
            get_transactions_by_date_range,
            get_categories,
            create_category,
            update_category,
            delete_category,
            get_tags,
            create_tag,
            update_tag,
            delete_tag,
            add_tags_to_transaction,
            remove_tags_from_transaction,
            get_transaction_tags,
            get_transactions_by_tag,
            migrate_nomad_categories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}