<script lang="ts">
  /* eslint-disable no-undef */
  /* https://github.com/sveltejs/eslint-plugin-svelte3/issues/109 */

  import LZString from "lz-string";
  import { browser } from "$app/env";
  import Diagwiz from "./_diagwiz.svelte";
  import Playground from "./_playground.svelte";

  function decompressURI(compressed) {
    try {
      return LZString.decompressFromEncodedURIComponent(compressed);
    } catch (_e) {
      return "";
    }
  }

  const defaultContent: string = `
alias ali = "Alice"

ali->Bob: "Hello!"
Bob->Bob: "(Bob thinks)"
Bob-->ali: "Hello back!"
`.trim();
  let initialContent: string;

  if (browser) {
    const urlParams = new URLSearchParams(window.location.search);
    const input = urlParams.get("input");

    initialContent = decompressURI(input) || defaultContent;
  }
</script>

<svelte:head>
  <title>Playground • Diagwiz</title>
  <meta name="description" content="Diagrams as code" />
</svelte:head>

<Diagwiz initialInput={initialContent}>
  <svelte:fragment slot="loaded" let:stdout let:stderr let:render let:executionTime>
    <Playground {initialContent} {stdout} {stderr} {render} {executionTime} />
  </svelte:fragment>
  <svelte:fragment slot="error" let:stderr>
    <Playground initialContent="" {stderr} />
  </svelte:fragment>
</Diagwiz>
