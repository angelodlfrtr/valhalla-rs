fn compile() -> String {
    let dst = cmake::Config::new("valhalla")
        // .define("ENABLE_PYTHON_BINDINGS", "ON")
        .build();

    dst.display().to_string()
}

fn generate_bindings(out_dir: String) {
    let mut b = autocxx_build::build(
        "src/lib.rs",
        &[
            "src".to_string(),
            format!("{}/include", out_dir),
            "valhalla/third_party/date/include".to_string(),
            "valhalla/third_party/rapidjson/include".to_string(),
        ],
        &[],
    )
    .unwrap();

    b.flag_if_supported("-std=c++14")
        .file("src/actor_wrapper.cc")
        .compile("valhalla");

    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn compile_protos() {
    tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .format(false)
        .compile(&["valhalla/proto/api.proto"], &["valhalla/proto/"])
        .expect("compiling protos");
}

fn main() {
    let out_dir = compile();
    generate_bindings(out_dir);
    compile_protos();
}
