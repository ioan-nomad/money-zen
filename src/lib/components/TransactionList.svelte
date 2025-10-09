<script lang="ts">
  import type { Transaction, Account, Category, Tag } from '../database';
  import { Database } from '../database';
  import TransactionItem from './TransactionItem.svelte';
  import AdvancedFilters, { type TransactionFilters } from './AdvancedFilters.svelte';

  export let transactions: Transaction[];
  export let accounts: Account[] = [];
  export let categories: Category[] = [];
  export let tags: Tag[] = [];

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
        const transactionTags = await Database.getTransactionTags(transaction.id);
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
      await Database.deleteTransaction(transactionId);

      // Remove from local array to update UI immediately
      transactions = transactions.filter(t => t.id !== transactionId);
    } catch (error) {
      console.error('Failed to delete transaction:', error);
      alert('Eroare la ștergerea tranzacției. Încearcă din nou.');
    }
  }
</script>

<div class="space-y-4">
  <!-- Advanced Filters -->
  <AdvancedFilters
    {accounts}
    {categories}
    {tags}
    on:filtersChanged={handleFiltersChanged}
  />

  <!-- Transactions Card -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">
        All Transactions ({filteredTransactions.length})
        {#if filteredTransactions.length !== transactions.length}
          <span class="text-sm font-normal text-base-content/70">
            of {transactions.length} total
          </span>
        {/if}
      </h2>

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
              {transaction}
              {accounts}
              {categories}
              {tags}
              on:delete={handleDelete}
            />
          {/each}
        {/if}
      </div>
    </div>
  </div>
</div>