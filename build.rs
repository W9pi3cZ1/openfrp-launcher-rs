fn main() {
    slint_build::compile_with_config("ui/main.slint",slint_build::CompilerConfiguration::new().with_style("fluent-light".to_string())).unwrap();    
    // slint_build::compile_with_config("ui/pages/settings-page.slint",slint_build::CompilerConfiguration::new().with_style("fluent-light".to_string())).unwrap();    
}