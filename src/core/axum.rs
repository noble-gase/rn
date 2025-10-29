use std::vec;

use tera::Tera;

pub fn global() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        ("Cargo.toml", include_str!("../../template/axum/Cargo.tera")),
        (
            ".dockerignore",
            include_str!("../../template/dockerignore.tera"),
        ),
        (".gitignore", include_str!("../../template/gitignore.tera")),
        ("README.md", include_str!("../../template/axum/README.tera")),
    ])
    .unwrap();
    tera
}

pub fn app() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // Cargo.toml
        (
            "Cargo.toml",
            include_str!("../../template/axum/app/Cargo.tera"),
        ),
        // config.toml
        (
            "config.toml",
            include_str!("../../template/axum/app/config.tera"),
        ),
        // main.rs
        (
            "src/main.rs",
            include_str!("../../template/axum/app/src/main.tera"),
        ),
        // cmd
        (
            "src/cmd/mod.rs",
            include_str!("../../template/axum/app/src/cmd/mod.tera"),
        ),
        (
            "src/cmd/hello.rs",
            include_str!("../../template/axum/app/src/cmd/hello.tera"),
        ),
        (
            "src/cmd/serve.rs",
            include_str!("../../template/axum/app/src/cmd/serve.tera"),
        ),
        // handler
        (
            "src/handler/mod.rs",
            include_str!("../../template/axum/app/src/handler/mod.tera"),
        ),
        (
            "src/handler/greeter.rs",
            include_str!("../../template/axum/app/src/handler/greeter.tera"),
        ),
        // middleware
        (
            "src/middleware/mod.rs",
            include_str!("../../template/axum/app/src/middleware/mod.tera"),
        ),
        (
            "src/middleware/auth.rs",
            include_str!("../../template/axum/app/src/middleware/auth.tera"),
        ),
        // router
        (
            "src/router/mod.rs",
            include_str!("../../template/axum/app/src/router/mod.tera"),
        ),
        (
            "src/router/route.rs",
            include_str!("../../template/axum/app/src/router/route.tera"),
        ),
        // service
        (
            "src/service/mod.rs",
            include_str!("../../template/axum/app/src/service/mod.tera"),
        ),
        (
            "src/service/greeter.rs",
            include_str!("../../template/axum/app/src/service/greeter.tera"),
        ),
    ])
    .unwrap();
    tera
}

pub fn domain() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // Cargo.toml
        (
            "Cargo.toml",
            include_str!("../../template/axum/domain/Cargo.tera"),
        ),
        // lib.rs
        (
            "src/lib.rs",
            include_str!("../../template/axum/domain/src/lib.tera"),
        ),
        // repo
        (
            "src/repo/mod.rs",
            include_str!("../../template/axum/domain/src/repo/mod.tera"),
        ),
        (
            "src/repo/demo.rs",
            include_str!("../../template/axum/domain/src/repo/demo.tera"),
        ),
        // schema
        (
            "src/schema/mod.rs",
            include_str!("../../template/axum/domain/src/schema/mod.tera"),
        ),
        (
            "src/schema/model.rs",
            include_str!("../../template/axum/domain/src/schema/model.tera"),
        ),
        (
            "src/schema/table.rs",
            include_str!("../../template/axum/domain/src/schema/table.tera"),
        ),
    ])
    .unwrap();
    tera
}

pub fn infra() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // Cargo.toml
        (
            "Cargo.toml",
            include_str!("../../template/axum/infra/Cargo.tera"),
        ),
        // lib.rs
        (
            "src/lib.rs",
            include_str!("../../template/axum/infra/src/lib.tera"),
        ),
        // code
        (
            "src/code/mod.rs",
            include_str!("../../template/axum/infra/src/code/mod.tera"),
        ),
        (
            "src/code/rejection.rs",
            include_str!("../../template/axum/infra/src/code/rejection.tera"),
        ),
        // core
        (
            "src/core/mod.rs",
            include_str!("../../template/axum/infra/src/core/mod.tera"),
        ),
        (
            "src/core/config.rs",
            include_str!("../../template/axum/infra/src/core/config.tera"),
        ),
        (
            "src/core/db.rs",
            include_str!("../../template/axum/infra/src/core/db.tera"),
        ),
        (
            "src/core/logger.rs",
            include_str!("../../template/axum/infra/src/core/logger.tera"),
        ),
        (
            "src/core/redis_pool.rs",
            include_str!("../../template/axum/infra/src/core/redis_pool.tera"),
        ),
        // middleware
        (
            "src/middleware/mod.rs",
            include_str!("../../template/axum/infra/src/middleware/mod.tera"),
        ),
        (
            "src/middleware/log.rs",
            include_str!("../../template/axum/infra/src/middleware/log.tera"),
        ),
        (
            "src/middleware/metrics.rs",
            include_str!("../../template/axum/infra/src/middleware/metrics.tera"),
        ),
        (
            "src/middleware/panic.rs",
            include_str!("../../template/axum/infra/src/middleware/panic.tera"),
        ),
        (
            "src/middleware/trace.rs",
            include_str!("../../template/axum/infra/src/middleware/trace.tera"),
        ),
        // util
        (
            "src/util/mod.rs",
            include_str!("../../template/axum/infra/src/util/mod.tera"),
        ),
        (
            "src/util/curd.rs",
            include_str!("../../template/axum/infra/src/util/curd.tera"),
        ),
        (
            "src/util/helper.rs",
            include_str!("../../template/axum/infra/src/util/helper.tera"),
        ),
        (
            "src/util/identity.rs",
            include_str!("../../template/axum/infra/src/util/identity.tera"),
        ),
    ])
    .unwrap();
    tera
}
