<script lang="ts">
  import { notificationStore } from '../../stores/notificationStore';
  import { fly, fade } from 'svelte/transition';
</script>

<div class="toast toast-end toast-top">
  {#each $notificationStore as notification (notification.id)}
    <div
      class="alert alert-{notification.type}"
      transition:fly={{ y: -100, duration: 300 }}
    >
      {#if notification.type === 'success'}
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
      {:else if notification.type === 'error'}
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
      {/if}
      <span>{notification.message}</span>
      <button
        class="btn btn-ghost btn-xs"
        on:click={() => notificationStore.remove(notification.id)}
      >
        ✕
      </button>
    </div>
  {/each}
</div>