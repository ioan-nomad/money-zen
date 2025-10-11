<script lang="ts">
  import { onMount } from 'svelte';
  import { accountStore } from '../stores/accountStore';
  import type { Account } from '../core/entities/Account';
  import AccountList from '../../lib/components/AccountList.svelte';

  let error = '';

  // Edit modal state
  let showEditModal = false;
  let editAccount: Account | null = null;
  let editName = '';
  let editType = '';
  let editCurrency = '';

  // Delete confirmation state
  let showDeleteModal = false;
  let deleteAccountId = '';

  onMount(async () => {
    await loadAccounts();
  });

  async function loadAccounts() {
    try {
      await accountStore.load();
    } catch (err) {
      error = String(err);
    }
  }

  function handleEdit(account: Account) {
    editAccount = account;
    editName = account.name;
    editType = account.account_type;
    editCurrency = account.currency;
    showEditModal = true;
  }

  async function saveEdit() {
    if (!editAccount || !editName) return;

    try {
      await accountStore.updateAccount(editAccount.id, {
        name: editName,
        account_type: editType,
        currency: editCurrency
      });
      showEditModal = false;
      await loadAccounts();
    } catch (err) {
      error = String(err);
    }
  }

  function handleDelete(accountId: string) {
    deleteAccountId = accountId;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    try {
      await accountStore.remove(deleteAccountId);
      showDeleteModal = false;
      await loadAccounts();
    } catch (err) {
      error = String(err);
    }
  }

  // Reactive subscription
  $: accounts = $accountStore;
</script>

<div class="max-w-4xl mx-auto p-6">
  {#if error}
    <div class="alert alert-error mb-6">
      <span>Error: {error}</span>
    </div>
  {/if}

  <AccountList {accounts} onEdit={handleEdit} onDelete={handleDelete} />

  <!-- Edit Modal -->
  {#if showEditModal}
    <div class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">Edit Account</h3>

        <div class="form-control mb-3">
          <label class="label" for="edit-name"><span class="label-text">Account Name</span></label>
          <input id="edit-name" type="text" class="input input-bordered" bind:value={editName} />
        </div>

        <div class="form-control mb-3">
          <label class="label" for="edit-type"><span class="label-text">Account Type</span></label>
          <select id="edit-type" class="select select-bordered" bind:value={editType}>
            <option value="checking">Checking</option>
            <option value="savings">Savings</option>
            <option value="credit">Credit Card</option>
            <option value="cash">Cash</option>
          </select>
        </div>

        <div class="form-control mb-4">
          <label class="label" for="edit-currency"><span class="label-text">Currency</span></label>
          <input id="edit-currency" type="text" class="input input-bordered" bind:value={editCurrency} />
        </div>

        <div class="modal-action">
          <button class="btn" on:click={() => showEditModal = false}>Cancel</button>
          <button class="btn btn-primary" on:click={saveEdit} disabled={!editName}>Save</button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Delete Confirmation Modal -->
  {#if showDeleteModal}
    <div class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">Delete Account</h3>
        <p class="mb-4">Are you sure? This will also delete all transactions for this account.</p>

        <div class="modal-action">
          <button class="btn" on:click={() => showDeleteModal = false}>Cancel</button>
          <button class="btn btn-error" on:click={confirmDelete}>Delete</button>
        </div>
      </div>
    </div>
  {/if}
</div>