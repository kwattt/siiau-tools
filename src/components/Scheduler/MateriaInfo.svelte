<script lang='ts'>
    import type {MateriasData} from "./types"
    import Materia from "./Materia.svelte"
    import {Modal} from 'carbon-components-svelte'

    export let materias : MateriasData = {}
    type Showing = {
        [key: string]: boolean
    }
    let isShowing : Showing = {}

    $: {
        Object.keys(materias).forEach(clave => {
            if (!(clave in isShowing)) {
                isShowing[clave] = false
            }
        })
        isShowing = Object.keys(isShowing).reduce((acc: Showing, curr: string) => {
            if(curr in materias)
                acc[curr] = isShowing[curr]
            return acc
        } , {})
    }

</script>

<div
    class="contenedor-materias"
    style:max-height = "50vh"
    style:overflow-y = "auto"
>
    <div>
        {#each Object.keys(materias) as materia}
        <div>
            <div
                on:click = {() => {
                    isShowing[materia] = true
                }}
                class="materia-info"
            >
                <h5>{materias[materia][0].clave} - {materias[materia][0].nombre} - {materias[materia][0].creditos}</h5>
            </div>
            <Modal 
                primaryButtonText="Cerrar" 
                modalHeading={materia} 
                bind:open={isShowing[materia]} 
                on:open 
                on:close
                hasScrollingContent
                on:click:button--primary={() => {
                    isShowing[materia] = false
                }}
                size="lg"
            >
                <Materia bind:materias={materias[materia]}/>
            </Modal>
        </div>
        {/each}
    </div>

</div>

<style lang="sass">
    .materia-info 
        cursor: pointer
        padding: 1%
        
    .materia-info:hover
        background-color: #6f6f6f

    :global(::-webkit-scrollbar)
        width: 0.6rem
    :global(::-webkit-scrollbar-thumb)
        background-color: darkgrey
        outline: 1px solid slategrey

</style>