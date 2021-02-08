const {performance, PerformanceObserver} = require('perf_hooks');

const perfObserver = new PerformanceObserver((items) => {
    console.log('\nPerformance status:')
    items.getEntries().forEach((entry) => console.log(`- ${entry.name} ${entry.duration}ms`))
    console.log();
});

perfObserver.observe({entryTypes: ["measure"], buffered: true})

const gamescanner = require("../lib");


performance.mark('start');
const games = gamescanner.games();
performance.mark('end');
console.log(games);
performance.measure("games", "start", "end");
