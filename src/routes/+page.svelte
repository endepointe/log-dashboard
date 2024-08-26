<script lang="ts">
    import Dashboard from "$lib/dashboard.svelte";
</script>

<div class="container">
    <Dashboard/>
    <footer class="ip2locattr">
        This demo uses IP2Location.io <a href="https://www.ip2location.io">IP geolocation</a> web service.
    </footer>
</div>

<style>
:root {
font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
font-size: 16px;
line-height: 24px;
font-weight: 400;

color: #0f0f0f;
background-color: #f6f6f6;

font-synthesis: none;
text-rendering: optimizeLegibility;
-webkit-font-smoothing: antialiased;
-moz-osx-font-smoothing: grayscale;
-webkit-text-size-adjust: 100%;
}

.container {
    margin:0;
    padding:0;
}

.ip2locattr{
    position: absolute;
    bottom: 0;
    left:0;
    right:0;
    width: 100%;
    text-align: center;
}
@media (prefers-color-scheme: dark) {
:root {
  color: #f6f6f6;
  background-color: #2f2f2f;
}
}
</style>
