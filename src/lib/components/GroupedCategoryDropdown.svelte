<script lang="ts">
  import type { Category } from '../database';

  export let categories: Category[] = [];
  export let selectedId: string = '';
  export let onChange: (categoryId: string) => void;

  let isOpen = false;
  let searchQuery = '';

  // Get selected category object
  $: selectedCategory = categories.find(c => c.id === selectedId);

  // Group categories by type
  $: incomeCategories = categories.filter(c => c.category_type === 'income');
  $: expenseCategories = categories.filter(c => c.category_type === 'expense');

  function toggleDropdown() {
    isOpen = !isOpen;
    if (isOpen) {
      searchQuery = ''; // Reset search when opening
    }
  }

  function selectCategory(categoryId: string) {
    onChange(categoryId);
    isOpen = false; // Close dropdown after selection
  }
</script>

<!-- Toggle Button -->
<div class="relative w-full">
  <button
    type="button"
    on:click={toggleDropdown}
    class="btn btn-outline w-full justify-between text-left"
  >
    <span class="flex items-center gap-2">
      {#if selectedCategory}
        <span class="text-xl">{selectedCategory.icon}</span>
        <span>{selectedCategory.name}</span>
      {:else}
        <span class="text-gray-400">Select category...</span>
      {/if}
    </span>
    <span class="text-lg">{isOpen ? '▲' : '▼'}</span>
  </button>

  <!-- Dropdown Panel -->
  {#if isOpen}
    <div class="absolute z-50 w-full mt-2 bg-base-200 rounded-lg shadow-xl max-h-96 overflow-y-auto">

      <!-- Income Group -->
      {#if incomeCategories.length > 0}
        <div class="p-2">
          <div class="text-xs font-bold text-success px-3 py-2">💰 INCOME SOURCES</div>
          {#each incomeCategories as category}
            <button
              type="button"
              on:click={() => selectCategory(category.id)}
              class="w-full text-left px-3 py-2 hover:bg-base-300 rounded flex items-center gap-2"
              class:bg-primary={category.id === selectedId}
              class:text-primary-content={category.id === selectedId}
            >
              <span class="text-xl">{category.icon}</span>
              <span>{category.name}</span>
            </button>
          {/each}
        </div>
      {/if}

      <!-- Separator -->
      {#if incomeCategories.length > 0 && expenseCategories.length > 0}
        <div class="divider my-0"></div>
      {/if}

      <!-- Expense Group -->
      {#if expenseCategories.length > 0}
        <div class="p-2">
          <div class="text-xs font-bold text-error px-3 py-2">💳 EXPENSES</div>
          {#each expenseCategories as category}
            <button
              type="button"
              on:click={() => selectCategory(category.id)}
              class="w-full text-left px-3 py-2 hover:bg-base-300 rounded flex items-center gap-2"
              class:bg-primary={category.id === selectedId}
              class:text-primary-content={category.id === selectedId}
            >
              <span class="text-xl">{category.icon}</span>
              <span>{category.name}</span>
            </button>
          {/each}
        </div>
      {/if}

    </div>
  {/if}
</div>

<style>
  /* Custom styles will go here */
</style>