<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Transaction, Account, Category } from '../database';
  import { formatCurrency, formatDate } from '../utils';
  import EditTransactionModal from './EditTransactionModal.svelte';
  import { Database } from '../database';

  export let transaction: Transaction;
  export let accounts: Account[] = [];
  export let categories: Category[] = [];

  const dispatch = createEventDispatcher();

  let expanded = false;
  let showDeleteConfirm = false;
  let showEditModal = false;

  // Helper functions
  $: account = accounts.find(a => a.id === transaction.account_id);
  $: category = categories.find(c => c.id === transaction.category_id);

  function toggleExpand() {
    expanded = !expanded;
    showDeleteConfirm = false; // Reset confirmation when collapsing
  }

  function handleDelete() {
    showDeleteConfirm = true;
  }

  function confirmDelete() {
    dispatch('delete', transaction.id);
    showDeleteConfirm = false;
  }

  function cancelDelete() {
    showDeleteConfirm = false;
  }

  function handleEdit() {
    showEditModal = true;
  }

  async function handleUpdate(
    id: string,
    accountId: string,
    categoryId: string,
    amount: number,
    description: string,
    transactionType: 'income' | 'expense',
    date: string
  ) {
    try {
      const updatedTransaction = await Database.updateTransaction(
        id,
        accountId,
        categoryId,
        amount,
        description,
        transactionType,
        date
      );

      // Update local transaction data
      transaction = updatedTransaction;
      showEditModal = false;
    } catch (error) {
      console.error('Failed to update transaction:', error);
      alert('Eroare la actualizarea tranzacției. Încearcă din nou.');
    }
  }
</script>

<div class="rounded-lg overflow-hidden transition-all duration-300">
  <!-- Main Transaction Row (Always Visible) -->
  <div
    class="flex items-center justify-between p-3 bg-base-200 hover:bg-base-300 transition-colors cursor-pointer"
    on:click={toggleExpand}
    on:keypress={(e) => e.key === 'Enter' && toggleExpand()}
    role="button"
    tabindex="0"
  >
    <span
      class="font-bold text-lg min-w-[120px]"
      class:text-success={transaction.transaction_type === 'income'}
      class:text-error={transaction.transaction_type === 'expense'}
    >
      {transaction.transaction_type === 'income' ? '+' : '-'}{formatCurrency(transaction.amount)}
    </span>
    <span class="flex-1 px-4 text-base-content">{transaction.description}</span>
    <span class="text-sm opacity-70 min-w-[100px] text-right">{formatDate(transaction.date)}</span>

    <!-- Expand/Collapse Icon -->
    <span class="ml-2 text-lg transition-transform duration-300" class:rotate-180={expanded}>
      ▼
    </span>
  </div>

  <!-- Expanded Details Section -->
  {#if expanded}
    <div class="bg-base-300 p-4 space-y-3 animate-fade-in">
      <!-- Transaction Details Grid -->
      <div class="grid grid-cols-2 gap-3 text-sm">
        <div>
          <span class="opacity-70">Account:</span>
          <span class="ml-2 font-semibold">{account?.name || 'Unknown'}</span>
        </div>
        <div>
          <span class="opacity-70">Category:</span>
          <span class="ml-2">{category?.icon || ''} {category?.name || 'Unknown'}</span>
        </div>
        <div>
          <span class="opacity-70">Type:</span>
          <span class="ml-2 capitalize">{transaction.transaction_type}</span>
        </div>
        <div>
          <span class="opacity-70">Date:</span>
          <span class="ml-2">{formatDate(transaction.date)}</span>
        </div>
      </div>

      <!-- Action Buttons -->
      {#if !showDeleteConfirm}
        <div class="flex gap-2 pt-2">
          <button class="btn btn-sm btn-primary" on:click={handleEdit}>
            ✏️ Edit
          </button>
          <button class="btn btn-sm btn-error" on:click={handleDelete}>
            🗑️ Delete
          </button>
        </div>
      {:else}
        <!-- Delete Confirmation -->
        <div class="bg-error/10 p-3 rounded-lg space-y-2">
          <p class="text-sm font-semibold">Ștergi această tranzacție?</p>
          <div class="flex gap-2">
            <button class="btn btn-sm btn-error" on:click={confirmDelete}>
              ✓ Da, șterge
            </button>
            <button class="btn btn-sm btn-ghost" on:click={cancelDelete}>
              ✗ Anulează
            </button>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<!-- Edit Modal -->
{#if showEditModal}
  <EditTransactionModal
    {transaction}
    {accounts}
    {categories}
    onUpdate={handleUpdate}
    on:close={() => showEditModal = false}
  />
{/if}

<style>
  @keyframes fade-in {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .animate-fade-in {
    animation: fade-in 0.3s ease-out;
  }

  .rotate-180 {
    transform: rotate(180deg);
  }
</style>