<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, emit } from '@tauri-apps/api/event'


const displayedTemp = ref(0);
const displayedCity = ref("Targu Mures");
const displayedIcon = ref("");
const visible = ref(false);
const favoriteCities = ref([]);
const currentLatitude= ref(0);
const currentLongitude =ref(0);
const searchedLocation=ref("");
const computerLatitude= ref(0);
const computerLongitude= ref(0);
const computerLocation=ref("");

getFavorites();
getCurrentLocationInformation();
getInformation();
getUpdated();

async function getFavorites() {
  let mnv=await invoke ("getcontent");
  favoriteCities.value=mnv.split(",");  
  //console.log(getFavorites);
}

async function getInformation() {
  visible.value=false;
  let localCity;
  if(searchedLocation.value!=""){
    localCity=searchedLocation.value;
  }else localCity=displayedCity.value;
  let Coords : [number, number]=await invoke("get_coord_by_city", {city:localCity});
  //console.log(Coords);
  currentLatitude.value=Coords[0];
  currentLongitude.value=Coords[1];
  
  //displayedTemp.value = await invoke("get_temperature", {latitude: 36.71, longitude:-80.97})
  let mnv: string =await invoke("get_temperature", {latitude: currentLatitude.value, longitude: currentLongitude.value})
  console.log(mnv);
  displayedTemp.value = parseInt(mnv, 10);
  displayedIcon.value= await invoke("get_icon", {latitude: currentLatitude.value, longitude: currentLongitude.value})
  displayedCity.value=localCity;
  visible.value=true;
}

async function useCurrentLocation() {
  visible.value=false;
  currentLatitude.value=computerLatitude.value;
  currentLongitude.value=computerLongitude.value;
  displayedCity.value=computerLocation.value;
  displayedTemp.value = await invoke("get_temperature", {latitude: currentLatitude.value, longitude: currentLongitude.value})
  displayedIcon.value= await invoke("get_icon", {latitude: currentLatitude.value, longitude: currentLongitude.value})
  visible.value=true;
}

async function getUpdated(){
  await invoke('init_process');
    const unlisten=await listen('event-name', (event)=>{
      getCurrentLocationInformation();
      getInformation();
      console.log("Updated info");
    })
}

async function getCurrentLocationInformation() {
    let info = await fetch("https://location.services.mozilla.com/v1/geolocate?key=test");
    let infoProcessed = await info.json();
    computerLatitude.value= infoProcessed["location"]["lat"];
    computerLongitude.value= infoProcessed["location"]["lng"];
    computerLocation.value= await invoke ("get_city_by_coord", {latitude: computerLatitude.value, longitude: computerLongitude.value});

  /* TO-DO (9)
    Be creative! Expand upon your app and add whatever functionalities you want.
    A few ideas could be: holiday suggestions based on favorite city forecasts, dynamic
    background color based on the weather, storing the weather history and displaying a chart of
    the temperature changes, dynamic weather timeline, weather fetching in parallel for multiple cities etc.
  */
}

async function addToFavorites() {
  if(favoriteCities.value.includes(displayedCity.value)==false){
    favoriteCities.value.push(displayedCity.value); 
    //console.log(favoriteCities.toString());
    await invoke("writetofile", {text: favoriteCities.value.toString()});
    getFavorites();
  }
}

async function removeLocation(city: string) {
  if(favoriteCities.value.includes(city)==true){
    favoriteCities.value=favoriteCities.value.filter((e, i) => e !== city);
    //console.log(favoriteCities.toString());
    await invoke("writetofile", {text: favoriteCities.value.toString()});
    getFavorites();
  }
}

async function useSelectedLocation(city:string) {
  searchedLocation.value=city;
  getInformation();
}

</script>

<template>
  <div class="container">

    <div v-if="visible">
      <div style="display: flex; flex-direction: row; justify-content: center; align-items: center;">
      <h1>{{ displayedCity }}</h1>
      <button  @click="addToFavorites()" class="stared_button">&#11088;</button>
      </div>
      <h2>{{ displayedTemp }} &degC</h2>
      <div class="imageContainer">
      <img :src="displayedIcon">
      </div>
    </div>
    <h2 v-else>Loading information...</h2>
    <!-- TO-DO (7)
        Make it so that the user can favorite a certain city that they searched.
        The user can then see all of their favorited cities as a list on their home screen.
        When selecting one of their favorite cities, they will be taken to a new page
        where they can see the details of that city.
        You can also make a self-updating temperature display for this page as well.
    -->
    <br>
    <input v-model="searchedLocation" placeholder="City">
    <br>
    <button @click="getInformation()">Search weather</button>

    <div>
    <p>Current Position: {{computerLocation}} (lat:{{ computerLatitude }}; lng:{{computerLongitude}})</p>
    <button  @click="useCurrentLocation()">Use Current Location</button>
    </div>

    <br>
    <h2>Favorite Locations:</h2>
    <div v-for="city in favoriteCities" @click="getInformation()">
      <button @click="useSelectedLocation(city)">{{ city }}</button>
      <button @click="removeLocation(city)">X</button>
  </div>
  </div>
</template>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
  .stared_button{
    height: 30px;
    width: 30px;
    padding: 0;
  }

  .centered-icon {
    width: 50px;
    height: 50px;
  }
  .imageContainer{
    height: 100px;
    background-color: black;
  }
</style>
