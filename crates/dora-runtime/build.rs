use std::{env, fs, path::Path, process::Command};

const MLIR_OPT: &str = "mlir-opt";
const MLIR_TRANSLATE: &str = "mlir-translate";
const LLC: &str = "llc";
const AR: &str = "ar";

const PRECOMPILE_FILES: &[&str] = &["sha2_256.mlir"];

fn precompiles_mlir() {
    let precompiles_folder = Path::new(".").join("src").join("precompiles");
    let out_dir = env::var("OUT_DIR").unwrap();
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    for file in PRECOMPILE_FILES {
        let mlir_file = precompiles_folder.join(file);
        let file_stem = mlir_file.file_stem().unwrap().to_str().unwrap();
        let llvm_mlir_file = temp_dir.path().join(format!("{}_llvm.mlir", file_stem));
        let ll_file = temp_dir.path().join(format!("{}.ll", file_stem));
        let output_file = temp_dir.path().join(format!("{}.o", file_stem));
        let lib_file = Path::new(&out_dir).join(format!("lib{}.a", file_stem));

        Command::new(MLIR_OPT)
            .args([
                mlir_file.to_str().unwrap(),
                "--convert-scf-to-cf",
                "--convert-arith-to-llvm",
                "--convert-math-to-llvm",
                "--convert-index-to-llvm",
                "--convert-func-to-llvm",
                "--finalize-memref-to-llvm",
                "--reconcile-unrealized-casts",
                "-o",
                llvm_mlir_file.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to compile MLIR to LLVM IR");

        Command::new(MLIR_TRANSLATE)
            .args([
                "--mlir-to-llvmir",
                llvm_mlir_file.to_str().unwrap(),
                "-o",
                ll_file.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to translate LLVM dialect MLIR to LLVM IR");

        Command::new(LLC)
            .args([
                ll_file.to_str().unwrap(),
                "-filetype=obj",
                "-o",
                output_file.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to compile LLVM IR to object file");

        Command::new(AR)
            .args([
                "crus",
                lib_file.to_str().unwrap(),
                output_file.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to create static library");

        let final_output = Path::new(&out_dir).join(format!("{}.o", file_stem));
        fs::copy(&output_file, &final_output).expect("Failed to copy object file to out directory");

        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static={}", file_stem);

        println!("cargo:rerun-if-changed={}", mlir_file.display());
    }
}

fn main() {
    precompiles_mlir();
}
