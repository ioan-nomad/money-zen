<script lang="ts">
  import * as XLSX from 'xlsx';
  import { open } from '@tauri-apps/plugin-dialog';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { invoke } from '@tauri-apps/api/core';

  let fileName = '';
  let rowCount = 0;
  let columns: string[] = [];
  let rawData: any[] = [];
  let error = '';
  let columnMapping = {};
  let previewData = [];
  let canImport = false;
  let isImporting = false;

  async function selectFile() {
    try {
      error = '';
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Excel Files',
          extensions: ['xlsx', 'xls']
        }]
      });

      if (!selected) return;

      // Handle different Tauri dialog response formats
      if (typeof selected === 'string') {
        // If selected is just a path string
        fileName = selected.split(/[\\/]/).pop() || 'Unknown';
      } else if (selected?.name) {
        fileName = selected.name;
      } else if (selected?.path) {
        fileName = selected.path.split(/[\\/]/).pop() || 'Unknown';
      } else {
        fileName = 'Unknown';
      }

      // Read file
      const filePath = typeof selected === 'string' ? selected : selected.path;
      const fileData = await readFile(filePath);

      // Parse Excel
      const workbook = XLSX.read(fileData, {
        cellStyles: true,
        cellFormulas: true,
        cellDates: true
      });

      const firstSheet = workbook.Sheets[workbook.SheetNames[0]];
      rawData = XLSX.utils.sheet_to_json(firstSheet);

      rowCount = rawData.length;
      columns = rawData.length > 0 ? Object.keys(rawData[0]) : [];

      // Store preview data (first 3 rows)
      previewData = rawData;

      // Auto-detect column mappings
      columnMapping = autoDetectMapping(columns);

      // Check if required fields are mapped (check values not keys)
      const mappedFields = Object.values(columnMapping);
      canImport = mappedFields.includes('date') &&
                  mappedFields.includes('amount') &&
                  mappedFields.includes('description');

    } catch (e: any) {
      error = e.message || 'Failed to read file';
      console.error('Import error:', e);
    }
  }

  function autoDetectMapping(columns) {
    const mapping = {};

    columns.forEach(col => {
      const lower = col.toLowerCase().trim();

      // Romanian column name detection
      if (lower === 'data' || lower === 'date') {
        mapping[col] = 'date';
      } else if (lower === 'suma' || lower === 'amount' || lower === 'valoare') {
        mapping[col] = 'amount';
      } else if (lower === 'descriere' || lower === 'description' || lower === 'detalii') {
        mapping[col] = 'description';
      } else if (lower === 'tip' || lower === 'type' || lower === 'categorie') {
        mapping[col] = 'transaction_type';
      }
    });

    return mapping;
  }

  async function importTransactions() {
    console.log('🔥 IMPORT STARTED - Function called!', { transactionCount: previewData.length });
    isImporting = true;

    try {
      // Get accounts and categories from database first
      const accounts = await invoke('get_accounts');
      const categories = await invoke('get_categories');

      if (!accounts || accounts.length === 0) {
        throw new Error('No accounts found. Please create an account first.');
      }

      if (!categories || categories.length === 0) {
        throw new Error('No categories found. Please create a category first.');
      }

      const defaultAccountId = accounts[0].id;
      const defaultCategoryId = categories[0].id;

      console.log('Using account:', defaultAccountId, 'and category:', defaultCategoryId);

      // Prepare transactions for import
      const transactions = previewData.map(row => {
        // Find which Excel column maps to each field
        const dateCol = Object.keys(columnMapping).find(k => columnMapping[k] === 'date');
        const amountCol = Object.keys(columnMapping).find(k => columnMapping[k] === 'amount');
        const descCol = Object.keys(columnMapping).find(k => columnMapping[k] === 'description');
        const typeCol = Object.keys(columnMapping).find(k => columnMapping[k] === 'transaction_type');

        // Convert DD.MM.YYYY to YYYY-MM-DD
        const dateParts = row[dateCol].split('.');
        const isoDate = `${dateParts[2]}-${dateParts[1]}-${dateParts[0]}`;

        return {
          account_id: defaultAccountId,
          category_id: defaultCategoryId,
          amount: parseFloat(row[amountCol]),
          description: row[descCol] || "",
          transaction_type: row[typeCol] || "expense",
          date: isoDate
        };
      });

      console.log('📦 Transactions prepared:', transactions);
      console.log('📡 About to call backend with:', { count: transactions.length });

      // Call Rust backend
      const result = await invoke('batch_insert_transactions', { transactions });

      // Show success message
      alert(`✅ Import reușit! ${result.inserted} tranzacții noi adăugate. ${result.duplicates || 0} duplicate ignorate.`);

      // Reset state
      fileName = '';
      rowCount = 0;
      columns = [];
      columnMapping = {};
      previewData = [];
      canImport = false;

    } catch (error) {
      alert(`❌ Eroare la import: ${error.message || error}`);
      console.error('Import error:', error);
    } finally {
      isImporting = false;
    }
  }
</script>

<div class="p-6">
  <h1 class="text-2xl font-bold mb-6">Import Transactions</h1>

  <div class="card bg-base-200 p-6 mb-4">
    <button class="btn btn-primary" on:click={selectFile}>
      Select Excel File
    </button>
  </div>

  {#if error}
    <div class="alert alert-error mb-4">
      <span>{error}</span>
    </div>
  {/if}

  {#if fileName}
    <div class="card bg-base-200 p-6">
      <h2 class="text-lg font-semibold mb-2">File Info</h2>
      <p><strong>Name:</strong> {fileName}</p>
      <p><strong>Rows:</strong> {rowCount}</p>
      <p><strong>Columns:</strong> {columns.join(', ')}</p>
    </div>
  {/if}

  <!-- Column Mapping Section (show only if file loaded) -->
  {#if fileName && rowCount > 0}
    <div class="mt-8">
      <div class="card bg-base-200 p-6">
        <h3 class="text-xl font-semibold mb-4">Map Columns</h3>
        <p class="text-sm text-gray-400 mb-4">Match your Excel columns to transaction fields:</p>

        <div class="space-y-3">
          {#each columns as column, index}
            <div class="flex items-center gap-4">
              <div class="w-1/3 font-medium">{column}</div>
              <select
                class="select select-bordered w-2/3"
                bind:value={columnMapping[column]}
              >
                <option value="">-- Select Field --</option>
                <option value="date">Date</option>
                <option value="amount">Amount</option>
                <option value="description">Description</option>
                <option value="transaction_type">Type (income/expense)</option>
                <option value="account_id">Account ID</option>
                <option value="category_id">Category ID</option>
              </select>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  <!-- Preview Section -->
  {#if fileName && rowCount > 0 && Object.keys(columnMapping).length > 0}
    <div class="mt-8">
      <div class="card bg-base-200 p-6">
        <h3 class="text-xl font-semibold mb-4">Preview Data</h3>
        <p class="text-sm text-gray-400 mb-4">First 3 rows from your Excel file:</p>

        <div class="overflow-x-auto">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                {#each columns as col}
                  <th>{col} → {columnMapping[col] || 'Not mapped'}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each previewData.slice(0, 3) as row}
                <tr>
                  {#each columns as col}
                    <td>{row[col]}</td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- Import Button -->
    <div class="mt-8 flex justify-center">
      <button
        class="btn btn-primary btn-lg"
        disabled={!canImport || isImporting}
        on:click={importTransactions}
      >
        {#if isImporting}
          Se procesează...
        {:else}
          Import {rowCount} Transactions
        {/if}
      </button>
    </div>
  {/if}
</div>