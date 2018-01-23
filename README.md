# SpaceOut PC
a rust game<br/>
使用SDL2开发的一个小游戏<br/>
<img src="https://github.com/planet0104/spaceout_pc/blob/master/screenshot/20180123205844.png" /><br />
<img src="https://github.com/planet0104/spaceout_pc/blob/master/screenshot/20180123212736.png" /><br />
游戏源码参考《游戏编程入门》：<br />
<img src="https://img3.doubanio.com/lpic/s26278533.jpg" /><br /><br />
代码在Widows 10 64bit编译通过，其他平台需要配置才能编译。<br />
SDL2配置参考这里：https://github.com/Rust-SDL2/rust-sdl2<br /><br />

代码结构:<br /><br />
<b>游戏引擎部分</b><br />
<i>src/engine.rs</i> 游戏引擎<br />
<i>src/sprite.rs</i> 精灵<br />
<i>src/timer.rs</i> 计时器<br /><br />
<b>SpaceOut游戏</b><br />
<i>src/main.rs</i> 窗口程序<br />
<i>src/spaceout.rs</i> 游戏主代码<br />
<i>src/alien_sprite.rs</i> 外星人<br />
<i>src/backgkround.rs</i> 星星闪烁的背景<br />
<i>resources</i> 资源文件<br />
<i>gnu-mingw</i> 编译和运行用到的库文件(win64)<br /><br />
<b>release_win64.zip是编译好的win64执行文件，下载可直接运行。</b><br /><br />

这是此游戏的WebAssembly版本，代码只有少许改动：https://github.com/planet0104/spaceout<br /><br />
另外，我是一个Rust新手，很多代码可能写的不太好，请酌情参考。
