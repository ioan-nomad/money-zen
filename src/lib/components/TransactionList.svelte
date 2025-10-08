<script lang="ts">
  import type { Transaction, Account, Category } from '../database';
  import { Database } from '../database';  // ← NEW IMPORT
  import TransactionItem from './TransactionItem.svelte';

  export let transactions: Transaction[];
  export let accounts: Account[] = [];
  export let categories: Category[] = [];

  let searchQuery = '';
  let filterAccount = 'all';
  let filterCategory = 'all';
  let filterType: 'all' | 'income' | 'expense' = 'all';

  $: filteredTransactions = transactions.filter(t => {
    const matchesSearch = t.description.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesAccount = filterAccount === 'all' || t.account_id === filterAccount;
    const matchesCategory = filterCategory === 'all' || t.category_id === filterCategory;
    const matchesType = filterType === 'all' || t.transaction_type === filterType;
    return matchesSearch && matchesAccount && matchesCategory && matchesType;
  });

  // ← NEW FUNCTION: Handle delete event
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

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h2 class="card-title">All Transactions ({filteredTransactions.length})</h2>

    <!-- Filters -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-3 mb-4">
      <input
        type="text"
        placeholder="Search description..."
        class="input input-bordered"
        bind:value={searchQuery}
      />

      <select class="select select-bordered" bind:value={filterAccount}>
        <option value="all">All Accounts</option>
        {#each accounts as account}
          <option value={account.id}>{account.name}</option>
        {/each}
      </select>

      <select class="select select-bordered" bind:value={filterCategory}>
        <option value="all">All Categories</option>
        {#each categories as category}
          <option value={category.id}>{category.icon} {category.name}</option>
        {/each}
      </select>

      <select class="select select-bordered" bind:value={filterType}>
        <option value="all">All Types</option>
        <option value="income">Income</option>
        <option value="expense">Expense</option>
      </select>
    </div>

    <!-- Transactions List -->
    <div class="space-y-2">
      {#if filteredTransactions.length === 0}
        <div class="text-center py-8 text-base-content/50">
          No transactions found
        </div>
      {:else}
        {#each filteredTransactions as transaction}
          <TransactionItem
            {transaction}
            {accounts}
            {categories}
            on:delete={handleDelete}
          />
        {/each}
      {/if}
    </div>
  </div>
</div>