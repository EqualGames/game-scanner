const {performance, PerformanceObserver} = require('perf_hooks');

const perfObserver = new PerformanceObserver((items) => {
    console.log('\nPerformance status:')
    items.getEntries().forEach((entry) => console.log(`- ${entry.name} ${entry.duration}ms`))
    console.log();
});

perfObserver.observe({entryTypes: ["measure"], buffered: true})

const game_scanner = require("../lib");

try {
    performance.mark('start');
    const games = game_scanner.amazon.executable();
    performance.mark('end');
    console.log(games);
    performance.measure("amazon", "start", "end");
} catch (error) {
    console.error(error);
}

try {
    performance.mark('start');
    const games = game_scanner.blizzard.executable();
    performance.mark('end');
    console.log(games);
    performance.measure("blizzard", "start", "end");
} catch (error) {
    console.error(error);
}

try {
    performance.mark('start');
    const games = game_scanner.epicgames.executable();
    performance.mark('end');
    console.log(games);
    performance.measure("epicgames", "start", "end");
} catch (error) {
    console.error(error);
}

try {
    performance.mark('start');
    const games = game_scanner.gog.executable();
    performance.mark('end');
    console.log(games);
    performance.measure("gog", "start", "end");
} catch (error) {
    console.error(error);
}

try {
    performance.mark('start');
    const games = game_scanner.origin.executable();
    performance.mark('end');
    console.log(games);
    performance.measure("origin", "start", "end");
} catch (error) {
    console.error(error);
}

try {
    performance.mark('start');
    const games = game_scanner.riotgames.executable();
    performance.mark('end');
    console.log(games);
    performance.measure("riotgames", "start", "end");
} catch (error) {
    console.error(error);
}

try {
    performance.mark('start');
    const games = game_scanner.steam.executable();
    performance.mark('end');
    console.log(games);
    performance.measure("steam", "start", "end");
} catch (error) {
    console.error(error);
}

try {
    performance.mark('start');
    const games = game_scanner.ubisoft.executable();
    performance.mark('end');
    console.log(games);
    performance.measure("ubisoft", "start", "end");
} catch (error) {
    console.error(error);
}
