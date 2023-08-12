# request-window-attention

## 功能

根据`Windows`操作系统的`Native GUI`窗体**标题名**，寻址窗体句柄，闪烁窗体在桌面任务栏内的占位图标，以吸引计算机操作员的注意力。

### 窗体标题名

从这里看，

![图片](https://github-production-user-asset-6210df.s3.amazonaws.com/13935927/260208018-b585829c-d644-4ac0-9494-aee5bd51dc4b.png)

## 动机

自从[nwjs 40+](https://nwjs.io/)以后，闪烁电脑桌面任务栏图标的`js api`接口`win.requestAttention(attention)`就对`Windows`操作系统丧失了兼容性（和报错`Unchecked runtime.lastError: The context from which the function was called did not have an associated app window.`）。哎！应用面最广的操作系统又一次不受待见可苦了我们这些做应用程序开发的码农了。

缺陷工单如预期般被提交。甲方爸爸可不管这是操作系统被嫌弃、容器自身缺陷、还是应用程序被躺枪的问题。反正，缺陷你得给我解决了。所以，只能想点儿“歪招”，绕过`nwjs`容器，从更底层的`WIN32 COM ABI`闪烁应用程序的桌面任务栏图标。

## 用法

任务栏图标的闪烁功能被以三种形式封装，以适用于不同的调用端场景：

1. `dll`动态链接库 —— 允许调用端以**增量更新**的方式集成入该功能模块
2. `rlib / lib`静态链接库 —— 仅图省事的懒人来用这个。只要项目工期预算充足，花半天时间重编译都不叫事。正好，还有光面堂皇的理由摸鱼。
3. `nodejs C addons`模块 —— 前端开发看这里。伪装成`Commonjs Module`的`C`插件是给亲们的专供。

### 额外说明

1. 虽然工程编译也会输出机器码的【可执行文件】，但它仅被用来做测试用，和不接受任何的命令行参数。
2. 【链接库】输出格式会附增`C++`头文件，以描述链接库接口格式。
   1. 文件位置`target\<profile>\request-window-attention.h`
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
extern "C" {
    /// 开始闪烁。
    /// （1）在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
    /// （2）在窗体获得了焦点之后，闪烁与高亮才都会结束。
    /// @param winTitle   被闪烁窗体“标题名”
    /// @param blinkCount 闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
    /// @param blinkRate  相邻闪烁的间隔时间（单位：毫秒）
    void startFlashC(const char *win_title, unsigned int count, unsigned int blink_rate);
    /// 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
    /// @param winTitle 被闪烁窗体“标题名”
    void stopFlashC(const char *win_title);
} // extern "C"
```

```typescript
/**
 * 开始闪烁。
 * （1）在 stopFlashJs() 接口被调用后，闪烁会停止但高亮会继续。
 * （2）在窗体获得了焦点之后，闪烁与高亮才都会结束。
 * @param winTitle   被闪烁窗体“标题名”
 * @param blinkCount 闪烁次数。超过闪烁次数之后，任务栏会一直保持高亮状态。
 * @param blinkRate  相邻闪烁的间隔时间（单位：毫秒）
 */
export function startFlashJs(winTitle: string, blinkCount: number, blinkRate: number);
/**
 * 结束闪烁，但窗口任务栏还会继续高亮，直到窗体获得用户操作的焦点
 * @param winTitle 被闪烁窗体“标题名”
 */
export function stopFlashJs(winTitle: string);
```

### `nwjs`调用端样例

自测于`nodejs 10 / 12 /16 win-64`运行时环境

```javascript
// 以`Commonjs Module`的形式，导入 C addons 插件
const attention = require('./dist/v16.4.0/win-x64/request-window-attention.node');
(async () => {
    // 通知操作系统，开始闪烁桌面任务栏图标
    attention.startFlashJs('有道云笔记', 10, 500);
    await new Promise(resolve => setTimeout(resolve, 10000));
    // 通知操作系统，停止闪烁桌面任务栏图标。但，任务栏图标还会继续高亮。
    attention.stopFlashJs('有道云笔记');
})();
```

另外，你也可以直接在工程根目录下运行指令`node test.js`来执行测试。

## （预编译包）兼容性说明

### 链接库

要求`Windows 7+ x64`。更低版本或`32`位的操作系统，我个人没有硬软件条件测试。所以，不知道能不能正常链接与运行。

### `nodejs C addons`模块

要求`nodejs 10+ win-x64`，因为从`10`版本往上`nodejs`运行时才开始全面地支持`N-API`的`C`插件扩展接口。

虽然预编译`.node`文件是基于`nodejs v16.4.0 win-x64`编译的，但理论上凡是遵循`N-API`标准接口的`C`插件对`nodejs`版本应该是无感的。

### `node-webkit(nw) C addons`模块

要求`nwjs 0.49.1+ win-x64`。已经自测不支持`x86`的`ia32`位`nw`。若必须支持，请下载代码自行交叉编译。

## 技术细节

没啥技术，整个工程就仅只是`WIN32 COM ABI`的一个层“胶水”代码。

### `nodejs / nw`交叉编译输出目录

![image](https://github.com/stuartZhang/request-window-attention/assets/13935927/e985092e-2761-45a9-b65a-8dd889b4f76b)

### 链接库编译输出目录

![image](https://github.com/stuartZhang/request-window-attention/assets/13935927/ac48f74f-a6da-47c6-aaac-1f68341b0eb9)
