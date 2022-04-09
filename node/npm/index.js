'use strict';

if (process.arch === 'x64') {
    if (process.platform === 'win32') {
        module.exports = require("./native/windows-x64.node");
    } else if (process.platform === 'darwin') {
        module.exports = require("./native/darwin-x64.node");
    } else if (process.platform === 'linux') {
        module.exports = require("./native/linux-x64.node");
    } else {
        throw new Error(`platform (${process.platform}) not supported`);
    }
} else if (process.arch === 'arm64') {
    if (process.platform === 'win32') {
        module.exports = require("./native/windows-arm64.node");
    } else if (process.platform === 'darwin') {
        module.exports = require("./native/darwin-arm64.node");
    } else if (process.platform === 'linux') {
        module.exports = require("./native/linux-arm64.node");
    } else {
        throw new Error(`platform (${process.platform}) not supported`);
    }
} else {
    throw new Error(`architecture (${process.arch}) not supported`);
}
