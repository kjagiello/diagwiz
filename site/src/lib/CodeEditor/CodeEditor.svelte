<script lang="ts">
  import { onMount } from "svelte";
  import { EditorState, EditorView, basicSetup } from "@codemirror/basic-setup";
  import type { ViewUpdate } from "@codemirror/view";
  import { diagwiz } from "./lang";

  export let initialContent = "";
  export let content = initialContent;

  let container: HTMLElement;
  let initialized = false;

  onMount(() => {
    new EditorView({
      state: EditorState.create({
        doc: initialContent,
        extensions: [
          basicSetup,
          diagwiz(),
          EditorView.updateListener.of((vu: ViewUpdate) => {
            const doc = vu.state.doc;
            if (!initialized || vu.docChanged) {
              initialized = true;
              content = doc.toString();
            }
          }),
        ],
      }),
      parent: container,
    });
  });
</script>

<div bind:this={container} />
