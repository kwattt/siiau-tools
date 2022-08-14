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
    id="top-scheduler"
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
  #top-scheduler
    border: solid 1px rgba(255, 255, 255, 0.2)

  .scheduler-element
    margin-top: 1.5%
</style>