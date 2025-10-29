# 氡-Rn

[<img alt="crates.io" src="https://img.shields.io/crates/v/rnx.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rnx)
[<img alt="MIT" src="http://img.shields.io/badge/license-MIT-brightgreen.svg?style=for-the-badge" height="20">](http://opensource.org/licenses/MIT)

[氡-Rn] Rust Web开发脚手架，支持 `salvo` 和 `axum` 框架，并同时支持创建「单应用」和「多应用」

## 安装

```shell
cargo install rnx
```

## 特点

- DB使用 [sqlx](https://github.com/launchbadge/sqlx)
- Redis使用 [redis-rs](https://github.com/redis-rs/redis-rs)
- 日志使用 [tracing](https://github.com/tokio-rs/tracing)
- 配置使用 [config-rs](https://github.com/mehcode/config-rs)
- 命令行使用 [clap](https://github.com/clap-rs/clap)
- 异步运行时使用 [tokio](https://github.com/tokio-rs/tokio)
- 参数验证器使用 [validator](https://github.com/Keats/validator)
- 包含 基于 sqlx 和 sea-query 的 curd 封装
- 支持 Prometheus Metrics 和 Request 中间件
- 包含 TraceId、认证、请求日志、Panic捕获 中间件
- 简单好用的 API Result（宏定义错误码）统一输出方式

## 创建项目

### 单应用

```shell
# salvo
rnx new # 在当前目录初始化项目
rnx new --name demo # 创建demo项目

# axum
rnx new --axum # 在当前目录初始化项目
rnx new --name demo --axum # 创建demo项目

.
├── Cargo.toml
├── Dockerfile
├── app/
│   ├── Cargo.toml
│   ├── config.toml
│   └── src/
│       ├── cmd/
│       ├── handler/
│       ├── middleware/
│       ├── router/
│       ├── service/
│       └── main.rs
├── domain/
│   ├── Cargo.toml
│   └── src/
│       ├── repo/
│       ├── schema/
│       └── lib.rs
└── infra/
    ├── Cargo.toml
    └── src/
        ├── code/
        ├── core/
        ├── middleware/
        ├── util/
        └── lib.rs

```

### 多应用

```shell
# salvo
rnx new --app foo --app bar # 在当前目录初始化项目
rnx new --name demo --app foo --app bar # 创建demo项目

# axum
rnx new --app foo --app bar --axum # 在当前目录初始化项目
rnx new --name demo --app foo --app bar --axum # 创建demo项目

.
├── Cargo.toml
├── Dockerfile.foo
├── Dockerfile.bar
├── app/
│   ├── foo/
│   │   ├── Cargo.toml
│   │   ├── config.toml
│   │   └── src/
│   │       ├── cmd/
│   │       ├── handler/
│   │       ├── middleware/
│   │       ├── router/
│   │       ├── service/
│   │       └── main.rs
│   └── bar/
│       ├── Cargo.toml
│       ├── config.toml
│       └── src/
│           ├── ...
│           └── main.rs
├── domain/
│   ├── Cargo.toml
│   └── src/
│       ├── repo/
│       ├── schema/
│       └── lib.rs
└── infra/
    ├── Cargo.toml
    └── src/
        ├── code/
        ├── core/
        ├── middleware/
        ├── util/
        └── lib.rs
```

## 创建应用

> 多应用项目适用，需在项目根目录执行（即：`Cargo.toml` 所在目录）

```shell
# salvo
rnx app --name foo --name bar

# axum
rnx app --name foo --name bar --axum

.
├── Cargo.toml
├── Dockerfile.foo
├── Dockerfile.bar
├── app/
│   ├── foo/
│   │   ├── Cargo.toml
│   │   ├── config.toml
│   │   └── src/
│   │       ├── cmd/
│   │       ├── handler/
│   │       ├── middleware/
│   │       ├── router/
│   │       ├── service/
│   │       └── main.rs
│   └── bar/
│       ├── Cargo.toml
│       ├── config.toml
│       └── src/
│           ├── ...
│           └── main.rs
├── domain/
│   ├── Cargo.toml
│   └── src/
│       ├── ...
│       └── lib.rs
└── infra/
    ├── Cargo.toml
    └── src/
        ├── ...
        └── lib.rs
```

**Enjoy 😊**
