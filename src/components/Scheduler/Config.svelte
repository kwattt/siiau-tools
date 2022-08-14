<script lang="ts">
    import { save, open, message} from '@tauri-apps/api/dialog';
    import { readTextFile, writeTextFile } from "@tauri-apps/api/fs"
    import {Button} from "carbon-components-svelte"
    import {materias_data, last_update, periodo, materias_query, horario, horarios_generados, selected_item, config} from "./stores"

    let showSave = true
    const zeroPad = (num: number, places: number) => String(num).padStart(places, '0')

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
            const data = JSON.stringify(
                {
                    periodo: $periodo,
                    config: $config,
                    last_update: $last_update,
                    materias_data: $materias_data,
                    materias_query: $materias_query,
                    horario: $horario,
                    selected_item: $selected_item,
                    horarios_generados: $horarios_generados
                }
            )
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
                await message("El archivo no es un JSON válido", "Error")
            }
            if(data)
            {
                if(isValidJsonData(data)){
                    $periodo = data.periodo
                    if(data.hasOwnProperty('config'))
                        $config = data.config
                    $last_update = new Date(data.last_update)
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

    {#if $last_update}
        {#key $last_update}
            <div
                style:margin-top='5px'
            >
            Ultima actualizacion: {$last_update.getDay()}/{$last_update.getMonth()}/{$last_update.getFullYear()}
            A las {zeroPad($last_update.getHours(), 2)}:{zeroPad($last_update.getMinutes(), 2)}:{zeroPad($last_update.getSeconds(), 2)}
            </div>
        {/key}
    {/if}
</div>

<style lang="sass">
    #sch-config 
        margin-inline: 1%
</style>