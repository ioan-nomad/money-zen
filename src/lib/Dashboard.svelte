<script lang="ts">
  import { onMount } from 'svelte';
  import { Database, type Account, type Transaction } from './database';
  import AccountCard from './components/AccountCard.svelte';
  import TransactionItem from './components/TransactionItem.svelte';
  import AddTransactionForm from './components/AddTransactionForm.svelte';

  let accounts: Account[] = [];
  let transactions: Transaction[] = [];
  let categories = [];
  let error = '';

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      accounts = await Database.getAccounts();
      transactions = await Database.getTransactions();
      categories = await Database.getCategories();
    } catch (err) {
      error = String(err);
    }
  }

  async function handleTransactionSubmit(data: {
    accountId: string;
    categoryId: string;
    amount: number;
    description: string;
    type: 'income' | 'expense';
  }) {
    try {
      await Database.createTransaction(
        data.accountId,
        data.categoryId,
        data.amount,
        data.description,
        data.type
      );
      await loadData();
    } catch (err) {
      error = String(err);
    }
  }

  $: recentTransactions = transactions.slice(0, 5);
  $: totalBalance = accounts.reduce((sum, acc) => sum + acc.balance, 0);
</script>

<div class="max-w-6xl mx-auto p-6 space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-4xl font-bold">Dashboard</h1>
    <div class="badge badge-lg badge-primary">Total: {totalBalance.toFixed(2)} RON</div>
  </div>

  {#if error}
    <div class="alert alert-error">
      <span>Error: {error}</span>
    </div>
  {/if}

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">Accounts ({accounts.length})</h2>
        <div class="space-y-2">
          {#each accounts as account}
            <AccountCard {account} />
          {/each}
        </div>
      </div>
    </div>

    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">Recent Transactions</h2>
        <div class="space-y-2">
          {#each recentTransactions as transaction}
            <TransactionItem {transaction} {accounts} {categories} />
          {/each}
        </div>
      </div>
    </div>
  </div>

  <AddTransactionForm {accounts} {categories} onSubmit={handleTransactionSubmit} />
</div>