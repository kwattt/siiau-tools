<script lang="ts">
    import { Button, Checkbox, NumberInput, Modal } from "carbon-components-svelte";
    import {selected, horario, horarios_generados, selected_item, config} from './stores'
    import {cartesian, parse_materias} from './cartesian'
    import type { Materia } from "./types";
    import HourLister from "./HourLister.svelte";

    let toggle = true
    let totalTime = ""

    let modalFilter = false

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

                if($config.avaliable && nrc.disponibles < 1)
                    save = false
                
                // wrong hours check
                nrc.horas.some(ses => {
                    if(!save)
                        return true

                    if(!$config.wrong && (ses.entrada < 7 || ses.salida < 7))
                    {
                        save = false
                        return true
                    }

                    // block hours check

                    ses.dias.some(dia => {
                        if(!save)
                            return true
                        
                        Array.from(new Array(ses.salida-ses.entrada), (_, i) => i + ses.entrada).some(hora => {
                            if(!save)
                                return true

                            if(dia in $config.blockTime && $config.blockTime[dia].includes(hora))
                            {
                                save = false
                                return true
                            }
                        })
                    })

                })



                if(save)
                    acc2.push(nrc)

                return acc2
            }, [])
            acc.push(newMat)

            return acc
        }, [])

        let result = cartesian($config.maxIterations, ...to_cartesian)

        $selected_item = -1
        $horarios_generados = parse_materias(result, $config.maxIterations, $config.maxHorarios, $config.deathHours+1)
    }

</script>

<div
    id="pconfig"
    style:margin='10px'
>
<div
    class="ptitle"
    on:click={() => {toggle=!toggle}}
>
    <h4>
        Parametros
    </h4>
</div>
<div
    id="c-content"
>

{#if toggle}
<div
    class="parametros"
>
    <div class="dselector">
    <NumberInput hideSteppers class="dselector" bind:value={$config.maxIterations} label="Iteraciones máximas"/>
    </div>
        <div class="dselector">
        <NumberInput hideSteppers class="dselector" bind:value={$config.maxHorarios} label="Horarios máximos"/>
    </div>
        <div class="dselector">
        <NumberInput class="dselector" bind:value={$config.deathHours} label="Horas muertas"/>
    </div>

        <div class="dselector pselecter">
        <Checkbox
            labelText="Cupos disponibles"
            bind:checked={$config.avaliable}
        />
    </div>
    <div class="dselector pselecter">
        <Checkbox
            labelText="Horarios incorrectos"
            bind:checked={$config.wrong}
        />
    </div>
</div>
<div
    style:padding='5px'
    style:margin-bottom='10px'
>
    <Button
        size="small"
        kind="tertiary"
        on:click={()=>{modalFilter=true}}
    >
        Filtrar Horas
    </Button>
    <Modal
        primaryButtonText="Cerrar" 
        modalHeading="Bloquear horas" 
        bind:open={modalFilter} 
        on:open 
        on:close
        hasScrollingContent
        on:click:button--primary={() => {
            modalFilter = false
        }}
        size="sm"
    >
        <HourLister bind:hourFilter={$config.blockTime}/>
    </Modal>
</div>
{/if}

<div

    style:padding-left='5px'
>
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
</div>


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
    .parametros
        display: flex
        align-items: center

    .ptitle 
        cursor: pointer
        padding: 1%
        margin-bottom: 5px

    .ptitle:hover
        background-color: #6f6f6f
        
    #pconfig
        padding-inline: 2px
        border-inline: solid 1px rgba(255, 255, 255, 0.2)

    #c-content
        padding-inline: 0.5%

</style>