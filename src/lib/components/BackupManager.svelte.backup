<script lang="ts">
  import { Database } from '../database';
  import { open } from '@tauri-apps/plugin-dialog';

  let loading = false;
  let message = '';
  let isError = false;
  let showConfirmModal = false;
  let selectedBackupPath = '';

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

  async function selectBackupFile() {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Database Backup',
        extensions: ['db']
      }]
    });

    if (selected && typeof selected === 'string') {
      selectedBackupPath = selected;
      showConfirmModal = true;
    }
  }

  async function confirmRestore() {
    showConfirmModal = false;
    loading = true;
    message = '';
    isError = false;

    try {
      const result = await Database.restoreDatabase(selectedBackupPath);
      message = result + ' - Please restart the app to see changes.';
      isError = false;
    } catch (error) {
      message = `Error: ${error}`;
      isError = true;
    } finally {
      loading = false;
      selectedBackupPath = '';
    }
  }

  function cancelRestore() {
    showConfirmModal = false;
    selectedBackupPath = '';
  }
</script>

<div class="backup-manager">
  <h3>Database Backup & Restore</h3>

  <div class="button-group">
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

    <button
      class="btn btn-secondary"
      on:click={selectBackupFile}
      disabled={loading}
    >
      Restore from Backup
    </button>
  </div>

  {#if message}
    <div class="alert" class:alert-success={!isError} class:alert-error={isError}>
      {message}
    </div>
  {/if}
</div>

<!-- Confirmation Modal -->
{#if showConfirmModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg">⚠️ Warning: Destructive Action</h3>
      <p class="py-4">
        Restoring from backup will <strong>OVERWRITE</strong> your current database.
        All current data will be replaced with the backup data.
        <br><br>
        <strong>This action cannot be undone!</strong>
      </p>
      <div class="modal-action">
        <button class="btn btn-ghost" on:click={cancelRestore}>Cancel</button>
        <button class="btn btn-error" on:click={confirmRestore}>Restore Anyway</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .backup-manager {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .button-group {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
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