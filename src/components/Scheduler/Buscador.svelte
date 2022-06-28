<script lang="ts">
    import {TextInput, Button } from "carbon-components-svelte"
    import {invoke} from "@tauri-apps/api"
    import type { MateriaPayload, MateriasData } from "./types"
    import { periodo, materias_data, materias_query } from './stores'
    let materia_select = ''

    const updateMateriasData = (materias : any) => {
        const dict = materias.reduce((acc: MateriasData, materia: any) => {
            if(!(materia.clave in acc))
                acc[materia.clave] = [materia]
            else
                acc[materia.clave] = [...acc[materia.clave], materia]
            return acc
        }, {})

        $materias_data = dict
    }

    const mouseOverDelete = (e : MouseEvent) => {
        const target = e.target as HTMLElement
        target.innerText = 'Eliminar'
    }

    const mouseOverLeave = (e : MouseEvent, name: string) => {
        const target = e.target as HTMLElement
        target.innerText = name
    }

    const addNewMateria = () => {
        if(materia_select && !$materias_query.includes(materia_select) && materia_select.length < 10){
            $materias_query = [...$materias_query, materia_select.toUpperCase()]
            materia_select = ''
            // get dom element with materia_input id
            const input = document.getElementById('materia_input') as HTMLInputElement
            input.focus()
        }
    }

    const buscarMaterias = () => {
        invoke('get_info_materias', {materias: $materias_query, ciclo: $periodo.ciclo, centro: $periodo.centro})
        .then((data: any) => {
            const res = data as MateriaPayload
            updateMateriasData(res.materias)
        })
    }

    const mouseClickDelete = (_ : MouseEvent, name: string) => {
        const newItems = $materias_query.filter(item => item !== name)
        $materias_query = newItems

        if(name in $materias_data){
            let newdata = $materias_data
            delete newdata[name]
            $materias_data = newdata
        }
    }
</script>

<div
    style:margin = "10px"
>
    <h4>Selector de materias</h4>
    <div
        style:display = 'flex'
    >
        <div
            style:width="50%"
        >
            <h5>Añadir</h5>
            <TextInput 
                id="materia_input"
                bind:value={materia_select}
                placeholder="Clave materia" 
                labelText="Clave" 
                autofocus={true}
                on:keydown={(e) => {
                    if(e.key === 'Enter'){
                        addNewMateria()
                    }
                }}
            />
            <Button
                style="margin-top: 10px"
                kind="secondary"
                size="small"
                on:click={addNewMateria}
            >
                Añadir 
            </Button>
            {#if $materias_query.length > 0}
            <Button 
                style="margin-top: 10px"
                size="small" 
                on:click={buscarMaterias}
            >
                Buscar
            </Button>
            {/if}

        </div>
        <div
            style:width="50%"
            style:margin-left="25px"
        >
            <h5>Materias seleccionadas</h5>
            <div
                style:margin-top="10px"
                style:display = 'grid'
                style:grid-gap="10px"
                style:grid-template-columns="repeat(4, 22.8%)"
            >
                {#if $materias_query.length > 0}
                    {#each $materias_query as item, id}
                        <div
                            on:mouseover={mouseOverDelete}
                            on:focus={() => {}}
                            on:mouseleave={(e) => mouseOverLeave(e, item)}
                            on:click={(e) => mouseClickDelete(e, item)}
                            class="materia"
                        >
                            {item}
                        </div>
                    {/each}
                {/if}
            </div>
        </div>
    </div>
</div>

<style lang="sass">
  .materia 
      background-color: #6f6f6f
      padding: 8%

  .materia:hover 
      background-color: #b27575
      cursor: pointer
</style>