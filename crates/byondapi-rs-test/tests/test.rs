use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

use tempfile::TempDir;

#[test]
fn test_byondapi_with_dreamdaemon() {
    let dll = build_dylib();
    compile();

    let tempdir = tempfile::tempdir().expect("Failed to create temporary directory");
    copy_to_tmp(&dll, &tempdir);
    run_dreamdaemon(&tempdir);
    check_output_rust(&tempdir);
    check_output_dd(&tempdir);
}

fn bin_path() -> PathBuf {
    match std::env::var("BYOND_LOCATION") {
        Ok(value) => {
            println!("Using byond from dir {value}");
            value.into()
        }
        Err(_) => {
            println!("Byond not found, using default location");
            println!("To set a location for byond, set the BYOND_LOCATION environment variable to a path");
            println!("Keep in mind that this path has to point to the /bin folder of byond");
            "C:\\Program Files (x86)\\BYOND\\bin".into()
        }
    }
}

fn find_dm() -> PathBuf {
    if cfg!(windows) {
        bin_path().join("dm.exe")
    } else {
        "DreamMaker".into()
    }
}

fn find_dd() -> PathBuf {
    if cfg!(windows) {
        bin_path().join("dd.exe")
    } else {
        "DreamDaemon".into()
    }
}

fn project_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("dm_project")
}

fn build_dylib() -> PathBuf {
    let mut cmd = Command::new(option_env!("CARGO").unwrap_or("cargo"));

    cmd.arg("build").arg("--message-format=json").arg("--lib");
    #[cfg(windows)]
    cmd.arg("--target=i686-pc-windows-msvc");
    #[cfg(unix)]
    cmd.arg("--target=i686-unknown-linux-gnu");
    cmd.stderr(std::process::Stdio::inherit());
    parse_output(cmd.output().unwrap())
}

fn parse_output(res: Output) -> PathBuf {
    let mut artifact = None;
    for message in cargo_metadata::Message::parse_stream(res.stdout.as_slice()) {
        match message.unwrap() {
            cargo_metadata::Message::CompilerMessage(m) => eprintln!("{}", m),
            cargo_metadata::Message::CompilerArtifact(a) => artifact = Some(a),
            _ => (),
        }
    }

    if !res.status.success() {
        panic!("Failed to build")
    }
    artifact.unwrap().filenames[0].clone().into()
}

fn compile() {
    let dm_compiler = find_dm();

    let output = Command::new(dm_compiler)
        .current_dir(project_dir())
        .arg("dm_project.dme")
        .output()
        .expect("Failed to compile DM project");

    assert!(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("dm_project")
            .join("dm_project.dmb")
            .exists(),
        "dm_project.dmb was not created: {:#?}",
        output
    )
}

fn copy_to_tmp(dll: &Path, tempdir: &TempDir) {
    let target = tempdir.path();

    std::fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("dm_project")
            .join("dm_project.dmb"),
        target.join("dm_project.dmb"),
    )
    .expect("Failed to copy dm_project.dmb");

    std::fs::copy(dll, target.join("byondapi_test.dll")).expect("Failed to copy byondapi_test.dll");
}

fn run_dreamdaemon(tempdir: &TempDir) {
    let dream_daemon = find_dd();

    let dd_output = Command::new(dream_daemon)
        .current_dir(tempdir.path())
        .arg("dm_project.dmb")
        .arg("-trusted")
        .output()
        .expect("DreamDaemon crashed");

    println!("{:#?}", dd_output);
}

fn check_output_dd(tempdir: &TempDir) {
    let log = tempdir.path().join("dd_log.txt");

    assert!(log.exists(), "The test did not produce any output");

    let log = std::fs::read_to_string(log).expect("Failed to read log");

    eprintln!("{}", log);

    assert!(
        !log.contains("runtime error:"),
        "Log contained runtime errors!"
    );
}

fn check_output_rust(tempdir: &TempDir) {
    let log = tempdir.path().join("rust_log.txt");

    if log.exists() {
        let log = std::fs::read_to_string(log).expect("Failed to read log");
        eprintln!("{}", log);
        panic!("Rust error log was produced!");
    }
}
