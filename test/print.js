const gamescan = require("../lib");

console.time("Scan Games");
const games = gamescan.games();
console.timeEnd("Scan Games");

console.log(games);
