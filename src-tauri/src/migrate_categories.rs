// migrate_categories.rs
// Migration script for N-OMAD categories import

use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CategoryData {
    name: String,
    icon: String,
    color: String,
    #[serde(rename = "type")]
    category_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CategoryGroup {
    group_name: String,
    group_icon: String,
    group_color: String,
    categories: Vec<CategoryData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CategoryGroups {
    income: Vec<CategoryGroup>,
    expense: Vec<CategoryGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CategoryGroupsFile {
    groups: CategoryGroups,
}

pub async fn migrate_nomad_categories(pool: &Pool<Sqlite>) -> Result<String, String> {
    println!("🚀 Starting N-OMAD Categories Migration...");

    // Step 1: Read category-groups.json
    let json_content = std::fs::read_to_string("../category-groups.json")
        .map_err(|e| format!("Failed to read category-groups.json: {}", e))?;

    let groups_file: CategoryGroupsFile = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    println!("📊 Loaded category groups from JSON");

    // Step 2: Get existing categories
    println!("📋 Checking existing categories...");

    let existing_categories: Vec<String> = sqlx::query_scalar("SELECT name FROM categories")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to get existing categories: {}", e))?;

    println!("📊 Found {} existing categories", existing_categories.len());

    // Step 3: Insert new categories (skip existing ones)
    let mut total_inserted = 0;

    // Insert income categories
    for group in groups_file.groups.income {
        for cat in group.categories {
            // Skip if category already exists
            if existing_categories.contains(&cat.name) {
                println!("  ⏭️  {} {} (already exists)", cat.icon, cat.name);
                continue;
            }

            let id = Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();

            sqlx::query(
                "INSERT INTO categories (id, name, color, icon, category_type, created_at)
                 VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(&cat.name)
            .bind(&cat.color)
            .bind(&cat.icon)
            .bind("income")
            .bind(&now)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to insert category {}: {}", cat.name, e))?;

            println!("  ✅ {} {} ({})", cat.icon, cat.name, cat.color);
            total_inserted += 1;
        }
    }

    // Insert expense categories
    for group in groups_file.groups.expense {
        for cat in group.categories {
            // Skip if category already exists
            if existing_categories.contains(&cat.name) {
                println!("  ⏭️  {} {} (already exists)", cat.icon, cat.name);
                continue;
            }

            let id = Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();

            sqlx::query(
                "INSERT INTO categories (id, name, color, icon, category_type, created_at)
                 VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(&cat.name)
            .bind(&cat.color)
            .bind(&cat.icon)
            .bind("expense")
            .bind(&now)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to insert category {}: {}", cat.name, e))?;

            println!("  ✅ {} {} ({})", cat.icon, cat.name, cat.color);
            total_inserted += 1;
        }
    }

    println!("\n📈 MIGRATION COMPLETE!");
    println!("   ✅ Categories inserted: {}", total_inserted);

    Ok(format!("Migration successful! Inserted {} categories.", total_inserted))
}