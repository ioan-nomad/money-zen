<script lang="ts">
  import type { Account, Category, Tag } from '../database';
  import GroupedCategoryDropdown from './GroupedCategoryDropdown.svelte';
  import GroupedAccountDropdown from './GroupedAccountDropdown.svelte';
  import TagPicker from './TagPicker.svelte';

  export let accounts: Account[];
  export let categories: Category[];
  export let tags: Tag[];
  export let onSubmit: (data: {
    accountId: string;
    categoryId: string;
    amount: number;
    description: string;
    type: 'income' | 'expense';
    tagIds: string[];
  }) => void;

  let selectedAccountId = accounts.length > 0 ? accounts[0].id : '';
  let selectedCategoryId = '';
  let transactionType: 'income' | 'expense' = 'expense';
  let amount = 0;
  let description = '';
  let selectedTagIds: string[] = [];

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
        type: transactionType,
        tagIds: selectedTagIds
      });
      amount = 0;
      description = '';
      selectedTagIds = [];
    }
  }

  function handleTagChange(event: CustomEvent) {
    selectedTagIds = event.detail.selectedTagIds;
  }
</script>

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h3 class="card-title">💸 Add Transaction</h3>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
      <GroupedAccountDropdown
        accounts={accounts}
        selectedAccountId={selectedAccountId}
        onSelect={(id) => selectedAccountId = id}
      />

      <select class="select select-bordered" bind:value={transactionType}>
        <option value="expense">Expense</option>
        <option value="income">Income</option>
      </select>

      <GroupedCategoryDropdown
        categories={filteredCategories}
        selectedId={selectedCategoryId}
        onChange={(id) => { selectedCategoryId = id; }}
      />

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

      <!-- Tag Picker (spans full width) -->
      <div class="md:col-span-2">
        <label class="label">
          <span class="label-text">Tags (optional)</span>
        </label>
        <TagPicker
          {tags}
          {selectedTagIds}
          placeholder="Select tags to organize this transaction..."
          on:change={handleTagChange}
        />
      </div>

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
