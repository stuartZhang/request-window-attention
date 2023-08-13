/**
 * 模块版本信息
 */
export interface GitEdition {
    branch: string;
    tag: string;
    latestCommitId: string;
    pkgName: string;
    pkgVersion: string;
    bundleDateTime: string;
}
/**
 * 请对【日志·输出】回调函数，务必做好`try-catch`异常捕获，因为来自`js`端的
 * 异常目前会级联地引发`c-addon`程序崩溃。结果不可控！
 */
export interface Logger {
    (text: string): void;
}
/**
 * 挂载全域【日志·输出】回调函数。通过指定回调函数，可将此`c-addon`内部日志
 * 中继转发到`js`应用层。
 *
 * 重复挂载新的【日志·输出】回调函数不需要手工卸载之前的【日志】回调函数，因为
 * 被采用的`RAII`设计模式可确保`napi_function`是析构的。
 * @param log
 */
export function setLogger(log: Logger);
/**
 * 手工卸载当前的【日志·输出】回调函数，以防止内存泄漏。
 *
 * 注：重复卸载操作不会导致程序崩溃。
 */
export function unsetLogger();
/**
 * 开始闪烁。
 * 1. 在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
 * 2. 在窗体获得了焦点之后，闪烁与高亮才都会结束。
 * @param winTitle 被闪烁窗体“标题名”
 * @param blinkCount  闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
 * @param blinkRate   相邻闪烁的间隔时间（单位：毫秒）
 * @param log         【可选】日志回调函数
 */
export function startFlashByTitleJs(winTitle: string, blinkCount: number, blinkRate: number);
/**
 * 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
 * @param winTitle 被闪烁窗体“标题名”
 * @param log      【可选】日志回调函数
 */
export function stopFlashByTitleJs(winTitle: string);
/**
 * 开始闪烁。
 * 1. 在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
 * 2. 在窗体获得了焦点之后，闪烁与高亮才都会结束。
 * @param ppid  被闪烁窗体的进程ID或nwjs的进程PID
 * @param blinkCount  闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
 * @param blinkRate   相邻闪烁的间隔时间（单位：毫秒）
 */
export function startFlashByPpidJs(ppid: number, blinkCount: number, blinkRate: number);
/**
 * 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
 * @param ppid  被闪烁窗体的进程ID或nwjs的进程PID
 */
export function stopFlashByPpidJs(ppid: number);
/**
 * 模块版本信息
 * @returns GitEdition
 */
export function getEdition(): GitEdition;
