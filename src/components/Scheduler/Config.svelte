<script lang="ts">
    import { save, open, message} from '@tauri-apps/api/dialog';
    import { readTextFile, writeTextFile } from "@tauri-apps/api/fs"
    import {Button} from "carbon-components-svelte"
    import {materias_data, last_update, periodo, materias_query, horario, horarios_generados, selected_item} from "./stores"

    let showSave = true

    const isValidJsonData = (data: any) => {
        if(
            data.hasOwnProperty('periodo') && 
            data.hasOwnProperty('last_update') &&
            data.hasOwnProperty('materias_data') &&
            data.hasOwnProperty('materias_query') &&
            data.hasOwnProperty('horario') && 
            data.hasOwnProperty('horarios_generados') &&
            data.hasOwnProperty('selected_item')
        )
            return true
        return false
    }

    const saveFile = async () => {
        const filePath = await save({
        filters: [{
            name: 'JSON',
            extensions: ['json']
        }]
        });

        if(filePath) {
            console.log("saving to", filePath)
            const data = JSON.stringify(
                {
                    periodo: $periodo,
                    last_update: $last_update,
                    materias_data: $materias_data,
                    materias_query: $materias_query,
                    horario: $horario,
                    selected_item: $selected_item,
                    horarios_generados: $horarios_generados
                }
            )
            console.log(data)
            await writeTextFile(filePath, data)
        }
    }

    const loadFile = async () => {
        let filePath = await open({
            multiple: false,
            filters: [{
                name: 'JSON',
                extensions: ['json']
            }]
        })
        if(filePath) {
            filePath = filePath as string
            const object = await readTextFile(filePath)
            let data = null
            try { 
                data = JSON.parse(object)
            }
            catch(e) {
                console.log
                await message("El archivo no es un JSON válido", "Error")
            }
            if(data)
            {
                if(isValidJsonData(data)){
                    $periodo = data.periodo
                    $last_update = data.last_update
                    $materias_data = data.materias_data
                    $materias_query = data.materias_query
                    $horario = data.horario
                    $horarios_generados = data.horarios_generados
                    $selected_item = data.selected_item
                }
                else {
                    message("El archivo no es válido")
                }
            }
        }
    }
</script>

<div 
    id="sch-config"
>
    <Button on:click={loadFile} kind="tertiary" size="small">Cargar configuración</Button>
    {#if showSave}
    <Button on:click={saveFile} kind="tertiary" size="small">Guardar configuración</Button>
    {/if}
</div>

<style lang="sass">
    #sch-config 
        padding-left: 5px
        margin: 1%
</style>