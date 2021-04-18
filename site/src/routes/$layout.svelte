<script lang="ts">
  import logo from "../assets/logo.svg";
  import "../app.css";
</script>

<div class="container">
  <header>
    <h1 class="logo"><img src={logo} alt="diagwiz logo" /> DIAGWIZ</h1>
    <h2 class="subname">playground<sup>beta</sup></h2>
  </header>
  <main class="container">
    <slot />
  </main>
</div>

<style>
  .container {
    height: 100%;
    max-height: 100%;
    display: flex;
    align-items: stretch;
    flex-direction: column;
    overflow: auto;
  }

  header {
    color: #fff;
    margin: 0;
    padding: 0.8em 1em;
    background: var(--secondary-bg-color);
    display: flex;
    align-items: center;
  }

  h1,
  h2 {
    margin: 0;
    padding: 0;
    font-weight: normal;
  }

  .logo {
    font-size: 1.9em;
    font-family: RedHatDisplay;
    font-weight: bold;
  }

  .logo img {
    width: 1.35em;
    margin-right: 0.15em;
    vertical-align: top;
  }

  .subname {
    font-size: 1em;
    margin-left: 0.5em;
    opacity: 0.75;
    margin-top: 8px;
  }

  .subname sup {
    font-size: 0.5em;
    opacity: 0.8;
  }
</style>
