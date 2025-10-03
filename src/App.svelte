<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  let greetMsg = ''
  let name = ''

  async function greet() {
    greetMsg = await invoke('greet', { name })
  }
</script>

<main>
  <div class="container">
    <h1>💰 Welcome to MoneyZen</h1>

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
</style>