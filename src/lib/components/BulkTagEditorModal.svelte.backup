<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Tag } from '../database';

  export let transactionCount: number;
  export let tags: Tag[] = [];
  export let onUpdate: (tagsToAdd: string[], tagsToRemove: string[]) => Promise<void>;

  const dispatch = createEventDispatcher();

  let selectedTagsToAdd: Set<string> = new Set();
  let selectedTagsToRemove: Set<string> = new Set();
  let isSubmitting = false;
  let error = '';

  function toggleTagAdd(tagId: string) {
    const newSet = new Set(selectedTagsToAdd);
    if (newSet.has(tagId)) {
      newSet.delete(tagId);
    } else {
      newSet.add(tagId);
      // Remove from "remove" set if it was there
      selectedTagsToRemove.delete(tagId);
      selectedTagsToRemove = new Set(selectedTagsToRemove);
    }
    selectedTagsToAdd = newSet;
  }

  function toggleTagRemove(tagId: string) {
    const newSet = new Set(selectedTagsToRemove);
    if (newSet.has(tagId)) {
      newSet.delete(tagId);
    } else {
      newSet.add(tagId);
      // Remove from "add" set if it was there
      selectedTagsToAdd.delete(tagId);
      selectedTagsToAdd = new Set(selectedTagsToAdd);
    }
    selectedTagsToRemove = newSet;
  }

  async function handleSubmit() {
    if (selectedTagsToAdd.size === 0 && selectedTagsToRemove.size === 0) {
      error = 'Please select at least one tag to add or remove';
      return;
    }

    isSubmitting = true;
    error = '';

    try {
      await onUpdate(
        Array.from(selectedTagsToAdd),
        Array.from(selectedTagsToRemove)
      );
      dispatch('close');
    } catch (err) {
      error = String(err);
      console.error('Failed to update tags:', err);
    } finally {
      isSubmitting = false;
    }
  }

  function handleClose() {
    dispatch('close');
  }
</script>

<!-- Modal Backdrop -->
<div
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
  on:click={handleClose}
  role="dialog"
  aria-modal="true"
>
  <!-- Modal Content -->
  <div
    class="bg-base-200 rounded-lg p-6 w-full max-w-2xl max-h-[80vh] overflow-y-auto"
    on:click={(e) => e.stopPropagation()}
  >
    <h2 class="text-2xl font-bold mb-4">Edit Tags for {transactionCount} Transaction{transactionCount !== 1 ? 's' : ''}</h2>

    {#if error}
      <div class="alert alert-error mb-4">
        <span>{error}</span>
      </div>
    {/if}

    <!-- Add Tags Section -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold mb-3 text-success">➕ Add Tags</h3>
      {#if tags.length === 0}
        <p class="text-base-content/50">No tags available</p>
      {:else}
        <div class="grid grid-cols-2 gap-2">
          {#each tags as tag}
            <label class="flex items-center gap-2 cursor-pointer p-2 rounded hover:bg-base-300">
              <input
                type="checkbox"
                class="checkbox checkbox-sm checkbox-success"
                checked={selectedTagsToAdd.has(tag.id)}
                on:change={() => toggleTagAdd(tag.id)}
              />
              <span
                class="badge"
                style="background-color: {tag.color}; color: white;"
              >
                {tag.name}
              </span>
            </label>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Remove Tags Section -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold mb-3 text-error">➖ Remove Tags</h3>
      {#if tags.length === 0}
        <p class="text-base-content/50">No tags available</p>
      {:else}
        <div class="grid grid-cols-2 gap-2">
          {#each tags as tag}
            <label class="flex items-center gap-2 cursor-pointer p-2 rounded hover:bg-base-300">
              <input
                type="checkbox"
                class="checkbox checkbox-sm checkbox-error"
                checked={selectedTagsToRemove.has(tag.id)}
                on:change={() => toggleTagRemove(tag.id)}
              />
              <span
                class="badge"
                style="background-color: {tag.color}; color: white;"
              >
                {tag.name}
              </span>
            </label>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Action Buttons -->
    <div class="flex justify-end gap-2">
      <button
        class="btn btn-ghost"
        on:click={handleClose}
        disabled={isSubmitting}
      >
        Cancel
      </button>
      <button
        class="btn btn-primary"
        on:click={handleSubmit}
        disabled={isSubmitting || (selectedTagsToAdd.size === 0 && selectedTagsToRemove.size === 0)}
      >
        {#if isSubmitting}
          <span class="loading loading-spinner loading-sm"></span>
          Updating...
        {:else}
          Update {transactionCount} Transaction{transactionCount !== 1 ? 's' : ''}
        {/if}
      </button>
    </div>
  </div>
</div>