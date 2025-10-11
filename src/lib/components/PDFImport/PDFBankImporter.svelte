<script lang="ts">
  import { PDFParserService } from '../../../core/services/PDFParserService';
  import { transactionStore } from '../../../ui/stores/transactionStore';
  import { categoryStore } from '../../../ui/stores/categoryStore';
  import { accountStore } from '../../../ui/stores/accountStore';
  import { notificationStore } from '../../../ui/stores/notificationStore';
  import type { Transaction } from '../../../core/entities/Transaction';
  import { onMount } from 'svelte';

  interface ParsedTransaction {
    date: string;
    description: string;
    amount: number;
    type: 'income' | 'expense';
    suggestedCategory?: string;
    confidence?: number;
  }

  interface TransactionPreview extends ParsedTransaction {
    selected: boolean;
    account_id: string;
    category_id: string;
  }

  let fileInput: HTMLInputElement;
  let isProcessing = false;
  let dragActive = false;
  let parsedTransactions: TransactionPreview[] = [];
  let selectedFile: File | null = null;
  let bankDetected = '';
  let accounts: any[] = [];
  let categories: any[] = [];

  const pdfParser = PDFParserService.getInstance();

  onMount(async () => {
    accounts = await accountStore.getAll();
    categories = await categoryStore.getAll();
  });

  function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files[0]) {
      processFile(target.files[0]);
    }
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    dragActive = false;

    if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
      processFile(event.dataTransfer.files[0]);
    }
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    dragActive = true;
  }

  function handleDragLeave() {
    dragActive = false;
  }

  async function processFile(file: File) {
    if (!file.type.includes('pdf')) {
      notificationStore.error('Vă rog selectați un fișier PDF valid');
      return;
    }

    selectedFile = file;
    isProcessing = true;
    bankDetected = '';
    parsedTransactions = [];

    try {
      const parsed = await pdfParser.parsePDF(file);

      if (parsed.length === 0) {
        notificationStore.warning('Nu s-au găsit tranzacții în acest PDF');
        return;
      }

      // Convertim la format preview cu setări default
      parsedTransactions = parsed.map(tx => ({
        ...tx,
        selected: true,
        account_id: accounts[0]?.id || '',
        category_id: findCategoryByName(tx.suggestedCategory || 'Diverse')?.id || categories[0]?.id || ''
      }));

      // Detectăm banca din primul rezultat
      detectBank(file.name);

      notificationStore.success(`S-au găsit ${parsed.length} tranzacții în extractul bancar`);

    } catch (error) {
      console.error('PDF parsing error:', error);
      notificationStore.error('Eroare la procesarea PDF-ului: ' + error.message);
    } finally {
      isProcessing = false;
    }
  }

  function detectBank(filename: string) {
    const name = filename.toLowerCase();
    if (name.includes('bt') || name.includes('transilvania')) {
      bankDetected = 'Banca Transilvania';
    } else if (name.includes('bcr')) {
      bankDetected = 'BCR';
    } else if (name.includes('ing')) {
      bankDetected = 'ING Bank';
    } else {
      bankDetected = 'Necunoscută';
    }
  }

  function findCategoryByName(name: string) {
    return categories.find(cat =>
      cat.name.toLowerCase() === name.toLowerCase()
    );
  }

  function toggleTransactionSelection(index: number) {
    parsedTransactions[index].selected = !parsedTransactions[index].selected;
  }

  function toggleSelectAll() {
    const allSelected = parsedTransactions.every(tx => tx.selected);
    parsedTransactions = parsedTransactions.map(tx => ({
      ...tx,
      selected: !allSelected
    }));
  }

  function updateTransactionCategory(index: number, categoryId: string) {
    parsedTransactions[index].category_id = categoryId;

    // Învață pattern-ul pentru viitor
    const category = categories.find(c => c.id === categoryId);
    if (category) {
      pdfParser.learnPattern(parsedTransactions[index].description, category.name);
    }
  }

  function updateTransactionAccount(index: number, accountId: string) {
    parsedTransactions[index].account_id = accountId;
  }

  async function importSelectedTransactions() {
    const selected = parsedTransactions.filter(tx => tx.selected);

    if (selected.length === 0) {
      notificationStore.warning('Vă rog selectați cel puțin o tranzacție pentru import');
      return;
    }

    isProcessing = true;
    let successCount = 0;
    let errorCount = 0;

    try {
      for (const tx of selected) {
        try {
          const transaction: Omit<Transaction, 'id' | 'created_at' | 'updated_at'> = {
            account_id: tx.account_id,
            category_id: tx.category_id,
            amount: tx.amount,
            description: tx.description,
            transaction_type: tx.type,
            date: tx.date,
            tags: []
          };

          await transactionStore.create(transaction);
          successCount++;
        } catch (error) {
          console.error('Error importing transaction:', error);
          errorCount++;
        }
      }

      if (successCount > 0) {
        notificationStore.success(`S-au importat cu succes ${successCount} tranzacții`);
      }

      if (errorCount > 0) {
        notificationStore.error(`${errorCount} tranzacții nu au putut fi importate`);
      }

      // Reset după import
      if (successCount === selected.length) {
        resetImport();
      }

    } finally {
      isProcessing = false;
    }
  }

  function resetImport() {
    selectedFile = null;
    parsedTransactions = [];
    bankDetected = '';
    if (fileInput) fileInput.value = '';
  }

  function formatAmount(amount: number, type: 'income' | 'expense') {
    const formatted = amount.toFixed(2);
    return type === 'income' ? `+${formatted} RON` : `-${formatted} RON`;
  }

  function getConfidenceColor(confidence?: number) {
    if (!confidence) return 'badge-ghost';
    if (confidence >= 90) return 'badge-success';
    if (confidence >= 70) return 'badge-warning';
    return 'badge-error';
  }
</script>

<div class="card bg-base-100 shadow-xl max-w-4xl mx-auto">
  <div class="card-body">
    <h2 class="card-title text-2xl mb-6">
      📄 Import Extract Bancar PDF
    </h2>

    <!-- File Upload Area -->
    <div class="mb-6">
      <div
        class="border-2 border-dashed border-base-300 rounded-lg p-8 text-center transition-colors {dragActive ? 'border-primary bg-primary bg-opacity-5' : ''}"
        on:drop={handleDrop}
        on:dragover={handleDragOver}
        on:dragleave={handleDragLeave}
      >
        {#if selectedFile}
          <div class="flex items-center justify-center gap-4">
            <div class="text-success">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"></path>
              </svg>
            </div>
            <div>
              <p class="font-semibold">{selectedFile.name}</p>
              {#if bankDetected}
                <p class="text-sm text-base-content/70">Bancă detectată: {bankDetected}</p>
              {/if}
            </div>
            <button class="btn btn-ghost btn-sm" on:click={resetImport}>
              Șterge
            </button>
          </div>
        {:else}
          <div class="space-y-4">
            <div class="text-base-content/50">
              <svg class="w-12 h-12 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
              </svg>
            </div>
            <div>
              <p class="text-lg font-semibold mb-2">Glisează PDF-ul aici sau click pentru a selecta</p>
              <p class="text-sm text-base-content/70">Suportăm extracte de la BT, BCR și ING Bank</p>
            </div>
            <input
              bind:this={fileInput}
              type="file"
              accept=".pdf"
              class="file-input file-input-bordered file-input-primary w-full max-w-xs"
              on:change={handleFileSelect}
            />
          </div>
        {/if}
      </div>
    </div>

    <!-- Loading State -->
    {#if isProcessing}
      <div class="text-center py-8">
        <div class="loading loading-spinner loading-lg text-primary"></div>
        <p class="mt-4">Se procesează PDF-ul...</p>
      </div>
    {/if}

    <!-- Transaction Preview -->
    {#if parsedTransactions.length > 0 && !isProcessing}
      <div class="space-y-4">
        <!-- Controls -->
        <div class="flex justify-between items-center">
          <div class="flex gap-2">
            <button class="btn btn-sm btn-ghost" on:click={toggleSelectAll}>
              {parsedTransactions.every(tx => tx.selected) ? 'Deselectează tot' : 'Selectează tot'}
            </button>
            <div class="badge badge-info">
              {parsedTransactions.filter(tx => tx.selected).length} / {parsedTransactions.length} selectate
            </div>
          </div>
          <button
            class="btn btn-primary"
            class:loading={isProcessing}
            disabled={!parsedTransactions.some(tx => tx.selected) || isProcessing}
            on:click={importSelectedTransactions}
          >
            Importă Tranzacțiile
          </button>
        </div>

        <!-- Transaction List -->
        <div class="overflow-x-auto">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                <th>Selectat</th>
                <th>Data</th>
                <th>Descriere</th>
                <th>Sumă</th>
                <th>Cont</th>
                <th>Categorie</th>
                <th>Încredere</th>
              </tr>
            </thead>
            <tbody>
              {#each parsedTransactions as transaction, index}
                <tr class="hover" class:opacity-50={!transaction.selected}>
                  <td>
                    <input
                      type="checkbox"
                      class="checkbox checkbox-primary"
                      bind:checked={transaction.selected}
                      on:change={() => toggleTransactionSelection(index)}
                    />
                  </td>
                  <td class="font-mono text-sm">{transaction.date}</td>
                  <td>
                    <div class="max-w-xs truncate" title={transaction.description}>
                      {transaction.description}
                    </div>
                  </td>
                  <td>
                    <span class="font-semibold" class:text-success={transaction.type === 'income'} class:text-error={transaction.type === 'expense'}>
                      {formatAmount(transaction.amount, transaction.type)}
                    </span>
                  </td>
                  <td>
                    <select
                      class="select select-bordered select-sm w-full max-w-xs"
                      bind:value={transaction.account_id}
                      on:change={(e) => updateTransactionAccount(index, e.target.value)}
                    >
                      {#each accounts as account}
                        <option value={account.id}>{account.name}</option>
                      {/each}
                    </select>
                  </td>
                  <td>
                    <select
                      class="select select-bordered select-sm w-full max-w-xs"
                      bind:value={transaction.category_id}
                      on:change={(e) => updateTransactionCategory(index, e.target.value)}
                    >
                      {#each categories as category}
                        <option value={category.id}>{category.name}</option>
                      {/each}
                    </select>
                  </td>
                  <td>
                    {#if transaction.confidence}
                      <div class="badge {getConfidenceColor(transaction.confidence)}">
                        {transaction.confidence}%
                      </div>
                    {:else}
                      <div class="badge badge-ghost">Manual</div>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}

    <!-- Help Text -->
    <div class="mt-6 p-4 bg-base-200 rounded-lg">
      <h3 class="font-semibold mb-2">💡 Sfaturi pentru import:</h3>
      <ul class="text-sm space-y-1 text-base-content/70">
        <li>• Suportăm extracte PDF de la Banca Transilvania, BCR și ING Bank</li>
        <li>• Sistemul învață automat categoriile pe baza descrierilor</li>
        <li>• Poți modifica contul și categoria pentru fiecare tranzacție înainte de import</li>
        <li>• Tranzacțiile cu încredere mare (90%+) sunt clasificate automat</li>
      </ul>
    </div>
  </div>
</div>