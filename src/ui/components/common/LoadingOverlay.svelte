<script lang="ts">
  import { loadingStore } from '../../stores/loadingStore';
  import { fade } from 'svelte/transition';
</script>

{#if $loadingStore.isLoading}
  <div
    class="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center"
    transition:fade={{ duration: 200 }}
  >
    <div class="bg-base-100 p-6 rounded-lg shadow-xl">
      <div class="flex items-center gap-4">
        <span class="loading loading-spinner loading-lg text-primary"></span>
        <span class="text-lg">
          {$loadingStore.loadingText || 'Se încarcă...'}
        </span>
      </div>
    </div>
  </div>
{/if}