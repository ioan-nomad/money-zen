<script lang="ts">
  import { onMount } from 'svelte';
  import { accountStore } from '../../ui/stores/accountStore';
  import { categoryStore } from '../../ui/stores/categoryStore';
  import { tagStore } from '../../ui/stores/tagStore';
  import { transactionStore } from '../../ui/stores/transactionStore';
  import GroupedCategoryDropdown from './GroupedCategoryDropdown.svelte';
  import GroupedAccountDropdown from './GroupedAccountDropdown.svelte';
  import TagPicker from './TagPicker.svelte';

  let selectedAccountId = '';
  let selectedCategoryId = '';
  let transactionType: 'income' | 'expense' = 'expense';
  let amount = 0;
  let description = '';
  let selectedTagIds: string[] = [];

  $: filteredCategories = $categoryStore.filter(c => c.category_type === transactionType);
  $: {
    if ($accountStore.length > 0 && !selectedAccountId) {
      selectedAccountId = $accountStore[0].id;
    }
  }
  $: {
    if (filteredCategories.length > 0 && !filteredCategories.find(c => c.id === selectedCategoryId)) {
      selectedCategoryId = filteredCategories[0].id;
    }
  }

  onMount(async () => {
    await Promise.all([
      accountStore.load(),
      categoryStore.load(),
      tagStore.load()
    ]);
  });

  async function handleSubmit() {
    if (selectedAccountId && selectedCategoryId && amount > 0) {
      try {
        await transactionStore.create({
          account_id: selectedAccountId,
          category_id: selectedCategoryId,
          amount,
          description,
          transaction_type: transactionType,
          tag_ids: selectedTagIds,
          transaction_date: new Date().toISOString().split('T')[0]
        });

        // Reset form
        amount = 0;
        description = '';
        selectedTagIds = [];
      } catch (error) {
        console.error('Failed to create transaction:', error);
      }
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
        accounts={$accountStore}
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
