<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import type { Transaction, Account, Category, Tag } from '../database';
  import { Database } from '../database';
  import GroupedAccountDropdown from './GroupedAccountDropdown.svelte';
  import GroupedCategoryDropdown from './GroupedCategoryDropdown.svelte';
  import TagPicker from './TagPicker.svelte';

  export let transaction: Transaction;
  export let accounts: Account[] = [];
  export let categories: Category[] = [];
  export let tags: Tag[] = [];
  export let onUpdate: (
    id: string,
    accountId: string,
    categoryId: string,
    amount: number,
    description: string,
    transactionType: 'income' | 'expense',
    date: string
  ) => Promise<void>;

  const dispatch = createEventDispatcher();

  // Pre-populate form with transaction data
  let selectedAccountId = transaction.account_id;
  let selectedCategoryId = transaction.category_id;
  let transactionType: 'income' | 'expense' = transaction.transaction_type;
  let amount = transaction.amount;
  let description = transaction.description;
  let dateInput = transaction.date.split('T')[0]; // For date input display
  let timeString = transaction.date.split('T')[1] || '00:00:00Z'; // Preserve original time
  let selectedTagIds: string[] = [];
  let originalTagIds: string[] = [];

  // Filter categories based on transaction type
  $: filteredCategories = categories.filter(c => c.category_type === transactionType);

  onMount(async () => {
    // Load existing tags for this transaction
    try {
      const existingTags = await Database.getTransactionTags(transaction.id);
      originalTagIds = existingTags.map(tag => tag.id);
      selectedTagIds = [...originalTagIds];
    } catch (error) {
      console.error('Failed to load transaction tags:', error);
    }
  });

  async function handleSubmit() {
    if (!selectedAccountId || !selectedCategoryId || amount <= 0 || !description.trim()) {
      return;
    }

    const fullDate = `${dateInput}T${timeString}`;

    // Update the transaction first
    await onUpdate(
      transaction.id,
      selectedAccountId,
      selectedCategoryId,
      amount,
      description,
      transactionType,
      fullDate
    );

    // Handle tag updates separately
    await updateTransactionTags();

    dispatch('close');
  }

  async function updateTransactionTags() {
    // Find tags to add and remove
    const tagsToAdd = selectedTagIds.filter(id => !originalTagIds.includes(id));
    const tagsToRemove = originalTagIds.filter(id => !selectedTagIds.includes(id));

    try {
      if (tagsToRemove.length > 0) {
        await Database.removeTagsFromTransaction(transaction.id, tagsToRemove);
      }
      if (tagsToAdd.length > 0) {
        await Database.addTagsToTransaction(transaction.id, tagsToAdd);
      }
    } catch (error) {
      console.error('Failed to update transaction tags:', error);
    }
  }

  function handleTagChange(event: CustomEvent) {
    selectedTagIds = event.detail.selectedTagIds;
  }

  function handleClose() {
    dispatch('close');
  }
</script>

<!-- Modal Backdrop -->
<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" on:click={handleClose}>
  <!-- Modal Content -->
  <div class="card bg-base-100 shadow-2xl w-full max-w-2xl m-4" on:click|stopPropagation>
    <div class="card-body">
      <!-- Header with Close Button -->
      <div class="flex justify-between items-center mb-4">
        <h2 class="card-title text-2xl">✏️ Edit Transaction</h2>
        <button class="btn btn-sm btn-circle btn-ghost" on:click={handleClose}>✕</button>
      </div>

      <!-- Form Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Account Selection -->
        <div class="form-control">
          <label class="label" for="account">
            <span class="label-text">Account</span>
          </label>
          <GroupedAccountDropdown
            {accounts}
            bind:selectedAccountId
          />
        </div>

        <!-- Transaction Type -->
        <div class="form-control">
          <label class="label" for="type">
            <span class="label-text">Type</span>
          </label>
          <select
            id="type"
            class="select select-bordered"
            bind:value={transactionType}
          >
            <option value="income">💰 Income</option>
            <option value="expense">💳 Expense</option>
          </select>
        </div>

        <!-- Category Selection -->
        <div class="form-control">
          <label class="label" for="category">
            <span class="label-text">Category</span>
          </label>
          <GroupedCategoryDropdown
            categories={filteredCategories}
            selectedId={selectedCategoryId}
            onChange={(id) => selectedCategoryId = id}
          />
        </div>

        <!-- Amount -->
        <div class="form-control">
          <label class="label" for="amount">
            <span class="label-text">Amount</span>
          </label>
          <input
            id="amount"
            type="number"
            step="0.01"
            min="0.01"
            placeholder="0.00"
            class="input input-bordered"
            bind:value={amount}
          />
        </div>

        <!-- Date -->
        <div class="form-control">
          <label class="label" for="date">
            <span class="label-text">Date</span>
          </label>
          <input
            id="date"
            type="date"
            class="input input-bordered"
            bind:value={dateInput}
          />
        </div>

        <!-- Description (Full Width) -->
        <div class="form-control md:col-span-2">
          <label class="label" for="description">
            <span class="label-text">Description</span>
          </label>
          <input
            id="description"
            type="text"
            placeholder="Transaction description"
            class="input input-bordered"
            bind:value={description}
          />
        </div>

        <!-- Tags (Full Width) -->
        <div class="form-control md:col-span-2">
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
      </div>

      <!-- Action Buttons -->
      <div class="card-actions justify-end mt-6">
        <button class="btn btn-ghost" on:click={handleClose}>
          Cancel
        </button>
        <button
          class="btn btn-primary"
          on:click={handleSubmit}
          disabled={!selectedAccountId || !selectedCategoryId || amount <= 0 || !description.trim()}
        >
          💾 Update Transaction
        </button>
      </div>
    </div>
  </div>
</div>