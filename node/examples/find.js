const {performance, PerformanceObserver} = require('perf_hooks');

const perfObserver = new PerformanceObserver((items) => {
    console.log('\nPerformance status:')
    items.getEntries().forEach((entry) => console.log(`- ${entry.name} ${entry.duration}ms`))
    console.log();
});

perfObserver.observe({entryTypes: ["measure"], buffered: true})

const game_scanner = require("../lib");

let games;

{
    performance.mark('start');
    games = game_scanner.steam.games();
    performance.mark('end');
    console.log(games);
    performance.measure("games", "start", "end");
}

{
    performance.mark('start');
    const game = game_scanner.steam.find(games[0].id);
    performance.mark('end');
    console.log(game);
    performance.measure("find", "start", "end");
}
