fn main() {
    println!("cargo:rerun-if-changed=src/electrical_synapse.c");
    println!("cargo:rerun-if-changed=src/electrical_synapse.h");
    cc::Build::new()
        .file("src/electrical_synapse.c")
        .compile("electrical_synapse_core");
}
