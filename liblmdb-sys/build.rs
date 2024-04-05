extern crate cc;

fn main() {
    let target = std::env::var("TARGET").unwrap();

    let mut build = cc::Build::new();
    build.file("mdb/libraries/liblmdb/mdb.c")
         .file("mdb/libraries/liblmdb/midl.c");
    build.opt_level(2);

    if target.contains("dragonfly") {
        build.flag("-DMDB_DSYNC=O_SYNC");
        build.flag("-DMDB_FDATASYNC=fsync");
    }
    build.compile("liblmdb.a");
}