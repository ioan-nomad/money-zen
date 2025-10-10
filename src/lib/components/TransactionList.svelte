<script lang="ts">
  import { onMount } from 'svelte';
  import { accountStore } from '../../ui/stores/accountStore';
  import { categoryStore } from '../../ui/stores/categoryStore';
  import { tagStore } from '../../ui/stores/tagStore';
  import { transactionStore } from '../../ui/stores/transactionStore';
  import TransactionItem from './TransactionItem.svelte';
  import AdvancedFilters, { type TransactionFilters } from './AdvancedFilters.svelte';
  import BulkTagEditorModal from './BulkTagEditorModal.svelte';

  let currentFilters: TransactionFilters = {
    searchQuery: '',
    accountIds: [],
    categoryIds: [],
    tagIds: [],
    tagMode: 'or',
    transactionType: 'all',
    dateFrom: '',
    dateTo: '',
    amountMin: 0,
    amountMax: 10000
  };

  // Reactive store subscriptions
  $: transactions = $transactionStore;
  $: accounts = $accountStore;
  $: categories = $categoryStore;
  $: tags = $tagStore;

  onMount(async () => {
    await Promise.all([
      transactionStore.load(),
      accountStore.load(),
      categoryStore.load(),
      tagStore.load()
    ]);
  });

  // Bulk operations state
  let selectedTransactionIds: Set<string> = new Set();
  let selectAll = false;
  let showBulkDeleteModal = false;
  let showBulkTagEditor = false;
  let error = '';

  // Track transaction tags for filtering
  let transactionTagsMap = new Map<string, string[]>();
  let isLoadingTags = false;

  // Load tags for all transactions when transactions or tag filters change
  $: if (transactions.length > 0 && currentFilters.tagIds.length > 0) {
    loadTransactionTags();
  }

  // Filter transactions based on current filters
  $: filteredTransactions = filterTransactions(transactions, currentFilters, transactionTagsMap);

  async function loadTransactionTags() {
    if (isLoadingTags) return;
    isLoadingTags = true;

    try {
      const tagPromises = transactions.map(async (transaction) => {
        const transactionTags = await tagStore.loadByTransaction(transaction.id);
        return { transactionId: transaction.id, tagIds: transactionTags.map(tag => tag.id) };
      });

      const results = await Promise.all(tagPromises);
      transactionTagsMap = new Map(results.map(r => [r.transactionId, r.tagIds]));
    } catch (error) {
      console.error('Failed to load transaction tags:', error);
    } finally {
      isLoadingTags = false;
    }
  }

  function filterTransactions(transactions: Transaction[], filters: TransactionFilters, tagsMap: Map<string, string[]>): Transaction[] {
    return transactions.filter(t => {
      // Search query filter
      if (filters.searchQuery.trim()) {
        const searchLower = filters.searchQuery.toLowerCase();
        if (!t.description.toLowerCase().includes(searchLower)) {
          return false;
        }
      }

      // Account filter
      if (filters.accountIds.length > 0) {
        if (!filters.accountIds.includes(t.account_id)) {
          return false;
        }
      }

      // Category filter
      if (filters.categoryIds.length > 0) {
        if (!filters.categoryIds.includes(t.category_id)) {
          return false;
        }
      }

      // Transaction type filter
      if (filters.transactionType !== 'all') {
        if (t.transaction_type !== filters.transactionType) {
          return false;
        }
      }

      // Date range filter
      if (filters.dateFrom || filters.dateTo) {
        const transactionDate = new Date(t.date).toISOString().split('T')[0];

        if (filters.dateFrom && transactionDate < filters.dateFrom) {
          return false;
        }

        if (filters.dateTo && transactionDate > filters.dateTo) {
          return false;
        }
      }

      // Amount range filter
      if (t.amount < filters.amountMin || t.amount > filters.amountMax) {
        return false;
      }

      // Tag filter
      if (filters.tagIds.length > 0) {
        const transactionTagIds = tagsMap.get(t.id) || [];

        if (filters.tagMode === 'and') {
          // ALL selected tags must be present
          if (!filters.tagIds.every(tagId => transactionTagIds.includes(tagId))) {
            return false;
          }
        } else {
          // OR mode: ANY of the selected tags must be present
          if (!filters.tagIds.some(tagId => transactionTagIds.includes(tagId))) {
            return false;
          }
        }
      }

      return true;
    });
  }

  function handleFiltersChanged(event: CustomEvent) {
    currentFilters = event.detail.filters;
  }

  // Handle delete event
  async function handleDelete(event: CustomEvent<string>) {
    const transactionId = event.detail;

    try {
      await transactionStore.remove(transactionId);
      // Store automatically updates the UI, no need to manually filter
    } catch (error) {
      console.error('Failed to delete transaction:', error);
      alert('Eroare la ștergerea tranzacției. Încearcă din nou.');
    }
  }

  // Toggle individual transaction selection
  function toggleSelection(transactionId: string) {
    const newSelection = new Set(selectedTransactionIds);
    if (newSelection.has(transactionId)) {
      newSelection.delete(transactionId);
    } else {
      newSelection.add(transactionId);
    }
    selectedTransactionIds = newSelection;
    selectAll = filteredTransactions.length > 0 && newSelection.size === filteredTransactions.length;
  }

  // Toggle select all
  function toggleSelectAll() {
    if (selectAll) {
      selectedTransactionIds = new Set();
      selectAll = false;
    } else {
      selectedTransactionIds = new Set(filteredTransactions.map(t => t.id));
      selectAll = true;
    }
  }

  // Bulk operations handlers
  function handleBulkDelete() {
    if (selectedTransactionIds.size === 0) return;
    showBulkDeleteModal = true;
  }

  async function confirmBulkDelete() {
    try {
      const idsArray = Array.from(selectedTransactionIds);
      const deletedCount = await transactionStore.removeMultiple(idsArray);

      showBulkDeleteModal = false;
      selectedTransactionIds = new Set();
      selectAll = false;

      // Store automatically updates the UI, no need to manually filter

      console.log(`Successfully deleted ${deletedCount} transactions`);
    } catch (err) {
      console.error('Bulk delete failed:', err);
      alert('Error deleting transactions. Please try again.');
    }
  }

  function cancelBulkDelete() {
    showBulkDeleteModal = false;
  }

  function openBulkTagEditor() {
    if (selectedTransactionIds.size === 0) return;
    showBulkTagEditor = true;
  }

</script>

<div class="space-y-4">
  <!-- Advanced Filters -->
  <AdvancedFilters
    accounts={accounts}
    categories={categories}
    tags={tags}
    on:filtersChanged={handleFiltersChanged}
  />

  <!-- Transactions Card -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <!-- Header with Bulk Operations Controls -->
      <div class="flex justify-between items-center mb-4">
        <div class="flex items-center gap-4">
          <h2 class="text-2xl font-bold">Transaction History ({filteredTransactions.length})</h2>

          {#if filteredTransactions.length > 0}
            <label class="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                class="checkbox checkbox-sm"
                checked={selectAll}
                on:change={toggleSelectAll}
              />
              <span class="text-sm">Select All</span>
            </label>
          {/if}
        </div>

        {#if selectedTransactionIds.size > 0}
          <div class="flex gap-2 items-center">
            <span class="badge badge-primary">{selectedTransactionIds.size} selected</span>
            <button class="btn btn-sm btn-error" on:click={handleBulkDelete}>
              🗑️ Delete Selected
            </button>
            <button class="btn btn-sm btn-primary" on:click={openBulkTagEditor}>
              🏷️ Edit Tags
            </button>
          </div>
        {/if}
      </div>

      <!-- Transactions List -->
      <div class="space-y-2">
        {#if filteredTransactions.length === 0}
          <div class="text-center py-8 text-base-content/50">
            {#if transactions.length === 0}
              No transactions found
            {:else}
              No transactions match the current filters
              <div class="mt-2">
                <button
                  class="btn btn-sm btn-outline"
                  on:click={() => {
                    currentFilters = {
                      searchQuery: '',
                      accountIds: [],
                      categoryIds: [],
                      tagIds: [],
                      tagMode: 'or',
                      transactionType: 'all',
                      dateFrom: '',
                      dateTo: '',
                      amountMin: 0,
                      amountMax: 10000
                    };
                  }}
                >
                  Clear all filters
                </button>
              </div>
            {/if}
          </div>
        {:else}
          {#each filteredTransactions as transaction}
            <TransactionItem
              transaction={transaction}
              accounts={accounts}
              categories={categories}
              tags={tags}
              isSelected={selectedTransactionIds.has(transaction.id)}
              onToggleSelection={toggleSelection}
              on:delete={handleDelete}
            />
          {/each}
        {/if}
      </div>
    </div>
  </div>
</div>

<!-- Bulk Delete Confirmation Modal -->
{#if showBulkDeleteModal}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-base-100 p-6 rounded-lg shadow-xl max-w-md w-full mx-4">
      <h3 class="text-lg font-bold mb-4">Confirm Bulk Delete</h3>
      <p class="mb-6">
        Are you sure you want to delete <span class="font-bold text-error">{selectedTransactionIds.size}</span>
        selected transaction{selectedTransactionIds.size === 1 ? '' : 's'}?
      </p>
      <p class="text-sm opacity-70 mb-6">This action cannot be undone.</p>

      <div class="flex gap-3 justify-end">
        <button
          class="btn btn-ghost"
          on:click={cancelBulkDelete}
        >
          Cancel
        </button>
        <button
          class="btn btn-error"
          on:click={confirmBulkDelete}
        >
          🗑️ Delete {selectedTransactionIds.size} Transaction{selectedTransactionIds.size === 1 ? '' : 's'}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Bulk Tag Editor Modal -->
{#if showBulkTagEditor}
  <BulkTagEditorModal
    selectedTransactionIds={selectedTransactionIds}
    on:close={() => { showBulkTagEditor = false; selectedTransactionIds = new Set(); selectAll = false; }}
  />
{/if}