pub mod formatting {
    /// Turn a string into a standard folder name
    pub fn format_folder(name: &str) -> String {
        let mut converted = name.to_string();
        for c in ["_", " ", "."] {
            converted = converted.replace(c, "-");
        }
        converted.to_lowercase()
    }

    /// Turn a string into a standard python folder name
    pub fn format_py_folder(name: &str) -> String {
        let mut converted = name.to_string();
        for c in ["-", " ", "."] {
            converted = converted.replace(c, "_");
        }
        converted.to_lowercase()
    }

    pub fn to_dot_case(content: &str) -> String {
        let mut capital_divided = String::with_capacity(content.len());
        /* Prepare the string by dividing out the uppercase */
        for x in content.chars() {
            if x.is_uppercase() {
                capital_divided.push('-');
            }

            match x {
                '_' | '.' | ' ' => {
                    capital_divided.push('-');
                }
                _ => {
                    capital_divided.push(x);
                }
            }
        }
        /* Format to Dot Case */
        let arr: Vec<String> = capital_divided
            .split('-')
            .filter(|s| !s.is_empty())
            .map(|word| word.to_lowercase())
            .collect();

        arr.join(".")
    }
}

pub mod file {
    use log::{debug, error, warn};
    use std::{io::Write, path::PathBuf};

    pub fn create_dir(path: &PathBuf) -> bool {
        if path.exists() == false {
            if let Err(e) = std::fs::create_dir_all(path) {
                error!("Error while creating folders {}", path.display());
                debug!("{:?}", e);
                return false;
            }
        }
        return true;
    }

    pub fn write_content(path: &PathBuf, content: &str) -> bool {
        let mut out_file = match std::fs::File::create(&path) {
            Ok(x) => x,
            Err(e) => {
                error!("Failed to create file at {}", path.display());
                debug!("{:?}", e);
                return false;
            }
        };

        if let Err(e) = out_file.write_all(content.as_bytes()) {
            error!("Failed to write into  {}", path.display());
            debug!("{:?}", e);
            return false;
        }
        return true;
    }

    pub fn remove(path: &PathBuf) -> bool {
        let res = match path.is_file() {
            true => {
                warn!("Deleting file from the local disk: {}", path.display());
                std::fs::remove_file(path).err()
            }
            false => {
                warn!("Deleting folder from the local disk: {}", path.display());
                std::fs::read_dir(path).err()
            }
        };
        if let Some(x) = &res {
            error!("Failed to remove {}", path.display());
            debug!("{:?}", x);
        }
        res.is_none()
    }
}
