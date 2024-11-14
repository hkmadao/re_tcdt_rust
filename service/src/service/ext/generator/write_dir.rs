use clap::{Parser, ValueEnum};
use std::io::prelude::*;
use zip::{result::ZipError, write::SimpleFileOptions};

use std::fs::File;
use std::path::{Path, PathBuf};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    // Source directory
    source: PathBuf,
    // Destination zipfile
    destination: PathBuf,
    // Compression method
    #[arg(value_enum)]
    compression_method: CompressionMethod,
}

#[derive(Clone, ValueEnum)]
enum CompressionMethod {
    Stored,
    Deflated,
    DeflatedZlib,
    DeflatedZlibNg,
    Bzip2,
    Zstd,
}

fn main() {
    let args = Args {
        source: PathBuf::from("temp/source"),
        destination: PathBuf::from("temp/dest/demo.zip"),
        compression_method: CompressionMethod::Stored,
    };
    match_compression_method(args).unwrap();
}

pub fn folder_zip(source: &str, destination: &str) -> Result<(), TcdtServiceError> {
    let args = Args {
        source: PathBuf::from(source),
        destination: PathBuf::from(destination),
        compression_method: CompressionMethod::Stored,
    };
    match_compression_method(args)?;
    Ok(())
}

fn match_compression_method(args: Args) -> Result<(), TcdtServiceError> {
    let src_dir = &args.source;
    let dst_file = &args.destination;
    let method = match args.compression_method {
        CompressionMethod::Stored => zip::CompressionMethod::Stored,
        CompressionMethod::Deflated => {
            #[cfg(not(feature = "deflate-flate2"))]
            {
                println!("The `deflate-flate2` feature is not enabled");
                return Err(TcdtServiceError::build_internal_msg(
                    "The `deflate-flate2` feature is not enabled",
                ));
            }
            #[cfg(feature = "deflate-flate2")]
            zip::CompressionMethod::Deflated
        }
        CompressionMethod::DeflatedZlib => {
            #[cfg(not(feature = "deflate-zlib"))]
            {
                println!("The `deflate-zlib` feature is not enabled");
                return Err(TcdtServiceError::build_internal_msg(
                    "The `deflate-zlib` feature is not enabled",
                ));
            }
            #[cfg(feature = "deflate-zlib")]
            zip::CompressionMethod::Deflated
        }
        CompressionMethod::DeflatedZlibNg => {
            #[cfg(not(feature = "deflate-zlib-ng"))]
            {
                println!("The `deflate-zlib-ng` feature is not enabled");
                return Err(TcdtServiceError::build_internal_msg(
                    "The `deflate-zlib-ng` feature is not enabled",
                ));
            }
            #[cfg(feature = "deflate-zlib-ng")]
            zip::CompressionMethod::Deflated
        }
        CompressionMethod::Bzip2 => {
            #[cfg(not(feature = "bzip2"))]
            {
                println!("The `bzip2` feature is not enabled");
                return Err(TcdtServiceError::build_internal_msg(
                    "The `bzip2` feature is not enabled",
                ));
            }
            #[cfg(feature = "bzip2")]
            zip::CompressionMethod::Bzip2
        }
        CompressionMethod::Zstd => {
            #[cfg(not(feature = "zstd"))]
            {
                println!("The `zstd` feature is not enabled");
                return Err(TcdtServiceError::build_internal_msg(
                    "The `zstd` feature is not enabled",
                ));
            }
            #[cfg(feature = "zstd")]
            zip::CompressionMethod::Zstd
        }
    };
    check_and_zip(src_dir, dst_file, method)?;

    log::info!("done: {:?} written to {:?}", src_dir, dst_file);

    Ok(())
}

fn check_and_zip(
    src_dir: &Path,
    dst_file: &Path,
    method: zip::CompressionMethod,
) -> Result<(), TcdtServiceError> {
    if !Path::new(src_dir).is_dir() {
        return Err(TcdtServiceError::build_internal_msg_error(
            "src not a dir",
            ZipError::FileNotFound,
        ));
    }

    let path = Path::new(dst_file);
    let file = File::create(path).map_err(|err| {
        TcdtServiceError::build_internal_msg_error(&format!("create file: '{:?}' error", path), err)
    })?;

    let walk_dir = WalkDir::new(src_dir);
    let it = walk_dir.into_iter();

    zip_from_iterator(
        &mut it.filter_map(|e| e.ok()),
        src_dir,
        file,
        Some(path),
        method,
    )
    .map_err(|err| {
        TcdtServiceError::build_internal_msg_error(&format!("do zip '{:?}' failed", src_dir), err)
    })?;

    Ok(())
}

fn zip_from_iterator<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &Path,
    writer: T,
    zip_file_path: Option<&Path>,
    method: zip::CompressionMethod,
) -> Result<(), TcdtServiceError>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let prefix = Path::new(prefix);
    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();
        let path_as_string =
            name.to_str()
                .map(str::to_owned)
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "{name:?} Is a Non UTF-8 Path"
                )))?;

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            if let Some(zip_file_path) = zip_file_path {
                if zip_file_path.eq(path) {
                    log::debug!("ignore file {path:?} ...");
                    continue;
                }
            }
            log::debug!("adding file {path:?} as {name:?} ...");
            zip.start_file(path_as_string.clone(), options)
                .map_err(|err| {
                    TcdtServiceError::build_internal_msg_error(
                        &format!("zip start_file: '{}’ error", path_as_string),
                        err,
                    )
                })?;
            let mut f = File::open(path).map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    &format!("open file: '{:?}’ error", path),
                    err,
                )
            })?;

            f.read_to_end(&mut buffer).map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    &format!("file read_to_end: '{:?}’ error", path),
                    err,
                )
            })?;
            zip.write_all(&buffer).map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    &format!("zip write_all: '{:?}’ error", prefix),
                    err,
                )
            })?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path_as_string:?} as {name:?} ...");
            zip.add_directory(path_as_string.clone(), options)
                .map_err(|err| {
                    TcdtServiceError::build_internal_msg_error(
                        &format!("zip add_directory: '{}’ error", path_as_string),
                        err,
                    )
                })?;
        }
    }
    zip.finish().map_err(|err| {
        TcdtServiceError::build_internal_msg_error(
            &format!("zip finish: '{:?}’ error", prefix),
            err,
        )
    })?;
    Ok(())
}
