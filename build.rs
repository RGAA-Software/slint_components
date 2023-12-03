fn main() {
    // slint_build::compile("ui/main.slint").unwrap();

    let config =
        slint_build::CompilerConfiguration::new()
            .with_style("material".into());
    slint_build::compile_with_config("example.slint", config).unwrap();
}