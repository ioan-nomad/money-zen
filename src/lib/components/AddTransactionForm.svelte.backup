<script lang="ts">
  import type { Account, Category } from '../database';

  export let accounts: Account[];
  export let categories: Category[];
  export let onSubmit: (data: {
    accountId: string;
    categoryId: string;
    amount: number;
    description: string;
    type: 'income' | 'expense';
  }) => void;

  let selectedAccountId = accounts.length > 0 ? accounts[0].id : '';
  let selectedCategoryId = '';
  let transactionType: 'income' | 'expense' = 'expense';
  let amount = 0;
  let description = '';

  $: filteredCategories = categories.filter(c => c.category_type === transactionType);
  $: {
    if (filteredCategories.length > 0 && !filteredCategories.find(c => c.id === selectedCategoryId)) {
      selectedCategoryId = filteredCategories[0].id;
    }
  }

  function handleSubmit() {
    if (selectedAccountId && selectedCategoryId && amount > 0) {
      onSubmit({
        accountId: selectedAccountId,
        categoryId: selectedCategoryId,
        amount,
        description,
        type: transactionType
      });
      amount = 0;
      description = '';
    }
  }
</script>

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h3 class="card-title">💸 Add Transaction</h3>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
      <select class="select select-bordered" bind:value={selectedAccountId}>
        {#each accounts as account}
          <option value={account.id}>{account.name}</option>
        {/each}
      </select>

      <select class="select select-bordered" bind:value={transactionType}>
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
        bind:value={amount}
      />

      <input
        type="text"
        placeholder="Description"
        class="input input-bordered"
        bind:value={description}
      />

      <button
        class="btn btn-primary md:col-span-2"
        on:click={handleSubmit}
        disabled={!selectedAccountId || !selectedCategoryId || amount <= 0}
      >
        Add Transaction
      </button>
    </div>
  </div>
</div>
