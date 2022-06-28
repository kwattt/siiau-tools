type idname = {
  id: string;
  name: string;
}

export type Ciclo = idname;
export type Centro = idname;

export type CiclerPayload = {
  ciclos: Ciclo[];
  centros: Centro[];
}

export type Sesion =  {
  entrada: number,
  salida: number,
  dias: string[],
  edificio: string,
  aula: string,
  periodo: string
}

export type Materia = {
  activo: bool,
  nrc: string,
  clave: string,
  nombre: string,
  seccion: string,
  creditos: number,
  cupos: number,
  disponibles: number,
  profesores: string[],
  horas: Sesion[]
}

export type MateriaPayload = {
  materias : Materia[];
}

export type MateriasData = {
  [key: string]: Materia[];
}

