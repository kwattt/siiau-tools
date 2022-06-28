<script lang="ts"> 
    import { materias_data } from "./stores"
    import MateriaInfo from "./MateriaInfo.svelte"
</script>

<div 
style:margin = "10px"
>
<div
    style:display = "flex"
    style:flex-direction = "column"
    style:margin-top = "10px"
>
    <MateriaInfo bind:materias={$materias_data}/>
</div>
</div>