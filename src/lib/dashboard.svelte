<script lang="ts">
    import LinePlot from "$lib/lineplot.svelte";
    console.log(LinePlot);
    import GlobalMap from "$lib/globalmap.svelte";

    import { invoke } from "@tauri-apps/api/tauri";

    type ParsedResult = {
        ip: String;
        date: String;
        proto: String;
    }
    let submit: Boolean = false;
    let query : String = "";

    function parse_ip(input: String): String | null 
    {
        const regex = /\s*(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})/;
        const match = input.match(regex); 
        if (match) 
        {
            const [ip] = match;
            return ip;
        }
        console.error("invalid query format. Expected: ip = [0-255].[0-255].[0-255].[0-255]");
        return null;
    }
    function parse_date(input: String): String | null 
    {
        const regex = /\s*(\d{4}-\d{2}-\d{2})/;
        const match = input.match(regex); 
        if (match) 
        {
            const [date] = match;
            return date;
        }
        console.error("invalid query format. Expected: date = yyyy-mm-dd");
        return null;
    }

    function parse_proto(input: String): String | null 
    {
        const regex = /^[A-Za-z]+$/; 
        const match = input.match(regex); 
        if (match) 
        {
            const [proto] = match;
            return proto;
        }
        console.error("invalid query format. Expected: proto = [A-Za-z]");
        return null;
    }

    function parse_query(input: String): ParsedResult | null
    {
        let result: ParsedResult | null = {};
        const parts = input.split(',');
        parts.forEach(part => {
            const [key, value] = part.split('=').map(s => s.trim());
            console.log(key, value);
            let res = null;
            switch (key) {
                case "ip": 
                    res = parse_ip(value);
                    if (res !== null) {
                        result.ip = value;
                    }
                    break;
                case "date": 
                    res = parse_date(value);
                    if (res !== null) {
                        result.date = value;
                    }
                    break;                
                case "proto": 
                    res = parse_proto(value);
                    if (res !== null) {
                        result.proto = value
                    }
                    break;
                default: 
                    break;
            }
        });
        return result;
    }

    function handle_input()
    {
        // give ui feedback to the user whether their input is valid
        // Example: ip = 127.0.0.1, dat= 07-02-2024. The 'dat=...' should be highlighted in red
    }

    async function submit_query()
    {
        let result : ParsedResult = parse_query(query);
        console.log(result);
        if (Object.keys(result).length > 0) {
           let data = await invoke('zeek_search', { query: JSON.stringify(result) })
           console.log(data);
        }
    }
</script>

<style>
    .dashboard {
        display: grid;
        grid-template-areas:
            "header header"
            "sidebar content";
        grid-template-rows: 60px 1fr;
        grid-template-columns: 200px 1fr;
        height: 100vh;
    }

    .header {
        grid-area: header;
        background-color: #333;
        color: white;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .sidebar {
        grid-area: sidebar;
        background-color: #f4f4f4;
        padding: 20px;
        color: black;
    }

    .content {
        grid-area: content;
        padding: 20px;
    }

    .search-container {
        display: flex;
    }
    .search-input {
        width: 80%;
        padding: 10px;
        font-size: 16px;
    }
    .search-button {
        width: 20%;
        padding: 10px;
        font-size: 16px;
        cursor: pointer;
        background-color: #333;
        color: white;
        border: none;
        border-radius: 4px;
    }
    .search-button:hover {
        background-color: #555;
    }
    .content-container {
        width: 90%;
    }
</style>

<div class="dashboard">
    <aside class="sidebar">
        sidebar navigation</aside>
    <main class="content">
        <div class="search-container">
            <input 
                type="text" 
                class="search-input" 
                placeholder="Enter your search..."
                bind:value={query} 
                on:input={handle_input}>
            <button class="search-button" on:click={submit_query}>Search</button>
        </div>
        <div class="content-container">
            <GlobalMap width={600} height={600}/> 
        </div>
    </main>
</div>
