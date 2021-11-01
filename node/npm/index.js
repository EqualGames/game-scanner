'use strict';

if (process.arch === 'x64') {
    if (process.platform === 'win32') {
        module.exports = require("./native/windows.node");
    } else if (process.platform === 'darwin') {
        module.exports = require("./native/darwin.node");
    } else if (process.platform === 'linux') {
        module.exports = require("./native/linux.node");
    } else {
        throw new Error(`platform (${process.platform}) not supported`);
    }
} else {
    throw new Error(`architecture (${process.arch}) not supported`);
}
