use std::{env, io::Result, path::PathBuf};

fn main() -> Result<()> {
    let out = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let out = PathBuf::from(out).join("cel.bin");
    let mut config = prost_build::Config::new();
    config.disable_comments(&["."]);
    config.file_descriptor_set_path(out);
    config.compile_protos(
        &["cel-spec/proto/test/v1/simple.proto"],
        &["cel-spec/proto/", "googleapis/"],
    )
}
