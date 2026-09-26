fn main() {
    if let Some(e) = option_env!("VULKAN_SDK") {
        println!("cargo::rustc-link-search=static={e}/Lib");
    }
}
