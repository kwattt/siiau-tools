<script lang="ts">
  import { ContentSwitcher, Switch } from 'carbon-components-svelte'

  import Selector from './components/Selector/Selector.svelte'
  import Test2 from './components/Test2.svelte'

  import Bullhorn from 'carbon-icons-svelte/lib/Bullhorn.svelte'
  import Analytics from 'carbon-icons-svelte/lib/Analytics.svelte'
  document.addEventListener('contextmenu', (event) => event.preventDefault())

  let currentIndex = 1
</script>

<div style:margin="1%">
  <ContentSwitcher bind:selectedIndex={currentIndex} size="xl">
    <Switch text="Uno" />
    <Switch text="Dos" />
  </ContentSwitcher>
</div>

<div style:margin="1.5%">
  {#if currentIndex === 0}
    <Selector />
  {:else if currentIndex === 1}
    <Test2 />
  {/if}
</div>


<style lang="sass">
  :global(html) 
    user-select: none
</style>