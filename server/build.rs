fn main() {
    #[cfg(all(feature = "proxy", feature = "server"))]
    compile_error!("Features proxy and server are mutually exclusive");
}
