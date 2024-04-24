use dagger_sdk::HostDirectoryOptsBuilder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let client = dagger_sdk::connect().await?;

    let src = client.host().directory_opts(
        "sample-app/",
        HostDirectoryOptsBuilder::default()
            .include(vec!["**/Cargo.toml", "**/Cargo.lock", "**/*.rs", ".cargo/"])
            .build()?,
    );

    let sample_app = client
        .container()
        .from("rust:1.77-bookworm")
        .with_workdir("/mnt/src")
        .with_directory(".", src)
        .with_exec(vec!["cargo", "build", "--release"])
        .with_exec(vec!["./target/release/sample-app", "-h"])
        .file("target/release/sample-app");

    let final_image = client
        .container()
        .with_file("/usr/local/bin/sample-app", sample_app)
        .with_exec(vec!["sample-app", "-h"])
        .with_entrypoint(vec!["sample-app"]);

    // Alpine debug
    // let final_image = client
    //     .container()
    //     .from("alpine:latest")
    //     .with_exec(vec!["apk", "add", "libc-utils"])
    //     .with_file("/usr/local/bin/sample-app", sample_app)
    //     .with_exec(vec!["ldd", "/usr/local/bin/sample-app"])
    //     .with_exec(vec!["sample-app", "-h"])
    //     .with_entrypoint(vec!["sample-app"]);

    // Glibc based
    // let final_image = client
    //     .container()
    //     .from("debian:bookworm")
    //     .with_file("/usr/local/bin/sample-app", sample_app)
    //     .with_exec(vec!["sample-app", "-h"])
    //     .with_entrypoint(vec!["sample-app"]);

    // Optional: Publish final image to a docker registry
    // final_image.publish("kasperhermansen/building-rust-for-production:sample-app-sample-app").await?;

    let output = final_image.with_exec(vec!["-h"]).stdout().await?;

    println!(
        "sample app in linux using a fully statically compiled app: \n{}",
        output
    );

    let output = final_image.with_exec(vec!["bench"]).stdout().await?;

    println!(
        "sample app in linux using a fully statically compiled app: \n{}",
        output
    );

    Ok(())
}
