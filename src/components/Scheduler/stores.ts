import { writable, derived } from "svelte/store";
import type { MateriasData } from "./types";

export const materias_data = writable<MateriasData>({});

export const selected = derived(
  materias_data,
  $materias_data => {
    // cicle materias 
    return Object.entries($materias_data)
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

export const materias_query = writable<string[]>([])