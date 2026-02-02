fn main() {
    println!("cargo:rerun-if-changed=src/electrical_synapse.cpp");
    println!("cargo:rerun-if-changed=src/electrical_synapse.h");
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .file("src/electrical_synapse.cpp")
        .compile("electrical_synapse_core");
}
