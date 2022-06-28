<script lang="ts">
import { onDestroy } from "svelte";

    let validDays = ["Lunes", "Martes", "Miercoles", "Jueves", "Viernes", "Sabado"]
    let validHours = [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]

    // create dict with valid days as keys
    let calendario = validDays.reduce((acc: any, day) => {
        acc[day] = validHours.reduce((acc2: any, hour) => {
                acc2[hour] = '|'
            return acc2
        }, {}) 

        return acc;
    }, {});

    let calendarContainer : HTMLDivElement
    let calendarClockBar: HTMLDivElement

    const updateBar = setInterval(() => {

        // get current time, hour minute and seconds normalized in 0-100
        let currentTime = new Date()
        let currentHour = currentTime.getHours()
        if(currentHour > 7 && currentHour < 23){
            let currentMinute = currentTime.getMinutes()
            let currentSecond = currentTime.getSeconds()

            let currentHourPercent = (currentHour * 60 * 60 + currentMinute * 60 + currentSecond) / (24 * 60 * 60) * 100
            currentHourPercent -= 5.5

            let calendarHeight = calendarContainer.clientHeight
            let currentHourHeight = currentHourPercent * calendarHeight / 100

            calendarClockBar.style.top = `${currentHourHeight}px`
        }

    }, 1000)

    onDestroy(() => clearInterval(updateBar))

</script>


<div
    id="calendar-container"
    style:margin = '1%'
>
    <div
        style:position = 'relative'
        style:background-color = 'rgba(111,111,111, 0.5)'
        bind:this={calendarClockBar}
        style:height = '4px'
        >
    </div>

    <div
        style:display = 'flex'
        bind:this={calendarContainer}
    >

        <div
        >
            <div class="hour-left">_</div>
            {#each validHours as hour}
                <div 
                    class="hour-left calendar-hour chour"
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
                                    ~{calendario[day][hour]}~
                                {:else}
                                    |
                                {/if}
                            {:else if calendario[day][hour] == '|'}
                                {'|'}
                            {:else}
                                {calendario[day][hour]}
                            {/if}
                        </div>
                    {/each}
            </div>
        {/each}
        </div>
    </div>
</div>
<style lang="sass">

    .chour 
        max-width: 15.4vw


    .hour-left
        padding-right: 3vw
        padding-block: 1vh

    .hour 
        padding-right: 6vw
        padding-block: 1vh

    .day
        padding-right: 6vw
        padding-block: 1vh
</style>