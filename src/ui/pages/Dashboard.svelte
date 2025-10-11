<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Chart, registerables } from 'chart.js';
  import { transactionStore } from '../ui/stores/transactionStore';
  import { accountStore } from '../ui/stores/accountStore';
  import { categoryStore } from '../ui/stores/categoryStore';
  import jsPDF from 'jspdf';
  import autoTable from 'jspdf-autotable';
  import * as XLSX from 'xlsx';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeFile } from '@tauri-apps/plugin-fs';
  import AddTransactionForm from './components/AddTransactionForm.svelte';

  Chart.register(...registerables);

  // Reactive store subscriptions
  $: accounts = $accountStore;
  $: transactions = $transactionStore;
  $: categories = $categoryStore;

  // Statistics
  let totalIncome = 0;
  let totalExpense = 0;
  let totalSavings = 0;
  let transactionCount = 0;

  // Charts
  let categoryChart = null;
  let trendChart = null;
  let topCategoriesChart = null;
  let accountsChart = null;

  // Filters
  let selectedMonth = new Date().getMonth();
  let selectedYear = new Date().getFullYear();

  onMount(async () => {
    await loadData();
    createCharts();
  });

  onDestroy(() => {
    // Cleanup charts
    if (categoryChart) categoryChart.destroy();
    if (trendChart) trendChart.destroy();
    if (topCategoriesChart) topCategoriesChart.destroy();
    if (accountsChart) accountsChart.destroy();
  });

  async function loadData() {
    try {
      await Promise.all([
        accountStore.load(),
        transactionStore.load(),
        categoryStore.load()
      ]);
      calculateStatistics();
    } catch (err) {
      console.error('Error loading data:', err);
    }
  }

  function calculateStatistics() {
    // Filter transactions for current month
    const currentMonthTransactions = transactions.filter(t => {
      const date = new Date(t.date);
      return date.getMonth() === selectedMonth && date.getFullYear() === selectedYear;
    });

    totalIncome = currentMonthTransactions
      .filter(t => t.type === 'income')
      .reduce((sum, t) => sum + t.amount, 0);

    totalExpense = currentMonthTransactions
      .filter(t => t.type === 'expense')
      .reduce((sum, t) => sum + t.amount, 0);

    totalSavings = totalIncome - totalExpense;
    transactionCount = currentMonthTransactions.length;
  }

  function createCharts() {
    createCategoryDonutChart();
    createTrendLineChart();
    createTopCategoriesChart();
    createAccountsChart();
  }

  function createCategoryDonutChart() {
    const ctx = document.getElementById('categoryChart') as HTMLCanvasElement;
    if (!ctx) return;

    // Group expenses by category
    const categoryData = {};
    transactions
      .filter(t => t.type === 'expense')
      .filter(t => {
        const date = new Date(t.date);
        return date.getMonth() === selectedMonth && date.getFullYear() === selectedYear;
      })
      .forEach(t => {
        const cat = categories.find(c => c.id === t.category_id);
        if (cat) {
          categoryData[cat.name] = (categoryData[cat.name] || 0) + t.amount;
        }
      });

    categoryChart = new Chart(ctx, {
      type: 'doughnut',
      data: {
        labels: Object.keys(categoryData),
        datasets: [{
          data: Object.values(categoryData),
          backgroundColor: [
            '#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6',
            '#ec4899', '#14b8a6', '#f97316', '#06b6d4', '#84cc16'
          ],
          borderWidth: 2,
          borderColor: '#1f2937'
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: 'right',
            labels: {
              color: '#e5e7eb',
              padding: 15,
              font: { size: 12 }
            }
          }
        }
      }
    });
  }

  function createTrendLineChart() {
    const ctx = document.getElementById('trendChart') as HTMLCanvasElement;
    if (!ctx) return;

    // Last 6 months data
    const monthsData = [];
    for (let i = 5; i >= 0; i--) {
      const date = new Date();
      date.setMonth(date.getMonth() - i);

      const monthTransactions = transactions.filter(t => {
        const tDate = new Date(t.date);
        return tDate.getMonth() === date.getMonth() &&
               tDate.getFullYear() === date.getFullYear();
      });

      const income = monthTransactions
        .filter(t => t.type === 'income')
        .reduce((sum, t) => sum + t.amount, 0);

      const expense = monthTransactions
        .filter(t => t.type === 'expense')
        .reduce((sum, t) => sum + t.amount, 0);

      monthsData.push({
        month: date.toLocaleDateString('ro-RO', { month: 'short' }),
        income,
        expense,
        savings: income - expense
      });
    }

    trendChart = new Chart(ctx, {
      type: 'line',
      data: {
        labels: monthsData.map(d => d.month),
        datasets: [
          {
            label: 'Venituri',
            data: monthsData.map(d => d.income),
            borderColor: '#10b981',
            backgroundColor: 'rgba(16, 185, 129, 0.1)',
            tension: 0.4
          },
          {
            label: 'Cheltuieli',
            data: monthsData.map(d => d.expense),
            borderColor: '#ef4444',
            backgroundColor: 'rgba(239, 68, 68, 0.1)',
            tension: 0.4
          },
          {
            label: 'Economii',
            data: monthsData.map(d => d.savings),
            borderColor: '#3b82f6',
            backgroundColor: 'rgba(59, 130, 246, 0.1)',
            tension: 0.4
          }
        ]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            ticks: { color: '#9ca3af' },
            grid: { color: 'rgba(156, 163, 175, 0.1)' }
          },
          x: {
            ticks: { color: '#9ca3af' },
            grid: { display: false }
          }
        },
        plugins: {
          legend: {
            labels: { color: '#e5e7eb' }
          }
        }
      }
    });
  }

  function createTopCategoriesChart() {
    const ctx = document.getElementById('topCategoriesChart') as HTMLCanvasElement;
    if (!ctx) return;

    // Get top 5 expense categories
    const categoryTotals = {};
    transactions
      .filter(t => t.type === 'expense')
      .forEach(t => {
        const cat = categories.find(c => c.id === t.category_id);
        if (cat) {
          categoryTotals[cat.name] = (categoryTotals[cat.name] || 0) + t.amount;
        }
      });

    const sortedCategories = Object.entries(categoryTotals)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 5);

    topCategoriesChart = new Chart(ctx, {
      type: 'bar',
      data: {
        labels: sortedCategories.map(([name]) => name),
        datasets: [{
          label: 'Cheltuieli',
          data: sortedCategories.map(([_, value]) => value),
          backgroundColor: '#8b5cf6'
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            ticks: { color: '#9ca3af' },
            grid: { color: 'rgba(156, 163, 175, 0.1)' }
          },
          x: {
            ticks: { color: '#9ca3af' },
            grid: { display: false }
          }
        },
        plugins: {
          legend: { display: false }
        }
      }
    });
  }

  function createAccountsChart() {
    const ctx = document.getElementById('accountsChart') as HTMLCanvasElement;
    if (!ctx) return;

    const accountBalances = accounts.map(acc => {
      const accountTransactions = transactions.filter(t => t.account_id === acc.id);
      const income = accountTransactions
        .filter(t => t.type === 'income')
        .reduce((sum, t) => sum + t.amount, 0);
      const expense = accountTransactions
        .filter(t => t.type === 'expense')
        .reduce((sum, t) => sum + t.amount, 0);
      return {
        name: acc.name,
        balance: acc.balance + income - expense
      };
    });

    accountsChart = new Chart(ctx, {
      type: 'polarArea',
      data: {
        labels: accountBalances.map(a => a.name),
        datasets: [{
          data: accountBalances.map(a => Math.abs(a.balance)),
          backgroundColor: [
            'rgba(59, 130, 246, 0.5)',
            'rgba(16, 185, 129, 0.5)',
            'rgba(245, 158, 11, 0.5)',
            'rgba(239, 68, 68, 0.5)'
          ]
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: 'bottom',
            labels: { color: '#e5e7eb' }
          }
        },
        scales: {
          r: {
            ticks: { display: false },
            grid: { color: 'rgba(156, 163, 175, 0.1)' }
          }
        }
      }
    });
  }

  // Helper functions for export
  function stripEmoji(text: string): string {
    return text.replace(/[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]/gu, '');
  }

  function removeDiacritics(str: string): string {
    return str.normalize('NFD').replace(/[\u0300-\u036f]/g, '');
  }

  function formatAmount(amount: number): string {
    return `${amount.toFixed(2)} RON`;
  }

  // Export to PDF
  async function exportPDF() {
    const doc = new jsPDF();

    // Header
    doc.setFontSize(20);
    doc.text('MoneyZen Dashboard', 14, 22);

    doc.setFontSize(16);
    const monthNames = ['Ianuarie', 'Februarie', 'Martie', 'Aprilie', 'Mai', 'Iunie',
                       'Iulie', 'August', 'Septembrie', 'Octombrie', 'Noiembrie', 'Decembrie'];
    doc.text(`Raport Financiar - ${monthNames[selectedMonth]} ${selectedYear}`, 14, 32);

    doc.setFontSize(12);
    doc.text(`Generat: ${new Date().toLocaleDateString('ro-RO')}`, 14, 42);
    doc.text(`Tranzacții: ${transactionCount}`, 14, 48);

    // Statistics Summary
    doc.setFontSize(14);
    doc.text('Sumar Financiar', 14, 60);
    autoTable(doc, {
      startY: 65,
      head: [['Tip', 'Sumă']],
      body: [
        ['Venituri Totale', formatAmount(totalIncome)],
        ['Cheltuieli Totale', formatAmount(totalExpense)],
        ['Economii', formatAmount(totalSavings)],
        ['Rata Economisire', `${((totalSavings / totalIncome) * 100 || 0).toFixed(1)}%`]
      ],
      styles: { fontSize: 10, cellPadding: 3 },
      headStyles: { fillColor: [59, 130, 246], textColor: [255, 255, 255] }
    });

    // Category breakdown
    const categoryData = {};
    transactions
      .filter(t => t.type === 'expense')
      .filter(t => {
        const date = new Date(t.date);
        return date.getMonth() === selectedMonth && date.getFullYear() === selectedYear;
      })
      .forEach(t => {
        const cat = categories.find(c => c.id === t.category_id);
        if (cat) {
          categoryData[cat.name] = (categoryData[cat.name] || 0) + t.amount;
        }
      });

    if (Object.keys(categoryData).length > 0) {
      doc.text('Cheltuieli pe Categorii', 14, doc.lastAutoTable.finalY + 15);
      autoTable(doc, {
        startY: doc.lastAutoTable.finalY + 20,
        head: [['Categorie', 'Sumă', 'Procentaj']],
        body: Object.entries(categoryData)
          .sort((a, b) => b[1] - a[1])
          .map(([name, amount]) => [
            stripEmoji(removeDiacritics(name)),
            formatAmount(amount),
            `${((amount / totalExpense) * 100).toFixed(1)}%`
          ]),
        styles: { fontSize: 9, cellPadding: 2 },
        headStyles: { fillColor: [239, 68, 68], textColor: [255, 255, 255] }
      });
    }

    // Account balances
    if (accounts.length > 0) {
      doc.text('Solduri Conturi', 14, doc.lastAutoTable.finalY + 15);
      autoTable(doc, {
        startY: doc.lastAutoTable.finalY + 20,
        head: [['Cont', 'Sold Curent']],
        body: accounts.map(acc => {
          const accountTransactions = transactions.filter(t => t.account_id === acc.id);
          const income = accountTransactions
            .filter(t => t.type === 'income')
            .reduce((sum, t) => sum + t.amount, 0);
          const expense = accountTransactions
            .filter(t => t.type === 'expense')
            .reduce((sum, t) => sum + t.amount, 0);
          const balance = acc.balance + income - expense;
          return [stripEmoji(removeDiacritics(acc.name)), formatAmount(balance)];
        }),
        styles: { fontSize: 9, cellPadding: 2 },
        headStyles: { fillColor: [16, 185, 129], textColor: [255, 255, 255] }
      });
    }

    // Save PDF
    try {
      const fileName = `MoneyZen_Dashboard_${monthNames[selectedMonth]}_${selectedYear}.pdf`;
      const filePath = await save({
        defaultPath: fileName,
        filters: [{ name: 'PDF Files', extensions: ['pdf'] }]
      });

      if (filePath) {
        const pdfBytes = doc.output('arraybuffer');
        await writeFile(filePath, new Uint8Array(pdfBytes));
        alert('PDF export completed successfully!');
      }
    } catch (error) {
      console.error('Error exporting PDF:', error);
      alert('Error exporting PDF: ' + error);
    }
  }

  // Export to Excel
  async function exportExcel() {
    const wb = XLSX.utils.book_new();

    // Statistics sheet
    const statsData = [
      ['Sumar Financiar', '', ''],
      ['Tip', 'Sumă', 'Detalii'],
      ['Venituri Totale', totalIncome, formatAmount(totalIncome)],
      ['Cheltuieli Totale', totalExpense, formatAmount(totalExpense)],
      ['Economii', totalSavings, formatAmount(totalSavings)],
      ['Rata Economisire', ((totalSavings / totalIncome) * 100 || 0).toFixed(1), '%'],
      ['Tranzacții', transactionCount, 'bucăți'],
      ['', '', ''],
      ['Generat', new Date().toLocaleDateString('ro-RO'), '']
    ];

    const statsWs = XLSX.utils.aoa_to_sheet(statsData);
    XLSX.utils.book_append_sheet(wb, statsWs, 'Statistici');

    // Category breakdown sheet
    const categoryData = {};
    transactions
      .filter(t => t.type === 'expense')
      .filter(t => {
        const date = new Date(t.date);
        return date.getMonth() === selectedMonth && date.getFullYear() === selectedYear;
      })
      .forEach(t => {
        const cat = categories.find(c => c.id === t.category_id);
        if (cat) {
          categoryData[cat.name] = (categoryData[cat.name] || 0) + t.amount;
        }
      });

    if (Object.keys(categoryData).length > 0) {
      const categorySheetData = [
        ['Cheltuieli pe Categorii', '', ''],
        ['Categorie', 'Sumă', 'Procentaj'],
        ...Object.entries(categoryData)
          .sort((a, b) => b[1] - a[1])
          .map(([name, amount]) => [
            stripEmoji(removeDiacritics(name)),
            amount,
            `${((amount / totalExpense) * 100).toFixed(1)}%`
          ])
      ];

      const categoryWs = XLSX.utils.aoa_to_sheet(categorySheetData);
      XLSX.utils.book_append_sheet(wb, categoryWs, 'Categorii');
    }

    // Accounts sheet
    if (accounts.length > 0) {
      const accountsSheetData = [
        ['Solduri Conturi', ''],
        ['Cont', 'Sold Curent'],
        ...accounts.map(acc => {
          const accountTransactions = transactions.filter(t => t.account_id === acc.id);
          const income = accountTransactions
            .filter(t => t.type === 'income')
            .reduce((sum, t) => sum + t.amount, 0);
          const expense = accountTransactions
            .filter(t => t.type === 'expense')
            .reduce((sum, t) => sum + t.amount, 0);
          const balance = acc.balance + income - expense;
          return [stripEmoji(removeDiacritics(acc.name)), balance];
        })
      ];

      const accountsWs = XLSX.utils.aoa_to_sheet(accountsSheetData);
      XLSX.utils.book_append_sheet(wb, accountsWs, 'Conturi');
    }

    // Transactions sheet for current month
    const currentMonthTransactions = transactions.filter(t => {
      const date = new Date(t.date);
      return date.getMonth() === selectedMonth && date.getFullYear() === selectedYear;
    });

    if (currentMonthTransactions.length > 0) {
      const transactionsSheetData = [
        ['Tranzacții - ' + ['Ianuarie', 'Februarie', 'Martie', 'Aprilie', 'Mai', 'Iunie',
                           'Iulie', 'August', 'Septembrie', 'Octombrie', 'Noiembrie', 'Decembrie'][selectedMonth] + ' ' + selectedYear, '', '', '', ''],
        ['Data', 'Tip', 'Sumă', 'Categorie', 'Descriere'],
        ...currentMonthTransactions.map(t => {
          const cat = categories.find(c => c.id === t.category_id);
          return [
            new Date(t.date).toLocaleDateString('ro-RO'),
            t.type === 'income' ? 'Venit' : 'Cheltuială',
            t.amount,
            cat ? stripEmoji(removeDiacritics(cat.name)) : 'N/A',
            stripEmoji(removeDiacritics(t.description || ''))
          ];
        })
      ];

      const transactionsWs = XLSX.utils.aoa_to_sheet(transactionsSheetData);
      XLSX.utils.book_append_sheet(wb, transactionsWs, 'Tranzacții');
    }

    // Save Excel file
    try {
      const monthNames = ['Ianuarie', 'Februarie', 'Martie', 'Aprilie', 'Mai', 'Iunie',
                         'Iulie', 'August', 'Septembrie', 'Octombrie', 'Noiembrie', 'Decembrie'];
      const fileName = `MoneyZen_Dashboard_${monthNames[selectedMonth]}_${selectedYear}.xlsx`;
      const filePath = await save({
        defaultPath: fileName,
        filters: [{ name: 'Excel Files', extensions: ['xlsx'] }]
      });

      if (filePath) {
        const excelBuffer = XLSX.write(wb, { bookType: 'xlsx', type: 'array' });
        await writeFile(filePath, new Uint8Array(excelBuffer));
        alert('Excel export completed successfully!');
      }
    } catch (error) {
      console.error('Error exporting Excel:', error);
      alert('Error exporting Excel: ' + error);
    }
  }

  async function handleMonthChange() {
    await loadData();
    // Recreate charts with new data
    if (categoryChart) categoryChart.destroy();
    if (trendChart) trendChart.destroy();
    if (topCategoriesChart) topCategoriesChart.destroy();
    if (accountsChart) accountsChart.destroy();
    createCharts();
  }
</script>

<div class="dashboard">
  <!-- Header with Statistics -->
  <div class="stats-header">
    <h1 class="dashboard-title">📊 Dashboard Financiar</h1>

    <div class="header-controls">
      <!-- Month/Year Filter -->
      <div class="filters">
        <select bind:value={selectedMonth} on:change={handleMonthChange} class="select select-bordered">
          <option value={0}>Ianuarie</option>
          <option value={1}>Februarie</option>
          <option value={2}>Martie</option>
          <option value={3}>Aprilie</option>
          <option value={4}>Mai</option>
          <option value={5}>Iunie</option>
          <option value={6}>Iulie</option>
          <option value={7}>August</option>
          <option value={8}>Septembrie</option>
          <option value={9}>Octombrie</option>
          <option value={10}>Noiembrie</option>
          <option value={11}>Decembrie</option>
        </select>

        <select bind:value={selectedYear} on:change={handleMonthChange} class="select select-bordered">
          <option value={2024}>2024</option>
          <option value={2025}>2025</option>
        </select>
      </div>

      <!-- Export buttons -->
      <div class="export-buttons">
        <button class="btn btn-primary btn-sm" on:click={exportPDF}>
          📄 Export PDF
        </button>
        <button class="btn btn-success btn-sm" on:click={exportExcel}>
          📊 Export Excel
        </button>
      </div>
    </div>
  </div>

  <!-- Statistics Cards -->
  <div class="stats shadow mb-8">
    <div class="stat">
      <div class="stat-title">Venituri</div>
      <div class="stat-value text-success">{totalIncome.toFixed(2)} RON</div>
      <div class="stat-desc">Luna {selectedMonth + 1}/{selectedYear}</div>
    </div>

    <div class="stat">
      <div class="stat-title">Cheltuieli</div>
      <div class="stat-value text-error">{totalExpense.toFixed(2)} RON</div>
      <div class="stat-desc">{transactionCount} tranzacții</div>
    </div>

    <div class="stat">
      <div class="stat-title">Economii</div>
      <div class="stat-value text-info">{totalSavings.toFixed(2)} RON</div>
      <div class="stat-desc">{((totalSavings / totalIncome) * 100 || 0).toFixed(1)}% rată economisire</div>
    </div>
  </div>

  <div class="divider"></div>

  <!-- Add Transaction Form -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">➕ Adaugă Tranzacție Nouă</h2>
      <AddTransactionForm />
    </div>
  </div>

  <!-- Charts Grid -->
  <div class="charts-grid">
    <div class="chart-card">
      <h3>Cheltuieli pe Categorii</h3>
      <div class="chart-container">
        <canvas id="categoryChart"></canvas>
      </div>
    </div>

    <div class="chart-card">
      <h3>Trend Lunar (6 luni)</h3>
      <div class="chart-container">
        <canvas id="trendChart"></canvas>
      </div>
    </div>

    <div class="chart-card">
      <h3>Top 5 Categorii</h3>
      <div class="chart-container">
        <canvas id="topCategoriesChart"></canvas>
      </div>
    </div>

    <div class="chart-card">
      <h3>Distribuție Conturi</h3>
      <div class="chart-container">
        <canvas id="accountsChart"></canvas>
      </div>
    </div>
  </div>
</div>

<style>
  .dashboard {
    padding: 2rem;
  }

  .stats-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .dashboard-title {
    font-size: 2rem;
    font-weight: bold;
  }

  .header-controls {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .filters {
    display: flex;
    gap: 0.5rem;
  }

  .export-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .charts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: 1.5rem;
  }

  .chart-card {
    background: rgba(31, 41, 55, 0.5);
    backdrop-filter: blur(10px);
    border-radius: 1rem;
    padding: 1.5rem;
    border: 1px solid rgba(75, 85, 99, 0.2);
  }

  .chart-card h3 {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #e5e7eb;
  }

  .chart-container {
    position: relative;
    height: 300px;
  }
</style>