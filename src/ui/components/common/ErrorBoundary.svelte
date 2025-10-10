<script lang="ts">
  import { onMount } from 'svelte';
  import { notificationStore } from '../../stores/notificationStore';
  
  export let resetKeys: any[] = [];
  export let fallback: string = 'Ceva nu a mers bine. Reîncarcă pagina.';
  
  let hasError = false;
  let errorMessage = '';
  
  const resetError = () => {
    hasError = false;
    errorMessage = '';
  };
  
  const handleError = (error: Error) => {
    console.error('ErrorBoundary caught:', error);
    hasError = true;
    errorMessage = error.message || fallback;
    notificationStore.error(errorMessage);
  };
  
  onMount(() => {
    const handleUnhandledError = (event: ErrorEvent) => {
      handleError(new Error(event.message));
    };
    
    const handleUnhandledRejection = (event: PromiseRejectionEvent) => {
      handleError(new Error(event.reason));
    };
    
    window.addEventListener('error', handleUnhandledError);
    window.addEventListener('unhandledrejection', handleUnhandledRejection);
    
    return () => {
      window.removeEventListener('error', handleUnhandledError);
      window.removeEventListener('unhandledrejection', handleUnhandledRejection);
    };
  });
  
  $: if (resetKeys.length) resetError();
</script>

{#if hasError}
  <div class="error-boundary">
    <div class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <div>
        <h3 class="font-bold">Eroare!</h3>
        <div class="text-xs">{errorMessage}</div>
      </div>
      <button class="btn btn-sm" on:click={resetError}>Încearcă din nou</button>
    </div>
  </div>
{:else}
  <slot />
{/if}
