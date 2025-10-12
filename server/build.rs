fn main() {
    #[cfg(all(feature = "proxy", feature = "client-handler"))]
    compile_error!("Features proxy and client-handler are mutually exclusive");
}
