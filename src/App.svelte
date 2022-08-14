<script lang="ts">
  import Test2 from './components/Test2.svelte'
  import Scheduler from './components/Scheduler/Scheduler.svelte'

  document.addEventListener('contextmenu', (event) => event.preventDefault())
  let currentIndex = 0
</script>

<div style:margin="1.5%">
  {#if currentIndex === 0}
    <Scheduler/>
  {:else if currentIndex === 1}
    <Test2 />
  {/if}
</div>

<style lang="sass">
  :global(html) 
    user-select: none
  :global(body::-webkit-scrollbar) 
    width: 0.6rem
  :global(body::-webkit-scrollbar-thumb)
    background-color: darkgrey
    outline: 1px solid slategrey
</style>