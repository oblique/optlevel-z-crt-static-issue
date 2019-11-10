fn main() {
    zeromq_src::Build::new()
        .link_static(true)
        .build_debug(false)
        .build()
        .print_cargo_metadata();
}
