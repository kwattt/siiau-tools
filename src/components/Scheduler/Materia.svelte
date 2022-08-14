<script lang="ts">
  import { DataTable, Checkbox } from "carbon-components-svelte";
  import type { Materia } from "./types";
  export let materias : Materia[] 
  export let hideCheck = false

  const formatMateriaData = () => {
    const materias_data = materias.map((materia, id) => {
      const { 
        nombre,
        nrc,
        disponibles, 
        cupos,
        seccion,
      } = materia;

      return {
        id,
        nrc,
        seccion,
        cupos: `${cupos} | ${disponibles}`,
        horario: ``,
        profesores: ``,
        status: nombre
      };
    });

    return materias_data
  }

</script>

{#if materias.length > 0}
<div 
  class="data-table-materia"
>
<DataTable
  size={hideCheck ? "compact" : "short"}
  headers={[
    { key: "nrc", value: "NRC" },
    { key: "seccion", value: "Sección" },
    { key: "cupos", value: "Cupo" },
    { key: "horario", value: "Horario" },
    { key: "profesores", value: "Profesores" },
    { key: "status", value: hideCheck ? "Nombre" : "Incluir" },
  ]}
  rows={formatMateriaData()}
>
  <svelte:fragment slot="cell" let:row let:cell>
    {#if cell.key == "profesores" && row.id in materias}
      {#each materias[row.id].profesores as profesor}
        <div>{profesor}</div>
      {/each}
    {/if}
    {#if cell.key == "horario" && row.id in materias}
      <table 
        class="sesion-table"
      >
        <thead>
          <tr>
            <th>Hora</th>
            <th>Dias</th>
            <th>Edificio</th>
            <th>Aula</th>
          </tr>
        </thead>
        <tbody>
          {#each materias[row.id].horas as {entrada, salida, dias, edificio, aula, periodo}}
            <tr>
              <td>{entrada}-{salida}</td>
              <td>
                {#each dias as day}
                  <div>{day}</div>
                {/each}
              </td>
              <td>{edificio}</td>
              <td>{aula}</td>
            </tr>
          {/each}
        </tbody>
      </table>

    {/if}
    {#if cell.key === "status" && row.id in materias && !hideCheck} 
      <Checkbox 
        hideLabel
        bind:checked={materias[row.id].activo}
      /> 
    {:else}
      <div
        style:max-width="100px"
      >
        {cell.value}
      </div>
    {/if}
  </svelte:fragment>
</DataTable>
</div>
{/if}

<style lang="sass">
  .sesion-table th
    padding: 0.6rem
  .sesion-table td
    margin: 0px
    padding: 0.6rem

  :global(td)
    user-select: text
</style>