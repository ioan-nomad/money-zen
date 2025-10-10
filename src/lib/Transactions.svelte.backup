<script lang="ts">
  import { onMount } from 'svelte';
  import { Database, type Transaction, type Account, type Category, type Tag } from './database';
  import TransactionList from './components/TransactionList.svelte';

  let transactions: Transaction[] = [];
  let accounts: Account[] = [];
  let categories: Category[] = [];
  let tags: Tag[] = [];
  let error = '';

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      transactions = await Database.getTransactions();
      accounts = await Database.getAccounts();
      categories = await Database.getCategories();
      tags = await Database.getTags();
    } catch (err) {
      error = String(err);
    }
  }
</script>

<div class="max-w-6xl mx-auto p-6">
  {#if error}
    <div class="alert alert-error mb-6">
      <span>Error: {error}</span>
    </div>
  {/if}

  <TransactionList {transactions} {accounts} {categories} {tags} />
</div>
