use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::{fs, io};
use walkdir::{DirEntry, WalkDir};
use zip::write::SimpleFileOptions;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum ArchiveMode {
    Tar,
    Zip,
    #[default]
    Skip,
}

impl ArchiveMode {
    /// Create either a tar or zip file of the provided source path
    pub fn make_archive(
        &self,
        src: impl AsRef<Path>,
        to: impl AsRef<Path>,
    ) -> anyhow::Result<Option<PathBuf>> {
        match self {
            ArchiveMode::Tar => {
                let to = PathBuf::from(format!("{}.tar.gz", to.as_ref().display()));
                let file = File::create(&to)?;
                make_tarball(src, file)?;
                Ok(Some(to))
            }
            ArchiveMode::Zip => {
                let src = src.as_ref();
                let to = PathBuf::from(format!("{}.zip", to.as_ref().display()));
                let file = File::create(&to)?;
                let walkdir = WalkDir::new(src);
                let it = walkdir.into_iter();

                zip_dir(&mut it.filter_map(|e| e.ok()), src, file)?;
                Ok(Some(to))
            }
            ArchiveMode::Skip => Ok(None),
        }
    }
}

/// unzip file into location of `to`
pub fn unzip(file: impl AsRef<Path>, to: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = file.as_ref();
    let to = to.as_ref();
    let file = fs::File::open(file)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = to.join(file.mangled_name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

fn zip_dir<T>(
    it: &mut impl Iterator<Item = DirEntry>,
    prefix: impl AsRef<Path>,
    writer: T,
) -> anyhow::Result<()>
where
    T: Write + Seek,
{
    let prefix = prefix.as_ref();
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default().unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix)?;

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

fn make_tarball<T: Write>(src: impl AsRef<Path>, writer: T) -> anyhow::Result<()> {
    let enc = GzEncoder::new(writer, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", src)?;
    Ok(())
}
