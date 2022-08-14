<script lang="ts"> 
    import { materias_data } from "./stores"
    import MateriaInfo from "./MateriaInfo.svelte"
</script>

<div 
style:margin = "10px"
>
<div
    id="materia-selector"
>
    <h4
        style:padding="1%"
    >Materias seleccionadas</h4>
    <MateriaInfo bind:materias={$materias_data}/>
</div>
</div>

<style lang="sass">
    #materia-selector
        padding-inline: 2px
        border-inline: solid 1px rgba(255, 255, 255, 0.2)
</style>