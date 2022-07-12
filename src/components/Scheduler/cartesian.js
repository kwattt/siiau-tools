export const cartesian = (max_iterations, ...a) => {
  let iterations = 0  
  return a.reduce((a, b) => 
      {
        if(iterations >= max_iterations) return a

        return a.flatMap(d => {
          iterations += 1
          return b.map(e => [d, e].flat())
        })
      }
    );
  }

export const parse_materias = (result, max_iterations, max_horarios, horas_muertas) => {
  let iterations = 0
  let horarios = 0
  let possible = []
  let STOP = false
  result.some(element => {
    let inex = {}
    let no_save = false
    if(STOP)
      return true

    if(!(element instanceof Array)){
      STOP = true
      return true
    }

    element.some(e => {
      iterations+=1;
      if(iterations > max_iterations){
        no_save = true
        STOP = true
        return true
      }

      if (no_save)
        return true
      e.horas.some(ses => {
        if(no_save)
          return true

        ses.dias.some(d => {
          if(no_save)
            return true
          if (!inex[d]) {
            inex[d] = {}
          }

          Array.from(new Array(ses.salida-ses.entrada), (x, i) => i + ses.entrada).some(hora => {
            if (!inex[d][hora]){
              inex[d][hora] = e.nrc
            }
            else {
              no_save = true
              return true
            }
          })
        })
      })

    // cupo

    })

    // horas muertas
    Object.entries(inex).forEach(([_, h]) => {
      let lastHour = -1 
      Object.entries(h).forEach(([hora, _]) => {
        if(lastHour == -1)
          lastHour = hora
        if(hora - lastHour > horas_muertas){
          no_save = true
        }
        lastHour = hora
      })
    })

    if(!no_save){
      if(horarios >= max_horarios){
        no_save = true
        STOP = true
      }
      else
        possible.push(element)
      horarios+=1
    }
  });
  return possible
}