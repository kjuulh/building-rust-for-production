use dagger_sdk::{
    ContainerWithDirectoryOptsBuilder, ContainerWithNewFileOptsBuilder, HostDirectoryOptsBuilder,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let client = dagger_sdk::connect().await?;

    let src = client.host().directory_opts(
        "sample-app/",
        HostDirectoryOptsBuilder::default()
            .include(vec![
                "**/Cargo.toml",
                "**/Cargo.lock",
                // Excluding rust source entirely "**/*.rs",
                ".cargo/",
            ])
            .build()?,
    );

    let rust_src = client.host().directory_opts(
        "sample-app/",
        HostDirectoryOptsBuilder::default()
            .include(vec!["**/Cargo.toml", "**/Cargo.lock", "**/*.rs", ".cargo/"])
            .build()?,
    );

    let base_image = client
        .container()
        .from("rust:1.77-bookworm")
        .with_workdir("/mnt/src");

    let dependencies = base_image
        .with_directory(".", src)
        .with_new_file_opts(
            "src/main.rs",
            ContainerWithNewFileOptsBuilder::default()
                .contents(r#"
fn main() {
    panic!("THIS SHOULD NEVER BE EXECUTED, only here to fulfille requirements for basic main.rs file");
}
                    "#)
                .build()?,
        )
        .with_exec(vec!["cargo", "build", "--release"]);

    let target = dependencies.directory("target");
    let cargo_root = dependencies.directory("/usr/local/cargo");

    let sample_app = base_image
        .with_directory("/usr/local/cargo", cargo_root)
        .with_directory_opts(
            "target",
            target,
            ContainerWithDirectoryOptsBuilder::default()
                .exclude(vec!["**/*sample-app*"])
                .build()?,
        )
        .with_directory(".", rust_src)
        .with_exec(vec!["cargo", "build", "--release"])
        .file("target/release/sample-app");

    let final_image = client
        .container()
        .from("debian:bookworm")
        .with_file("/usr/local/bin/sample-app", sample_app)
        .with_exec(vec!["sample-app", "-h"])
        .with_entrypoint(vec!["sample-app"]);

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
