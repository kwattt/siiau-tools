<script lang="ts">
  import { ContentSwitcher, Switch } from 'carbon-components-svelte'
  import Test2 from './components/Test2.svelte'
  import Scheduler from './components/Scheduler/Scheduler.svelte'
  import CalendarHeatMap from "carbon-icons-svelte/lib/CalendarHeatMap.svelte";

  document.addEventListener('contextmenu', (event) => event.preventDefault())
  let currentIndex = 1
</script>

<div style:margin="1%">
  <ContentSwitcher bind:selectedIndex={currentIndex} size="xl">
    <Switch>
      <div style="display: flex; align-items: center;">
        <CalendarHeatMap style="margin-right: 0.5rem;" /> Scheduler
      </div>
    </Switch>
    <Switch text="Dos" />
  </ContentSwitcher>
</div>

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
    width: 1rem
  :global(body::-webkit-scrollbar-thumb)
    background-color: darkgrey
    outline: 1px solid slategrey
</style>