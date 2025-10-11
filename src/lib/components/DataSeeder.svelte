<script lang="ts">
  import { transactionStore } from '../../ui/stores/transactionStore';
  import { accountStore } from '../../ui/stores/accountStore';
  import { categoryStore } from '../../ui/stores/categoryStore';
  import { tagStore } from '../../ui/stores/tagStore';
  import { notificationStore } from '../../ui/stores/notificationStore';

  let isSeeding = false;
  let seedingProgress = '';

  const testTransactions = [
    // Income transactions
    { amount: 5000, description: "Salariu Ianuarie", transaction_type: "income", category: "Salariu", tags: ["Recurent", "Lunesar"] },
    { amount: 1200, description: "Freelancing proiect web", transaction_type: "income", category: "Freelancing", tags: ["Proiect", "Web"] },
    { amount: 800, description: "Vanzare laptop vechi", transaction_type: "income", category: "Vanzari", tags: ["Electronice"] },

    // Expense transactions
    { amount: 1500, description: "Chirie apartament", transaction_type: "expense", category: "Locuinta", tags: ["Recurent", "Lunesar"] },
    { amount: 350, description: "Cumparaturi Kaufland", transaction_type: "expense", category: "Mancare", tags: ["Supermarket"] },
    { amount: 120, description: "Benzina Petrom", transaction_type: "expense", category: "Transport", tags: ["Masina", "Combustibil"] },
    { amount: 200, description: "Factura curent electric", transaction_type: "expense", category: "Utilitati", tags: ["Recurent", "Facturi"] },
    { amount: 80, description: "Abonament telefon", transaction_type: "expense", category: "Comunicatii", tags: ["Recurent", "Telefon"] },
    { amount: 150, description: "Medicaments farmacie", transaction_type: "expense", category: "Sanatate", tags: ["Medicamente"] },
    { amount: 300, description: "Cina restaurant", transaction_type: "expense", category: "Divertisment", tags: ["Restaurant", "Weekend"] },
    { amount: 450, description: "Cumparaturi haine", transaction_type: "expense", category: "Imbracaminte", tags: ["Shopping", "Haine"] },
    { amount: 75, description: "Tuns frizerie", transaction_type: "expense", category: "Personal", tags: ["Ingrijire"] },
    { amount: 25, description: "Cafea Starbucks", transaction_type: "expense", category: "Mancare", tags: ["Cafea", "Takeaway"] },
    { amount: 180, description: "Taxi spre aeroport", transaction_type: "expense", category: "Transport", tags: ["Taxi", "Calatorie"] },
    { amount: 95, description: "Abonament Netflix", transaction_type: "expense", category: "Divertisment", tags: ["Streaming", "Recurent"] }
  ];

  async function seedTestData() {
    if (isSeeding) return;

    isSeeding = true;
    seedingProgress = 'Încărcăm datele...';

    try {
      // Load existing data first
      seedingProgress = 'Încărcăm conturile...';
      await accountStore.fetchAccounts();

      seedingProgress = 'Încărcăm categoriile...';
      await categoryStore.fetchCategories();

      seedingProgress = 'Încărcăm tag-urile...';
      await tagStore.fetchTags();

      const accounts = accountStore.getAll();
      const categories = categoryStore.getAll();
      const tags = tagStore.getAll();

      if (accounts.length === 0) {
        notificationStore.error('Nu există conturi! Creează mai întâi un cont.');
        return;
      }

      // Use the first account for all transactions
      const defaultAccount = accounts[0];

      seedingProgress = 'Adăugăm tranzacțiile...';
      let successCount = 0;

      for (let i = 0; i < testTransactions.length; i++) {
        const testTx = testTransactions[i];
        seedingProgress = `Adăugăm tranzacția ${i + 1}/${testTransactions.length}: ${testTx.description}`;

        // Find category by name or use first available
        let categoryId = '';
        const foundCategory = categories.find(cat => cat.name === testTx.category);
        if (foundCategory) {
          categoryId = foundCategory.id;
        } else if (categories.length > 0) {
          categoryId = categories[0].id;
        }

        // Find tags by name
        const selectedTagIds: string[] = [];
        testTx.tags.forEach(tagName => {
          const foundTag = tags.find(tag => tag.name === tagName);
          if (foundTag) {
            selectedTagIds.push(foundTag.id);
          }
        });

        // Create transaction with random date in the last 30 days
        const randomDaysAgo = Math.floor(Math.random() * 30);
        const transactionDate = new Date();
        transactionDate.setDate(transactionDate.getDate() - randomDaysAgo);

        try {
          await transactionStore.create({
            account_id: defaultAccount.id,
            category_id: categoryId,
            amount: testTx.amount,
            description: testTx.description,
            transaction_type: testTx.transaction_type as 'income' | 'expense',
            date: transactionDate.toISOString().split('T')[0],
            tags: selectedTagIds
          });

          successCount++;

          // Small delay to avoid overwhelming the database
          await new Promise(resolve => setTimeout(resolve, 100));

        } catch (error) {
          console.error(`Failed to create transaction: ${testTx.description}`, error);
        }
      }

      seedingProgress = `Finalizat! ${successCount}/${testTransactions.length} tranzacții adăugate cu succes.`;
      notificationStore.success(`${successCount} tranzacții de test adăugate cu succes!`);

      // Refresh stores to show new data
      await transactionStore.fetchTransactions();
      await accountStore.fetchAccounts(); // This will update balances

    } catch (error) {
      console.error('Error seeding data:', error);
      notificationStore.error('Eroare la adăugarea datelor de test!');
      seedingProgress = 'Eroare la adăugarea datelor!';
    } finally {
      isSeeding = false;
      setTimeout(() => {
        seedingProgress = '';
      }, 3000);
    }
  }

  async function clearAllTransactions() {
    if (isSeeding) return;

    if (!confirm('Sigur vrei să ștergi TOATE tranzacțiile? Această acțiune nu poate fi anulată!')) {
      return;
    }

    isSeeding = true;
    seedingProgress = 'Ștergem toate tranzacțiile...';

    try {
      await transactionStore.fetchTransactions();
      const transactions = transactionStore.getAll();

      let deletedCount = 0;
      for (let i = 0; i < transactions.length; i++) {
        seedingProgress = `Ștergem tranzacția ${i + 1}/${transactions.length}`;
        try {
          await transactionStore.delete(transactions[i].id);
          deletedCount++;
          await new Promise(resolve => setTimeout(resolve, 50));
        } catch (error) {
          console.error(`Failed to delete transaction ${transactions[i].id}`, error);
        }
      }

      seedingProgress = `Finalizat! ${deletedCount} tranzacții șterse.`;
      notificationStore.success(`${deletedCount} tranzacții șterse cu succes!`);

      // Refresh stores
      await transactionStore.fetchTransactions();
      await accountStore.fetchAccounts();

    } catch (error) {
      console.error('Error clearing data:', error);
      notificationStore.error('Eroare la ștergerea datelor!');
      seedingProgress = 'Eroare la ștergerea datelor!';
    } finally {
      isSeeding = false;
      setTimeout(() => {
        seedingProgress = '';
      }, 3000);
    }
  }
</script>

<div class="card bg-base-100 shadow-xl max-w-2xl mx-auto">
  <div class="card-body">
    <h2 class="card-title text-2xl mb-4">🌱 Date de Test pentru Dashboard</h2>

    <div class="space-y-4">
      <div class="alert alert-info">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <span>Această componentă adaugă 15 tranzacții de test pentru a testa Dashboard-ul cu grafice.</span>
      </div>

      {#if seedingProgress}
        <div class="alert alert-warning">
          <span class="loading loading-spinner loading-sm"></span>
          <span>{seedingProgress}</span>
        </div>
      {/if}

      <div class="flex gap-4 justify-center">
        <button
          class="btn btn-primary btn-wide"
          class:loading={isSeeding}
          disabled={isSeeding}
          on:click={seedTestData}
        >
          {#if isSeeding}
            Se adaugă...
          {:else}
            🌱 Adaugă Date de Test
          {/if}
        </button>

        <button
          class="btn btn-error btn-outline"
          class:loading={isSeeding}
          disabled={isSeeding}
          on:click={clearAllTransactions}
        >
          {#if isSeeding}
            Se șterge...
          {:else}
            🗑️ Șterge Toate
          {/if}
        </button>
      </div>

      <div class="text-sm text-base-content/70 space-y-2">
        <p><strong>Date incluse:</strong></p>
        <ul class="list-disc list-inside space-y-1">
          <li><strong>3 tranzacții de venit:</strong> salariu, freelancing, vânzări</li>
          <li><strong>12 tranzacții de cheltuieli:</strong> chirie, mâncare, transport, utilități, etc.</li>
          <li><strong>Date randomizate:</strong> în ultimele 30 de zile</li>
          <li><strong>Categorii variate:</strong> pentru graficele din Dashboard</li>
          <li><strong>Tag-uri multiple:</strong> pentru filtrare avansată</li>
        </ul>
      </div>
    </div>
  </div>
</div>