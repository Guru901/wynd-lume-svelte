<script lang="ts">
  let globalWs = $state<WebSocket | null>(null);
  let input = $state<string>("");
  let messages = $state<string[]>([]);

  $effect(() => {
    (async () => {
      const ws = new WebSocket("ws://localhost:3000");

      ws.onopen = () => {
        globalWs = ws;
      };

      ws.onmessage = (event) => {
        messages.push(event.data);
      };
    })();

    return () => {
      if (globalWs) {
        globalWs.close();
      }
    };
  });
</script>

<main>
  <div style="padding: 24px">
    <h1>Wynd + Lume + Svelte minimal starting point</h1>
    <p>Send get_users to get the list of all the users from the database</p>
    <div>
      <div>
        <input type="text" placeholder="Type something..." bind:value={input} />
        <button onclick={() => globalWs?.send(input)}>Send</button>
      </div>
      {#each messages as message}
        <div>{message}</div>
      {/each}
    </div>
  </div>
</main>
