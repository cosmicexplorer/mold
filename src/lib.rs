use std::io;
use std::path::{Path, PathBuf};

pub fn make_executable(
  object: &Path,
  clang_rt: &Path,
  out_path: &Path,
) -> io::Result<Path> {
  // Produce a mach-o main executable that has file type MH_EXECUTE.
  // Read in the object file.
  // Add symbols from the -lSystem library.
  // Add symbols from the clang runtime archive libclang_rt.osx.a.
  // Produce a binary for the x86_64 architecture.
  Ok(out_path)
}
