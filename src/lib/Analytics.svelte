<script lang="ts">
  import { onMount } from 'svelte';
  import { Database, type Transaction, type Category, type Account, type Tag } from './database';
  import jsPDF from 'jspdf';
  import autoTable from 'jspdf-autotable';

  let transactions: Transaction[] = [];
  let filteredTransactions: Transaction[] = [];
  let categories: Category[] = [];
  let accounts: Account[] = [];
  let tags: Tag[] = [];
  let error = '';

  // Report filter state
  let reportType: 'all' | 'monthly' | 'account' | 'category' | 'date_range' = 'all';
  let selectedYear: number = new Date().getFullYear();
  let selectedMonth: number = new Date().getMonth() + 1;
  let selectedAccountId: string = '';
  let selectedCategoryId: string = '';
  let startDate: string = '';
  let endDate: string = '';

  // Analytics data
  let totalIncome = 0;
  let totalExpense = 0;
  let spendingByCategory: { name: string; amount: number; color: string }[] = [];
  let spendingByTag: { name: string; amount: number; color: string; icon: string; count: number }[] = [];
  let topUsedTags: { name: string; icon: string; color: string; count: number; totalAmount: number }[] = [];
  let tagCombinations: { tags: string[]; count: number; totalAmount: number }[] = [];
  let tagTrends: { month: string; tagData: { tagName: string; amount: number }[] }[] = [];
  let transactionCount = 0;

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      transactions = await Database.getTransactions();
      categories = await Database.getCategories();
      accounts = await Database.getAccounts();
      tags = await Database.getTags();
      await applyFilters();
    } catch (err) {
      error = String(err);
    }
  }

  async function applyFilters() {
    try {
      switch (reportType) {
        case 'monthly':
          filteredTransactions = await Database.getTransactionsByMonth(selectedYear, selectedMonth);
          break;
        case 'account':
          if (selectedAccountId) {
            filteredTransactions = await Database.getTransactionsByAccount(selectedAccountId);
          } else {
            filteredTransactions = transactions;
          }
          break;
        case 'category':
          if (selectedCategoryId) {
            filteredTransactions = await Database.getTransactionsByCategory(selectedCategoryId);
          } else {
            filteredTransactions = transactions;
          }
          break;
        case 'date_range':
          if (startDate && endDate) {
            filteredTransactions = await Database.getTransactionsByDateRange(
              new Date(startDate).toISOString(),
              new Date(endDate + 'T23:59:59').toISOString()
            );
          } else {
            filteredTransactions = transactions;
          }
          break;
        default:
          filteredTransactions = transactions;
      }
      await calculateAnalytics();
    } catch (err) {
      error = String(err);
    }
  }

  async function calculateAnalytics() {
    transactionCount = filteredTransactions.length;

    totalIncome = filteredTransactions
      .filter(t => t.transaction_type === 'income')
      .reduce((sum, t) => sum + t.amount, 0);

    totalExpense = filteredTransactions
      .filter(t => t.transaction_type === 'expense')
      .reduce((sum, t) => sum + t.amount, 0);

    // Spending by category
    const categoryMap = new Map<string, number>();
    filteredTransactions
      .filter(t => t.transaction_type === 'expense')
      .forEach(t => {
        const current = categoryMap.get(t.category_id) || 0;
        categoryMap.set(t.category_id, current + t.amount);
      });

    spendingByCategory = Array.from(categoryMap.entries())
      .map(([catId, amount]) => {
        const cat = categories.find(c => c.id === catId);
        return {
          name: cat ? `${cat.icon} ${cat.name}` : 'Unknown',
          amount,
          color: cat?.color || '#666'
        };
      })
      .sort((a, b) => b.amount - a.amount)
      .slice(0, 5); // Top 5

    // Calculate tag analytics
    await calculateTagAnalytics();
  }

  async function calculateTagAnalytics() {
    try {
      // Build a map of transaction tags for all filtered transactions
      const transactionTagsMap = new Map<string, Tag[]>();
      for (const transaction of filteredTransactions) {
        const transactionTags = await Database.getTransactionTags(transaction.id);
        transactionTagsMap.set(transaction.id, transactionTags);
      }

      // Calculate spending by tag
      const tagAmountMap = new Map<string, { amount: number; count: number }>();
      filteredTransactions
        .filter(t => t.transaction_type === 'expense')
        .forEach(t => {
          const transactionTags = transactionTagsMap.get(t.id) || [];
          transactionTags.forEach(tag => {
            const current = tagAmountMap.get(tag.id) || { amount: 0, count: 0 };
            tagAmountMap.set(tag.id, {
              amount: current.amount + t.amount,
              count: current.count + 1
            });
          });
        });

      spendingByTag = Array.from(tagAmountMap.entries())
        .map(([tagId, data]) => {
          const tag = tags.find(t => t.id === tagId);
          return {
            name: tag ? tag.name : 'Unknown',
            amount: data.amount,
            color: tag?.color || '#666',
            icon: tag?.icon || '🏷️',
            count: data.count
          };
        })
        .sort((a, b) => b.amount - a.amount)
        .slice(0, 5);

      // Calculate top used tags (by frequency)
      const tagUsageMap = new Map<string, { count: number; totalAmount: number }>();
      filteredTransactions.forEach(t => {
        const transactionTags = transactionTagsMap.get(t.id) || [];
        transactionTags.forEach(tag => {
          const current = tagUsageMap.get(tag.id) || { count: 0, totalAmount: 0 };
          tagUsageMap.set(tag.id, {
            count: current.count + 1,
            totalAmount: current.totalAmount + (t.transaction_type === 'expense' ? t.amount : 0)
          });
        });
      });

      topUsedTags = Array.from(tagUsageMap.entries())
        .map(([tagId, data]) => {
          const tag = tags.find(t => t.id === tagId);
          return {
            name: tag ? tag.name : 'Unknown',
            icon: tag?.icon || '🏷️',
            color: tag?.color || '#666',
            count: data.count,
            totalAmount: data.totalAmount
          };
        })
        .sort((a, b) => b.count - a.count)
        .slice(0, 5);

      // Calculate tag combinations
      const combinationMap = new Map<string, { count: number; totalAmount: number }>();
      filteredTransactions.forEach(t => {
        const transactionTags = transactionTagsMap.get(t.id) || [];
        if (transactionTags.length >= 2) {
          // Sort tags by name for consistent combination keys
          const sortedTagNames = transactionTags
            .map(tag => tag.name)
            .sort();

          // Create combinations of 2 tags
          for (let i = 0; i < sortedTagNames.length; i++) {
            for (let j = i + 1; j < sortedTagNames.length; j++) {
              const combo = `${sortedTagNames[i]} + ${sortedTagNames[j]}`;
              const current = combinationMap.get(combo) || { count: 0, totalAmount: 0 };
              combinationMap.set(combo, {
                count: current.count + 1,
                totalAmount: current.totalAmount + (t.transaction_type === 'expense' ? t.amount : 0)
              });
            }
          }
        }
      });

      tagCombinations = Array.from(combinationMap.entries())
        .map(([combo, data]) => ({
          tags: combo.split(' + '),
          count: data.count,
          totalAmount: data.totalAmount
        }))
        .sort((a, b) => b.count - a.count)
        .slice(0, 5);

      // Calculate tag trends over time (last 6 months)
      const now = new Date();
      const monthlyTagData = new Map<string, Map<string, number>>();

      for (let i = 5; i >= 0; i--) {
        const date = new Date(now.getFullYear(), now.getMonth() - i, 1);
        const monthKey = `${date.getFullYear()}-${(date.getMonth() + 1).toString().padStart(2, '0')}`;
        monthlyTagData.set(monthKey, new Map());
      }

      filteredTransactions
        .filter(t => t.transaction_type === 'expense')
        .forEach(t => {
          const transactionDate = new Date(t.date);
          const monthKey = `${transactionDate.getFullYear()}-${(transactionDate.getMonth() + 1).toString().padStart(2, '0')}`;

          if (monthlyTagData.has(monthKey)) {
            const transactionTags = transactionTagsMap.get(t.id) || [];
            const monthData = monthlyTagData.get(monthKey)!;

            transactionTags.forEach(tag => {
              const current = monthData.get(tag.name) || 0;
              monthData.set(tag.name, current + t.amount);
            });
          }
        });

      tagTrends = Array.from(monthlyTagData.entries())
        .map(([month, tagData]) => ({
          month,
          tagData: Array.from(tagData.entries())
            .map(([tagName, amount]) => ({ tagName, amount }))
            .sort((a, b) => b.amount - a.amount)
            .slice(0, 3) // Top 3 tags per month
        }))
        .filter(trend => trend.tagData.length > 0);

    } catch (err) {
      console.error('Error calculating tag analytics:', err);
    }
  }

  function stripEmoji(text: string): string {
    return text.replace(/[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]/gu, '').trim();
  }

  // Remove Romanian diacritics for PDF compatibility
  function removeDiacritics(str: string): string {
    return str
      .replace(/ă/g, 'a')
      .replace(/â/g, 'a')
      .replace(/î/g, 'i')
      .replace(/ș/g, 's')
      .replace(/ț/g, 't')
      .replace(/Ă/g, 'A')
      .replace(/Â/g, 'A')
      .replace(/Î/g, 'I')
      .replace(/Ș/g, 'S')
      .replace(/Ț/g, 'T');
  }

  function formatAmount(amount: number, type?: string): string {
    const absAmount = Math.abs(amount);
    return `${absAmount.toFixed(2)} RON`;
  }

  function getReportTitle(): string {
    switch (reportType) {
      case 'monthly':
        return `Monthly Report - ${selectedYear}/${selectedMonth.toString().padStart(2, '0')}`;
      case 'account':
        const account = accounts.find(a => a.id === selectedAccountId);
        return `Account Report - ${account?.name || 'All Accounts'}`;
      case 'category':
        const category = categories.find(c => c.id === selectedCategoryId);
        const categoryName = category ? stripEmoji(category.name) : 'All Categories';
        return `Category Report - ${categoryName}`;
      case 'date_range':
        return `Date Range Report - ${startDate} to ${endDate}`;
      default:
        return 'Financial Report - All Transactions';
    }
  }

  function exportPDF() {
    const doc = new jsPDF();

    doc.setFontSize(20);
    doc.text('MoneyZen', 14, 22);

    doc.setFontSize(16);
    doc.text(getReportTitle(), 14, 32);

    doc.setFontSize(12);
    doc.text(`Generated: ${new Date().toLocaleDateString()}`, 14, 42);
    doc.text(`Transactions: ${transactionCount}`, 14, 48);

    // Summary
    doc.setFontSize(14);
    doc.text('Summary', 14, 60);
    autoTable(doc, {
      startY: 65,
      head: [['Type', 'Amount']],
      body: [
        ['Total Income', formatAmount(totalIncome)],
        ['Total Expense', formatAmount(totalExpense)],
        ['Net Balance', formatAmount(totalIncome - totalExpense)]
      ],
      styles: { fontSize: 10, cellPadding: 3 },
      headStyles: { fillColor: [59, 130, 246], textColor: [255, 255, 255] }
    });

    // Top spending categories
    if (spendingByCategory.length > 0) {
      doc.text('Top Spending Categories', 14, doc.lastAutoTable.finalY + 15);
      autoTable(doc, {
        startY: doc.lastAutoTable.finalY + 20,
        head: [['Category', 'Amount']],
        body: spendingByCategory.map(c => [
          stripEmoji(removeDiacritics(c.name)),
          formatAmount(c.amount)
        ]),
        styles: { fontSize: 9, cellPadding: 2 },
        headStyles: { fillColor: [239, 68, 68], textColor: [255, 255, 255] }
      });
    }

    // Top spending tags
    if (spendingByTag.length > 0) {
      doc.text('Top Spending Tags', 14, doc.lastAutoTable.finalY + 15);
      autoTable(doc, {
        startY: doc.lastAutoTable.finalY + 20,
        head: [['Tag', 'Amount', 'Uses']],
        body: spendingByTag.map(t => [
          stripEmoji(removeDiacritics(t.name)),
          formatAmount(t.amount),
          `${t.count} times`
        ]),
        styles: { fontSize: 9, cellPadding: 2 },
        headStyles: { fillColor: [34, 197, 94], textColor: [255, 255, 255] }
      });
    }

    // Tag combinations
    if (tagCombinations.length > 0) {
      doc.text('Popular Tag Combinations', 14, doc.lastAutoTable.finalY + 15);
      autoTable(doc, {
        startY: doc.lastAutoTable.finalY + 20,
        head: [['Tag Combination', 'Frequency', 'Total Amount']],
        body: tagCombinations.slice(0, 3).map(c => [
          removeDiacritics(c.tags.join(' + ')),
          `${c.count} times`,
          formatAmount(c.totalAmount)
        ]),
        styles: { fontSize: 9, cellPadding: 2 },
        headStyles: { fillColor: [168, 85, 247], textColor: [255, 255, 255] }
      });
    }

    // Detailed transactions (last 10)
    if (filteredTransactions.length > 0) {
      doc.text('Recent Transactions', 14, doc.lastAutoTable.finalY + 15);
      const recentTransactions = filteredTransactions.slice(0, 10);
      autoTable(doc, {
        startY: doc.lastAutoTable.finalY + 20,
        head: [['Date', 'Description', 'Type', 'Amount']],
        body: recentTransactions.map(t => [
          new Date(t.date).toLocaleDateString(),
          removeDiacritics(t.description.length > 30 ? t.description.substring(0, 30) + '...' : t.description),
          t.transaction_type === 'income' ? 'Income' : 'Expense',
          formatAmount(t.amount)
        ]),
        styles: { fontSize: 8, cellPadding: 2 },
        headStyles: { fillColor: [34, 197, 94], textColor: [255, 255, 255] },
        columnStyles: {
          0: { cellWidth: 25 },
          1: { cellWidth: 60 },
          2: { cellWidth: 20 },
          3: { cellWidth: 25, halign: 'right' }
        }
      });
    }

    const filename = `moneyzen-${reportType}-${new Date().toISOString().split('T')[0]}.pdf`;
    doc.save(filename);
  }
</script>

<div class="max-w-6xl mx-auto p-6 space-y-6">
  <div class="flex justify-between items-center">
    <h1 class="text-4xl font-bold">Analytics</h1>
    <button class="btn btn-primary" on:click={exportPDF}>
      📄 Export PDF Report
    </button>
  </div>

  {#if error}
    <div class="alert alert-error"><span>{error}</span></div>
  {/if}

  <!-- Report Filter Controls -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">Report Filters</h2>

      <!-- Report Type Selector -->
      <div class="form-control w-full max-w-xs">
        <label class="label" for="reportType">
          <span class="label-text">Report Type</span>
        </label>
        <select id="reportType" class="select select-bordered" bind:value={reportType} on:change={applyFilters}>
          <option value="all">All Transactions</option>
          <option value="monthly">Monthly</option>
          <option value="account">By Account</option>
          <option value="category">By Category</option>
          <option value="date_range">Date Range</option>
        </select>
      </div>

      <!-- Conditional Filter Controls -->
      <div class="flex flex-wrap gap-4 mt-4">
        {#if reportType === 'monthly'}
          <div class="form-control">
            <label class="label" for="selectedYear">
              <span class="label-text">Year</span>
            </label>
            <input
              id="selectedYear"
              type="number"
              class="input input-bordered w-24"
              bind:value={selectedYear}
              on:change={applyFilters}
              min="2020"
              max="2030"
            />
          </div>
          <div class="form-control">
            <label class="label" for="selectedMonth">
              <span class="label-text">Month</span>
            </label>
            <select id="selectedMonth" class="select select-bordered" bind:value={selectedMonth} on:change={applyFilters}>
              <option value={1}>January</option>
              <option value={2}>February</option>
              <option value={3}>March</option>
              <option value={4}>April</option>
              <option value={5}>May</option>
              <option value={6}>June</option>
              <option value={7}>July</option>
              <option value={8}>August</option>
              <option value={9}>September</option>
              <option value={10}>October</option>
              <option value={11}>November</option>
              <option value={12}>December</option>
            </select>
          </div>
        {:else if reportType === 'account'}
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
        {:else if reportType === 'category'}
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
        {:else if reportType === 'date_range'}
          <div class="form-control">
            <label class="label" for="startDate">
              <span class="label-text">Start Date</span>
            </label>
            <input
              id="startDate"
              type="date"
              class="input input-bordered"
              bind:value={startDate}
              on:change={applyFilters}
            />
          </div>
          <div class="form-control">
            <label class="label" for="endDate">
              <span class="label-text">End Date</span>
            </label>
            <input
              id="endDate"
              type="date"
              class="input input-bordered"
              bind:value={endDate}
              on:change={applyFilters}
            />
          </div>
        {/if}
      </div>

      <!-- Preview Section -->
      <div class="mt-4 p-4 bg-base-200 rounded-lg">
        <h3 class="font-semibold mb-2">Report Preview</h3>
        <div class="stats shadow">
          <div class="stat">
            <div class="stat-title">Transactions</div>
            <div class="stat-value text-primary">{transactionCount}</div>
          </div>
          <div class="stat">
            <div class="stat-title">Total Income</div>
            <div class="stat-value text-success">{totalIncome.toFixed(2)} RON</div>
          </div>
          <div class="stat">
            <div class="stat-title">Total Expense</div>
            <div class="stat-value text-error">{totalExpense.toFixed(2)} RON</div>
          </div>
          <div class="stat">
            <div class="stat-title">Net Balance</div>
            <div class="stat-value {(totalIncome - totalExpense) >= 0 ? 'text-success' : 'text-error'}">
              {(totalIncome - totalExpense).toFixed(2)} RON
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Summary Cards -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    <div class="card bg-success text-success-content">
      <div class="card-body">
        <h2 class="card-title">Total Income</h2>
        <p class="text-3xl font-bold">{totalIncome.toFixed(2)} RON</p>
      </div>
    </div>
    <div class="card bg-error text-error-content">
      <div class="card-body">
        <h2 class="card-title">Total Expense</h2>
        <p class="text-3xl font-bold">{totalExpense.toFixed(2)} RON</p>
      </div>
    </div>
    <div class="card bg-info text-info-content">
      <div class="card-body">
        <h2 class="card-title">Net Balance</h2>
        <p class="text-3xl font-bold">{(totalIncome - totalExpense).toFixed(2)} RON</p>
      </div>
    </div>
  </div>

  <!-- Top Categories -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">Top 5 Spending Categories</h2>
      <div class="space-y-3">
        {#each spendingByCategory as category}
          <div>
            <div class="flex justify-between mb-1">
              <span>{category.name}</span>
              <span class="font-bold">{category.amount.toFixed(2)} RON</span>
            </div>
            <progress
              class="progress progress-primary w-full"
              value={category.amount}
              max={spendingByCategory[0]?.amount || 1}
            ></progress>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Tag Analytics Section -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- Spending by Tag -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">💸 Spending by Tag</h2>
        <div class="space-y-3">
          {#each spendingByTag as tag}
            <div>
              <div class="flex justify-between items-center mb-1">
                <span class="flex items-center gap-2">
                  <span style="color: {tag.color}">{tag.icon}</span>
                  <span>{tag.name}</span>
                  <span class="badge badge-sm">{tag.count} times</span>
                </span>
                <span class="font-bold">{tag.amount.toFixed(2)} RON</span>
              </div>
              <progress
                class="progress w-full"
                style="--progress-color: {tag.color}"
                value={tag.amount}
                max={spendingByTag[0]?.amount || 1}
              ></progress>
            </div>
          {:else}
            <p class="text-gray-500 italic">No tag data available for the selected period.</p>
          {/each}
        </div>
      </div>
    </div>

    <!-- Top Used Tags -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">🏆 Most Used Tags</h2>
        <div class="space-y-3">
          {#each topUsedTags as tag}
            <div class="flex justify-between items-center p-3 bg-base-200 rounded-lg">
              <div class="flex items-center gap-3">
                <span style="color: {tag.color}" class="text-2xl">{tag.icon}</span>
                <div>
                  <div class="font-semibold">{tag.name}</div>
                  <div class="text-sm text-gray-500">
                    {tag.totalAmount.toFixed(2)} RON total
                  </div>
                </div>
              </div>
              <div class="text-right">
                <div class="text-lg font-bold text-primary">{tag.count}</div>
                <div class="text-xs text-gray-500">uses</div>
              </div>
            </div>
          {:else}
            <p class="text-gray-500 italic">No tag usage data available.</p>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <!-- Tag Combinations -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">🔗 Tag Combination Insights</h2>
      <div class="overflow-x-auto">
        <table class="table table-zebra">
          <thead>
            <tr>
              <th>Tag Combination</th>
              <th>Frequency</th>
              <th>Total Spent</th>
              <th>Avg per Transaction</th>
            </tr>
          </thead>
          <tbody>
            {#each tagCombinations as combo}
              <tr>
                <td>
                  <div class="flex items-center gap-2">
                    {#each combo.tags as tag, index}
                      <span class="badge badge-outline">{tag}</span>
                      {#if index < combo.tags.length - 1}
                        <span class="text-gray-400">+</span>
                      {/if}
                    {/each}
                  </div>
                </td>
                <td>
                  <span class="badge badge-primary">{combo.count} times</span>
                </td>
                <td class="font-mono">{combo.totalAmount.toFixed(2)} RON</td>
                <td class="font-mono">{(combo.totalAmount / combo.count).toFixed(2)} RON</td>
              </tr>
            {:else}
              <tr>
                <td colspan="4" class="text-center text-gray-500 italic">
                  No tag combinations found. Try adding multiple tags to transactions!
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>

  <!-- Tag Trends Over Time -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">📈 Tag Trends Over Time (Last 6 Months)</h2>
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>Month</th>
              <th>Top Tags</th>
              <th>Total</th>
            </tr>
          </thead>
          <tbody>
            {#each tagTrends as trend}
              <tr>
                <td class="font-semibold">{trend.month}</td>
                <td>
                  <div class="flex flex-wrap gap-2">
                    {#each trend.tagData as tagData}
                      <div class="badge badge-outline">
                        {tagData.tagName}: {tagData.amount.toFixed(0)} RON
                      </div>
                    {/each}
                  </div>
                </td>
                <td class="font-mono">
                  {trend.tagData.reduce((sum, tag) => sum + tag.amount, 0).toFixed(2)} RON
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="3" class="text-center text-gray-500 italic">
                  No tag trend data available for the selected period.
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <!-- Simple visual trend indicator -->
      {#if tagTrends.length > 0}
        <div class="mt-4">
          <h3 class="text-lg font-semibold mb-2">Monthly Trend</h3>
          <div class="flex items-end gap-2 h-20">
            {#each tagTrends as trend}
              {@const total = trend.tagData.reduce((sum, tag) => sum + tag.amount, 0)}
              {@const maxTotal = Math.max(...tagTrends.map(t => t.tagData.reduce((sum, tag) => sum + tag.amount, 0)))}
              {@const height = maxTotal > 0 ? (total / maxTotal) * 100 : 0}
              <div class="flex flex-col items-center gap-1 flex-1">
                <div
                  class="w-full bg-primary rounded-t"
                  style="height: {height}%"
                  title="{trend.month}: {total.toFixed(2)} RON"
                ></div>
                <span class="text-xs text-gray-500 transform rotate-45 origin-left">
                  {trend.month}
                </span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
