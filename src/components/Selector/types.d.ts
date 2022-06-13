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