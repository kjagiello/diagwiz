<script context="module">
  /**
   * @type {import('@sveltejs/kit').Load}
   */
  export async function load() {
    return {
      status: 307,
      redirect: "/playground",
    };
  }
</script>

<svelte:head>
  <title>Diagwiz</title>
</svelte:head>
