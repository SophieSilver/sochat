<script lang="ts">
  import { Chat, setChat } from "$lib/services/chat_service.svelte";
  import MessageTextInput from "./chat_window/MessageTextInput.svelte";
  import MessageWindow from "./chat_window/MessageWindow.svelte";

  setChat(new Chat());
</script>

<div class="chat_window flex-column">
  <MessageWindow />
  <MessageTextInput />
</div>

<style>
  .chat_window {
    flex-grow: 1;
  }
</style>
