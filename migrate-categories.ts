// Migration Script: Import N-OMAD Categories to MoneyZen
// Usage: Copy-paste acest cod în browser console când rulezi app-ul

import { Database } from './src/lib/database';

async function migrateNOMADCategories() {
  console.log('🚀 Starting N-OMAD Categories Migration...');

  try {
    // Read JSON file
    const response = await fetch('/nomad-categories.json');
    const data = await response.json();

    console.log(`📊 Found ${data.summary.total_categories} categories to import`);

    // Get existing categories to avoid duplicates
    const existingCategories = await Database.getCategories();
    const existingNames = new Set(existingCategories.map(c => c.name));

    let imported = 0;
    let skipped = 0;

    // Import income categories
    for (const cat of data.categories.income) {
      if (existingNames.has(cat.name)) {
        console.log(`⏭️  Skipped (already exists): ${cat.icon} ${cat.name}`);
        skipped++;
        continue;
      }

      // Note: Database.createCategory doesn't exist yet, we'll add it
      console.log(`✅ Would import: ${cat.icon} ${cat.name} (${cat.color})`);
      imported++;
    }

    // Import expense categories
    for (const cat of data.categories.expense) {
      if (existingNames.has(cat.name)) {
        console.log(`⏭️  Skipped (already exists): ${cat.icon} ${cat.name}`);
        skipped++;
        continue;
      }

      console.log(`✅ Would import: ${cat.icon} ${cat.name} (${cat.color})`);
      imported++;
    }

    console.log('\n📈 MIGRATION SUMMARY:');
    console.log(`   ✅ Categories to import: ${imported}`);
    console.log(`   ⏭️  Categories skipped: ${skipped}`);
    console.log(`   📊 Total processed: ${imported + skipped}`);

    return { imported, skipped };

  } catch (error) {
    console.error('❌ Migration failed:', error);
    throw error;
  }
}

// Export for use
export { migrateNOMADCategories };