use cargo_near_build::extended::BuildScriptOpts;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!(
        "cargo:warning={}",
        format!(
            "`devhub-community-factory` build script working dir: {:?}",
            std::env::current_dir().expect("get current dir")
        )
    );

    let env = ["KEY", "GOOGLE_QUERY"]
        .into_iter()
        .filter(|key| std::env::var(key).is_ok())
        .map(|key| (key.to_string(), std::env::var(key).unwrap()))
        .collect::<Vec<_>>();

    let opts = cargo_near_build::extended::BuildOptsExtended {
        workdir: "../community",
        build_opts: cargo_near_build::BuildOpts {
            env,
            mute_env: vec![
                // unix path of target contract from root of repo
                (cargo_near_build::env_keys::nep330::CONTRACT_PATH.into(), "community".into()),
            ],
            ..Default::default()
        },
        build_script_opts: BuildScriptOpts {
            result_env_key: Some("BUILD_RS_SUB_BUILD_DEVHUB-COMMUNITY"),
            rerun_if_changed_list: vec![
                "../discussions",
                "../community",
                "Cargo.toml",
                "../Cargo.lock",
            ],
            build_skipped_when_env_is: vec![
                // shorter build for `cargo check`
                ("PROFILE", "debug"),
                (cargo_near_build::env_keys::BUILD_RS_ABI_STEP_HINT, "true"),
            ],
            distinct_target_dir: Some("../target/build-rs-community-for-community-factory"),
            stub_path: Some("../target/community-stub.bin"),
        },
    };

    cargo_near_build::extended::build(opts)?;
    Ok(())
}
