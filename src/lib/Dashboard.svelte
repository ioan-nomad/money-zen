<script lang="ts">
  import { onMount } from 'svelte';
  import { accountStore } from '../ui/stores/accountStore';
  import { transactionStore } from '../ui/stores/transactionStore';
  import { categoryStore } from '../ui/stores/categoryStore';
  import { tagStore } from '../ui/stores/tagStore';
  import AccountCard from './components/AccountCard.svelte';
  import TransactionItem from './components/TransactionItem.svelte';
  import AddTransactionForm from './components/AddTransactionForm.svelte';

  let error = '';

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      await Promise.all([
        accountStore.load(),
        transactionStore.load(),
        categoryStore.load(),
        tagStore.load()
      ]);
    } catch (err) {
      error = String(err);
    }
  }


  // Reactive subscriptions to stores
  $: accounts = $accountStore;
  $: transactions = $transactionStore;
  $: categories = $categoryStore;
  $: tags = $tagStore;

  // Computed values
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
            <TransactionItem {transaction} {accounts} {categories} {tags} />
          {/each}
        </div>
      </div>
    </div>
  </div>

  <AddTransactionForm />
</div>