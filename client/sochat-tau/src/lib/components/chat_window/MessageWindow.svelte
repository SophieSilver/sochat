<script lang="ts">
  import { getChat } from "$lib/services/chat_service.svelte";
  import MessageBubble from "./message_window/MessageBubble.svelte";

  let chat = getChat();
</script>

<div class="message_window flex-column-reverse">
  <div class="message_container flex-column">
    {#each chat.messages as message}
      <MessageBubble {message} />
    {/each}
  </div>
</div>

<style>
  .message_window {
    flex-basis: 0;
    flex-grow: 1;
    overflow-y: scroll;
  }

  .message_container {
    flex-grow: 1;
    justify-content: center;
    margin: 0 20px;
  }
</style>
