<script lang="ts">
  import { Database } from '../database';

  let loading = false;
  let message = '';
  let isError = false;

  async function createBackup() {
    loading = true;
    message = '';
    isError = false;

    try {
      const result = await Database.backupDatabase();
      message = result;
      isError = false;
    } catch (error) {
      message = `Error: ${error}`;
      isError = true;
    } finally {
      loading = false;
    }
  }
</script>

<div class="backup-manager">
  <h3>Database Backup</h3>

  <button
    class="btn btn-primary"
    on:click={createBackup}
    disabled={loading}
  >
    {#if loading}
      Creating Backup...
    {:else}
      Create Backup
    {/if}
  </button>

  {#if message}
    <div class="alert" class:alert-success={!isError} class:alert-error={isError}>
      {message}
    </div>
  {/if}
</div>

<style>
  .backup-manager {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  h3 {
    margin: 0;
    color: #1a1a1a;
  }

  .alert {
    padding: 0.75rem;
    border-radius: 0.5rem;
    font-size: 0.9em;
  }
</style>