<script lang="ts"> 
  import Cicler from './Cicler.svelte'
  import Buscador from "./Buscador.svelte"
  import MateriasLista from './MateriasLista.svelte'
  import Calendar from './Calendar.svelte'
  import Generador from './Generador.svelte'
  import Config from './Config.svelte';
</script>

<div
  id="scheduler"
>
  <div
    class="scheduler-element"
  >
    <Config/>
  </div>
  <div
    class="scheduler-element"
  >
    <Cicler/> 
  </div>
  <div
    class="scheduler-element"
  >
  <Buscador/>
  </div>
  <div
    class="scheduler-element"
  >
  <MateriasLista/>
  </div>
  <div
    class="scheduler-element"
  >
  <Generador/>
  </div>
  <div
    class="scheduler-element"
  >
  <Calendar/>
  </div>
</div>

<style lang="sass">
  .scheduler-element
    margin-top: 2.5%
</style>