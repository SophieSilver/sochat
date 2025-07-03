<script lang="ts">
  import type { Snippet } from "svelte";
  type ButtonVariant = "filled";

  interface Props {
    variant?: ButtonVariant;
    onclick?: () => void;
    children: Snippet;
  }

  type VariantToClass = {
    [k in ButtonVariant]: string;
  };

  const variantToClass: VariantToClass = {
    filled: "rounded-container filled",
  };

  let { variant = "filled", onclick = () => {}, children }: Props = $props();

  const variantClass = variantToClass[variant];
</script>

<button class={[variantClass]} {onclick}>
  {@render children()}
</button>

<style>
  button {
    cursor: pointer;
    user-select: none;
    -webkit-user-select: none;
    font-size: inherit;
    border: inherit;
    background-color: inherit;
    color: inherit;
    transition-duration: 100ms;
  }

  button:hover {
    transform: scale(1.05);
  }

  button:active {
    transform: scale(0.95);
  }

  .filled {
    color: var(--on-primary-accent);
    background-color: var(--primary-accent);
  }

  button.filled:hover {
    background-color: var(--primary-accent-tint0);
  }

  button.filled:active {
    background-color: var(--primary-accent-tint1);
  }
</style>
