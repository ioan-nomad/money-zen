<script lang="ts">
  import type { Account } from '$lib/types';
  import { accountStore } from '../../ui/stores/accountStore';
  import AccountCard from './AccountCard.svelte';
  export let onEdit: (account: Account) => void;
  export let onDelete: (accountId: string) => void;
</script>

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h2 class="card-title">All Accounts ({$accountStore.length})</h2>

    <div class="space-y-2">
      {#if $accountStore.length === 0}
        <div class="text-center py-8 text-base-content/50">
          No accounts found. Create your first account above.
        </div>
      {:else}
        {#each $accountStore as account}
          <div class="flex items-center gap-2">
            <div class="flex-1">
              <AccountCard {account} />
            </div>
            <button 
              class="btn btn-sm btn-ghost"
              on:click={() => onEdit(account)}
              title="Edit account"
            >
              ✏️
            </button>
            <button 
              class="btn btn-sm btn-error"
              on:click={() => onDelete(account.id)}
              title="Delete account"
            >
              🗑️
            </button>
          </div>
        {/each}
      {/if}
    </div>
  </div>
</div>
