use dagger_sdk::HostDirectoryOptsBuilder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let client = dagger_sdk::connect().await?;

    let src = client.host().directory_opts(
        "musl/cross-compile-musl/",
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
        .with_exec(vec!["./target/release/cross-compile", "-h"])
        .file("target/release/cross-compile");

    let final_image = client
        .container()
        .with_file("/usr/local/bin/cross-compile", sample_app)
        .with_exec(vec!["cross-compile", "-h"])
        .with_entrypoint(vec!["cross-compile"]);

    // Alpine debug
    // let final_image = client
    //     .container()
    //     .from("alpine:latest")
    //     .with_exec(vec!["apk", "add", "libc-utils"])
    //     .with_file("/usr/local/bin/cross-compile", sample_app)
    //     .with_exec(vec!["ldd", "/usr/local/bin/cross-compile"])
    //     .with_exec(vec!["cross-compile", "-h"])
    //     .with_entrypoint(vec!["cross-compile"]);

    // Glibc based
    // let final_image = client
    //     .container()
    //     .from("debian:bookworm")
    //     .with_file("/usr/local/bin/cross-compile", sample_app)
    //     .with_exec(vec!["cross-compile", "-h"])
    //     .with_entrypoint(vec!["cross-compile"]);

    // Optional: Publish final image to a docker registry
    // final_image.publish("kasperhermansen/building-rust-for-production:cross-compile-cross-compile").await?;

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
