<script lang="ts">
  import type { Transaction } from '../database';
  import { formatCurrency, formatDate } from '../utils';

  export let transaction: Transaction;
</script>

<div class="flex items-center justify-between p-3 bg-base-200 rounded-lg hover:bg-base-300 transition-colors">
  <span 
    class="font-bold text-lg min-w-[120px]"
    class:text-success={transaction.transaction_type === 'income'}
    class:text-error={transaction.transaction_type === 'expense'}
  >
    {transaction.transaction_type === 'income' ? '+' : '-'}{formatCurrency(transaction.amount)}
  </span>
  <span class="flex-1 px-4 text-base-content">{transaction.description}</span>
  <span class="text-sm opacity-70 min-w-[100px] text-right">{formatDate(transaction.date)}</span>
</div>
