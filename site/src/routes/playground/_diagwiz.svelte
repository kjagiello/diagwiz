<script lang="ts">
  import initDiagwiz, { render } from "diagwiz";
  import { debounce, measureTime } from "$lib/utils";

  import { browser } from "$app/env";

  export let initialInput = null;

  let diagwiz = Promise.resolve(null);
  let stdout = "";
  let stderr = "";
  let executionTime = "0.000";

  const executeSync = (input: string) => {
    executionTime = measureTime(() => {
      try {
        stdout = render(input);
        stderr = "";
      } catch (e) {
        stderr = e;
      }
    }).toFixed(3);
    if (!stdout && !stderr) {
      stderr = "Warning: The given input did not produce any diagram.";
    }
  };
  const executeDebounced = debounce(executeSync);

  if (browser) {
    diagwiz = initDiagwiz()
      .then(() => {
        if (initialInput) {
          executeSync(initialInput);
        }
      })
      .catch((e) => {
        throw new Error(
          `Could not load diagwiz. Does your browser support WebAssembly?\n\nCause: ${e.stack}`
        );
      });
  }
</script>

{#await diagwiz}
  <slot name="loading" />
{:then _}
  <slot name="loaded" {stdout} {stderr} render={executeDebounced} {executionTime} />
{:catch error}
  <slot name="error" stderr={error} render={() => null} />
{/await}
