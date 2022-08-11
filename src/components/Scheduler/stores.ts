import { writable, derived } from "svelte/store";
import type { MateriasData, Materia, Horarios } from "./types";

export const materias_data = writable<MateriasData>({});
export const last_update = writable<Date>(new Date());

export const selected = derived(
  materias_data,
  $materias_data => {
    // cicle materias 
    return Object.entries($materias_data).reduce((acc: MateriasData, [key, value]) => {
      value.forEach(materia => {
        if(materia.activo){
          if(!(key in acc))
            acc[key] = [];
          acc[key].push(materia);
        }
      })
      return acc;
    }, {})
  }
) 

type rdata = {
  ciclo: string
  centro: string
}

export const periodo = writable<rdata>({
  ciclo: "",
  centro: ""
})

export const materias_hided = writable<string[]>([])
export const materias_query = writable<string[]>([])
export const horario = writable<Materia[]>([])

export const horarios_generados = writable<Horarios>([])
export const selected_item = writable<number>(-1)