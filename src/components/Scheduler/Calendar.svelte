<script lang="ts">
    import { onDestroy } from "svelte"
    import Materia from "./Materia.svelte"
    import {horario} from './stores'
``
    const validDays = ["Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sábado"]
    const validHours = [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]

    // create dict with valid days as keys
    
    const isBright = (color: string) => {
        let c = color.substring(1,7)
        let r = parseInt(c.substring(0,2), 16)
        let g = parseInt(c.substring(2,4), 16)
        let b = parseInt(c.substring(4,6), 16)
        return (r*0.299 + g*0.587 + b*0.114) > 186
    }

    const newCalendar = () => {
      return validDays.reduce((acc: any, day) => {
        acc[day] = validHours.reduce((acc2: any, hour) => {
                acc2[hour] = '|'
            return acc2
        }, {}) 
        return acc;
      }, {})
    }
    let calendario = newCalendar()

    $: {
      calendario = newCalendar()
      if($horario.length > 0){
        $horario.forEach(materia => {
          materia.horas.forEach(ses => {
            ses.dias.forEach(dia => {
              Array.from(new Array(ses.salida-ses.entrada), (x, i) => i + ses.entrada).some(hora => {
                calendario[dia][hora] = materia
              })
            })
          })
          })
        } 
    }

    let calendarContainer : HTMLDivElement
    let calendarClockBar: HTMLDivElement
    let startOffset: HTMLDivElement

    const updateBar = setInterval(() => {

        let currentTime = new Date()
        let currentHour = currentTime.getHours()
        if(currentHour >= 7 && currentHour < 23){
            let currentMinute = currentTime.getMinutes()
            let currentSecond = currentTime.getSeconds()

            // get current time in 0-100
            let currentTimeInPercent = ((currentHour-7) * 60 * 60 + currentMinute * 60 + currentSecond) / ((24-7) * 60 * 60) * 100

            let calendarHeight = calendarContainer.offsetHeight
            let currentHourHeight = currentTimeInPercent * calendarHeight / 100

            calendarClockBar.style.top = `${currentHourHeight+startOffset.clientHeight-4}px`
          calendarClockBar.style.visibility = 'visible'
        }
        else 
          calendarClockBar.style.visibility = 'hidden'
    }, 1000)

    onDestroy(() => clearInterval(updateBar))

  </script>
<div
  style:display='flex'
  style:flex-wrap='wrap'
>
    <div
    >
      <h4>Horario seleccionado</h4>
        {#key $horario}
          <Materia materias={$horario} hideCheck={true}/>
        {/key}
      </div>
    <div
      style:margin-left='6%'
    >
      <div
      style:position = 'relative'
      style:background-color = 'rgba(111,111,111, 0.5)'
      bind:this={calendarClockBar}
      style:height = '4px'
      style:margin = "-1%"
    >
    </div>
    <div
        id="calendar-container"
        style:margin = '1%'
    >
    
        <div
            style:display = 'flex'
            bind:this={calendarContainer}
        >
            <div
            >
                <div bind:this={startOffset} class="hour-left chour">&#8203;</div>
                {#each validHours as hour}
                    <div 
                        class="hour-left chour"
                    >{hour}</div>
                {/each}
            </div>
            <div
                style:display = 'flex'
            >
            {#each validDays as day}
                <div>
                    <div class="day chour">
                        {day}
                    </div>
                        {#each validHours as hour}
                            <div class="hour chour">
                                {#if
                                    (hour > 7 && calendario[day][hour] == calendario[day][hour-1])
                                }
                                    {#if
                                        (calendario[day][hour] != '|')
                                    }
                                      <div class="materia"
                                        style:background-color={`#${calendario[day][hour].color}`}
                                      >
                                        &#8203;
                                      </div>
                                    {:else}
                                      <div
                                        class="empty"
                                      >
                                        &#8203;
                                      </div>
                                    {/if}
                                {:else if calendario[day][hour] == '|'}
                                  <div
                                    class="empty"
                                  >
                                  &#8203;
                                  </div>
                                {:else}
                                    <div class="materia"
                                      style:background-color={`#${calendario[day][hour].color}`}
                                      style:color={isBright(`#${calendario[day][hour].color}`) ? 'black' : 'white'}
                                    > 
                                      {calendario[day][hour].nrc}
                                    </div>
                                {/if}
                            </div>
                        {/each}
                </div>
            {/each}
            </div>
        </div>
    </div>
  </div>
</div>


<style lang="sass">
  #calendar-container //. center
      display: flex
      flex-direction: column
      align-items: center
      justify-content: center
      width: 100%
      height: 100%

  .chour 
    max-width: 15.4vw

  .hour-left
    padding-right: 3vw
    border-bottom: 1px solid rgba(0, 0, 0, 0.3)
    padding: 1vh

  .hour 
    border-bottom: 1px solid rgba(0, 0, 0, 0.3)

  .day 
    padding-right: 5vw
    border-bottom: 1px solid rgba(0, 0, 0, 0.3)
    padding: 1vh

  .empty
    padding: 1vh

  .materia 
    padding: 1vh
    border-left: solid 0.2vh rgba(0, 0, 0, 0.3)
    user-select: all 
    border-right: solid 0.2vh rgba(0, 0, 0, 0.3)

</style>