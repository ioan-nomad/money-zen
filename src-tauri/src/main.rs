// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use database::{Database, Account, Transaction, Category};
use chrono::{DateTime, Utc};

type DatabaseState = Arc<Mutex<Database>>;

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
async fn backup_database() -> Result<String, String> {
    use std::fs;
    use chrono::Local;

    // Get paths - using the same path logic as main()
    let home_dir = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    let app_data_dir = std::path::Path::new(&home_dir).join("AppData").join("Local").join("MoneyZen");
    let db_path = app_data_dir.join("money-zen.db");

    // Create backup directory in Documents
    let documents_dir = std::path::Path::new(&home_dir).join("Documents");
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
async fn create_transaction(
    db: State<'_, DatabaseState>,
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

    db.create_transaction(account_id, category_id, amount, description, transaction_type, parsed_date)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transactions(db: State<'_, DatabaseState>) -> Result<Vec<Transaction>, String> {
    let db = db.lock().await;
    db.get_transactions().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_categories(db: State<'_, DatabaseState>) -> Result<Vec<Category>, String> {
    let db = db.lock().await;
    db.get_categories().await.map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() {
    // Get user's app data directory
    let home_dir = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    let app_data_dir = std::path::Path::new(&home_dir).join("AppData").join("Local").join("MoneyZen");
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

    let database_path = app_data_dir.join("money-zen.db");

    let database = Database::new(database_path).await.expect("Failed to connect to database");
    let db_state = Arc::new(Mutex::new(database));

    // Initialize schema
    {
        let db = db_state.lock().await;
        if let Err(e) = db.init_schema().await {
            eprintln!("Failed to initialize database schema: {}", e);
        }
    }

    tauri::Builder::default()
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            init_database,
            create_account,
            get_accounts,
            update_account,
            delete_account,
            backup_database,
            create_transaction,
            get_transactions,
            get_categories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}