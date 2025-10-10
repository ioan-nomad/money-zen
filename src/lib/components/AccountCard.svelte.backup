<script lang="ts">
  import type { Account } from '../database';
  import { formatCurrency } from '../utils';

  export let account: Account;
</script>

<div class="flex items-center justify-between p-3 bg-base-200 rounded-lg hover:bg-base-300 transition-colors">
  <div class="flex items-center gap-3">
    <div class="text-2xl">
      {#if account.account_type === 'checking'}💳
      {:else if account.account_type === 'savings'}🏦
      {:else if account.account_type === 'credit'}💳
      {:else}💵{/if}
    </div>
    <div>
      <span class="font-semibold text-base-content">{account.name}</span>
      <div class="badge badge-sm badge-ghost ml-2">{account.account_type}</div>
    </div>
  </div>
  <span class="font-bold text-lg" class:text-success={account.balance >= 0} class:text-error={account.balance < 0}>
    {formatCurrency(account.balance, account.currency)}
  </span>
</div>
