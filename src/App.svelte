<script lang="ts">
    import ToolUI from "./lib/ToolUI.svelte";

    let currentTool = "fileFormatter";

    function updateTool(tool: string) {
        currentTool = tool;
    }
</script>

<main class="container">
  <div class="app_container">
    <div class="tools">
      <button class="button" on:click={() => updateTool("fileFormatter")}>File Formatter</button>
      <button class="button" on:click={() => updateTool("renderTableCreator")}>Render Table Creator</button>
    </div>
    <div class="toolUI">
      <ToolUI {currentTool}/>
    </div>
  </div>
</main>

<style>
    .app_container {
        display: flex;
    }

    .tools {
        flex: 1;
        /*background-color: lightblue;*/
        padding: 10px;
    }

    .toolUI {
        flex: 3;
        /*background-color: green;*/
        padding: 10px;
    }

    /*#fileFormatterButton {*/
    /*    background-color: transparent;*/
    /*    border: 1px solid white;*/
    /*    color: white;*/
    /*    padding: 10px 20px;*/
    /*    font-size: 16px;*/
    /*    cursor: pointer;*/
    /*    border-radius: 5px;*/
    /*}*/

    /*!* Optional: Add hover effect *!*/
    /*#fileFormatterButton:hover {*/
    /*    background-color: white;*/
    /*    color: black;*/
    /*}*/

</style>
