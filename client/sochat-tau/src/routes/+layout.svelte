<script lang="ts">
  import "../styles/stylesheet.css";
  import { initLogForwarding } from "$lib/bindings/logging";
  import NavDrawer from "$lib/components/NavDrawer.svelte";

  $effect(() => {
    initLogForwarding();
  });
  let { children } = $props();

  function goBackOnEsc(e: KeyboardEvent) {
    if (e.code.toLowerCase() === "escape") {
      history.back();
      // WebviewWindow.getCurrent().setFocus();
    }
  }
</script>

<svelte:window
  onkeydown={goBackOnEsc}
  oncontextmenu={() => {
    alert("meow");
    return false;
  }}
/>
<div id="main_app" class="flex-row">
  <NavDrawer />
  {@render children()}
</div>

<style>
  #main_app {
    position: absolute;
    overflow: hidden;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;

    align-items: stretch;
  }

  :global(#main_app) {
    --bg-color: var(--bg-color-dark);
    --fg-color: var(--fg-color-dark);
    --bg-elevated-color0: var(--bg-elevated-color0-dark);
    --bg-elevated-color1: var(--bg-elevated-color1-dark);
    --bg-elevated-color2: var(--bg-elevated-color2-dark);
    --primary-accent: var(--primary-accent-dark);
    --primary-accent-tint0: var(--primary-accent-tint0-dark);
    --primary-accent-tint1: var(--primary-accent-tint1-dark);
    --on-primary-accent: var(--on-primary-accent-dark);
    --divider-color: var(--divider-color-dark);
    --bg-selected: var(--bg-selected-dark);
    --bg-selected-tint0: var(--bg-selected-tint0-dark);
    --bg-selected-tint1: var(--bg-selected-tint1-dark);
    background-color: var(--bg-color);
    color: var(--fg-color);
    line-height: var(--line-height);
    caret-color: var(--primary-accent);
  }
</style>
