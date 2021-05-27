const {performance, PerformanceObserver} = require('perf_hooks');

const perfObserver = new PerformanceObserver((items) => {
    console.log('\nPerformance status:')
    items.getEntries().forEach((entry) => console.log(`- ${entry.name} ${entry.duration}ms`))
    console.log();
});

perfObserver.observe({entryTypes: ["measure"], buffered: true})

const game_scanner = require("../lib");

async function main() {
    performance.mark('start');
    const games = game_scanner.steam.games();
    performance.mark('end');

    console.log(`${games.length} Steam game(s)`);

    performance.measure("games", "start", "end");

    const game = games[0];

    performance.mark('start');
    game_scanner.manager.launch_game(game);
    performance.mark('end');

    console.log(`Launching: ${game.name}`);

    performance.measure("launch_game", "start", "end");

    await delay();

    performance.mark('start');
    let processes = game_scanner.manager.get_processes(game);
    performance.mark('end');

    console.log('Processes: ', processes);

    performance.measure("game_processes", "start", "end");

    await delay();

    performance.mark('start');
    game_scanner.manager.close_game(game);
    performance.mark('end');

    performance.measure("close_game", "start", "end");
}

async function delay(ms = 15000) {
    return new Promise((resolve) => {
        setTimeout(() => resolve(), ms)
    })
}

main().catch(console.log)
