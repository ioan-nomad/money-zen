<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import IconPicker from './IconPicker.svelte';
  import ColorPicker from './ColorPicker.svelte';
  import type { Category } from '$lib/types';

  // Component props
  export let category: Category | null = null; // null for create, Category for edit
  export let isSubmitting = false;

  // Form fields
  let name = '';
  let categoryType: 'income' | 'expense' = 'expense';
  let selectedIcon = '💰';
  let selectedColor = '#3B82F6';

  // Validation states
  let nameError = '';
  let formTouched = false;

  // Event dispatcher for parent communication
  const dispatch = createEventDispatcher<{
    submit: {
      name: string;
      categoryType: 'income' | 'expense';
      icon: string;
      color: string;
    };
    cancel: void;
  }>();

  // Initialize form with category data if editing
  $: if (category) {
    name = category.name;
    categoryType = category.category_type;
    selectedIcon = category.icon;
    selectedColor = category.color;
  }

  // Form validation
  $: validateForm();

  function validateForm() {
    // Reset errors
    nameError = '';

    // Name validation
    if (formTouched && !name.trim()) {
      nameError = 'Category name is required';
    } else if (formTouched && name.trim().length < 2) {
      nameError = 'Category name must be at least 2 characters';
    } else if (formTouched && name.trim().length > 50) {
      nameError = 'Category name must be less than 50 characters';
    }
  }

  // Check if form is valid
  $: isFormValid = name.trim().length >= 2 &&
                   name.trim().length <= 50 &&
                   selectedIcon.trim() !== '' &&
                   selectedColor.trim() !== '';

  // Handle form submission
  function handleSubmit() {
    formTouched = true;
    validateForm();

    if (!isFormValid) {
      return;
    }

    dispatch('submit', {
      name: name.trim(),
      categoryType,
      icon: selectedIcon,
      color: selectedColor
    });
  }

  // Handle cancel
  function handleCancel() {
    dispatch('cancel');
  }

  // Handle icon selection
  function handleIconSelect(icon: string) {
    selectedIcon = icon;
  }

  // Handle color selection
  function handleColorSelect(color: string) {
    selectedColor = color;
  }

  // Handle name input to mark form as touched
  function handleNameInput() {
    if (!formTouched) {
      formTouched = true;
    }
  }

  // Reset form to initial state
  export function resetForm() {
    name = '';
    categoryType = 'expense';
    selectedIcon = '💰';
    selectedColor = '#3B82F6';
    nameError = '';
    formTouched = false;
  }

  // Get form title based on mode
  $: formTitle = category ? 'Edit Category' : 'Create New Category';
  $: submitButtonText = category ? 'Update Category' : 'Create Category';

  // Set default color based on category type
  $: if (!category && categoryType === 'income') {
    selectedColor = '#10B981'; // Green for income
  } else if (!category && categoryType === 'expense') {
    selectedColor = '#EF4444'; // Red for expense
  }
</script>

<div class="category-form">
  <h3 class="text-xl font-bold mb-6">{formTitle}</h3>

  <form on:submit|preventDefault={handleSubmit} class="space-y-6">
    <!-- Category Name -->
    <div class="form-control">
      <label class="label" for="category-name">
        <span class="label-text font-medium">Category Name</span>
        <span class="label-text-alt text-error">*</span>
      </label>
      <input
        id="category-name"
        type="text"
        class="input input-bordered"
        class:input-error={nameError}
        bind:value={name}
        on:input={handleNameInput}
        placeholder="Enter category name"
        disabled={isSubmitting}
        maxlength="50"
        required
      />
      {#if nameError}
        <label for="name-input" class="label">
          <span class="label-text-alt text-error">{nameError}</span>
        </label>
      {/if}
      <label for="name-input" class="label">
        <span class="label-text-alt">{name.length}/50 characters</span>
      </label>
    </div>

    <!-- Category Type -->
    <div class="form-control">
      <label class="label" for="category-type">
        <span class="label-text font-medium">Category Type</span>
        <span class="label-text-alt text-error">*</span>
      </label>
      <select
        id="category-type"
        class="select select-bordered"
        bind:value={categoryType}
        disabled={isSubmitting}
        required
      >
        <option value="expense">💸 Expense</option>
        <option value="income">💰 Income</option>
      </select>
      <label for="name-input" class="label">
        <span class="label-text-alt">
          {categoryType === 'income' ? 'Money coming in' : 'Money going out'}
        </span>
      </label>
    </div>

    <!-- Icon Picker -->
    <div class="form-control">
      <label for="name-input" class="label">
        <span class="label-text font-medium">Category Icon</span>
        <span class="label-text-alt text-error">*</span>
      </label>
      <div class="p-4 border border-base-300 rounded-lg bg-base-50">
        <IconPicker {selectedIcon} onSelect={handleIconSelect} />
      </div>
    </div>

    <!-- Color Picker -->
    <div class="form-control">
      <label for="name-input" class="label">
        <span class="label-text font-medium">Category Color</span>
        <span class="label-text-alt text-error">*</span>
      </label>
      <div class="p-4 border border-base-300 rounded-lg bg-base-50">
        <ColorPicker {selectedColor} onSelect={handleColorSelect} />
      </div>
    </div>

    <!-- Preview Section -->
    <div class="form-control">
      <label for="name-input" class="label">
        <span class="label-text font-medium">Preview</span>
      </label>
      <div class="p-4 bg-base-200 rounded-lg">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="text-2xl">{selectedIcon}</div>
            <div>
              <span class="font-semibold text-base-content">
                {name || 'Category Name'}
              </span>
              <div class="w-4 h-4 rounded-full inline-block ml-2" style="background-color: {selectedColor}"></div>
            </div>
          </div>
          <div class="badge" class:badge-success={categoryType === 'income'} class:badge-error={categoryType === 'expense'}>
            {categoryType}
          </div>
        </div>
      </div>
    </div>

    <!-- Form Actions -->
    <div class="flex gap-3 pt-4">
      <button
        type="button"
        class="btn btn-ghost flex-1"
        on:click={handleCancel}
        disabled={isSubmitting}
      >
        Cancel
      </button>
      <button
        type="submit"
        class="btn btn-primary flex-1"
        disabled={!isFormValid || isSubmitting}
      >
        {#if isSubmitting}
          <span class="loading loading-spinner loading-sm"></span>
          {category ? 'Updating...' : 'Creating...'}
        {:else}
          {submitButtonText}
        {/if}
      </button>
    </div>
  </form>
</div>

<style>
  .category-form {
    width: 100%;
    max-width: 500px;
  }

  .form-control {
    margin-bottom: 0; /* Override default spacing, we use space-y-6 */
  }

  /* Add subtle background to picker containers */
  .bg-base-50 {
    background-color: hsl(var(--b1) / 0.5);
  }

  /* Ensure proper spacing within pickers */
  .category-form :global(.icon-picker),
  .category-form :global(.color-picker) {
    margin: 0;
  }

  /* Loading spinner alignment */
  .loading {
    margin-right: 0.5rem;
  }

  /* Form validation styling */
  .input-error {
    border-color: hsl(var(--er));
  }

  .input-error:focus {
    border-color: hsl(var(--er));
    box-shadow: 0 0 0 2px hsl(var(--er) / 0.2);
  }
</style>