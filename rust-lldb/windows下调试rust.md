# 在 windows 上使用 clion 调试 rust 代码

clion 在 windows 上不能调试 msvc 工具链生成的程序，需要使用 gnu 工具链才能调试，搜了下没有多少介绍工具链的安装过程的文章，写下来记录下。

具体的步骤为：

1. 安装 mingw 环境
   
   安装 msys2 (带 mingw-64 )，下载地址见 [https://www.msys2.org/](https://www.msys2.org/) 。
   
   开一个 mingw 的终端，安装编译工具：
   
   ```undefined
    pacman -Syu pacman -S mingw-w64-x86_64-toolchain
   ```
   
   假设安装在 `c:\msys64` 目录下，则在系统的环境变量中，增加一个：
   
   ```dos
    MSYS2_HOME  C:\msys64 PATH        <原来的路径>;%MSYS2_HOME%\bin;%MSYS2_HOME%\mingw64\bin
   ```
   
   其它可参考 clion + msys2 的相关配置文章。

2. 安装 rust gnu 工具链
   
   在 windows 上使用 rustup 安装的 rust 编译环境默认使用了 msvc 编译链，需要安装 gnu 编译链
   
   ```cpp
    rustup install stable-gnu rustup default stable-gnu
   ```

3. 设置 clion 编译工具链
   
   在 clion 的 `File -> Settings -> Build, Execution, Deployment -> Toolchains` ，加上一个 mingw 的工具链，设置目录为 msys2 中的 mingw64 目录。如 msys2 安装在 `c:\msys64` ，则目录为 `c:\msys64\mingw64` ，目录正确的情况下，make 、 c-compiler 、 c++ compiler 、 debugger 等自动找到。

完成这些设置后，就可以使用 clion 调试 rust 了。
