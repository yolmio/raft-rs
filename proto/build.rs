// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::io::Result;

fn main() -> Result<()> {
    let base = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let proto_path = format!("{}/proto", base);

    prost_build::Config::new()
        .bytes(["."])
        .compile_protos(&[format!("{}/eraftpb.proto", proto_path)], &[proto_path])?;

    Ok(())
}
