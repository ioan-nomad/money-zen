<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Account, Category, Tag } from '../database';
  import TagPicker from './TagPicker.svelte';

  export let accounts: Account[] = [];
  export let categories: Category[] = [];
  export let tags: Tag[] = [];

  const dispatch = createEventDispatcher<{
    filtersChanged: {
      filters: TransactionFilters;
    };
  }>();

  export interface TransactionFilters {
    searchQuery: string;
    accountIds: string[];
    categoryIds: string[];
    tagIds: string[];
    tagMode: 'and' | 'or';
    transactionType: 'all' | 'income' | 'expense';
    dateFrom: string;
    dateTo: string;
    amountMin: number;
    amountMax: number;
  }

  // Filter state
  let isExpanded = false;
  let filters: TransactionFilters = {
    searchQuery: '',
    accountIds: [],
    categoryIds: [],
    tagIds: [],
    tagMode: 'or',
    transactionType: 'all',
    dateFrom: '',
    dateTo: '',
    amountMin: 0,
    amountMax: 10000
  };

  // Reactive variables for form controls
  $: incomeCategories = categories.filter(c => c.category_type === 'income');
  $: expenseCategories = categories.filter(c => c.category_type === 'expense');

  // Active filter count
  $: activeFilterCount = getActiveFilterCount();

  function getActiveFilterCount(): number {
    let count = 0;
    if (filters.searchQuery.trim()) count++;
    if (filters.accountIds.length > 0) count++;
    if (filters.categoryIds.length > 0) count++;
    if (filters.tagIds.length > 0) count++;
    if (filters.transactionType !== 'all') count++;
    if (filters.dateFrom || filters.dateTo) count++;
    if (filters.amountMin > 0 || filters.amountMax < 10000) count++;
    return count;
  }

  function toggleExpanded() {
    isExpanded = !isExpanded;
  }

  function handleAccountChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const accountId = target.value;

    if (target.checked) {
      filters.accountIds = [...filters.accountIds, accountId];
    } else {
      filters.accountIds = filters.accountIds.filter(id => id !== accountId);
    }
    applyFilters();
  }

  function handleCategoryChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const categoryId = target.value;

    if (target.checked) {
      filters.categoryIds = [...filters.categoryIds, categoryId];
    } else {
      filters.categoryIds = filters.categoryIds.filter(id => id !== categoryId);
    }
    applyFilters();
  }

  function handleTagChange(event: CustomEvent) {
    filters.tagIds = event.detail.selectedTagIds;
    applyFilters();
  }

  function clearAllFilters() {
    filters = {
      searchQuery: '',
      accountIds: [],
      categoryIds: [],
      tagIds: [],
      tagMode: 'or',
      transactionType: 'all',
      dateFrom: '',
      dateTo: '',
      amountMin: 0,
      amountMax: 10000
    };
    applyFilters();
  }

  function applyFilters() {
    dispatch('filtersChanged', { filters });
  }

  // Apply filters when any reactive variable changes
  $: if (filters.searchQuery !== undefined) applyFilters();
</script>

<div class="advanced-filters bg-base-100 border border-base-300 rounded-lg">
  <!-- Filter Header -->
  <div class="flex items-center justify-between p-4 border-b border-base-300">
    <div class="flex items-center gap-3">
      <button
        class="btn btn-sm btn-ghost"
        on:click={toggleExpanded}
        title={isExpanded ? 'Collapse filters' : 'Expand filters'}
      >
        <span class="text-lg transition-transform duration-200" class:rotate-180={isExpanded}>
          ▼
        </span>
        Advanced Filters
      </button>

      {#if activeFilterCount > 0}
        <div class="badge badge-primary">{activeFilterCount} active</div>
      {/if}
    </div>

    <div class="flex items-center gap-2">
      {#if activeFilterCount > 0}
        <button class="btn btn-sm btn-ghost" on:click={clearAllFilters}>
          Clear All
        </button>
      {/if}
    </div>
  </div>

  <!-- Active Filter Badges -->
  {#if activeFilterCount > 0 && !isExpanded}
    <div class="p-3 bg-base-50 border-b border-base-300">
      <div class="flex flex-wrap gap-1">
        {#if filters.searchQuery.trim()}
          <span class="badge badge-outline">
            Search: {filters.searchQuery}
          </span>
        {/if}

        {#if filters.accountIds.length > 0}
          <span class="badge badge-outline">
            {filters.accountIds.length} Account{filters.accountIds.length > 1 ? 's' : ''}
          </span>
        {/if}

        {#if filters.categoryIds.length > 0}
          <span class="badge badge-outline">
            {filters.categoryIds.length} Categor{filters.categoryIds.length > 1 ? 'ies' : 'y'}
          </span>
        {/if}

        {#if filters.tagIds.length > 0}
          <span class="badge badge-outline">
            {filters.tagIds.length} Tag{filters.tagIds.length > 1 ? 's' : ''} ({filters.tagMode.toUpperCase()})
          </span>
        {/if}

        {#if filters.transactionType !== 'all'}
          <span class="badge badge-outline">
            Type: {filters.transactionType}
          </span>
        {/if}

        {#if filters.dateFrom || filters.dateTo}
          <span class="badge badge-outline">
            Date Range
          </span>
        {/if}

        {#if filters.amountMin > 0 || filters.amountMax < 10000}
          <span class="badge badge-outline">
            Amount: {filters.amountMin} - {filters.amountMax}
          </span>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Expanded Filter Panel -->
  {#if isExpanded}
    <div class="p-4 space-y-6">
      <!-- Search Query -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-medium">Search Description</span>
        </label>
        <input
          type="text"
          placeholder="Search transaction descriptions..."
          class="input input-bordered"
          bind:value={filters.searchQuery}
        />
      </div>

      <!-- Grid Layout for Filters -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Account Filter -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-medium">Accounts</span>
          </label>
          <div class="space-y-2 max-h-32 overflow-y-auto border border-base-300 rounded-lg p-3">
            {#each accounts as account}
              <label class="cursor-pointer label justify-start gap-3">
                <input
                  type="checkbox"
                  class="checkbox checkbox-sm"
                  value={account.id}
                  checked={filters.accountIds.includes(account.id)}
                  on:change={handleAccountChange}
                />
                <span class="label-text">{account.name}</span>
              </label>
            {/each}
            {#if accounts.length === 0}
              <p class="text-base-content/50 text-sm">No accounts available</p>
            {/if}
          </div>
        </div>

        <!-- Category Filter -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-medium">Categories</span>
          </label>
          <div class="space-y-2 max-h-32 overflow-y-auto border border-base-300 rounded-lg p-3">
            {#if incomeCategories.length > 0}
              <div class="text-xs font-semibold text-success mb-1">Income</div>
              {#each incomeCategories as category}
                <label class="cursor-pointer label justify-start gap-3">
                  <input
                    type="checkbox"
                    class="checkbox checkbox-sm"
                    value={category.id}
                    checked={filters.categoryIds.includes(category.id)}
                    on:change={handleCategoryChange}
                  />
                  <span class="label-text">{category.icon} {category.name}</span>
                </label>
              {/each}
            {/if}

            {#if expenseCategories.length > 0}
              <div class="text-xs font-semibold text-error mb-1 {incomeCategories.length > 0 ? 'mt-3' : ''}">Expense</div>
              {#each expenseCategories as category}
                <label class="cursor-pointer label justify-start gap-3">
                  <input
                    type="checkbox"
                    class="checkbox checkbox-sm"
                    value={category.id}
                    checked={filters.categoryIds.includes(category.id)}
                    on:change={handleCategoryChange}
                  />
                  <span class="label-text">{category.icon} {category.name}</span>
                </label>
              {/each}
            {/if}

            {#if categories.length === 0}
              <p class="text-base-content/50 text-sm">No categories available</p>
            {/if}
          </div>
        </div>

        <!-- Transaction Type Filter -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-medium">Transaction Type</span>
          </label>
          <select class="select select-bordered" bind:value={filters.transactionType}>
            <option value="all">All Types</option>
            <option value="income">Income Only</option>
            <option value="expense">Expense Only</option>
          </select>
        </div>

        <!-- Date Range Filter -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-medium">Date Range</span>
          </label>
          <div class="flex gap-2">
            <input
              type="date"
              class="input input-bordered flex-1"
              bind:value={filters.dateFrom}
              placeholder="From"
            />
            <input
              type="date"
              class="input input-bordered flex-1"
              bind:value={filters.dateTo}
              placeholder="To"
            />
          </div>
        </div>
      </div>

      <!-- Tag Filter with AND/OR toggle -->
      <div class="form-control">
        <div class="flex items-center justify-between mb-2">
          <span class="label-text font-medium">Tags</span>
          <div class="join">
            <button
              class="btn btn-xs join-item"
              class:btn-active={filters.tagMode === 'or'}
              on:click={() => { filters.tagMode = 'or'; applyFilters(); }}
            >
              OR
            </button>
            <button
              class="btn btn-xs join-item"
              class:btn-active={filters.tagMode === 'and'}
              on:click={() => { filters.tagMode = 'and'; applyFilters(); }}
            >
              AND
            </button>
          </div>
        </div>
        <TagPicker
          {tags}
          selectedTagIds={filters.tagIds}
          placeholder="Select tags to filter by..."
          on:change={handleTagChange}
        />
        <label class="label">
          <span class="label-text-alt">
            {filters.tagMode === 'or' ? 'Show transactions with ANY of the selected tags' : 'Show transactions with ALL selected tags'}
          </span>
        </label>
      </div>

      <!-- Amount Range Filter -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-medium">Amount Range</span>
        </label>
        <div class="space-y-3">
          <div class="flex gap-2">
            <div class="flex-1">
              <label class="label-text-alt">Min Amount</label>
              <input
                type="number"
                step="0.01"
                min="0"
                class="input input-bordered w-full"
                bind:value={filters.amountMin}
                placeholder="0.00"
              />
            </div>
            <div class="flex-1">
              <label class="label-text-alt">Max Amount</label>
              <input
                type="number"
                step="0.01"
                min="0"
                class="input input-bordered w-full"
                bind:value={filters.amountMax}
                placeholder="10000.00"
              />
            </div>
          </div>
          <div class="text-center text-sm text-base-content/70">
            Current range: {filters.amountMin.toFixed(2)} - {filters.amountMax.toFixed(2)} RON
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex gap-3">
        <button class="btn btn-ghost flex-1" on:click={clearAllFilters}>
          Clear All Filters
        </button>
        <button class="btn btn-primary flex-1" on:click={toggleExpanded}>
          Apply & Close
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .advanced-filters {
    transition: all 0.2s ease-in-out;
  }

  .rotate-180 {
    transform: rotate(180deg);
  }

  .bg-base-50 {
    background-color: hsl(var(--b1) / 0.5);
  }

  /* Custom scrollbar for filter lists */
  .max-h-32::-webkit-scrollbar {
    width: 4px;
  }

  .max-h-32::-webkit-scrollbar-track {
    background: transparent;
  }

  .max-h-32::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 2px;
  }

  .max-h-32::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
  }
</style>