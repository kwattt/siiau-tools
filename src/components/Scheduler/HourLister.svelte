<script lang="ts">
  import type { hourFilter } from "./types";

  export let hourFilter : hourFilter = {}

  const validDays = ["Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sábado"]
  const validHours = [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]

  const toggleHour = (day: string, hour: number) => {
      if(day in hourFilter && hourFilter[day] && hourFilter[day].includes(hour)){
          hourFilter[day] = hourFilter[day].filter(h => h !== hour)
          if(hourFilter[day].length === 0){
              let nh = hourFilter
              delete nh[day]
              hourFilter = nh
          }
      } else {
          hourFilter[day] = hourFilter[day] ? hourFilter[day].concat(hour) : [hour]
      }
  }

</script>

<div
id="calendar-container"
style:margin = '1%'
>

<div
    style:display = 'flex'
>
    <div
        style:display = 'flex'
    >

    <div
    >
        <div  class="hour-left chour">&#8203;</div>
        {#each validHours as hour}
            <div 
                class="hour-left chour"
            >{hour}</div>
        {/each}
    </div>
    {#each validDays as day}
        <div>
            <div class="day chour">
                {day}
            </div>
                {#each validHours as hour}
                    <div class="hour chour">
                      {#if day in hourFilter && hourFilter[day].includes(hour)}
                      <div
                        on:click={() => toggleHour(day, hour)}
                        class="empty selectable selected"
                      >
                      &#8203;
                      </div>
                      {:else}
                      <div
                        on:click={() => toggleHour(day, hour)}
                        class="empty selectable"
                      >
                      &#8203;
                      </div>
                      {/if}
                    </div>
                {/each}
        </div>
    {/each}
    </div>
</div>
</div>

<style lang="sass">
  #calendar-container //. center
      display: flex
      flex-direction: column
      align-items: center
      justify-content: center

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
  
  .selectable
    cursor: pointer

  .empty:hover
    background-color: rgba(133, 133, 200, 0.4)

  .selected 
    background-color: rgb(160, 40, 40)

</style>