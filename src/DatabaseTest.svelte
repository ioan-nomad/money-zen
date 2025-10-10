<script lang="ts">
  import { onMount } from 'svelte';
  import { Database, type Account, type Category, type Transaction } from './lib/database';
  import { formatCurrency, formatDate } from './lib/utils';
  import AccountCard from './lib/components/AccountCard.svelte';
  import TransactionItem from './lib/components/TransactionItem.svelte';
  import CategoryBadge from './lib/components/CategoryBadge.svelte';
  import AddTransactionForm from './lib/components/AddTransactionForm.svelte';
  import BackupManager from './lib/components/BackupManager.svelte';

  let dbStatus = 'Checking...';
  let accounts: Account[] = [];
  let categories: Category[] = [];
  let transactions: Transaction[] = [];
  let error = '';

  // Form states
  let newAccountName = '';
  let newAccountType = 'checking';

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

  <BackupManager />

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
          <AccountCard {account} />
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
          <CategoryBadge {category} />
        {/each}
      </div>
    </div>
  </div>

  <!-- Add Transaction Section -->
  <AddTransactionForm />

  <!-- Transactions Section -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h3 class="card-title">📋 Transactions ({transactions.length})</h3>
      
      <div class="space-y-2">
        {#each transactions as transaction}
          <TransactionItem {transaction} />
        {/each}
      </div>
    </div>
  </div>
</div>
