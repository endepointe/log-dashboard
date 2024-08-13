<script lang="ts">
    //import Graph from "graphology";
    //import Sigma from "sigma";


    type ParsedResult = {
        ip: String;
        date: String;
        proto: String;
    }

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
        const regex = /\s*(\d{2}-\d{2}-\d{4})/;
        const match = input.match(regex); 
        if (match) 
        {
            const [date] = match;
            return date;
        }
        console.error("invalid query format. Expected: date = mm-dd-yyyy");
        return null;
    }

    let query = String = "";
    function parse_query(input: String): ParsedResult | null
    {
        let result: ParsedResult = {};
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
                        result.proto = value; 
                        break;
                default: 
                    break;
            }
        });
        return result;
    }

    function handle_search()
    {
        let result : ParsedResult = parse_query(query);
        console.log("handle_search: ", result);
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
    }

    .content {
        grid-area: content;
        padding: 20px;
    }

    .search-container {
        margin-bottom: 20px;
    }
    .search-input {
        width: 100%;
        padding: 10px;
        font-size: 16px;
    }
</style>

<div class="dashboard">
    <header class="header">Dashboard header</header>
    <aside class="sidebar">sidebar navigation</aside>
    <main class="content">main content area
        <input 
            type="text" 
            class="search-input" 
            placeholder="Enter your search..."
            bind:value={query} 
            on:input={handle_search}>
    </main>
</div>
