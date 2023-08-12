#!/usr/bin/env node
const attention = require('./dist/v16.4.0/win-x64/request-window-attention.node');
(async () => {
    attention.startFlashJs('有道云笔记', 10, 500);
    await new Promise(resolve => setTimeout(resolve, 10000));
    attention.stopFlashJs('有道云笔记');
})();