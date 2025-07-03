<script lang="ts">
  interface Props {
    selected?: boolean;
  }

  let { selected = false }: Props = $props();
</script>

<button class={["chat_nav", "flex-row", { selected }]} role="menuitem">meow</button>

<style>
  .chat_nav {
    cursor: pointer;
    user-select: none;
    -webkit-user-select: none;
    font-size: inherit;
    border: inherit;
    background-color: inherit;
    color: inherit;
    padding: 10px;
    transition-duration: 100ms;
  }
  .chat_nav:hover {
    background-color: var(--bg-elevated-color1);
  }
  .chat_nav:active {
    background-color: var(--bg-elevated-color2);
  }
  .chat_nav.selected {
    background-color: var(--bg-selected);
  }
  .chat_nav.selected:hover {
    background-color: var(--bg-selected-tint0);
  }
  .chat_nav.selected:active {
    background-color: var(--bg-selected-tint1);
  }

  .chat_nav:focus {
    /* Solves a bug where other items render behind the item */
    z-index: 1;
  }
</style>
