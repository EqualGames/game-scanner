const {performance, PerformanceObserver} = require('perf_hooks');

const perfObserver = new PerformanceObserver((items) => {
    console.log('\nPerformance status:')
    items.getEntries().forEach((entry) => console.log(`- ${entry.name} ${entry.duration}ms`))
    console.log();
});

perfObserver.observe({entryTypes: ["measure"], buffered: true})

const game_scanner = require("../lib");

{
    performance.mark('start');
    const games = game_scanner.amazon.games();
    performance.mark('end');
    console.log(JSON.stringify(games, null, 2));
    performance.measure("amazon", "start", "end");
}

{
    performance.mark('start');
    const games = game_scanner.blizzard.games();
    performance.mark('end');
    console.log(games);
    performance.measure("blizzard", "start", "end");
}

{
    performance.mark('start');
    const games = game_scanner.epicgames.games();
    performance.mark('end');
    console.log(games);
    performance.measure("epicgames", "start", "end");
}

{
    performance.mark('start');
    const games = game_scanner.gog.games();
    performance.mark('end');
    console.log(games);
    performance.measure("gog", "start", "end");
}

{
    performance.mark('start');
    const games = game_scanner.origin.games();
    performance.mark('end');
    console.log(games);
    performance.measure("origin", "start", "end");
}

{
    performance.mark('start');
    const games = game_scanner.riotgames.games();
    performance.mark('end');
    console.log(games);
    performance.measure("riotgames", "start", "end");
}

{
    performance.mark('start');
    const games = game_scanner.steam.games();
    performance.mark('end');
    console.log(games);
    performance.measure("steam", "start", "end");
}

{
    performance.mark('start');
    const games = game_scanner.ubisoft.games();
    performance.mark('end');
    console.log(games);
    performance.measure("ubisoft", "start", "end");
}
