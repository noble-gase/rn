use std::vec;

use tera::Tera;

pub fn global() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        (
            "Cargo.toml",
            include_str!("../../template/salvo/Cargo.tera"),
        ),
        (
            ".dockerignore",
            include_str!("../../template/dockerignore.tera"),
        ),
        (".gitignore", include_str!("../../template/gitignore.tera")),
        (
            "README.md",
            include_str!("../../template/salvo/README.tera"),
        ),
    ])
    .unwrap();
    tera
}

pub fn docker() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![(
        "Dockerfile",
        include_str!("../../template/Dockerfile.tera"),
    )])
    .unwrap();
    tera
}

pub fn other() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        ("dockerun.sh", include_str!("../../template/dockerun.tera")),
        ("config.toml", include_str!("../../template/config.tera")),
    ])
    .unwrap();
    tera
}

pub fn infra() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // lib.rs
        (
            "lib.rs",
            include_str!("../../template/salvo/infra/lib.tera"),
        ),
        // cache
        (
            "cache/mod.rs",
            include_str!("../../template/salvo/infra/cache/mod.tera"),
        ),
        (
            "cache/redis_pool.rs",
            include_str!("../../template/salvo/infra/cache/redis_pool.tera"),
        ),
        // code
        (
            "code/mod.rs",
            include_str!("../../template/salvo/infra/code/mod.tera"),
        ),
        // config
        (
            "config/mod.rs",
            include_str!("../../template/salvo/infra/config/mod.tera"),
        ),
        // db
        (
            "db/mod.rs",
            include_str!("../../template/salvo/infra/db/mod.tera"),
        ),
        (
            "db/curd.rs",
            include_str!("../../template/salvo/infra/db/curd.tera"),
        ),
        (
            "db/pool.rs",
            include_str!("../../template/salvo/infra/db/pool.tera"),
        ),
        // entity
        (
            "entity/mod.rs",
            include_str!("../../template/salvo/infra/entity/mod.tera"),
        ),
        (
            "entity/model.rs",
            include_str!("../../template/salvo/infra/entity/model.tera"),
        ),
        (
            "entity/table.rs",
            include_str!("../../template/salvo/infra/entity/table.tera"),
        ),
        // logger
        (
            "logger/mod.rs",
            include_str!("../../template/salvo/infra/logger/mod.tera"),
        ),
        // middleware
        (
            "middleware/mod.rs",
            include_str!("../../template/salvo/infra/middleware/mod.tera"),
        ),
        (
            "middleware/log.rs",
            include_str!("../../template/salvo/infra/middleware/log.tera"),
        ),
        (
            "middleware/metrics.rs",
            include_str!("../../template/salvo/infra/middleware/metrics.tera"),
        ),
        (
            "middleware/panic.rs",
            include_str!("../../template/salvo/infra/middleware/panic.tera"),
        ),
        (
            "middleware/trace.rs",
            include_str!("../../template/salvo/infra/middleware/trace.tera"),
        ),
        // util
        (
            "util/mod.rs",
            include_str!("../../template/salvo/infra/util/mod.tera"),
        ),
        (
            "util/helper.rs",
            include_str!("../../template/salvo/infra/util/helper.tera"),
        ),
        (
            "util/identity.rs",
            include_str!("../../template/salvo/infra/util/identity.tera"),
        ),
    ])
    .unwrap();
    tera
}

pub fn app() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // main.rs
        (
            "main.rs",
            include_str!("../../template/salvo/app/main.tera"),
        ),
        // cmd
        (
            "cmd/mod.rs",
            include_str!("../../template/salvo/app/cmd/mod.tera"),
        ),
        (
            "cmd/hello.rs",
            include_str!("../../template/salvo/app/cmd/hello.tera"),
        ),
        (
            "cmd/serve.rs",
            include_str!("../../template/salvo/app/cmd/serve.tera"),
        ),
        // handler
        (
            "handler/mod.rs",
            include_str!("../../template/salvo/app/handler/mod.tera"),
        ),
        (
            "handler/greeter.rs",
            include_str!("../../template/salvo/app/handler/greeter.tera"),
        ),
        // middleware
        (
            "middleware/mod.rs",
            include_str!("../../template/salvo/app/middleware/mod.tera"),
        ),
        (
            "middleware/auth.rs",
            include_str!("../../template/salvo/app/middleware/auth.tera"),
        ),
        // router
        (
            "router/mod.rs",
            include_str!("../../template/salvo/app/router/mod.tera"),
        ),
        (
            "router/route.rs",
            include_str!("../../template/salvo/app/router/route.tera"),
        ),
        // service
        (
            "service/mod.rs",
            include_str!("../../template/salvo/app/service/mod.tera"),
        ),
        (
            "service/greeter.rs",
            include_str!("../../template/salvo/app/service/greeter.tera"),
        ),
    ])
    .unwrap();
    tera
}
