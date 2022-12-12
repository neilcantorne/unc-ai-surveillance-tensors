use std::path::{ Path, PathBuf };
use std::process::Command;

fn main() {
    build_cl_sources();
}

fn build_cl_sources() {
    let rgb_ir = cl_to_llvm_ir(Path::new("rgb.cl"));
    let cnn_kernel_ir = cl_to_llvm_ir(Path::new("cnn_kernel.cl"));
    link_to_spirv([&rgb_ir, &cnn_kernel_ir], "cnn_kernel");
    std::fs::remove_file(cnn_kernel_ir).unwrap();
    std::fs::remove_file(rgb_ir).unwrap();
}

#[allow(unused_must_use)]
fn cl_to_llvm_ir(source: &Path) -> PathBuf {
    let target = std::env::current_dir()
    .unwrap()
    .join("src/kernels/spirv/")
    .join(source)
    .with_extension("bc");

    std::fs::remove_file(&target);

    let err = Command::new("clang")
    .current_dir(std::env::current_dir()
            .unwrap()
            .join("src/kernels/"))
        .args(["-c",
        "-target",
        "spir64",
        "-O2",
        "-emit-llvm",
        "-o",
        target.to_str().unwrap(),
        source.to_str().unwrap()]).output().unwrap();

    if !err.stderr.is_empty() {
        panic!("{}", String::from_utf8(err.stderr).unwrap())
    }

    return target;
}

fn link_to_spirv<const N: usize>(sources: [&Path; N], target: &str) {
    let mut args = sources.iter()
        .map(|item| {
            item.to_str()
            .unwrap()
            .to_owned()
        })
        .collect::<Vec<String>>();

    let target_ir = Path::join(Path::new("spirv/"), target)
        .with_extension("bc");

    args.push("-o".to_owned());
    args.push(target_ir.to_str().unwrap().to_owned());

    let err = Command::new("llvm-link")
    .current_dir(std::env::current_dir()
            .unwrap()
            .join("src/kernels/"))
        .args(&args)
        .output()
        .unwrap();

    if !err.stderr.is_empty() {
        panic!("{}", String::from_utf8(err.stderr).unwrap())
    }

    let err = Command::new("llvm-link")
    .current_dir(std::env::current_dir()
            .unwrap()
            .join("src/kernels/"))
        .args([
            target_ir.to_str().unwrap(),
            "-o",
            target_ir.with_extension("spv")
            .to_str()
            .unwrap()
        ])
        .output()
        .unwrap();

    if !err.stderr.is_empty() {
        panic!("{}", String::from_utf8(err.stderr).unwrap())
    }
}
