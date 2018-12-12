extern crate cc;

fn main() {
    let mut build = cc::Build::new();
    build.opt_level(2);
    build.target("aarch64-pc-windows-msvc");
    build.host("x86_64-pc-windows-msvc");
    let cc = build.get_compiler();
    println!("{:#?}", cc);
}
