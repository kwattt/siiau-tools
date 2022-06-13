<script lang="ts">
    import type { CiclerPayload, Ciclo, Centro } from "./types";
    import { onMount } from "svelte";
    import {invoke} from "@tauri-apps/api"
    import {Select, SelectItem} from "carbon-components-svelte";

    let centro = ''
    let ciclo = ''
    let ciclos_disponibles : Ciclo[] = []
    let centros_disponibles: Centro[] = []

    // on mount
    onMount(() => {
        invoke('get_info_cicler').then((data) => {
            if(data){
                const res = data as CiclerPayload
                centros_disponibles = res.centros
                ciclos_disponibles = res.ciclos
            }
            console.log(data)
        })
        .catch(err => {
            console.log(err)
        })
    })
</script>

<div
    style:display = 'flex'
>
<Select
    style="margin-inline: 10px"
    labelText="Centro"
>
    {#each centros_disponibles as {id, name}}
        <SelectItem value={id} text={name}/>
    {/each}
</Select>

<Select
    style="margin-inline: 10px"
    labelText="Ciclo"
>
    {#each ciclos_disponibles as {id, name}}
        <SelectItem value={id} text={name}/>
    {/each}
</Select>

</div>