const {performance, PerformanceObserver} = require('perf_hooks');

const perfObserver = new PerformanceObserver((items) => {
    console.log('\nPerformance status:')
    items.getEntries().forEach((entry) => console.log(`- ${entry.name} ${entry.duration}ms`))
    console.log();
});

perfObserver.observe({entryTypes: ["measure"], buffered: true})

const gamescanner = require("../lib");

{
    performance.mark('start');
    const games = gamescanner.amazon.games();
    performance.mark('end');
    console.log(JSON.stringify(games, null, 2));
    performance.measure("amazon", "start", "end");
}

{
    performance.mark('start');
    const games = gamescanner.blizzard.games();
    performance.mark('end');
    console.log(games);
    performance.measure("blizzard", "start", "end");
}

{
    performance.mark('start');
    const games = gamescanner.epicgames.games();
    performance.mark('end');
    console.log(games);
    performance.measure("epicgames", "start", "end");
}

{
    performance.mark('start');
    const games = gamescanner.gog.games();
    performance.mark('end');
    console.log(games);
    performance.measure("gog", "start", "end");
}

{
    performance.mark('start');
    const games = gamescanner.origin.games();
    performance.mark('end');
    console.log(games);
    performance.measure("origin", "start", "end");
}

{
    performance.mark('start');
    const games = gamescanner.riotgames.games();
    performance.mark('end');
    console.log(games);
    performance.measure("riotgames", "start", "end");
}

{
    performance.mark('start');
    const games = gamescanner.steam.games();
    performance.mark('end');
    console.log(games);
    performance.measure("steam", "start", "end");
}

{
    performance.mark('start');
    const games = gamescanner.ubisoft.games();
    performance.mark('end');
    console.log(games);
    performance.measure("ubisoft", "start", "end");
}
