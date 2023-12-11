#!/usr/bin/env node
const attention = require('./dist/nodejs/win-x64/request-window-attention.node');
(async () => {
    console.info('版本信息', attention.getEdition());
    //
    attention.setLogger(text => {
        console.log('[attention]', text);
    });
    attention.startFlashByTitleJs('有道云笔记', 10, 500);
    await new Promise(resolve => setTimeout(resolve, 500));
    attention.stopFlashByTitleJs('有道云笔记');
    setTimeout(() => {
        attention.startFlashByPpidJs(18928, 10, 500);
        setTimeout(() => {
            attention.stopFlashByPpidJs(18928);
            setTimeout(() => {
                attention.unsetLogger();
            }, 500);
        }, 1000);
    }, 500);
})();
