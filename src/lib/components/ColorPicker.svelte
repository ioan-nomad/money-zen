<script lang="ts">
  export let selectedColor: string = '#3B82F6';
  export let onSelect: (color: string) => void;

  // Predefined color palette organized by categories
  const colorPalette = {
    primary: {
      name: 'Primary',
      colors: [
        '#3B82F6', // Blue
        '#6366F1', // Indigo
        '#8B5CF6', // Violet
        '#A855F7', // Purple
        '#D946EF', // Fuchsia
        '#EC4899', // Pink
      ]
    },
    success: {
      name: 'Success',
      colors: [
        '#10B981', // Emerald
        '#059669', // Emerald 600
        '#22C55E', // Green
        '#16A34A', // Green 600
        '#84CC16', // Lime
        '#65A30D', // Lime 600
      ]
    },
    warning: {
      name: 'Warning',
      colors: [
        '#F59E0B', // Amber
        '#D97706', // Amber 600
        '#EAB308', // Yellow
        '#CA8A04', // Yellow 600
        '#F97316', // Orange
        '#EA580C', // Orange 600
      ]
    },
    error: {
      name: 'Error',
      colors: [
        '#EF4444', // Red
        '#DC2626', // Red 600
        '#F87171', // Red 400
        '#B91C1C', // Red 700
        '#991B1B', // Red 800
        '#7F1D1D', // Red 900
      ]
    },
    neutral: {
      name: 'Neutral',
      colors: [
        '#6B7280', // Gray 500
        '#4B5563', // Gray 600
        '#374151', // Gray 700
        '#1F2937', // Gray 800
        '#111827', // Gray 900
        '#000000', // Black
      ]
    },
    accent: {
      name: 'Accent',
      colors: [
        '#06B6D4', // Cyan
        '#0891B2', // Cyan 600
        '#0284C7', // Sky 600
        '#0369A1', // Sky 700
        '#075985', // Sky 800
        '#0C4A6E', // Sky 900
      ]
    }
  };

  // Flattened array of all colors for easy access
  const allColors = Object.values(colorPalette).flatMap(category => category.colors);

  // Common colors for finance apps
  const popularColors = [
    '#EF4444', // Red (expenses)
    '#10B981', // Green (income)
    '#3B82F6', // Blue (default)
    '#F59E0B', // Amber (warning)
    '#8B5CF6', // Purple (savings)
    '#EC4899', // Pink (shopping)
    '#06B6D4', // Cyan (utilities)
    '#84CC16', // Lime (food)
  ];

  let customColor = selectedColor;
  let showCustomInput = false;

  function selectColor(color: string) {
    selectedColor = color;
    customColor = color;
    onSelect(color);
  }

  function toggleCustomInput() {
    showCustomInput = !showCustomInput;
    if (showCustomInput) {
      customColor = selectedColor;
    }
  }

  function handleCustomColorChange() {
    selectColor(customColor);
  }

  function isColorSelected(color: string): boolean {
    return selectedColor.toLowerCase() === color.toLowerCase();
  }

  // Get color name for accessibility
  function getColorName(color: string): string {
    // Simple color name mapping for common colors
    const colorNames: Record<string, string> = {
      '#EF4444': 'Red',
      '#F59E0B': 'Amber',
      '#10B981': 'Green',
      '#3B82F6': 'Blue',
      '#8B5CF6': 'Purple',
      '#EC4899': 'Pink',
      '#6B7280': 'Gray',
      '#06B6D4': 'Cyan',
      '#84CC16': 'Lime',
      '#000000': 'Black'
    };

    return colorNames[color] || color;
  }
</script>

<div class="color-picker">
  <!-- Popular Colors Section -->
  <div class="mb-4">
    <div class="text-sm font-medium text-base-content/70 mb-2">Popular Colors</div>
    <div class="flex flex-wrap gap-2">
      {#each popularColors as color}
        <button
          type="button"
          class="color-swatch"
          class:selected={isColorSelected(color)}
          style="background-color: {color}"
          on:click={() => selectColor(color)}
          title={getColorName(color)}
          aria-label="Select {getColorName(color)} color"
        >
          {#if isColorSelected(color)}
            <span class="checkmark">✓</span>
          {/if}
        </button>
      {/each}
    </div>
  </div>

  <!-- Color Palette by Category -->
  {#each Object.entries(colorPalette) as [key, category]}
    <div class="mb-4">
      <div class="text-sm font-medium text-base-content/70 mb-2">{category.name}</div>
      <div class="flex flex-wrap gap-2">
        {#each category.colors as color}
          <button
            type="button"
            class="color-swatch"
            class:selected={isColorSelected(color)}
            style="background-color: {color}"
            on:click={() => selectColor(color)}
            title={getColorName(color)}
            aria-label="Select {getColorName(color)} color"
          >
            {#if isColorSelected(color)}
              <span class="checkmark">✓</span>
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/each}

  <!-- Custom Color Input -->
  <div class="mt-6 pt-4 border-t border-base-300">
    <button
      type="button"
      class="btn btn-sm btn-ghost mb-3"
      on:click={toggleCustomInput}
    >
      {showCustomInput ? '▼' : '▶'} Custom Color
    </button>

    {#if showCustomInput}
      <div class="space-y-3">
        <div class="flex gap-2 items-center">
          <input
            type="color"
            class="color-input"
            bind:value={customColor}
            on:input={handleCustomColorChange}
            title="Pick custom color"
          />
          <input
            type="text"
            class="input input-bordered input-sm flex-1"
            bind:value={customColor}
            on:input={handleCustomColorChange}
            placeholder="#3B82F6"
            pattern="^#[0-9A-Fa-f]{6}$"
            title="Enter hex color code"
          />
        </div>
        <div class="text-xs text-base-content/60">
          Enter a hex color code (e.g., #3B82F6) or use the color picker
        </div>
      </div>
    {/if}
  </div>

  <!-- Selected Color Preview -->
  {#if selectedColor}
    <div class="mt-4 p-3 bg-base-200 rounded-lg">
      <div class="flex items-center gap-3">
        <div
          class="w-8 h-8 rounded-lg border-2 border-base-300 shadow-sm"
          style="background-color: {selectedColor}"
        ></div>
        <div>
          <div class="font-medium">Selected Color</div>
          <div class="text-sm text-base-content/60 font-mono">{selectedColor.toUpperCase()}</div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .color-picker {
    width: 100%;
  }

  .color-swatch {
    width: 2rem;
    height: 2rem;
    border-radius: 0.5rem;
    border: 2px solid transparent;
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  }

  .color-swatch:hover {
    transform: scale(1.1);
    border-color: hsl(var(--bc) / 0.3);
  }

  .color-swatch.selected {
    border-color: hsl(var(--bc));
    transform: scale(1.05);
    box-shadow: 0 4px 12px 0 rgba(0, 0, 0, 0.15);
  }

  .checkmark {
    color: white;
    font-weight: bold;
    font-size: 0.75rem;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
    pointer-events: none;
  }

  .color-input {
    width: 3rem;
    height: 2rem;
    border: 1px solid hsl(var(--bc) / 0.2);
    border-radius: 0.375rem;
    cursor: pointer;
    background: none;
  }

  .color-input::-webkit-color-swatch-wrapper {
    padding: 0;
    border: none;
    border-radius: 0.25rem;
  }

  .color-input::-webkit-color-swatch {
    border: none;
    border-radius: 0.25rem;
  }

  .color-input::-moz-color-swatch {
    border: none;
    border-radius: 0.25rem;
  }

  /* Dark colors need white checkmark, light colors need black */
  .color-swatch[style*="#000000"] .checkmark,
  .color-swatch[style*="#111827"] .checkmark,
  .color-swatch[style*="#1F2937"] .checkmark,
  .color-swatch[style*="#374151"] .checkmark,
  .color-swatch[style*="#4B5563"] .checkmark,
  .color-swatch[style*="#6B7280"] .checkmark,
  .color-swatch[style*="#B91C1C"] .checkmark,
  .color-swatch[style*="#991B1B"] .checkmark,
  .color-swatch[style*="#7F1D1D"] .checkmark,
  .color-swatch[style*="#075985"] .checkmark,
  .color-swatch[style*="#0C4A6E"] .checkmark {
    color: white;
  }
</style>