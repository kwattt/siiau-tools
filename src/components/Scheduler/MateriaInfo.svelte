<script lang='ts'>
    import type {MateriasData} from "./types"
    import MateriaInfo from "./Materia.svelte"

    export let materias : MateriasData = {}
    let hideMateria : string[] = []


    const handleHideMateria = (materia: string) => {
        if(hideMateria.includes(materia)){
            hideMateria = hideMateria.filter(m => m !== materia)
        }else{
            hideMateria = [...hideMateria, materia]
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
    {#if !hideMateria.includes(materia)}
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

    .contenedor-materias::-webkit-scrollbar
        width: 1rem
    .contenedor-materias::-webkit-scrollbar-thumb
        background-color: darkgrey
        outline: 1px solid slategrey

</style>