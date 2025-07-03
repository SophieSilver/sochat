<script lang="ts">
  import ChatNav from "./nav_drawer/ChatNav.svelte";
</script>

<div class="nav_drawer flex-column">
  <ChatNav selected/>
  <ChatNav />
</div>

<style>
  .nav_drawer {
    background-color: var(--bg-elevated-color0);
    border-right: 1px solid var(--divider-color);
  }
</style>
