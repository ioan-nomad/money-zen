<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Tag } from '$lib/types';
  import { tagStore } from '../../ui/stores/tagStore';

  export let selectedTagIds: string[] = [];
  export let disabled = false;
  export let placeholder = 'Select tags...';

  const dispatch = createEventDispatcher<{
    change: { selectedTagIds: string[] };
  }>();

  let showDropdown = false;
  let searchQuery = '';
  let tagPickerContainer: HTMLElement;

  // Filter tags based on search query
  $: filteredTags = $tagStore.filter(tag =>
    tag.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  // Get selected tags for display
  $: selectedTags = $tagStore.filter(tag => selectedTagIds.includes(tag.id));

  function toggleTag(tagId: string) {
    if (disabled) return;

    let newSelectedIds: string[];
    if (selectedTagIds.includes(tagId)) {
      // Remove tag
      newSelectedIds = selectedTagIds.filter(id => id !== tagId);
    } else {
      // Add tag
      newSelectedIds = [...selectedTagIds, tagId];
    }

    dispatch('change', { selectedTagIds: newSelectedIds });
  }

  function removeTag(tagId: string) {
    if (disabled) return;

    const newSelectedIds = selectedTagIds.filter(id => id !== tagId);
    dispatch('change', { selectedTagIds: newSelectedIds });
  }

  function toggleDropdown() {
    if (disabled) return;
    showDropdown = !showDropdown;
    if (showDropdown) {
      searchQuery = '';
    }
  }

  function handleClickOutside(event: MouseEvent) {
    if (tagPickerContainer && !tagPickerContainer.contains(event.target as Node)) {
      showDropdown = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      showDropdown = false;
    }
  }

  // Handle outside clicks
  $: if (typeof window !== 'undefined') {
    if (showDropdown) {
      document.addEventListener('click', handleClickOutside);
      document.addEventListener('keydown', handleKeydown);
    } else {
      document.removeEventListener('click', handleClickOutside);
      document.removeEventListener('keydown', handleKeydown);
    }
  }
</script>

<div class="tag-picker relative" bind:this={tagPickerContainer}>
  <!-- Selected Tags Display -->
  <div
    class="min-h-[48px] w-full border border-base-300 rounded-lg bg-base-100 p-2 cursor-pointer"
    class:opacity-50={disabled}
    on:click={toggleDropdown}
    on:keypress={(e) => e.key === 'Enter' && toggleDropdown()}
    role="button"
    tabindex="0"
  >
    <div class="flex flex-wrap gap-1">
      {#if selectedTags.length === 0}
        <span class="text-base-content/50 py-2 px-1">{placeholder}</span>
      {:else}
        {#each selectedTags as tag}
          <span
            class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium text-white"
            style="background-color: {tag.color}"
          >
            <span>{tag.icon}</span>
            <span>{tag.name}</span>
            {#if !disabled}
              <button
                class="ml-1 hover:bg-black/20 rounded-full w-4 h-4 flex items-center justify-center"
                on:click|stopPropagation={() => removeTag(tag.id)}
                title="Remove tag"
              >
                ×
              </button>
            {/if}
          </span>
        {/each}
      {/if}
    </div>
  </div>

  <!-- Dropdown -->
  {#if showDropdown && !disabled}
    <div class="absolute top-full left-0 right-0 mt-1 bg-base-100 border border-base-300 rounded-lg shadow-lg z-50 max-h-60 overflow-hidden">
      <!-- Search Input -->
      <div class="p-3 border-b border-base-300">
        <input
          type="text"
          placeholder="Search tags..."
          class="input input-sm input-bordered w-full"
          bind:value={searchQuery}
          on:click|stopPropagation
        />
      </div>

      <!-- Tag List -->
      <div class="max-h-40 overflow-y-auto">
        {#if filteredTags.length === 0}
          <div class="p-3 text-center text-base-content/50">
            {searchQuery ? 'No tags match your search' : 'No tags available'}
          </div>
        {:else}
          {#each filteredTags as tag}
            <button
              class="w-full flex items-center gap-3 p-3 hover:bg-base-200 transition-colors text-left {selectedTagIds.includes(tag.id) ? 'bg-primary/10' : ''}"
              on:click|stopPropagation={() => toggleTag(tag.id)}
            >
              <!-- Checkbox -->
              <div class="flex-shrink-0">
                <input
                  type="checkbox"
                  class="checkbox checkbox-sm"
                  checked={selectedTagIds.includes(tag.id)}
                  on:change={() => toggleTag(tag.id)}
                />
              </div>

              <!-- Tag Icon -->
              <span class="text-lg">{tag.icon}</span>

              <!-- Tag Name and Color -->
              <div class="flex-1 flex items-center gap-2">
                <span class="font-medium">{tag.name}</span>
                <div
                  class="w-3 h-3 rounded-full"
                  style="background-color: {tag.color}"
                ></div>
              </div>
            </button>
          {/each}
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .tag-picker {
    /* Ensure proper z-index stacking */
    position: relative;
  }

  /* Ensure dropdown appears above other elements */
  .tag-picker > div:last-child {
    z-index: 1000;
  }

  /* Custom scrollbar for tag list */
  .max-h-40::-webkit-scrollbar {
    width: 6px;
  }

  .max-h-40::-webkit-scrollbar-track {
    background: transparent;
  }

  .max-h-40::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.3);
    border-radius: 3px;
  }

  .max-h-40::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.5);
  }
</style>