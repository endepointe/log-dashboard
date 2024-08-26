<script lang="ts">
import * as d3 from 'd3';
import * as db from '$lib/local/ip2loc.json';
import { onMount, tick} from 'svelte';

let svgElement;
let svg;
export let width : Number;
export let height: Number;
let projection;
let path;

let locations;
let ip: String = "";
let country_code: String = "";
let country_name: String = "";
let region_name: String = "";
let city_name: String = ""; 
let latitude: Number;
let longitude: Number;
let zip_code: String = "";
let time_zone: String = "";
let asn: String = "";
let as: String = "";
let is_proxy: Boolean;
let locs = [];
let counter = 0;

type LocationData = {
    ip: String;
    country_code: String;
    country_name: String;
    region_name: String;
    city_name: String; 
    latitude: Number;
    longitude: Number;
    zip_code: String;
    time_zone: String;
    asn: String;
    as: String;
    is_proxy: Boolean;
 
}
class LocationList {
    data: Array<LocationData>;
    
    constructor(items: Array<LocationData>) 
    {
        let list = []; 
        for (const [key,value] of items)
        {
            if (value.ip !== undefined) 
            {
                list.push(value)
            }
        }
        this.data = list;
    }
    display(): []
    {
        return this.data;
    }
}
onMount(async () => {

    locations = new LocationList(db.default.entries());
    locs = locations.data;


    svg = d3.select(svgElement)
        .attr('width', width)
        .attr('height', height);

    projection = d3.geoOrthographic()
        .scale(300)
        .translate([width / 2, height / 2])
        .precision(0.1);

    path = d3.geoPath().projection(projection);

    const graticule = d3.geoGraticule();

    svg.append('path')
        .datum(graticule())
        .attr('d', path)
        .attr('class', 'graticule')
        .style('fill', 'none')
        .style('stroke', '#ccc');

    const world = await d3.json('world.geojson');
        svg.append('path')
            .datum(world)
            .attr('d', path)
            .attr('class', 'land')
            .style('fill', '#69b3a2')
            .style('stroke', '#fff');

    for (let loc of locations.display())
    {
        await rotateToLocation(loc.latitude,loc.longitude);
    }
});
async function rotateToLocation(lat,lon) 
{
    const rotate = projection.rotate();
    const targetRotate = [-lon,-lat];
    const interpolate = d3.interpolate(rotate, targetRotate);
    await d3.transition()
        .duration(7000)
        .tween('rotate', () => {
            return (t) => {
                projection.rotate(interpolate(t));
                svg.selectAll('path').attr('d', path);
            }
        }).end(),
        // this is horrible... fine for now, this is hour 24 of being awake....
        console.log(locs[counter])
        ip = locs[counter]["ip"];
        country_code = locs[counter]["country_code"];
        country_name = locs[counter]["country_name"];
        region_name = locs[counter]["region_name"];
        city_name = locs[counter]["city_name"]; 
        latitude = locs[counter]["latitude"];
        longitude = locs[counter]["longitude"];
        zip_code = locs[counter]["zip_code"];
        time_zone = locs[counter]["time_zone"];
        asn = locs[counter]["asn"];
        as = locs[counter]["as"];
        is_proxy = locs[counter]["is_proxy"];
        counter = counter + 1;
        await tick();
    }
</script>

<style>
   .graticule {
      stroke-dasharray: 2, 2;
   }
   .row {
       display: flex;
       justify-content: center;
       align-items: center;
       align-self: stretch;
   }
   .col-2 {
       display: grid;
       grid-template-columns: 1fr 1fr;
       gap: 1em;
   }
   ul {
       list-style: none;
   }
</style>

<div class="row">
    <div class="col-2">
        <div>
            <svg bind:this={svgElement}></svg>
        </div>
        <div class="row">
            {#if ip !== ""}
            <ul>
            <li>ip: {ip}</li>
            <li>country_code: {country_code}</li>
            <li>country_name: {country_name}</li>
            <li>region_name: {region_name}</li>
            <li>city_name: {city_name}</li>
            <li>lat: {latitude}</li>
            <li>long: {longitude}</li>
            <li>zip: {zip_code}</li>
            <li>time_zone: {time_zone}</li>
            <li>asn: {asn}</li>
            <li>as: {as}</li>
            <li>is_proxy: {is_proxy}</li>
            </ul>
            {:else}
            <div>Loading data...</div>
            {/if}
        </div>
    </div>
</div>

