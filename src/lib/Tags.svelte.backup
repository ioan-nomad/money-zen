<script lang="ts">
  import { onMount } from 'svelte';
  import { Database, type Tag } from './database';
  import TagForm from './components/TagForm.svelte';

  let tags: Tag[] = [];
  let error = '';

  // Create modal state
  let showCreateModal = false;

  // Edit modal state
  let showEditModal = false;
  let editTag: Tag | null = null;

  // Form submission state
  let isSubmitting = false;

  // Delete confirmation state
  let showDeleteModal = false;
  let deleteTagId = '';
  let deleteTagName = '';
  let deleteTagTransactionCount = 0;

  // Sort tags alphabetically
  $: sortedTags = tags.sort((a, b) => a.name.localeCompare(b.name));

  onMount(async () => {
    await loadTags();
  });

  async function loadTags() {
    try {
      tags = await Database.getTags();
    } catch (err) {
      error = String(err);
    }
  }

  function openCreateModal() {
    showCreateModal = true;
  }

  async function handleCreateSubmit(event: CustomEvent) {
    isSubmitting = true;
    try {
      await Database.createTag(
        event.detail.name,
        event.detail.color,
        event.detail.icon
      );
      showCreateModal = false;
      await loadTags();
    } catch (err) {
      error = String(err);
    } finally {
      isSubmitting = false;
    }
  }

  function handleEdit(tag: Tag) {
    editTag = tag;
    showEditModal = true;
  }

  async function handleEditSubmit(event: CustomEvent) {
    if (!editTag) return;
    isSubmitting = true;
    try {
      await Database.updateTag(
        editTag.id,
        event.detail.name,
        event.detail.color,
        event.detail.icon
      );
      showEditModal = false;
      editTag = null;
      await loadTags();
    } catch (err) {
      error = String(err);
    } finally {
      isSubmitting = false;
    }
  }

  async function handleDelete(tagId: string, tagName: string) {
    // Get transaction count for this tag to show warning
    try {
      const transactions = await Database.getTransactionsByTag(tagId);
      deleteTagTransactionCount = transactions.length;
    } catch (err) {
      deleteTagTransactionCount = 0;
    }

    deleteTagId = tagId;
    deleteTagName = tagName;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    try {
      await Database.deleteTag(deleteTagId);
      showDeleteModal = false;
      await loadTags();
    } catch (err) {
      error = String(err);
    }
  }
</script>

<div class="max-w-4xl mx-auto p-6">
  {#if error}
    <div class="alert alert-error mb-6">
      <span>Error: {error}</span>
    </div>
  {/if}

  <!-- Header with Create Button -->
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-3xl font-bold">Tags</h1>
    <button class="btn btn-primary" on:click={openCreateModal}>
      ➕ Create Tag
    </button>
  </div>

  <!-- Tags List -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title text-primary">🏷️ All Tags ({sortedTags.length})</h2>

      <div class="space-y-2">
        {#if sortedTags.length === 0}
          <div class="text-center py-4 text-base-content/50">
            No tags found. Create your first tag to organize transactions.
          </div>
        {:else}
          {#each sortedTags as tag}
            <div class="flex items-center gap-2">
              <div class="flex-1">
                <div class="flex items-center justify-between p-3 bg-base-200 rounded-lg hover:bg-base-300 transition-colors">
                  <div class="flex items-center gap-3">
                    <div class="text-2xl">{tag.icon}</div>
                    <div>
                      <span class="font-semibold text-base-content">{tag.name}</span>
                      <div class="w-4 h-4 rounded-full inline-block ml-2" style="background-color: {tag.color}"></div>
                    </div>
                  </div>
                  <div class="badge badge-outline" style="border-color: {tag.color}; color: {tag.color}">tag</div>
                </div>
              </div>
              <button
                class="btn btn-sm btn-ghost"
                on:click={() => handleEdit(tag)}
                title="Edit tag"
              >
                ✏️
              </button>
              <button
                class="btn btn-sm btn-error"
                on:click={() => handleDelete(tag.id, tag.name)}
                title="Delete tag"
              >
                🗑️
              </button>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
</div>

<!-- Create Modal -->
{#if showCreateModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <TagForm
        {isSubmitting}
        on:submit={handleCreateSubmit}
        on:cancel={() => showCreateModal = false}
      />
    </div>
  </div>
{/if}

<!-- Edit Modal -->
{#if showEditModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <TagForm
        tag={editTag}
        {isSubmitting}
        on:submit={handleEditSubmit}
        on:cancel={() => { showEditModal = false; editTag = null; }}
      />
    </div>
  </div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">Delete Tag</h3>
      <p class="mb-4">Are you sure you want to delete "<strong>{deleteTagName}</strong>"?</p>

      {#if deleteTagTransactionCount > 0}
        <p class="mb-4 text-warning">⚠️ This tag is used in <strong>{deleteTagTransactionCount}</strong> transaction{deleteTagTransactionCount > 1 ? 's' : ''}.</p>
      {/if}

      <p class="mb-4 text-warning">⚠️ This action cannot be undone.</p>

      <div class="modal-action">
        <button class="btn" on:click={() => showDeleteModal = false}>Cancel</button>
        <button class="btn btn-error" on:click={confirmDelete}>Delete</button>
      </div>
    </div>
  </div>
{/if}

