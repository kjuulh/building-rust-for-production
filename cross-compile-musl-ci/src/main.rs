use dagger_sdk::HostDirectoryOptsBuilder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let client = dagger_sdk::connect().await?;

    let src = client.host().directory_opts(
        "cross-compile-musl/",
        HostDirectoryOptsBuilder::default()
            .include(vec!["**/Cargo.toml", "**/Cargo.lock", "**/*.rs", ".cargo/"])
            .build()?,
    );

    let sample_app = client
        .container()
        .from("rust:1.77-bookworm")
        .with_exec(vec!["apt", "update", "-y"])
        .with_exec(vec![
            "apt",
            "install",
            "-y",
            "musl-tools",
            "pkg-config",
            "build-essential",
            "cmake",
            "clang",
            "lld",
        ])
        .with_exec(vec![
            "rustup",
            "target",
            "add",
            "armv7-unknown-linux-musleabihf",
        ])
        .with_workdir("/mnt/src")
        .with_directory(".", src)
        .with_exec(vec![
            "cargo",
            "build",
            "--release",
            "--target=armv7-unknown-linux-musleabihf",
        ])
        .with_exec(vec![
            "./target/armv7-unknown-linux-musleabihf/release/cross-compile",
            "-h",
        ])
        .file("target/armv7-unknown-linux-musleabihf/release/cross-compile");

    let final_image = client
        .container()
        .with_file("/usr/local/bin/cross-compile", sample_app)
        .with_exec(vec!["cross-compile", "-h"])
        .with_entrypoint(vec!["cross-compile"]);

    // Optional: Publish final image to a docker registry
    // final_image.publish("kasperhermansen/building-rust-for-production:cross-compile-cross-compile").await?;

    let output = final_image.with_exec(vec!["-h"]).stdout().await?;

    println!(
        "sample app in linux using a fully statically compiled app: \n{}",
        output
    );

    Ok(())
}
