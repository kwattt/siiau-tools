<script lang='ts'>
    import type { CiclerPayload, Ciclo, Centro } from "./types"
    import {onMount} from 'svelte'
    import {invoke} from '@tauri-apps/api'
    import {Select, SelectItem } from "carbon-components-svelte"
    import {periodo} from './stores'

    let ciclos_disponibles : Ciclo[] = []
    let centros_disponibles: Centro[] = [] 

    onMount(() => {
        invoke('get_info_cicler').then((data) => {
            if(data){
                const res = data as CiclerPayload
                centros_disponibles = res.centros
                ciclos_disponibles = res.ciclos
            }
        })
        .catch(err => {
            console.error(err)
        })
    })

</script>

<div
    style:display = 'flex'
>
    <Select
        style="margin-inline: 10px"
        labelText="Centro"
        bind:selected={$periodo.centro}
    >
        <SelectItem
            value=""
            text="Centro"
        />
        {#each centros_disponibles as {id, name}}
            <SelectItem value={id} text={name}/>
        {/each}
    </Select>

    <Select
        style="margin-inline: 10px"
        labelText="Ciclo"
        bind:selected={$periodo.ciclo}
    >
    <SelectItem
        value=""
        text="Ciclo"
    />
        {#each ciclos_disponibles as {id, name}}
            <SelectItem value={id} text={name}/>
        {/each}
    </Select>
</div>