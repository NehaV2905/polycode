// Build script to compile Protocol Buffer definitions
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)  // We're a client, not a server
        .build_client(true)
        .compile(
            &["proto/ir_events.proto"],
            &["proto"],
        )?;

    println!("cargo:rerun-if-changed=proto/ir_events.proto");
    Ok(())
}
