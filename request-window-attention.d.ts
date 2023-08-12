/**
 * 开始闪烁。
 * （1）在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
 * （2）在窗体获得了焦点之后，闪烁与高亮才都会结束。
 * @param winTitle 被闪烁窗体“标题名”
 * @param blinkCount  闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
 * @param blinkRate   相邻闪烁的间隔时间（单位：毫秒）
 */
export function startFlashJs(winTitle: string, blinkCount: number, blinkRate: number);
/**
 * 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
 * @param winTitle 被闪烁窗体“标题名”
 */
export function stopFlashJs(winTitle: string);