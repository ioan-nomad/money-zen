<script lang="ts">
  import { onMount } from 'svelte';
  import { transactionStore } from '../stores/transactionStore';
  import { accountStore } from '../stores/accountStore';
  import { categoryStore } from '../stores/categoryStore';
  import { tagStore } from '../stores/tagStore';

  let error = '';

  // Filter state
  let reportType: 'all' | 'monthly' | 'account' | 'category' = 'all';
  let selectedAccountId: string = '';
  let selectedCategoryId: string = '';

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

  async function applyFilters() {
    try {
      switch (reportType) {
        case 'account':
          if (selectedAccountId) {
            await transactionStore.loadByAccount(selectedAccountId);
          } else {
            await transactionStore.load();
          }
          break;
        case 'category':
          if (selectedCategoryId) {
            await transactionStore.loadByCategory(selectedCategoryId);
          } else {
            await transactionStore.load();
          }
          break;
        default:
          await transactionStore.load();
      }
    } catch (err) {
      error = String(err);
    }
  }

  // Reactive subscriptions
  $: transactions = $transactionStore;
  $: accounts = $accountStore;
  $: categories = $categoryStore;
  $: tags = $tagStore;

  // Analytics calculations (reactive)
  $: totalIncome = transactions
    .filter(t => t.transaction_type === 'income')
    .reduce((sum, t) => sum + t.amount, 0);

  $: totalExpense = transactions
    .filter(t => t.transaction_type === 'expense')
    .reduce((sum, t) => sum + t.amount, 0);

  $: netBalance = totalIncome - totalExpense;

  // Top 5 spending categories
  $: topCategories = (() => {
    const categoryTotals = new Map<string, number>();

    transactions
      .filter(t => t.transaction_type === 'expense')
      .forEach(t => {
        const current = categoryTotals.get(t.category_id) || 0;
        categoryTotals.set(t.category_id, current + t.amount);
      });

    return Array.from(categoryTotals.entries())
      .map(([categoryId, total]) => {
        const category = categories.find(c => c.id === categoryId);
        return {
          id: categoryId,
          name: category?.name || 'Unknown',
          icon: category?.icon || '❓',
          color: category?.color || '#gray',
          total
        };
      })
      .sort((a, b) => b.total - a.total)
      .slice(0, 5);
  })();

  $: maxCategoryTotal = topCategories.length > 0 ? topCategories[0].total : 1;
</script>

<div class="max-w-6xl mx-auto p-6 space-y-6">
  <h1 class="text-4xl font-bold">📊 Analytics</h1>

  {#if error}
    <div class="alert alert-error">
      <span>Error: {error}</span>
    </div>
  {/if}

  <!-- Report Filter Controls -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">Report Filters</h2>

      <div class="flex flex-wrap gap-4">
        <!-- Report Type Selector -->
        <div class="form-control">
          <label class="label" for="reportType">
            <span class="label-text">Report Type</span>
          </label>
          <select id="reportType" class="select select-bordered" bind:value={reportType} on:change={applyFilters}>
            <option value="all">All Transactions</option>
            <option value="account">By Account</option>
            <option value="category">By Category</option>
          </select>
        </div>

        <!-- Account Filter -->
        {#if reportType === 'account'}
          <div class="form-control">
            <label class="label" for="selectedAccount">
              <span class="label-text">Account</span>
            </label>
            <select id="selectedAccount" class="select select-bordered" bind:value={selectedAccountId} on:change={applyFilters}>
              <option value="">All Accounts</option>
              {#each accounts as account}
                <option value={account.id}>{account.name}</option>
              {/each}
            </select>
          </div>
        {/if}

        <!-- Category Filter -->
        {#if reportType === 'category'}
          <div class="form-control">
            <label class="label" for="selectedCategory">
              <span class="label-text">Category</span>
            </label>
            <select id="selectedCategory" class="select select-bordered" bind:value={selectedCategoryId} on:change={applyFilters}>
              <option value="">All Categories</option>
              {#each categories as category}
                <option value={category.id}>{category.icon} {category.name}</option>
              {/each}
            </select>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Summary Cards -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    <div class="card bg-success text-success-content">
      <div class="card-body">
        <h2 class="card-title">💰 Total Income</h2>
        <p class="text-3xl font-bold">{totalIncome.toFixed(2)} RON</p>
      </div>
    </div>

    <div class="card bg-error text-error-content">
      <div class="card-body">
        <h2 class="card-title">💸 Total Expenses</h2>
        <p class="text-3xl font-bold">{totalExpense.toFixed(2)} RON</p>
      </div>
    </div>

    <div class="card bg-primary text-primary-content">
      <div class="card-body">
        <h2 class="card-title">💵 Net Balance</h2>
        <p class="text-3xl font-bold {netBalance >= 0 ? '' : 'text-error'}">{netBalance.toFixed(2)} RON</p>
      </div>
    </div>
  </div>

  <!-- Top 5 Spending Categories -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">Top 5 Spending Categories</h2>

      {#if topCategories.length === 0}
        <p class="text-center py-8 text-base-content/50">No expense data available</p>
      {:else}
        <div class="space-y-3">
          {#each topCategories as category}
            <div>
              <div class="flex justify-between mb-1">
                <span class="flex items-center gap-2">
                  <span class="text-xl">{category.icon}</span>
                  <span class="font-medium">{category.name}</span>
                </span>
                <span class="font-bold">{category.total.toFixed(2)} RON</span>
              </div>
              <progress
                class="progress progress-primary w-full"
                value={category.total}
                max={maxCategoryTotal}
              ></progress>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <!-- Transaction Summary -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">Transaction Summary</h2>
      <div class="stats shadow">
        <div class="stat">
          <div class="stat-title">Total Transactions</div>
          <div class="stat-value text-primary">{transactions.length}</div>
        </div>
        <div class="stat">
          <div class="stat-title">Income Transactions</div>
          <div class="stat-value text-success">{transactions.filter(t => t.transaction_type === 'income').length}</div>
        </div>
        <div class="stat">
          <div class="stat-title">Expense Transactions</div>
          <div class="stat-value text-error">{transactions.filter(t => t.transaction_type === 'expense').length}</div>
        </div>
      </div>
    </div>
  </div>
</div>