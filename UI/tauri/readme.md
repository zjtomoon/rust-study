# Tauri应用开发

## 官网
[tauri-app](https://tauri.studio/)

## 依赖
+ visual studio 2019及以上
+ [webview2runtime](https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/)
+ rust & cargo

## 项目创建
[创建项目](./docs/%E5%88%9B%E5%BB%BA%E9%A1%B9%E7%9B%AE.md)

## 已有项目初始化
[已有前端项目初始化](./docs//%E5%B7%B2%E6%9C%89%E9%A1%B9%E7%9B%AE%E5%88%9D%E5%A7%8B%E5%8C%96.md)

## windows编译注意事项
+ 选择rust-msvc版本 visual-studio安装2019及以上，vs2017编译后无法运行
+ 选择rust-gnu版本，仅能够执行tauri dev命令，tauri编译后缺少webview2loader.dll无法运行