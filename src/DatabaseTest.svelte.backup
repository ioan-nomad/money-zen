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

<div class="database-test">
  <h2>💾 MoneyZen Database Test</h2>

  <div class="status">
    <h3>Connection Status: {dbStatus}</h3>
    {#if error}
      <div class="error">Error: {error}</div>
    {/if}
  </div>

  <div class="section">
    <h3>📊 Accounts ({accounts.length})</h3>
    <div class="form">
      <input
        bind:value={newAccountName}
        placeholder="Account name"
      />
      <select bind:value={newAccountType}>
        <option value="checking">Checking</option>
        <option value="savings">Savings</option>
        <option value="credit">Credit Card</option>
        <option value="cash">Cash</option>
      </select>
      <button on:click={createAccount} disabled={!newAccountName}>
        Add Account
      </button>
    </div>
    <div class="list">
      {#each accounts as account}
        <div class="item">
          <span class="name">{account.name}</span>
          <span class="type">({account.account_type})</span>
          <span class="balance">{formatCurrency(account.balance, account.currency)}</span>
        </div>
      {/each}
    </div>
  </div>

  <div class="section">
    <h3>🏷️ Categories ({categories.length})</h3>
    <div class="categories">
      {#each categories as category}
        <div class="category" style="border-left: 4px solid {category.color}">
          <span class="icon">{category.icon}</span>
          <span class="name">{category.name}</span>
          <span class="type">({category.category_type})</span>
        </div>
      {/each}
    </div>
  </div>

  <div class="section">
    <h3>💸 Add Transaction</h3>
    <div class="form">
      <select bind:value={selectedAccountId}>
        {#each accounts as account}
          <option value={account.id}>{account.name}</option>
        {/each}
      </select>

      <select bind:value={newTransactionType} on:change={onTransactionTypeChange}>
        <option value="expense">Expense</option>
        <option value="income">Income</option>
      </select>

      <select bind:value={selectedCategoryId}>
        {#each filteredCategories as category}
          <option value={category.id}>{category.icon} {category.name}</option>
        {/each}
      </select>

      <input
        type="number"
        bind:value={newTransactionAmount}
        placeholder="Amount"
        step="0.01"
      />

      <input
        bind:value={newTransactionDescription}
        placeholder="Description"
      />

      <button
        on:click={createTransaction}
        disabled={!selectedAccountId || !selectedCategoryId || newTransactionAmount <= 0}
      >
        Add Transaction
      </button>
    </div>
  </div>

  <div class="section">
    <h3>📋 Transactions ({transactions.length})</h3>
    <div class="transactions">
      {#each transactions as transaction}
        <div class="transaction">
          <span class="amount" class:income={transaction.transaction_type === 'income'}>
            {transaction.transaction_type === 'income' ? '+' : '-'}{formatCurrency(transaction.amount)}
          </span>
          <span class="description">{transaction.description}</span>
          <span class="date">{formatDate(transaction.date)}</span>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .database-test {
    padding: 1rem;
    max-width: 800px;
    margin: 0 auto;
  }

  .status h3 {
    color: #333;
  }

  .error {
    color: red;
    background: #fee;
    padding: 0.5rem;
    border-radius: 4px;
    margin: 0.5rem 0;
  }

  .section {
    margin: 2rem 0;
    padding: 1rem;
    border: 1px solid #ddd;
    border-radius: 8px;
  }

  .form {
    display: flex;
    gap: 0.5rem;
    margin: 1rem 0;
    flex-wrap: wrap;
  }

  .form input, .form select, .form button {
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  .form button {
    background: #3B82F6;
    color: white;
    cursor: pointer;
  }

  .form button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .list, .categories, .transactions {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .item {
    color: #333;
  }

  .item .name {
    color: #1a1a1a;
  }

  .item .balance {
    color: #1a1a1a;
    font-weight: bold;
  }

  .item, .category, .transaction {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: #f9f9f9;
    border-radius: 4px;
  }

  .name {
    font-weight: bold;
  }

  .type {
    color: #666;
    font-size: 0.9em;
  }

  .balance {
    margin-left: auto;
    font-weight: bold;
  }

  .amount {
    font-weight: bold;
    color: #EF4444;
  }

  .amount.income {
    color: #22C55E;
  }

  .description {
    flex: 1;
  }

  .date {
    color: #666;
    font-size: 0.9em;
  }

  .icon {
    font-size: 1.2em;
  }
</style>