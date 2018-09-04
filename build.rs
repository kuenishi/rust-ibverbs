extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:include=vendor/rdma-core/build/include");
    println!("cargo:include=/usr/include");
    // println!("cargo:rustc-link-search=native=vendor/rdma-core/build/lib");
    println!("cargo:rustc-link-lib=ibverbs");

    // generate the bindings
    let bindings = bindgen::Builder::default()
        .header("/usr/include/infiniband/verbs.h")
        //.clang_arg("-Ivendor/rdma-core/build/include/")
        // https://github.com/servo/rust-bindgen/issues/550
        .blacklist_type("max_align_t")
        .whitelist_function("ibv_.*")
        .whitelist_type("ibv_.*")
        .bitfield_enum("ibv_access_flags")
        .bitfield_enum("ibv_qp_attr_mask")
        .bitfield_enum("ibv_wc_flags")
        .bitfield_enum("ibv_send_flags")
        .bitfield_enum("ibv_port_cap_flags")
        .constified_enum_module("ibv_qp_type")
        .constified_enum_module("ibv_qp_state")
        .constified_enum_module("ibv_port_state")
        .constified_enum_module("ibv_wc_opcode")
        .constified_enum_module("ibv_wr_opcode")
        .constified_enum_module("ibv_wc_status")
        //.constified_enum_module("IBV_WC_.*")
        //.constified_enum_module("IBV_WR_.*")
        //.constified_enum_module("IBV_QPS_.*")
        //.constified_enum_module("IBV_PORT_.*")
        .derive_default(true)
        .derive_debug(true)
        .prepend_enum_name(false)
        .blacklist_type("ibv_wc")
        .generate()
        .expect("Unable to generate bindings");

    // write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
