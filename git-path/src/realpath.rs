use std::path::PathBuf;

/// the error returned by [`realpath()`][super::realpath()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The maximum allowed number {} of symlinks in path is exceeded", .max_symlinks)]
    MaxSymlinksExceeded { max_symlinks: u8 },
    #[error(transparent)]
    ReadLink(#[from] std::io::Error),
    #[error("Empty is not a valid path")]
    EmptyPath,
    #[error("Parent component of {} does not exist", .path.display())]
    MissingParent { path: PathBuf },
}

pub(crate) mod function {
    use super::Error;
    use std::path::Component::{CurDir, Normal, ParentDir, Prefix, RootDir};
    use std::path::{Path, PathBuf};

    /// TODO
    pub fn realpath(path: impl AsRef<Path>, cwd: impl AsRef<Path>, max_symlinks: u8) -> Result<PathBuf, Error> {
        let path = path.as_ref();
        if path.as_os_str().is_empty() {
            return Err(Error::EmptyPath);
        }

        let mut real_path = PathBuf::new();
        if path.is_relative() {
            real_path.push(cwd);
        }

        fn traverse(
            mut input_path: std::path::Components<'_>,
            mut num_symlinks: u8,
            max_symlinks: u8,
            real_path: &mut PathBuf,
        ) -> Result<(), Error> {
            match input_path.next() {
                None => Ok(()),
                Some(part) => match part {
                    RootDir | Prefix(_) => {
                        real_path.push(part);
                        traverse(input_path, num_symlinks, max_symlinks, real_path)
                    }
                    CurDir => traverse(input_path, num_symlinks, max_symlinks, real_path),
                    ParentDir => {
                        if !real_path.pop() {
                            return Err(Error::MissingParent {
                                path: real_path.clone(),
                            });
                        }
                        traverse(input_path, num_symlinks, max_symlinks, real_path)
                    }
                    Normal(part) => {
                        real_path.push(part);
                        if real_path.is_symlink() {
                            num_symlinks += 1;
                            if num_symlinks > max_symlinks {
                                return Err(Error::MaxSymlinksExceeded { max_symlinks });
                            }
                            let mut resolved_symlink = std::fs::read_link(real_path.as_path())?;
                            if resolved_symlink.is_absolute() {
                                *real_path = PathBuf::from(std::path::MAIN_SEPARATOR.to_string());
                            } else {
                                *real_path = real_path
                                    .parent()
                                    .ok_or_else(|| Error::MissingParent {
                                        path: real_path.clone(),
                                    })?
                                    .into();
                            }
                            resolved_symlink.push(input_path.collect::<PathBuf>());
                            traverse(resolved_symlink.components(), num_symlinks, max_symlinks, real_path)
                        } else {
                            traverse(input_path, num_symlinks, max_symlinks, real_path)
                        }
                    }
                },
            }
        }

        traverse(path.components(), 0, max_symlinks, &mut real_path)?;
        Ok(real_path)
    }
}
