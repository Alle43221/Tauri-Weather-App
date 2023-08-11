<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'

const displayedTemp = ref("");
const displayedCity = ref("");
const displayedIcon = ref("");
const visible = ref(false);
const favoriteCities = ref([]);

getCurrentLocationInformation();

async function getInformation() {
  /* TO-DO (1)
    Make a GET request (from Rust) to the Weather API and show the current temperature at these coordinates:
      - latitude 36.71
      - longitude -80.97
    (HINT: https://openweathermap.org/current)
  */

  /* TO-DO (2)
    Make a request to find the coordinates of Târgu Mureș and then show the current temperature there.
    (HINT: https://openweathermap.org/api/geocoding-api)
  */
}

function getCurrentLocationInformation() {
  /* TO-DO (3)
    Using Geolocation API (in Typescript) and GET requests in Rust, show the user's current location
    and the current temperature on the screen.
    (HINT: https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)
  */

  /* TO-DO (5)
    Update the current location of the user and the current weather conditions every minute.
    (HINT: use a separate thread in Rust that notifies the frontend to update the temperature;
           and location display; you will need to use events: https://tauri.app/v1/guides/features/events/)
  */

  /* TO-DO (9)
    Be creative! Expand upon your app and add whatever functionalities you want.
    A few ideas could be: holiday suggestions based on favorite city forecasts, dynamic
    background color based on the weather, storing the weather history and displaying a chart of
    the temperature changes, dynamic weather timeline, weather fetching in parallel for multiple cities etc.
  */
}
</script>

<template>
  <div class="container">
    <div v-if="visible">
      <h1>{{ displayedCity }}</h1>
      <h2>{{ displayedTemp }}</h2>
      <!-- TO-DO (4)
          Show an icon on the screen that suggests the weather conditions (rainy, sunny etc)
          (HINT: https://openweathermap.org/weather-conditions) 
      -->
    </div>
    <h2 v-else>Loading information...</h2>
    <!-- TO-DO (7)
        Make it so that the user can favorite a certain city that they searched.
        The user can then see all of their favorited cities as a list on their home screen.
        When selecting one of their favorite cities, they will be taken to a new page
        where they can see the details of that city.
        You can also make a self-updating temperature display for this page as well.
    -->
    <button v-for="city in favoriteCities">
      {{ city }}
    </button>
    <!-- TO-DO (8)
        Make it so that the user's favorite cities are persistent even once the app has been closed
        and reopened again.
        (HINT: write the array of cities to a file)
    -->
  </div>
</template>

<!-- TO-DO (6)
    Allow the user to search for a specific city and what the weather is like there.
    You can choose to use a separate component for this. 
-->

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .centered-icon {
    width: 50px;
    height: 50px;
  }
</style>
