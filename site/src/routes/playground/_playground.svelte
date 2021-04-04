<script lang="ts">
  import LZString from "lz-string";
  import { SvelteToast } from "@zerodevx/svelte-toast";

  import { browser } from "$app/env";
  import CodeEditor from "$lib/CodeEditor/CodeEditor.svelte";
  import { copyTextToClipboard, toastError } from "$lib/utils";

  export let initialContent = "";
  export let render;
  export let stdout = "";
  export let stderr = "";
  export let executionTime;

  let content;
  let errorCount = 0;

  function share(e: MouseEvent) {
    e.preventDefault();
    const url = [
      window.location.origin,
      "/playground/?input=",
      LZString.compressToEncodedURIComponent(content),
    ].join("");
    if (url.length > 2000) {
      toastError("URL too long to share");
      return false;
    }
    copyTextToClipboard(url);
    return false;
  }

  const copy = (text: string) => (e: MouseEvent) => {
    e.preventDefault();
    copyTextToClipboard(text);
    return false;
  };

  const select = (e: MouseEvent) => {
    e.preventDefault();
    var range = document.createRange();
    range.selectNode(e.currentTarget as Node);
    window.getSelection().removeAllRanges();
    window.getSelection().addRange(range);
  };

  $: errorCount = Math.min(1, stderr.length);
  $: render(content);
</script>

<div class="container">
  <CodeEditor {initialContent} bind:content />
  <div class="output">
    <div class="output-panel">
      <div class="output-header output-header-composite">
        <h3>Output</h3>
        <ul>
          <li><a href={`#`} on:click={copy(stdout)}>Copy</a></li>
          <li><a href={`#`} on:click={share}>Share</a></li>
          <li>Execution time: {executionTime} ms</li>
        </ul>
      </div>
      <pre class="output-content" on:dblclick={select}>{stdout}</pre>
    </div>
    <div class="output-panel">
      <h3 class="output-header output-header-composite">
        <h3 class:output-header-alert={errorCount}>Errors ({errorCount})</h3>
        <ul>
          <li><a href={`#`} on:click={copy(stderr)}>Copy</a></li>
        </ul>
      </h3>
      <pre class="output-content output-content-padded" on:dblclick={select}>{stderr}</pre>
    </div>
  </div>
  {#if browser}
    <SvelteToast options={{ reversed: true, intro: { y: 20 } }} />
  {/if}
</div>

<style>
  :root {
    --panel-header-text-color: #5f6377;
    --panel-header-bg-color: #d5d8e6;
    --panel-separator-color: lightgrey;
    --output-bg-color: #f3f4ff;
  }

  .container {
    height: 100%;
    display: grid;
    grid-template-columns: 0.5fr 1fr;
    grid-template-rows: 1fr;
    gap: 0px 0px;
    grid-template-areas: ". .";
  }

  .output {
    background-color: var(--output-bg-color);
    border-left: solid 1px var(--panel-separator-color);
    margin: 0;
    font-size: 0.8em;
    overflow: auto;

    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: 2.5fr 1fr;
    gap: 0px 0px;
    grid-template-areas:
      "."
      ".";
  }

  .output > .output-panel:not(:first-child) {
    border-top: solid 1px var(--panel-separator-color);
  }

  .output-header {
    margin: 0;
    padding: 0.2em 0.5em;
  }

  .output-content {
    margin: 0;
    padding: 0.2em;
  }

  .output-header,
  .output-header-composite h3 {
    font-weight: normal;
    font-size: 1em;
    color: var(--panel-header-text-color);
    user-select: none;
    line-height: 2em;
  }

  .output-header {
    background-color: var(--panel-header-bg-color);
  }

  .output-content-padded {
    padding: 0;
  }

  .output-header-alert.output-header-alert {
    font-weight: bold;
  }

  .output-header-composite {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }

  .output-header-composite h3 {
    margin: 0;
    padding: 0;
  }

  .output-header-composite ul {
    list-style: none;
    user-select: text;
  }

  .output-header-composite ul,
  .output-header-composite li {
    margin: 0;
    padding: 0;
  }

  .output-header-composite li {
    display: inline;
  }

  .output-header-composite ul > li:not(:first-child)::before {
    /* &bull; */
    content: "\2022";
    margin-left: 0.47em;
    margin-right: 0.75em;
  }

  .output-content {
    overflow: auto;
    padding: 0.5em;
  }

  :global(.cm-wrap) {
    height: 100%;
  }
  :global(.cm-scroller) {
    overflow: auto;
  }

  /* Remove the outline on focus */
  :global(.cm-editor.cm-focused) {
    outline: 0;
  }
</style>
