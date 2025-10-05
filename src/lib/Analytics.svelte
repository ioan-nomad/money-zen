<script lang="ts">
  import { onMount } from 'svelte';
  import { Database, type Transaction, type Category } from './database';
  import jsPDF from 'jspdf';
  import autoTable from 'jspdf-autotable';

  let transactions: Transaction[] = [];
  let categories: Category[] = [];
  let error = '';

  // Analytics data
  let totalIncome = 0;
  let totalExpense = 0;
  let spendingByCategory: { name: string; amount: number; color: string }[] = [];

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      transactions = await Database.getTransactions();
      categories = await Database.getCategories();
      calculateAnalytics();
    } catch (err) {
      error = String(err);
    }
  }

  function calculateAnalytics() {
    totalIncome = transactions
      .filter(t => t.transaction_type === 'income')
      .reduce((sum, t) => sum + t.amount, 0);
    
    totalExpense = transactions
      .filter(t => t.transaction_type === 'expense')
      .reduce((sum, t) => sum + t.amount, 0);

    // Spending by category
    const categoryMap = new Map<string, number>();
    transactions
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
  }

  function exportPDF() {
    const doc = new jsPDF();
    
    doc.setFontSize(20);
    doc.text('MoneyZen - Financial Report', 14, 22);
    
    doc.setFontSize(12);
    doc.text(`Generated: ${new Date().toLocaleDateString()}`, 14, 30);
    
    // Summary
    doc.setFontSize(14);
    doc.text('Summary', 14, 45);
    autoTable(doc, {
      startY: 50,
      head: [['Type', 'Amount']],
      body: [
        ['Total Income', `${totalIncome.toFixed(2)} RON`],
        ['Total Expense', `${totalExpense.toFixed(2)} RON`],
        ['Net Balance', `${(totalIncome - totalExpense).toFixed(2)} RON`]
      ]
    });

    // Top spending categories
    doc.text('Top 5 Spending Categories', 14, doc.lastAutoTable.finalY + 15);
    autoTable(doc, {
      startY: doc.lastAutoTable.finalY + 20,
      head: [['Category', 'Amount']],
      body: spendingByCategory.map(c => [c.name, `${c.amount.toFixed(2)} RON`])
    });

    doc.save('moneyzen-report.pdf');
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
</div>
