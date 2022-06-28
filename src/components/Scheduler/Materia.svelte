<script lang="ts">
  import { Button, DataTable, Link } from "carbon-components-svelte";
  import { Checkmark, Close } from "carbon-icons-svelte";
  import type { Materia } from "./types";

  export let materias : Materia[] 

  const formatMateriaData = () => {

    // iterate Materias and return an array of objects

    const materias_data = materias.map((materia, id) => {
      const {
        activo,
        nrc,
        nombre,
        creditos,
        horas,
        profesores,
        disponibles, 
        cupos,
        seccion,
        clave
      } = materia;

      return {
        id,
        nrc,
        seccion,
        cupos: `${cupos} | ${disponibles}`,
        horario: ``,
        profesores: ``,
        status: activo ? "Activo" : "Inactivo"
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
  headers={[
    { key: "nrc", value: "NRC" },
    { key: "seccion", value: "Seccion" },
    { key: "cupos", value: "Cupos" },
    { key: "horario", value: "Horario" },
    { key: "profesores", value: "Profesores" },
    { key: "status", value: "Status" },
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
              <td>{dias.join(" | ")}</td>
              <td>{edificio}</td>
              <td>{aula}</td>
            </tr>
          {/each}
        </tbody>
      </table>

    {/if}
    {#if cell.key === "status" && row.id in materias} 
      <Button 
        kind={materias[row.id].activo ? "primary" : "danger"}
        size="small"
        on:click={() => {
          let newmaterias = materias
          newmaterias[row.id].activo = !newmaterias[row.id].activo
          materias = newmaterias
        }}
        iconDescription={materias[row.id].activo ? "Activo" : "Inactivo"}
        icon={materias[row.id].activo ? Checkmark : Close}
      /> 
    {:else}
      {cell.value}
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
    user-select: all
</style>