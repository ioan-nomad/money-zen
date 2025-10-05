<script lang="ts">
  import { onMount } from 'svelte';
  import { Database, type Account, type Category, type Transaction } from './lib/database';
  import { formatCurrency, formatDate } from './lib/utils';

  let dbStatus = 'Checking...';
  let accounts: Account[] = [];
  let categories: Category[] = [];
  let transactions: Transaction[] = [];
  let error = '';

  // Form states
  let newAccountName = '';
  let newAccountType = 'checking';
  let newTransactionAmount = 0;
  let newTransactionDescription = '';
  let newTransactionType: 'income' | 'expense' = 'expense';
  let selectedAccountId = '';
  let selectedCategoryId = '';

  onMount(async () => {
    try {
      dbStatus = 'Initializing database...';
      await Database.initDatabase();
      dbStatus = 'Connected ✅';

      await loadData();
    } catch (err) {
      dbStatus = 'Failed ❌';
      error = String(err);
      console.error('Database error:', err);
    }
  });

  async function loadData() {
    try {
      accounts = await Database.getAccounts();
      categories = await Database.getCategories();
      transactions = await Database.getTransactions();

      if (accounts.length > 0) {
        selectedAccountId = accounts[0].id;
      }
      if (categories.length > 0) {
        selectedCategoryId = categories.find(c => c.category_type === newTransactionType)?.id || '';
      }
    } catch (err) {
      error = String(err);
    }
  }

  async function createAccount() {
    try {
      await Database.createAccount(newAccountName, newAccountType);
      newAccountName = '';
      await loadData();
    } catch (err) {
      error = String(err);
    }
  }

  async function createTransaction() {
    try {
      await Database.createTransaction(
        selectedAccountId,
        selectedCategoryId,
        newTransactionAmount,
        newTransactionDescription,
        newTransactionType
      );
      newTransactionAmount = 0;
      newTransactionDescription = '';
      await loadData();
    } catch (err) {
      error = String(err);
    }
  }

  function onTransactionTypeChange() {
    const filteredCategories = categories.filter(c => c.category_type === newTransactionType);
    if (filteredCategories.length > 0) {
      selectedCategoryId = filteredCategories[0].id;
    }
  }

  $: filteredCategories = categories.filter(c => c.category_type === newTransactionType);
</script>

<div class="max-w-4xl mx-auto p-6 space-y-6">
  <div class="text-center">
    <h2 class="text-3xl font-bold mb-2">💾 MoneyZen Database Test</h2>
    <div class="badge badge-lg badge-primary">{dbStatus}</div>
  </div>

  {#if error}
    <div class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
      <span>Error: {error}</span>
    </div>
  {/if}

  <!-- Accounts Section -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h3 class="card-title">📊 Accounts ({accounts.length})</h3>
      
      <div class="flex flex-wrap gap-2">
        <input
          type="text"
          placeholder="Account name"
          class="input input-bordered flex-1 min-w-[200px]"
          bind:value={newAccountName}
        />
        <select class="select select-bordered" bind:value={newAccountType}>
          <option value="checking">Checking</option>
          <option value="savings">Savings</option>
          <option value="credit">Credit Card</option>
          <option value="cash">Cash</option>
        </select>
        <button 
          class="btn btn-primary" 
          on:click={createAccount} 
          disabled={!newAccountName}
        >
          Add Account
        </button>
      </div>

      <div class="space-y-2 mt-4">
        {#each accounts as account}
          <div class="flex items-center justify-between p-3 bg-base-200 rounded-lg">
            <div class="flex items-center gap-2">
              <span class="font-semibold">{account.name}</span>
              <span class="badge badge-sm badge-ghost">{account.account_type}</span>
            </div>
            <span class="font-bold">{formatCurrency(account.balance, account.currency)}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Categories Section -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h3 class="card-title">🏷️ Categories ({categories.length})</h3>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
        {#each categories as category}
          <div 
            class="flex items-center gap-3 p-3 bg-base-200 rounded-lg border-l-4"
            style="border-left-color: {category.color}"
          >
            <span class="text-2xl">{category.icon}</span>
            <div class="flex-1">
              <span class="font-semibold">{category.name}</span>
              <span class="badge badge-xs badge-ghost ml-2">{category.category_type}</span>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Add Transaction Section -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h3 class="card-title">💸 Add Transaction</h3>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <select class="select select-bordered" bind:value={selectedAccountId}>
          {#each accounts as account}
            <option value={account.id}>{account.name}</option>
          {/each}
        </select>

        <select 
          class="select select-bordered" 
          bind:value={newTransactionType} 
          on:change={onTransactionTypeChange}
        >
          <option value="expense">Expense</option>
          <option value="income">Income</option>
        </select>

        <select class="select select-bordered md:col-span-2" bind:value={selectedCategoryId}>
          {#each filteredCategories as category}
            <option value={category.id}>{category.icon} {category.name}</option>
          {/each}
        </select>

        <input
          type="number"
          placeholder="Amount"
          step="0.01"
          class="input input-bordered"
          bind:value={newTransactionAmount}
        />

        <input
          type="text"
          placeholder="Description"
          class="input input-bordered"
          bind:value={newTransactionDescription}
        />

        <button
          class="btn btn-primary md:col-span-2"
          on:click={createTransaction}
          disabled={!selectedAccountId || !selectedCategoryId || newTransactionAmount <= 0}
        >
          Add Transaction
        </button>
      </div>
    </div>
  </div>

  <!-- Transactions Section -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h3 class="card-title">📋 Transactions ({transactions.length})</h3>
      
      <div class="space-y-2">
        {#each transactions as transaction}
          <div class="flex items-center justify-between p-3 bg-base-200 rounded-lg">
            <span 
              class="font-bold text-lg"
              class:text-success={transaction.transaction_type === 'income'}
              class:text-error={transaction.transaction_type === 'expense'}
            >
              {transaction.transaction_type === 'income' ? '+' : '-'}{formatCurrency(transaction.amount)}
            </span>
            <span class="flex-1 px-4">{transaction.description}</span>
            <span class="text-sm opacity-70">{formatDate(transaction.date)}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
