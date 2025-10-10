<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import IconPicker from './IconPicker.svelte';
  import ColorPicker from './ColorPicker.svelte';
  import type { Tag } from '../database';

  // Component props
  export let tag: Tag | null = null; // null for create, Tag for edit
  export let isSubmitting = false;

  // Form fields
  let name = '';
  let selectedIcon = '🏷️';
  let selectedColor = '#3B82F6';

  // Validation states
  let nameError = '';
  let formTouched = false;

  // Event dispatcher for parent communication
  const dispatch = createEventDispatcher<{
    submit: {
      name: string;
      icon: string;
      color: string;
    };
    cancel: void;
  }>();

  // Initialize form with tag data if editing
  $: if (tag) {
    name = tag.name;
    selectedIcon = tag.icon;
    selectedColor = tag.color;
  }

  // Form validation
  $: validateForm();

  function validateForm() {
    // Reset errors
    nameError = '';

    // Name validation (2-30 chars as per requirements)
    if (formTouched && !name.trim()) {
      nameError = 'Tag name is required';
    } else if (formTouched && name.trim().length < 2) {
      nameError = 'Tag name must be at least 2 characters';
    } else if (formTouched && name.trim().length > 30) {
      nameError = 'Tag name must be less than 30 characters';
    }
  }

  // Check if form is valid
  $: isFormValid = name.trim().length >= 2 &&
                   name.trim().length <= 30 &&
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
    selectedIcon = '🏷️';
    selectedColor = '#3B82F6';
    nameError = '';
    formTouched = false;
  }

  // Get form title based on mode
  $: formTitle = tag ? 'Edit Tag' : 'Create New Tag';
  $: submitButtonText = tag ? 'Update Tag' : 'Create Tag';
</script>

<div class="tag-form">
  <h3 class="text-xl font-bold mb-6">{formTitle}</h3>

  <form on:submit|preventDefault={handleSubmit} class="space-y-6">
    <!-- Tag Name -->
    <div class="form-control">
      <label class="label" for="tag-name">
        <span class="label-text font-medium">Tag Name</span>
        <span class="label-text-alt text-error">*</span>
      </label>
      <input
        id="tag-name"
        type="text"
        class="input input-bordered"
        class:input-error={nameError}
        bind:value={name}
        on:input={handleNameInput}
        placeholder="Enter tag name"
        disabled={isSubmitting}
        maxlength="30"
        required
      />
      {#if nameError}
        <label class="label">
          <span class="label-text-alt text-error">{nameError}</span>
        </label>
      {/if}
      <label class="label">
        <span class="label-text-alt">{name.length}/30 characters</span>
      </label>
    </div>

    <!-- Icon Picker -->
    <div class="form-control">
      <label class="label">
        <span class="label-text font-medium">Tag Icon</span>
        <span class="label-text-alt text-error">*</span>
      </label>
      <div class="p-4 border border-base-300 rounded-lg bg-base-50">
        <IconPicker {selectedIcon} onSelect={handleIconSelect} />
      </div>
    </div>

    <!-- Color Picker -->
    <div class="form-control">
      <label class="label">
        <span class="label-text font-medium">Tag Color</span>
        <span class="label-text-alt text-error">*</span>
      </label>
      <div class="p-4 border border-base-300 rounded-lg bg-base-50">
        <ColorPicker {selectedColor} onSelect={handleColorSelect} />
      </div>
    </div>

    <!-- Live Preview Section -->
    <div class="form-control">
      <label class="label">
        <span class="label-text font-medium">Preview</span>
      </label>
      <div class="p-4 bg-base-200 rounded-lg">
        <div class="flex items-center gap-3">
          <div class="text-2xl">{selectedIcon}</div>
          <div class="flex items-center gap-2">
            <span class="font-semibold text-base-content">
              {name || 'Tag Name'}
            </span>
            <div class="w-4 h-4 rounded-full" style="background-color: {selectedColor}"></div>
          </div>
          <div class="badge badge-outline" style="border-color: {selectedColor}; color: {selectedColor}">
            tag
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
          {tag ? 'Updating...' : 'Creating...'}
        {:else}
          {submitButtonText}
        {/if}
      </button>
    </div>
  </form>
</div>

<style>
  .tag-form {
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
  .tag-form :global(.icon-picker),
  .tag-form :global(.color-picker) {
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