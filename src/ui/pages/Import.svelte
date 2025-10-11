<script lang="ts">
  import { onMount } from 'svelte';
  import { transactionStore } from '../ui/stores/transactionStore';
  import { accountStore } from '../ui/stores/accountStore';
  import { categoryStore } from '../ui/stores/categoryStore';
  import * as XLSX from 'xlsx';
  import { open } from '@tauri-apps/plugin-dialog';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { invoke } from '@tauri-apps/api/core';
  import PDFBankImporter from './components/PDFImport/PDFBankImporter.svelte';

  let error = '';
  let successMessage = '';
  let currentTab: 'excel' | 'pdf' = 'excel';

  // File import state
  let fileName = '';
  let rowCount = 0;
  let columns: string[] = [];
  let rawData: any[] = [];
  let columnMapping = {};
  let previewData = [];
  let canImport = false;
  let isImporting = false;

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      await Promise.all([
        accountStore.load(),
        categoryStore.load()
      ]);
    } catch (err) {
      error = String(err);
    }
  }

  // Reactive subscriptions
  $: accounts = $accountStore;
  $: categories = $categoryStore;

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

      // Check if required fields are mapped
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
    error = '';
    successMessage = '';

    try {
      if (!accounts || accounts.length === 0) {
        throw new Error('No accounts found. Please create an account first.');
      }

      if (!categories || categories.length === 0) {
        throw new Error('No categories found. Please create a category first.');
      }

      const defaultAccountId = accounts[0].id;
      const defaultCategoryId = categories[0].id;

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

      // Call Rust backend
      const result = await invoke('batch_insert_transactions', { transactions });

      // Reload transaction store to reflect imports
      await transactionStore.load();
      await accountStore.load(); // Reload accounts to update balances

      // Show success message
      successMessage = `✅ Import successful! ${result.inserted} new transactions added. ${result.duplicates || 0} duplicates ignored.`;

      // Reset state
      fileName = '';
      rowCount = 0;
      columns = [];
      columnMapping = {};
      previewData = [];
      canImport = false;

    } catch (err) {
      error = `❌ Import error: ${err.message || err}`;
      console.error('Import error:', err);
    } finally {
      isImporting = false;
    }
  }
</script>

<div class="max-w-6xl mx-auto p-6 space-y-6">
  <h1 class="text-4xl font-bold">📥 Import Transactions</h1>

  {#if error}
    <div class="alert alert-error">
      <span>{error}</span>
    </div>
  {/if}

  {#if successMessage}
    <div class="alert alert-success">
      <span>{successMessage}</span>
    </div>
  {/if}

  <!-- Import Method Tabs -->
  <div class="tabs tabs-boxed justify-center mb-6">
    <button
      class="tab tab-lg"
      class:tab-active={currentTab === 'excel'}
      on:click={() => currentTab = 'excel'}
    >
      📊 Excel/CSV
    </button>
    <button
      class="tab tab-lg"
      class:tab-active={currentTab === 'pdf'}
      on:click={() => currentTab = 'pdf'}
    >
      📄 PDF Extract Bancar
    </button>
  </div>

  {#if currentTab === 'excel'}
    <!-- Excel Import Section -->
    <!-- File Selection -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">Select Excel File</h2>
        <p>Import transactions from .xlsx or .xls files</p>
        <button class="btn btn-primary" on:click={selectFile}>
          📁 Select Excel File
        </button>
      </div>
    </div>

  <!-- File Info -->
  {#if fileName}
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">File Info</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <span class="font-semibold">Name:</span>
            <span>{fileName}</span>
          </div>
          <div>
            <span class="font-semibold">Rows:</span>
            <span>{rowCount}</span>
          </div>
          <div>
            <span class="font-semibold">Columns:</span>
            <span>{columns.length}</span>
          </div>
        </div>
        <p class="text-sm text-base-content/70 mt-2">
          Columns: {columns.join(', ')}
        </p>
      </div>
    </div>
  {/if}

  <!-- Column Mapping -->
  {#if fileName && rowCount > 0}
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">Map Columns</h2>
        <p class="text-sm text-base-content/70 mb-4">Match your Excel columns to transaction fields:</p>

        <div class="space-y-3">
          {#each columns as column}
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
              </select>
            </div>
          {/each}
        </div>

        <div class="mt-4 p-4 bg-base-200 rounded-lg">
          <p class="text-sm">
            <span class="font-semibold">Required fields:</span> Date, Amount, Description
          </p>
          <p class="text-sm">
            <span class="font-semibold">Using default:</span> Account: {accounts[0]?.name}, Category: {categories[0]?.name}
          </p>
        </div>
      </div>
    </div>
  {/if}

  <!-- Preview and Import -->
  {#if fileName && rowCount > 0 && Object.keys(columnMapping).length > 0}
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">Preview Data</h2>
        <p class="text-sm text-base-content/70 mb-4">First 3 rows from your Excel file:</p>

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

        <div class="flex justify-center mt-6">
          <button
            class="btn btn-primary btn-lg"
            disabled={!canImport || isImporting}
            on:click={importTransactions}
          >
            {#if isImporting}
              <span class="loading loading-spinner"></span>
              Processing...
            {:else}
              📊 Import {rowCount} Transactions
            {/if}
          </button>
        </div>
      </div>
    </div>
  {/if}

  {:else if currentTab === 'pdf'}
    <!-- PDF Import Section -->
    <PDFBankImporter />
  {/if}
</div>