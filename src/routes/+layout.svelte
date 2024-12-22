<script lang="ts">
 import { isTauri } from "@tauri-apps/api/core";
 import "../app.css";

 let { children } = $props();
</script>

{@render children()}

<svelte:window
 on:contextmenu|preventDefault
 on:keydown={event => {
  if (!isTauri()) return;
  if (
   event.key === "F5" ||
   (event.ctrlKey && event.key === "r") ||
   (event.metaKey && event.key === "r")
  ) {
   event.preventDefault();
  }
 }}
/>
