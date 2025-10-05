<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import DatabaseTest from './DatabaseTest.svelte'

  let greetMsg = ''
  let name = ''
  let showDatabaseTest = false

  async function greet() {
    greetMsg = await invoke('greet', { name })
  }
</script>

<main>
  <div class="container">
    <h1>💰 Welcome to MoneyZen</h1>

    <div class="navigation">
      <button
        class:active={!showDatabaseTest}
        on:click={() => showDatabaseTest = false}
      >
        Welcome
      </button>
      <button
        class:active={showDatabaseTest}
        on:click={() => showDatabaseTest = true}
      >
        Database Test
      </button>
    </div>

    {#if !showDatabaseTest}
      <div class="welcome">
        <div class="row">
          <form class="form" on:submit|preventDefault={greet}>
            <input
              id="greet-input"
              placeholder="Enter a name..."
              bind:value={name}
            />
            <button type="submit">Greet</button>
          </form>
        </div>

        <p>{greetMsg}</p>

        <div class="features">
          <h2>🚀 Phase 1 Features</h2>
          <ul>
            <li>✅ SQLite Database Integration</li>
            <li>✅ Account Management</li>
            <li>✅ Transaction Tracking</li>
            <li>✅ Category System</li>
            <li>⚡ Real-time Balance Updates</li>
          </ul>
        </div>
      </div>
    {:else}
      <DatabaseTest />
    {/if}
  </div>
</main>

<style>
  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin: 1rem 0;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f23;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }

  input,
  button {
    outline: none;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4rem;
    font-weight: 100;
  }

  .navigation {
    display: flex;
    gap: 1rem;
    margin: 2rem 0;
    justify-content: center;
  }

  .navigation button {
    padding: 0.75rem 1.5rem;
    border: 2px solid #3B82F6;
    background: white;
    color: #3B82F6;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.25s;
  }

  .navigation button.active {
    background: #3B82F6;
    color: white;
  }

  .navigation button:hover {
    background: #3B82F6;
    color: white;
  }

  .features {
    margin: 2rem 0;
    text-align: left;
  }

  .features ul {
    list-style: none;
    padding: 0;
  }

  .features li {
    padding: 0.5rem 0;
    font-size: 1.1rem;
  }
</style>