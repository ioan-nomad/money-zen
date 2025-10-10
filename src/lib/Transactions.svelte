<script lang="ts">
  import { onMount } from 'svelte';
  import { transactionStore } from '../ui/stores/transactionStore';
  import { accountStore } from '../ui/stores/accountStore';
  import { categoryStore } from '../ui/stores/categoryStore';
  import { tagStore } from '../ui/stores/tagStore';
  import TransactionList from './components/TransactionList.svelte';

  let error = '';

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      await Promise.all([
        transactionStore.load(),
        accountStore.load(),
        categoryStore.load(),
        tagStore.load()
      ]);
    } catch (err) {
      error = String(err);
    }
  }

  // Reactive subscriptions to stores
  $: transactions = $transactionStore;
  $: accounts = $accountStore;
  $: categories = $categoryStore;
  $: tags = $tagStore;
</script>

<div class="max-w-6xl mx-auto p-6">
  {#if error}
    <div class="alert alert-error mb-6">
      <span>Error: {error}</span>
    </div>
  {/if}

  <TransactionList {transactions} {accounts} {categories} {tags} />
</div>