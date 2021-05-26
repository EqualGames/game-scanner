'use strict';

if (process.platform === 'win32') {
    module.exports = require("./native/windows.node");
} else if (process.platform === 'darwin') {
    module.exports = require("./native/darwin.node");
} else if (process.platform === 'linux') {
    module.exports = require("./native/linux.node");
} else {
    throw new Error('invalid platform');
}
