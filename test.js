#!/usr/bin/env node
const attention = require('./dist/nodejs/win-x64/request-window-attention.node');
(async () => {
    console.info('版本信息', attention.getEdition());
    attention.startFlashByTitleJs('有道云笔记', 10, 500, logger);
    await new Promise(resolve => setTimeout(resolve, 10000));
    attention.stopFlashByTitleJs('有道云笔记');

    attention.startFlashByPpidJs(18928, 10, 500, logger);
    await new Promise(resolve => setTimeout(resolve, 10000));
    attention.stopFlashByPpidJs(18928);
})();
function logger(text){
    console.log('[attention]', text);
}
