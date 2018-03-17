#![feature(slice_concat_ext)]
#![feature(type_ascription)]

extern crate pathdiff;
extern crate reqwest;
extern crate tar;
extern crate tempdir;

use std::collections::HashMap;
use std::convert::From;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::os::unix::io::FromRawFd;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use std::time;

use reqwest::{IntoUrl, Response};
use tempdir::TempDir;

#[derive(Debug)]
pub enum FetchError {
  IoError(io::Error),
  RequestError(reqwest::Error),
}

impl From<io::Error> for FetchError {
  fn from(error: io::Error) -> Self {
    FetchError::IoError(error)
  }
}

impl From<reqwest::Error> for FetchError {
  fn from(error: reqwest::Error) -> Self {
    FetchError::RequestError(error)
  }
}

pub fn fetch_decompress<T: IntoUrl>(
  url: T,
) -> reqwest::Result<Response> {
  let client = reqwest::Client::builder()
    .timeout(time::Duration::new(120, 0))
    .gzip(true)
    .build()?;
  client.get(url).send()
}

pub fn extract_into<T: Read>(stream: T, dest_dir: PathBuf) -> io::Result<()> {
  let mut ar = tar::Archive::new(stream);
  ar.unpack(dest_dir)
}

pub fn fetch_and_extract<T: IntoUrl>(
  url: T,
  dest_dir: PathBuf,
) -> Result<(), FetchError> {
  let gz_stream = fetch_decompress(url)?;
  extract_into(gz_stream, dest_dir)?;
  Ok(())
}

#[derive(Debug)]
pub enum BuildError {
  ProcessInvocationError(io::Error),
  FailedCommand(String),
}

impl From<io::Error> for BuildError {
  fn from(error: io::Error) -> Self {
    BuildError::ProcessInvocationError(error)
  }
}

fn run_command(
  exe_name_or_path: &PathBuf,
  argv_not_first: &Vec<String>,
  cwd: &PathBuf,
  vars: &HashMap<String, String>,
) -> Result<ExitStatus, BuildError> {
  let status: ExitStatus = unsafe {
    Command::new(exe_name_or_path)
      .args(argv_not_first)
      .current_dir(cwd)
      .envs(vars)
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .status()?
  };
  if !status.success() {
    let cmd_str: String = argv_not_first.iter().fold(
      String::from(exe_name_or_path.to_str().unwrap()),
      |cmd, arg| format!("{} {}", cmd, arg),
    );
    Err(BuildError::FailedCommand(format!(
      "command '{}' failed: {}",
      cmd_str, status,
    )))
  } else {
    Ok(status)
  }
}

pub fn run_configure(
  prefix_dir: &PathBuf,
  build_dir: &PathBuf,
  source_dir: &PathBuf,
  args: &Vec<String>,
  vars: &HashMap<String, String>,
) -> Result<ExitStatus, BuildError> {
  let rel_path_to_source: PathBuf =
    pathdiff::diff_paths(&source_dir, &build_dir).unwrap();
  let configure_path: PathBuf =
    [rel_path_to_source, PathBuf::from("configure")]
      .iter()
      .collect();
  let mut all_configure_args = args.clone();
  eprintln!("!!!");
  all_configure_args.insert(0, format!("--prefix={:?}", prefix_dir));
  Ok(run_command(
    &configure_path,
    &all_configure_args,
    &build_dir,
    &vars,
  )?)
}

pub fn run_make(
  cwd: &PathBuf,
  args: &Vec<String>,
  vars: &HashMap<String, String>,
  parallelism: u8,
) -> Result<ExitStatus, BuildError> {
  let mut all_make_args = args.clone();
  all_make_args.insert(0, format!("-j{}", parallelism.to_string()));
  Ok(run_command(
    &PathBuf::from("make"),
    &all_make_args,
    &cwd,
    &vars,
  )?)
}

#[derive(Debug)]
pub enum BuildAutotoolsDependencyError {
  FetchErr(FetchError),
  BuildErr(BuildError),
}

impl From<FetchError> for BuildAutotoolsDependencyError {
  fn from(error: FetchError) -> Self {
    BuildAutotoolsDependencyError::FetchErr(error)
  }
}

impl From<BuildError> for BuildAutotoolsDependencyError {
  fn from(error: BuildError) -> Self {
    BuildAutotoolsDependencyError::BuildErr(error)
  }
}

impl From<io::Error> for BuildAutotoolsDependencyError {
  fn from(error: io::Error) -> Self {
    BuildAutotoolsDependencyError::FetchErr(FetchError::IoError(error))
  }
}

pub fn fetch_build_autotools_dep<T: IntoUrl>(
  url: T,
  outdir: PathBuf,
  src_dirname: PathBuf,
  configure_args: Vec<String>,
  env_vars: HashMap<String, String>,
  parallelism: u8,
) -> Result<PathBuf, BuildAutotoolsDependencyError> {
  let outdir_abs = fs::canonicalize(outdir)?;
  let dl_dir = TempDir::new("autotools-dl")?;
  eprintln!("dl_dir.path().to_path_buf(): {:?}", dl_dir.path().to_path_buf());
  fetch_and_extract(url, dl_dir.path().to_path_buf())?;
  let downloaded_source_abs = fs::canonicalize(
    [&dl_dir.path().to_path_buf(), &src_dirname]
      .iter()
      .collect(): PathBuf,
  )?;
  eprintln!("downloaded_source_abs: {:?}", downloaded_source_abs);
  let build_dir = TempDir::new("autotools-build")?;
  eprintln!("build_dir.path().to_path_buf(): {:?}", build_dir.path().to_path_buf());

  run_configure(
    &outdir_abs,
    &build_dir.path().to_path_buf(),
    &downloaded_source_abs,
    &configure_args,
    &env_vars,
  )?;

  eprintln!("still alive!");

  run_make(
    &build_dir.path().to_path_buf(),
    &vec![String::from("install")],
    &env_vars,
    parallelism,
  )?;
  Ok(outdir_abs)
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
