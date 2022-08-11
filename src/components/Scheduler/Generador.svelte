<script lang="ts">
    import { Button, Checkbox, NumberInput } from "carbon-components-svelte";
    import {selected, horario, horarios_generados, selected_item} from './stores'
    import {cartesian, parse_materias} from './cartesian'
    import type { Horarios, Materia } from "./types";

    let maxIterations = 10000 
    let maxHorarios = 100
    let deathHours = 2 
    let avaliable = false 
    let wrong = false 
    let toggle = false
    let totalTime = ""

    const randColor = () => {return Math.floor(Math.random()*16777215).toString(16)}

    const seleccionarHorario = (val: Materia[], index: number) => {
        $selected_item = index
        let withColors = val.map(materia => {
            return {
                ...materia,
                color: randColor()
            }
        })
        $horario = withColors 
    }

    const GenerarMaterias = async () => {
        // get current timestamp
        let beforeTime = new Date()

        await generarHorarios()

        let afterTime = new Date()

        totalTime = `${afterTime.getTime() - beforeTime.getTime()}ms.`
    }

    const generarHorarios = async () => 
    {

        if(Object.keys($selected).length <= 0)
            return

        let to_cartesian = Object.entries($selected).reduce((acc: any, [key, mat]) => {
            let newMat = mat.reduce((acc2: any, nrc) => {
                let save = true
                
                // cupos check

                if(avaliable && nrc.disponibles < 1)
                    save = false
                
                // wrong hours check
                nrc.horas.some(ses => {
                    if(!wrong && (ses.entrada < 7 || ses.salida < 7))
                    {
                        save = false
                        return true
                    }
                })

                if(save)
                    acc2.push(nrc)

                return acc2
            }, [])
            acc.push(newMat)

            return acc
        }, [])

        let result = cartesian(maxIterations, ...to_cartesian)

        $selected_item = -1
        $horarios_generados = parse_materias(result, maxIterations, maxHorarios, deathHours+1)
    }

</script>

<div
    style:margin='10px'
>
<div
    id="ptitle"
    on:click={() => {toggle=!toggle}}
>
    <h4>
        Parametros
    </h4>
</div>

{#if toggle}
<div
    id="parametros"
>
    <div class="dselector">
    <NumberInput class="dselector" bind:value={maxIterations} label="Iteraciones máximas"/>
</div>
    <div class="dselector">
    <NumberInput class="dselector" bind:value={maxHorarios} label="Horarios máximos"/>
</div>
    <div class="dselector">
    <NumberInput class="dselector" bind:value={deathHours} label="Horas muertas"/>
</div>

    <div class="dselector pselecter">
        <Checkbox
            labelText="Cupos disponibles"
            bind:checked={avaliable}
        />
    </div>

    <div class="dselector pselecter">
        <Checkbox
            labelText="Horarios incorrectos"
            bind:checked={wrong}
        />
    </div>

</div>
{/if}

<Button
kind="primary"
size="small"
on:click={async () => {await GenerarMaterias()}}
>
Generar
</Button>

{#if $horarios_generados.length > 0 && totalTime} 
    Tiempo de ejecución: {totalTime}
{/if}

<div
    id="resultados"
>
    {#each $horarios_generados as val, index}
        <div 
            class={index == $selected_item ? "result selected" : "result"}
            on:click={()=>seleccionarHorario(val, index)}
        >
            {index+1}
        </div>
    {/each}
</div>
</div>

<style lang="sass">
    .result
        font-size: 1rem
        margin: 2px
        cursor: pointer
        padding: 5px

    .result:hover
        background-color: rgba(255,255,255,0.5)
        color: #000

    .selected
        background-color: #f5f5f5
        color: #000
    .dselector
        padding-inline: 0.3rem

    #resultados
        display: flex
        overflow-x: auto
        padding: 5px   
    .pselecter
        margin-block: 1.2rem
    #parametros
        margin-bottom: 10px
        display: flex
        align-items: center
    #ptitle 
        cursor: pointer
        padding: 1%
        margin-bottom: 10px
    #ptitle:hover
        background-color: #6f6f6f
</style>