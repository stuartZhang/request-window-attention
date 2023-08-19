#!/usr/bin/env node
const attention = require('./dist/nodejs/win-x64/request-window-attention.node');
(async () => {
    console.info('版本信息', attention.getEdition());
    attention.startFlashJs('有道云笔记', 10, 500);
    await new Promise(resolve => setTimeout(resolve, 10000));
    attention.stopFlashJs('有道云笔记');
})();