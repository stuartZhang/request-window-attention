# request-window-attention

## 功能

根据`Windows`操作系统的`Native GUI`窗体**标题名**，寻址窗体句柄，闪烁窗体在桌面任务栏内的占位图标，以吸引计算机操作员的注意力。

### 窗体标题名

从这里看，

![图片](https://github-production-user-asset-6210df.s3.amazonaws.com/13935927/260208018-b585829c-d644-4ac0-9494-aee5bd51dc4b.png)

## 动机

自从[nwjs 40+](https://nwjs.io/)以后，闪烁电脑桌面任务栏图标的`js api`接口`win.requestAttention(attention)`就对`Windows`操作系统丧失了兼容性（和报错[Unchecked runtime.lastError: The context from which the function was called did not have an associated app window.](https://github.com/nwjs/nw.js/issues/7659)）。哎！应用面最广的操作系统又一次不受待见可苦了我们这些做应用程序开发的码农了。

缺陷工单如预期般被提交。甲方爸爸可不管这是操作系统被嫌弃、容器自身缺陷、还是应用程序被躺枪的问题。反正，缺陷你得给我解决了。所以，只能想点儿“歪招”，绕过`nwjs`容器，从更底层的`WIN32 COM ABI`闪烁应用程序的桌面任务栏图标。

## 用法

任务栏图标的闪烁功能被以三种形式封装，以适用于不同的调用端场景：

1. `dll`动态链接库 —— 允许调用端以**增量更新**的方式集成入该功能模块
2. `rlib / lib`静态链接库 —— 仅图省事的懒人来用这个。只要项目工期预算充足，花半天时间重编译都不叫事。正好，还有光面堂皇的理由摸鱼。
3. `nodejs C addons`模块 —— 前端开发看这里。伪装成`Commonjs Module`的`C`插件是给亲们的专供。
4. `nwjs C addions`模块 —— 前端兄弟们注意了：“`nwjs`与`nodejs`的`C addions`并不通用”。它们的`C addions`头文件有差异。所以，交叉使用会链接失败的。

### 额外说明

1. 虽然工程编译也会输出机器码的【可执行文件】，但它仅被用来做测试用，和不接受任何的命令行参数。
2. 【链接库】输出格式会附增`C++`头文件，以描述链接库接口格式。
   1. 文件位置`target-win-x64\<profile>\request-window-attention.h`
3. `nodejs C addons`输出格式也会附增`.d.ts`类型说明文件，以给调用端提供基本的代码提示与参数作用解释。
   1. 文件位置`dist\nodejs\v16.4.0\win-x64\request-window-attention.d.ts`

因为文件篇幅不长，所以我将`C++`头文件和`.d.ts`类型说明文件的内容就直接贴到这里了

```cpp
#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
struct GitEdition {
  const char *branch;
  const char *tag;
  const char *latest_commit_id;
  const char *pkg_name;
  const char *pkg_version;
  const char *bundle_time;
};
extern "C" {
    /// 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
    /// @param winTitle 被闪烁窗体“标题名”
    void stopFlashByTitleC(const char *win_title);
    /// /// 开始闪烁。
    /// （1）在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
    /// （2）在窗体获得了焦点之后，闪烁与高亮才都会结束。
    /// @param winTitle 被闪烁窗体“标题名”
    /// @param blinkCount  闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
    /// @param blinkRate   相邻闪烁的间隔时间（单位：毫秒）
    void startFlashByTitleC(const char *win_title,
                            unsigned int count,
                            unsigned int blink_rate);
    /// 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
    /// @param process_id 被闪烁窗体的进程ID或nwjs的进程PID
    void stopFlashByPpidC(unsigned int process_id) ;
    /// /// 开始闪烁。
    /// （1）在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
    /// （2）在窗体获得了焦点之后，闪烁与高亮才都会结束。
    /// @param process_id  被闪烁窗体的进程ID或nwjs的进程PID
    /// @param blinkCount  闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
    /// @param blinkRate   相邻闪烁的间隔时间（单位：毫秒）
    void startFlashByPpidC(unsigned int process_id,
                           unsigned int count,
                           unsigned int blink_rate);
    /// 模块版本信息
    GitEdition *getEditionC();
} // extern "C"

```

```typescript
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
 */
export function startFlashByTitleJs(winTitle: string, blinkCount: number, blinkRate: number);
/**
 * 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
 * @param winTitle 被闪烁窗体“标题名”
 * @param log     【可选】日志回调函数
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
```

### `nwjs`调用端样例

自测于`nodejs 10 / 12 /16 win-64`运行时环境

```javascript
// 以`Commonjs Module`的形式，导入 C addons 插件
const attention = require('./dist/nodejs/win-x64/request-window-attention.node');
(async () => {
    // 读取与输出`c-addon`的版本信息
    console.info('版本信息', attention.getEdition());
    // 挂载全局【日志·输出】回调函数钩子。
    attention.setLogger(text => {
        console.log('[attention]', text);
    });
    // 根据【主窗体】名，通知操作系统，开始闪烁桌面任务栏图标
    attention.startFlashByTitleJs('有道云笔记', 10, 500);
    await new Promise(resolve => setTimeout(resolve, 500));
    // 根据【主窗体】名，通知操作系统，停止闪烁桌面任务栏图标。
    // 但，任务栏图标还会继续高亮。
    attention.stopFlashByTitleJs('有道云笔记');
    // 分隔线
    await new Promise(resolve => setTimeout(resolve, 500));
    // 根据【主窗体】进程ID，通知操作系统，开始闪烁桌面任务栏图标
    attention.startFlashByPpidJs(18928, 10, 500);
    await new Promise(resolve => setTimeout(resolve, 1000));
    // 根据【主窗体】进程ID，通知操作系统，停止闪烁桌面任务栏图标。
    // 但，任务栏图标还会继续高亮。
    attention.stopFlashByPpidJs(18928);
    // 卸载全局【日志·输出】回调函数钩子，以避免内存泄漏。
    attention.unsetLogger();
})();
function logger(text){
    console.log('[attention]', text);
}
```

另外，你也可以直接在工程根目录下运行指令`node test.js`来执行测试。

### 安装

```shell
npm i request-window-attention
```

### 导入被安装的`request-window-attention`模块

* 在`nodejs x64`环境，`require('request-window-attention')`。
* 在`nodejs x86/ia32`环境，`require('request-window-attention/dist/nodejs/win-ia32/request-window-attention.node')`。
* 在`nwjs x64`容器内，`require('request-window-attention/dist/nw/win-x64/request-window-attention.node')`。
* 在`nwjs ia32`容器内，`require('request-window-attention/dist/nw/win-ia32/request-window-attention.node')`。

## （预编译包）兼容性说明

### 链接库

要求`Windows 7+`。

### `nodejs C addons`模块

要求`nodejs 10+`，因为从`10`版本往上`nodejs`运行时才开始全面地支持`N-API`的`C`插件扩展接口。

虽然预编译`.node`文件是基于`nodejs v16.4.0`编译的，但理论上凡是遵循`N-API`标准接口的`C`插件对`nodejs`版本应该是无感的。

### `node-webkit(nw) C addons`模块

要求`nwjs 0.49.2+`。

## 技术细节

没啥技术，整个工程就仅只是`WIN32 COM ABI`的一个层“胶水”代码。

### `nodejs / nw`交叉编译输出目录

![image](https://github.com/stuartZhang/request-window-attention/assets/13935927/e985092e-2761-45a9-b65a-8dd889b4f76b)

### 链接库编译输出目录

![image](https://github.com/stuartZhang/request-window-attention/assets/13935927/ac48f74f-a6da-47c6-aaac-1f68341b0eb9)

### 编译整个工程

`cd`至工程根目录和执行`win32 bat`脚本

```bat
build4publish.cmd
```

## 使用效果

### `nodejs`调用场景

![image](https://github.com/stuartZhang/request-window-attention/assets/13935927/3e82ac33-1d79-4228-9720-304f91cbf553)

### `nwjs`调用场景

![image](https://github.com/stuartZhang/request-window-attention/assets/13935927/83db5172-6fd8-4240-9804-70c5161deba4)
