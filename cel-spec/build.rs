use std::{
    env,
    fs::File,
    io::{Result, Write},
    path::PathBuf,
    process::{Command, Stdio},
};
use std::fs;

fn main() -> Result<()> {
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"));

    let mut config = prost_build::Config::new();
    config.disable_comments(&["."]);
    config.file_descriptor_set_path(out.join("cel.bin"));
    config.compile_protos(
        &["cel-spec/proto/test/v1/simple.proto"],
        &["cel-spec/proto/", "googleapis/"],
    )?;

    let protoc = protoc_prebuilt::init("27.0").expect("failed to download protoc");

    let mut stamp = String::from("use std::collections::HashMap;\n");
    stamp.push_str("use std::sync::Mutex;\n");

    let mut items = String::new();

    for s in fs::read_dir("cel-spec/tests/simple/testdata").unwrap() {
        let filename = PathBuf::from("cel-spec/tests/simple/testdata")
            .join(s.unwrap().file_name().to_str().unwrap());

        if !filename.is_file() {
            continue;
        }
        if !filename.extension().is_some_and(|f| f == "textproto") {
            continue;
        }

        let output = File::create(out.join(filename.with_extension("bin").file_name().unwrap()))?;
        let name = filename.with_extension("");
        let name = name.file_name().unwrap().to_string_lossy();
        stamp.push_str("\n");
        stamp.push_str(
            format!(
                r#"const {name}: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/{out}.bin"));"#,
                name = name.to_uppercase(),
                out = name
            )
            .as_str(),
        );

        items.push_str("\n\t");
        items.push_str(format!(r#"("{name}", {var}),"#, name = name, var = name.to_uppercase()).as_str());

        let input = File::open(filename)?;

        Command::new(&protoc.0)
            .arg("-Icel-spec/proto/")
            .arg("-Igoogleapis/")
            .arg("--encode=google.api.expr.test.v1.SimpleTestFile")
            .arg("cel-spec/proto/test/v1/simple.proto")
            .stdin(Stdio::from(input))
            .stdout(Stdio::from(output))
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;
    }

    stamp.push_str("\n");
    stamp.push_str("lazy_static::lazy_static! {\n");
    stamp.push_str("static ref TESTS: Mutex<HashMap<&'static str, &'static [u8]>> = Mutex::new(HashMap::from([");

    stamp.extend(items.chars().into_iter());
    stamp.push_str("\n]));\n}");
    File::create(out.join("tests.rs"))?.write_all(stamp.as_bytes())?;

    Ok(())
}
