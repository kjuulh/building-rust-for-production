use dagger_sdk::HostDirectoryOptsBuilder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let client = dagger_sdk::connect().await?;

    let src = client.host().directory_opts(
        "cross-compile-wasm/",
        HostDirectoryOptsBuilder::default()
            .include(vec!["**/Cargo.toml", "**/Cargo.lock", "**/*.rs", ".cargo/"])
            .build()?,
    );

    let sample_app = client
        .container()
        .from("rust:1.77-bookworm")
        .with_exec(vec!["rustup", "target", "add", "wasm32-wasi"])
        .with_workdir("/mnt/src")
        .with_directory(".", src)
        .with_exec(vec!["cargo", "build", "--release", "--target=wasm32-wasi"])
        .file("target/wasm32-wasi/release/cross-compile.wasm");

    let final_image = client
        .container()
        .from("chainguard/wasmtime:latest")
        .with_file("/usr/local/bin/cross-compile.wasm", sample_app)
        .with_entrypoint(vec!["wasmtime", "run", "/usr/local/bin/cross-compile.wasm"]);

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
