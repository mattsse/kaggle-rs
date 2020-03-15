use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArchiveMode {
    Tar,
    Zip,
    Skip,
}

impl ArchiveMode {
    /// Create either a tar or zip file of the provided source path
    pub fn make_archive(
        &self,
        _src: impl AsRef<Path>,
        _to: impl AsRef<Path>,
    ) -> anyhow::Result<Option<PathBuf>> {
        // TODO implement
        match self {
            ArchiveMode::Tar => {}
            ArchiveMode::Zip => {}
            ArchiveMode::Skip => {}
        }
        Ok(None)
    }
}

impl Default for ArchiveMode {
    fn default() -> Self {
        ArchiveMode::Skip
    }
}

pub(crate) fn unzip(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = file.as_ref();

    let file = fs::File::open(file)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file.sanitized_name();

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
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
