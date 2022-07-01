fn main() {
    let doc_build = cfg!(doc) || std::env::var("DOCS_RS").is_ok();
    if !doc_build && cfg!(feature = "install") {
        // TODO Update commit to stable version once binaries are published correctly
        near_sandbox_utils::install_with_version("VLAD-test/c90ac67bc16ba9acbd43104eeb1ba73b6600cca9")
            .expect("Could not install sandbox");
    }
}
