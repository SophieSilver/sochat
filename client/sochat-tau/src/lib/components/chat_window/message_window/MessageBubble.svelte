<script lang="ts">
  import type { Message } from "$lib/model/message";

  interface Props {
    message: Message;
  }

  let { message }: Props = $props();
</script>

<div class="message_bubble flex-column">
  <div class="text_container flex-column rounded-container">
    {message.text}
  </div>
</div>

<style>
  .message_bubble {
    align-items: start;
    margin: 2.5px 0;
  }
  .text_container {
    background-color: var(--bg-elevated-color1);
    border-end-start-radius: 0.8em;
    white-space: pre-wrap;
  }
</style>
