<script lang='ts'>
    import type {MateriasData} from "./types"
    import MateriaInfo from "./Materia.svelte"
    import {materias_hided} from './stores'
    export let materias : MateriasData = {}

    const handleHideMateria = (materia: string) => {
        if($materias_hided.includes(materia)){
            $materias_hided = $materias_hided.filter(m => m !== materia)
        }else{
            $materias_hided = [...$materias_hided, materia]
        }
    }

</script>

<div
    class="contenedor-materias"
    style:max-height = "50vh"
    style:overflow-y = "auto"
>

{#each Object.keys(materias) as materia}
    <div
        class="materia-info"
        on:click={() => handleHideMateria(materia)}
    >
        <h5>{materias[materia][0].clave} - {materias[materia][0].nombre} - {materias[materia][0].creditos}</h5>
    </div>
    {#if !$materias_hided.includes(materia)}
        <MateriaInfo bind:materias={materias[materia]}/>
    {/if}
{/each}

</div>

<style lang="sass">
    .materia-info 
        cursor: pointer
        padding: 1%
        
    .materia-info:hover
        background-color: #6f6f6f

    :global(::-webkit-scrollbar)
        width: 1rem
    :global(::-webkit-scrollbar-thumb)
        background-color: darkgrey
        outline: 1px solid slategrey

</style>