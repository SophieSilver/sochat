<script lang="ts">
  import { getChat } from "$lib/services/chat_service.svelte";
  import Button from "../Button.svelte";

  interface Props {
    maxLines?: number;
  }

  let { maxLines = 8 }: Props = $props();

  let input = $state("");
  const chat = getChat();

  function submit() {
    const trimmed = input.trim();
    if (trimmed.length === 0) {
      return;
    }
    chat.sendMessage({ text: trimmed });
    input = "";
  }

  function submitOnEnter(e: KeyboardEvent) {
    if (!e.shiftKey && e.code.toLowerCase() === "enter") {
      submit();
      e.preventDefault();
    }
  }
</script>

<div class="message_input flex-row">
  <div class="textfield_container flex-row rounded-container">
    <div
      class="textfield"
      role="textbox"
      tabindex="0"
      contenteditable="true"
      enterkeyhint="send"
      style="--text-field-max-lines:{maxLines}"
      onkeydown={submitOnEnter}
      bind:innerText={input}
    ></div>
  </div>
  <div class="button_container flex-column-reverse">
    <Button onclick={submit}>Send</Button>
  </div>
</div>

<style>
  .message_input {
    padding: 10px 5px;
    border-top: 1px solid var(--divider-color);
  }

  .message_input > * {
    margin-left: 5px;
    margin-right: 5px;
  }

  .textfield_container {
    flex-grow: 1;
    background-color: var(--bg-elevated-color0);
    transition-duration: 100ms;
  }

  .textfield {
    outline: none;
    flex: 1 1 auto;
    max-height: calc(var(--line-height) * 1em * var(--text-field-max-lines));
    overflow-y: scroll;
    overflow-x: hidden;
    overflow-wrap: anywhere;
    scroll-padding: calc(var(--line-height) * 1em);
  }
</style>
