<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import DatabaseTest from './DatabaseTest.svelte'
  import Dashboard from './ui/pages/Dashboard.svelte'
  import Transactions from './ui/pages/Transactions.svelte'
  import Accounts from './ui/pages/Accounts.svelte'
  import Analytics from './ui/pages/Analytics.svelte'
  import Import from './ui/pages/Import.svelte'
  import Categories from './ui/pages/Categories.svelte'
  import Tags from './ui/pages/Tags.svelte'
  import DataSeeder from './lib/components/DataSeeder.svelte'
  import ToastContainer from './ui/components/common/ToastContainer.svelte'
  import LoadingOverlay from './ui/components/common/LoadingOverlay.svelte'

  let greetMsg = ''
  let name = ''
  let currentView: 'welcome' | 'dashboard' | 'transactions' | 'accounts' | 'categories' | 'tags' | 'analytics' | 'import' | 'test' | 'seeder' = 'dashboard'

  async function greet() {
    greetMsg = await invoke('greet', { name })
  }
</script>

<main class="min-h-screen flex items-center justify-center bg-base-200">
  <div class="container mx-auto px-4 py-12 text-center">
    <h1 class="text-6xl font-thin uppercase text-primary mb-8">
      💰 Welcome to MoneyZen
    </h1>

    <div class="tabs tabs-boxed justify-center mb-8 max-w-lg mx-auto">
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'dashboard'}
        on:click={() => currentView = 'dashboard'}
      >
        Dashboard
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'transactions'}
        on:click={() => currentView = 'transactions'}
      >
        Transactions
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'accounts'}
        on:click={() => currentView = 'accounts'}
      >
        Accounts
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'categories'}
        on:click={() => currentView = 'categories'}
      >
        🏷️ Categories
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'tags'}
        on:click={() => currentView = 'tags'}
      >
        🏷️ Tags
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'analytics'}
        on:click={() => currentView = 'analytics'}
      >
        Analytics
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'import'}
        on:click={() => currentView = 'import'}
      >
        Import
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'welcome'}
        on:click={() => currentView = 'welcome'}
      >
        Welcome
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'test'}
        on:click={() => currentView = 'test'}
      >
        Database Test
      </button>
      <button
        class="tab tab-lg"
        class:tab-active={currentView === 'seeder'}
        on:click={() => currentView = 'seeder'}
      >
        🌱 Test Data
      </button>
    </div>

    {#if currentView === 'dashboard'}
      <Dashboard />
    {:else if currentView === 'transactions'}
      <Transactions />
    {:else if currentView === 'accounts'}
      <Accounts />
    {:else if currentView === 'categories'}
      <Categories />
    {:else if currentView === 'tags'}
      <Tags />
    {:else if currentView === 'analytics'}
      <Analytics />
    {:else if currentView === 'import'}
      <Import />
    {:else if currentView === 'welcome'}
      <div class="welcome space-y-6">
        <form class="flex flex-col gap-2 max-w-xs mx-auto" on:submit|preventDefault={greet}>
          <input
            id="greet-input"
            type="text"
            placeholder="Enter a name..."
            class="input input-bordered input-primary w-full"
            bind:value={name}
          />
          <button type="submit" class="btn btn-primary">
            Greet
          </button>
        </form>

        {#if greetMsg}
          <p class="text-lg text-base-content">{greetMsg}</p>
        {/if}

        <div class="card bg-base-100 shadow-xl max-w-md mx-auto mt-8">
          <div class="card-body text-left">
            <h2 class="card-title text-2xl mb-4">🚀 Phase 1 Features</h2>
            <ul class="space-y-2">
              <li class="text-lg">✅ SQLite Database Integration</li>
              <li class="text-lg">✅ Account Management</li>
              <li class="text-lg">✅ Transaction Tracking</li>
              <li class="text-lg">✅ Category System</li>
              <li class="text-lg">⚡ Real-time Balance Updates</li>
            </ul>
          </div>
        </div>
      </div>
    {:else if currentView === 'test'}
      <DatabaseTest />
    {:else if currentView === 'seeder'}
      <DataSeeder />
    {/if}
  </div>
</main>

<!-- Global Components -->
<ToastContainer />
<LoadingOverlay />
