use cargo_near_build::{extended::BuildScriptOpts, BuildOpts};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!(
        "cargo:warning={}",
        format!(
            "`devhub-community` build script working dir: {:?}",
            std::env::current_dir().expect("get current dir")
        )
    );
    let env = ["KEY", "GOOGLE_QUERY"]
        .into_iter()
        .filter(|key| std::env::var(key).is_ok())
        .map(|key| (key.to_string(), std::env::var(key).unwrap()))
        .collect::<Vec<_>>();

    let build_opts = BuildOpts {
        env,
        ..Default::default()
    };

    let opts = cargo_near_build::extended::BuildOptsExtended {
        workdir: "../discussions",
        env: vec![
            // unix path of target contract from root of repo
            (cargo_near_build::env_keys::nep330::CONTRACT_PATH, "discussions"),
        ],
        build_opts,
        build_script_opts: BuildScriptOpts {
            result_env_key: Some("BUILD_RS_SUB_BUILD_DEVHUB-DISCUSSIONS"),
            rerun_if_changed_list: vec!["../discussions", "Cargo.toml", "../Cargo.lock"],
            build_skipped_when_env_is: vec![
                // shorter build for `cargo check`
                ("PROFILE", "debug"),
                (cargo_near_build::env_keys::BUILD_RS_ABI_STEP_HINT, "true"),
            ],
            distinct_target_dir: Some("../target/build-rs-discussions-for-community"),
            stub_path: Some("../target/discussions-stub.bin"),
        },
    };

    cargo_near_build::extended::build(opts)?;
    Ok(())
}
