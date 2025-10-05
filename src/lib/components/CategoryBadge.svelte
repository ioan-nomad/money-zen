<script lang="ts">
  import type { Category } from '../database';

  export let category: Category;
</script>

<div 
  class="flex items-center gap-3 p-3 bg-base-200 rounded-lg border-l-4 hover:bg-base-300 transition-colors"
  style="border-left-color: {category.color}"
>
  <span class="text-2xl">{category.icon}</span>
  <div class="flex-1">
    <span class="font-semibold text-base-content">{category.name}</span>
    <span class="badge badge-xs badge-ghost ml-2">{category.category_type}</span>
  </div>
</div>
