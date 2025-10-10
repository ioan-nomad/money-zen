<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { accountStore } from '../../ui/stores/accountStore';
  import { categoryStore } from '../../ui/stores/categoryStore';
  import { tagStore } from '../../ui/stores/tagStore';
  import { transactionStore } from '../../ui/stores/transactionStore';
  import type { Transaction } from '../../core/entities/Transaction';
  import GroupedAccountDropdown from './GroupedAccountDropdown.svelte';
  import GroupedCategoryDropdown from './GroupedCategoryDropdown.svelte';
  import TagPicker from './TagPicker.svelte';

  export let transactionId: string;

  const dispatch = createEventDispatcher();

  // Store subscriptions
  $: accounts = $accountStore;
  $: categories = $categoryStore;
  $: tags = $tagStore;

  // Local transaction data
  let transaction: Transaction | null = null;

  // Form fields (will be populated in onMount)
  let selectedAccountId = '';
  let selectedCategoryId = '';
  let transactionType: 'income' | 'expense' = 'expense';
  let amount = 0;
  let description = '';
  let dateInput = '';
  let timeString = '00:00:00Z';
  let selectedTagIds: string[] = [];
  let originalTagIds: string[] = [];
  let isLoading = true;

  // Filter categories based on transaction type
  $: filteredCategories = categories.filter(c => c.category_type === transactionType);

  onMount(async () => {
    try {
      // Load all store data
      await Promise.all([
        accountStore.load(),
        categoryStore.load(),
        tagStore.load(),
        transactionStore.load()
      ]);

      // Find the transaction in the loaded data
      const transactions = $transactionStore;
      transaction = transactions.find(t => t.id === transactionId) || null;

      if (transaction) {
        // Pre-populate form with transaction data
        selectedAccountId = transaction.account_id;
        selectedCategoryId = transaction.category_id;
        transactionType = transaction.transaction_type;
        amount = transaction.amount;
        description = transaction.description;
        dateInput = transaction.date.split('T')[0];
        timeString = transaction.date.split('T')[1] || '00:00:00Z';

        // Load existing tags for this transaction
        const existingTags = await tagStore.loadByTransaction(transaction.id);
        originalTagIds = existingTags.map(tag => tag.id);
        selectedTagIds = [...originalTagIds];
      }
    } catch (error) {
      console.error('Failed to load transaction data:', error);
    } finally {
      isLoading = false;
    }
  });

  async function handleSubmit() {
    if (!transaction || !selectedAccountId || !selectedCategoryId || amount <= 0 || !description.trim()) {
      return;
    }

    try {
      const fullDate = `${dateInput}T${timeString}`;

      // Update the transaction using the store
      await transactionStore.updateTransaction(transaction.id, {
        account_id: selectedAccountId,
        category_id: selectedCategoryId,
        amount,
        description,
        transaction_type: transactionType,
        date: fullDate
      });

      // Handle tag updates separately
      await updateTransactionTags();

      dispatch('close');
    } catch (error) {
      console.error('Failed to update transaction:', error);
    }
  }

  async function updateTransactionTags() {
    if (!transaction) return;

    // Find tags to add and remove
    const tagsToAdd = selectedTagIds.filter(id => !originalTagIds.includes(id));
    const tagsToRemove = originalTagIds.filter(id => !selectedTagIds.includes(id));

    try {
      if (tagsToRemove.length > 0) {
        await tagStore.removeFromTransaction(transaction.id, tagsToRemove);
      }
      if (tagsToAdd.length > 0) {
        await tagStore.addToTransaction(transaction.id, tagsToAdd);
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

      {#if isLoading}
        <!-- Loading State -->
        <div class="flex items-center justify-center py-8">
          <span class="loading loading-spinner loading-lg"></span>
          <span class="ml-3">Loading transaction data...</span>
        </div>
      {:else if !transaction}
        <!-- Error State -->
        <div class="alert alert-error">
          <span>Transaction not found</span>
        </div>
      {:else}

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
      {/if}
    </div>
  </div>
</div>