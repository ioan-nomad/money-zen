<script lang="ts">
  export let selectedIcon: string = '';
  export let onSelect: (icon: string) => void;

  let searchTerm = '';
  let selectedCategory = 'all';

  // Icon categories with associated emojis
  const iconCategories = {
    all: {
      name: 'All',
      icon: '🌟',
      icons: []
    },
    money: {
      name: 'Money',
      icon: '💰',
      icons: ['💰', '💸', '💵', '💴', '💶', '💷', '🪙', '💳', '💎', '🏦', '📊', '📈', '📉', '💹', '🧾', '💰']
    },
    food: {
      name: 'Food',
      icon: '🍔',
      icons: ['🍔', '🍕', '🌮', '🍝', '🍜', '🍲', '🥗', '🍗', '🥘', '🍱', '🍣', '🍤', '🍙', '🥟', '🥡', '☕', '🍺', '🍷', '🥤', '🧋']
    },
    transport: {
      name: 'Transport',
      icon: '🚗',
      icons: ['🚗', '🚙', '🚐', '🚚', '🚛', '🚜', '🏎️', '🚓', '🚑', '🚒', '🚌', '🚎', '🏍️', '🛵', '🚲', '🛴', '✈️', '🚁', '🚂', '🚄']
    },
    home: {
      name: 'Home',
      icon: '🏠',
      icons: ['🏠', '🏡', '🏘️', '🏢', '🏬', '🏭', '🏗️', '🏰', '🏯', '🛏️', '🛋️', '🪑', '🚿', '🛁', '🚽', '🧹', '🧽', '🧴', '🧻', '💡']
    },
    shopping: {
      name: 'Shopping',
      icon: '🛒',
      icons: ['🛒', '🛍️', '👕', '👔', '👗', '👠', '👟', '👑', '💄', '💅', '🛁', '🧴', '👜', '🎒', '💍', '⌚', '🕶️', '🧢', '👒', '🧤']
    },
    entertainment: {
      name: 'Entertainment',
      icon: '🎬',
      icons: ['🎬', '🎭', '🎪', '🎨', '🎯', '🎲', '🃏', '🎮', '🕹️', '🎸', '🎹', '🎤', '🎧', '📱', '💻', '📺', '📷', '📹', '🎵', '🎶']
    },
    health: {
      name: 'Health',
      icon: '💊',
      icons: ['💊', '🩺', '💉', '🌡️', '🩹', '🦷', '👁️', '🧠', '❤️', '🫁', '🦴', '💪', '🏃', '🧘', '🤸', '🏋️', '🚴', '🏊', '⚽', '🏀']
    },
    education: {
      name: 'Education',
      icon: '📚',
      icons: ['📚', '📖', '📝', '✏️', '🖊️', '🖍️', '📏', '📐', '🔬', '🧪', '🧬', '🔭', '📊', '📈', '🎓', '👨‍🎓', '👩‍🎓', '👨‍🏫', '👩‍🏫', '🏫']
    },
    travel: {
      name: 'Travel',
      icon: '✈️',
      icons: ['✈️', '🏖️', '🏝️', '🗺️', '🧳', '🎒', '📸', '🏨', '🗽', '🗿', '🏛️', '⛩️', '🕌', '⛪', '🏔️', '🏕️', '🎡', '🎢', '🎠', '🎪']
    },
    work: {
      name: 'Work',
      icon: '💼',
      icons: ['💼', '👔', '💻', '🖥️', '⌨️', '🖱️', '📱', '☎️', '📞', '📠', '🖨️', '📧', '📄', '📋', '📌', '📎', '🖇️', '📁', '📂', '🗂️']
    },
    utilities: {
      name: 'Utilities',
      icon: '⚡',
      icons: ['⚡', '💡', '🔌', '🔋', '💧', '🚿', '🌡️', '❄️', '🔥', '📡', '📶', '📳', '📴', '🔇', '🔊', '📢', '📣', '📯', '🔔', '🔕']
    },
    symbols: {
      name: 'Symbols',
      icon: '🔧',
      icons: ['🔧', '🔨', '⚒️', '🛠️', '⚙️', '🔩', '⛓️', '🧲', '🔫', '💣', '🧨', '🔪', '⚔️', '🛡️', '🚬', '⚰️', '⚱️', '🏺', '🔮', '📿']
    }
  };

  // Create flattened list of all icons for "All" category
  iconCategories.all.icons = Object.values(iconCategories)
    .filter(cat => cat.name !== 'All')
    .flatMap(cat => cat.icons);

  // Remove duplicates from all icons
  iconCategories.all.icons = [...new Set(iconCategories.all.icons)];

  $: filteredIcons = getFilteredIcons();

  function getFilteredIcons(): string[] {
    const categoryIcons = iconCategories[selectedCategory]?.icons || [];

    if (!searchTerm) {
      return categoryIcons;
    }

    // Simple search - could be enhanced with icon descriptions
    return categoryIcons.filter(icon => {
      // Search by the icon itself or by category keywords
      const searchLower = searchTerm.toLowerCase();

      // Find category that contains this icon
      const containingCategory = Object.values(iconCategories).find(cat =>
        cat.icons.includes(icon)
      );

      return icon.includes(searchTerm) ||
             (containingCategory?.name.toLowerCase().includes(searchLower));
    });
  }

  function selectIcon(icon: string) {
    selectedIcon = icon;
    onSelect(icon);
  }

  function clearSearch() {
    searchTerm = '';
  }
</script>

<div class="icon-picker">
  <!-- Search Input -->
  <div class="form-control mb-4">
    <div class="input-group">
      <input
        type="text"
        placeholder="Search icons..."
        class="input input-bordered input-sm flex-1"
        bind:value={searchTerm}
      />
      {#if searchTerm}
        <button class="btn btn-sm btn-square" on:click={clearSearch}>
          ✕
        </button>
      {:else}
        <span class="btn btn-sm btn-square btn-ghost">
          🔍
        </span>
      {/if}
    </div>
  </div>

  <!-- Category Tabs -->
  <div class="tabs tabs-boxed tabs-xs mb-4 overflow-x-auto">
    {#each Object.entries(iconCategories) as [key, category]}
      <button
        class="tab tab-xs"
        class:tab-active={selectedCategory === key}
        on:click={() => selectedCategory = key}
        title={category.name}
      >
        <span class="mr-1">{category.icon}</span>
        <span class="hidden sm:inline">{category.name}</span>
      </button>
    {/each}
  </div>

  <!-- Search Results Info -->
  {#if searchTerm}
    <div class="text-sm text-base-content/60 mb-3">
      {filteredIcons.length} icons found for "{searchTerm}"
    </div>
  {/if}

  <!-- Icons Grid -->
  <div class="grid grid-cols-8 sm:grid-cols-10 md:grid-cols-12 gap-2 max-h-64 overflow-y-auto">
    {#if filteredIcons.length === 0}
      <div class="col-span-full text-center py-8 text-base-content/50">
        {#if searchTerm}
          No icons found matching "{searchTerm}"
        {:else}
          No icons in this category
        {/if}
      </div>
    {:else}
      {#each filteredIcons as icon}
        <button
          type="button"
          class="btn btn-sm aspect-square p-1 text-xl hover:scale-110 transition-transform"
          class:btn-primary={selectedIcon === icon}
          class:btn-ghost={selectedIcon !== icon}
          on:click={() => selectIcon(icon)}
          title="Select {icon}"
        >
          {icon}
        </button>
      {/each}
    {/if}
  </div>

  <!-- Selected Icon Display -->
  {#if selectedIcon}
    <div class="mt-4 p-3 bg-base-200 rounded-lg">
      <div class="flex items-center gap-3">
        <span class="text-2xl">{selectedIcon}</span>
        <div>
          <div class="font-medium">Selected Icon</div>
          <div class="text-sm text-base-content/60">Click an icon above to change selection</div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .icon-picker {
    width: 100%;
  }

  .tabs {
    flex-wrap: nowrap;
  }

  .grid {
    scrollbar-width: thin;
  }

  .grid::-webkit-scrollbar {
    width: 6px;
  }

  .grid::-webkit-scrollbar-track {
    background: hsl(var(--b2));
    border-radius: 3px;
  }

  .grid::-webkit-scrollbar-thumb {
    background: hsl(var(--bc) / 0.3);
    border-radius: 3px;
  }

  .grid::-webkit-scrollbar-thumb:hover {
    background: hsl(var(--bc) / 0.5);
  }
</style>