# rust webassembly环境准备

## 安装wasm-pack
```bash
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## 安装cargo-generate

```bash
    cargo install cargo-generate
```

## 安装nodejs和npm


# quick start

+ 1、克隆模板

```bash
    cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

+ 2、wasm-pack编译

```bash
    wasm-pack build
```

+ 3、部署到web

```bash
    npm init wasm-app www
```

+ 4、安装node_modules

```bash
    npm install
```
+ 5、dev运行

```bash
    npm run start
```

+ 6、编译web项目

```bash
    npm run build
```

[参考文档](https://rustwasm.github.io/docs/book/introduction.html)