use wlr_libpy::bld_cfg::configure_static_libs;

fn main() {
    configure_static_libs().unwrap().emit_link_flags();
}
