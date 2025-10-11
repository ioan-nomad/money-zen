<script lang="ts">
  import { onMount } from 'svelte';
  import { categoryStore } from '../ui/stores/categoryStore';
  import type { Category } from '../core/entities/Category';
  import CategoryForm from './components/CategoryForm.svelte';

  let error = '';

  // Create modal state
  let showCreateModal = false;

  // Edit modal state
  let showEditModal = false;
  let editCategory: Category | null = null;

  // Form submission state
  let isSubmitting = false;

  // Delete confirmation state
  let showDeleteModal = false;
  let deleteCategoryId = '';
  let deleteCategoryName = '';

  onMount(async () => {
    await loadCategories();
  });

  async function loadCategories() {
    try {
      await categoryStore.load();
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
      await categoryStore.create({
        name: event.detail.name,
        icon: event.detail.icon,
        category_type: event.detail.categoryType,
        color: event.detail.color
      });
      showCreateModal = false;
      await loadCategories();
    } catch (err) {
      error = String(err);
    } finally {
      isSubmitting = false;
    }
  }

  function handleEdit(category: Category) {
    editCategory = category;
    showEditModal = true;
  }

  async function handleEditSubmit(event: CustomEvent) {
    if (!editCategory) return;
    isSubmitting = true;
    try {
      await categoryStore.updateCategory(editCategory.id, {
        name: event.detail.name,
        icon: event.detail.icon,
        category_type: event.detail.categoryType,
        color: event.detail.color
      });
      showEditModal = false;
      editCategory = null;
      await loadCategories();
    } catch (err) {
      error = String(err);
    } finally {
      isSubmitting = false;
    }
  }

  function handleDelete(categoryId: string, categoryName: string) {
    deleteCategoryId = categoryId;
    deleteCategoryName = categoryName;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    try {
      await categoryStore.remove(deleteCategoryId);
      showDeleteModal = false;
      await loadCategories();
    } catch (err) {
      error = String(err);
    }
  }

  // Reactive subscriptions
  $: categories = $categoryStore;
  $: incomeCategories = categories.filter(cat => cat.category_type === 'income');
  $: expenseCategories = categories.filter(cat => cat.category_type === 'expense');
</script>

<div class="max-w-4xl mx-auto p-6">
  {#if error}
    <div class="alert alert-error mb-6">
      <span>Error: {error}</span>
    </div>
  {/if}

  <!-- Header with Create Button -->
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-3xl font-bold">Categories</h1>
    <button class="btn btn-primary" on:click={openCreateModal}>
      ➕ Create Category
    </button>
  </div>

  <!-- Income Categories -->
  <div class="card bg-base-100 shadow-xl mb-6">
    <div class="card-body">
      <h2 class="card-title text-success">💰 Income Categories ({incomeCategories.length})</h2>

      <div class="space-y-2">
        {#if incomeCategories.length === 0}
          <div class="text-center py-4 text-base-content/50">
            No income categories found.
          </div>
        {:else}
          {#each incomeCategories as category}
            <div class="flex items-center gap-2">
              <div class="flex-1">
                <div class="flex items-center justify-between p-3 bg-base-200 rounded-lg hover:bg-base-300 transition-colors">
                  <div class="flex items-center gap-3">
                    <div class="text-2xl">{category.icon}</div>
                    <div>
                      <span class="font-semibold text-base-content">{category.name}</span>
                      <div class="w-4 h-4 rounded-full inline-block ml-2" style="background-color: {category.color}"></div>
                    </div>
                  </div>
                  <div class="badge badge-success">income</div>
                </div>
              </div>
              <button
                class="btn btn-sm btn-ghost"
                on:click={() => handleEdit(category)}
                title="Edit category"
              >
                ✏️
              </button>
              <button
                class="btn btn-sm btn-error"
                on:click={() => handleDelete(category.id, category.name)}
                title="Delete category"
              >
                🗑️
              </button>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>

  <!-- Expense Categories -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title text-error">💸 Expense Categories ({expenseCategories.length})</h2>

      <div class="space-y-2">
        {#if expenseCategories.length === 0}
          <div class="text-center py-4 text-base-content/50">
            No expense categories found.
          </div>
        {:else}
          {#each expenseCategories as category}
            <div class="flex items-center gap-2">
              <div class="flex-1">
                <div class="flex items-center justify-between p-3 bg-base-200 rounded-lg hover:bg-base-300 transition-colors">
                  <div class="flex items-center gap-3">
                    <div class="text-2xl">{category.icon}</div>
                    <div>
                      <span class="font-semibold text-base-content">{category.name}</span>
                      <div class="w-4 h-4 rounded-full inline-block ml-2" style="background-color: {category.color}"></div>
                    </div>
                  </div>
                  <div class="badge badge-error">expense</div>
                </div>
              </div>
              <button
                class="btn btn-sm btn-ghost"
                on:click={() => handleEdit(category)}
                title="Edit category"
              >
                ✏️
              </button>
              <button
                class="btn btn-sm btn-error"
                on:click={() => handleDelete(category.id, category.name)}
                title="Delete category"
              >
                🗑️
              </button>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>

  <!-- Create Modal -->
  {#if showCreateModal}
    <div class="modal modal-open">
      <div class="modal-box max-w-2xl">
        <CategoryForm
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
        <CategoryForm
          category={editCategory}
          {isSubmitting}
          on:submit={handleEditSubmit}
          on:cancel={() => { showEditModal = false; editCategory = null; }}
        />
      </div>
    </div>
  {/if}

  <!-- Delete Confirmation Modal -->
  {#if showDeleteModal}
    <div class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">Delete Category</h3>
        <p class="mb-4">Are you sure you want to delete "<strong>{deleteCategoryName}</strong>"?</p>
        <p class="mb-4 text-warning">⚠️ This action cannot be undone. The category cannot be deleted if there are transactions using it.</p>

        <div class="modal-action">
          <button class="btn" on:click={() => showDeleteModal = false}>Cancel</button>
          <button class="btn btn-error" on:click={confirmDelete}>Delete</button>
        </div>
      </div>
    </div>
  {/if}
</div>