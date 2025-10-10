<script lang="ts">
  import type { Account } from '$lib/types';
  import { accountStore } from '../../ui/stores/accountStore';
  export let selectedAccountId: string;
  export let onSelect: (accountId: string) => void;

  // Group accounts by owner
  $: groupedAccounts = $accountStore.reduce((groups, account) => {
    const owner = account.owner || 'Unknown';
    if (!groups[owner]) {
      groups[owner] = [];
    }
    groups[owner].push(account);
    return groups;
  }, {} as Record<string, Account[]>);

  // Get emoji for owner
  function getOwnerEmoji(owner: string): string {
    if (owner === 'Ioan') return '👤';
    if (owner === 'Nico') return '👤';
    if (owner === 'Firmă') return '🏢';
    if (owner.toLowerCase().includes('cash')) return '💵';
    return '📁';
  }

  function handleSelect(event: Event) {
    const target = event.target as HTMLSelectElement;
    onSelect(target.value);
  }
</script>

<select
  class="select select-bordered w-full"
  value={selectedAccountId}
  on:change={handleSelect}
>
  <option disabled selected={!selectedAccountId}>Select Account</option>

  {#each Object.entries(groupedAccounts) as [owner, ownerAccounts]}
    <optgroup label="{getOwnerEmoji(owner)} {owner.toUpperCase()}">
      {#each ownerAccounts as account}
        <option value={account.id}>
          {account.name} ({account.balance.toFixed(2)} {account.currency})
        </option>
      {/each}
    </optgroup>
  {/each}
</select>