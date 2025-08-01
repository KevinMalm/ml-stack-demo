use std::{io::Write, path::Path};

fn main() {
    let template_folder = Path::new("templates");
    let template_file = Path::new("src").join("template.rs");

    if template_folder.exists() == false {
        println!("WARNING: Incomplete project folder. No Template directory");
        if let Err(e) = std::fs::create_dir_all(template_folder) {
            println!("{:?}", e);
            panic!("Failed to create the template directory.")
        }
        return;
    }

    let mut template_rs_content = String::new();

    /* Walk through all templates */
    for src in match std::fs::read_dir(template_folder) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            panic!("Failed to read the template directory.")
        }
    } {
        /* Validate File */
        let entry = match src {
            Ok(x) => x.path(),
            Err(e) => {
                println!("{:?}", e);
                panic!("Failed to read one of template directory files")
            }
        };
        if entry.is_file() == false {
            println!("Skipping {}", entry.display());
            continue;
        }
        /* Extract File Name */
        let filename = match entry.file_name() {
            Some(x) => x.to_string_lossy(),
            None => {
                panic!("Failed to pull file name from {}", entry.display());
            }
        };
        let formatted_name: String = filename
            .chars()
            .map(|x| {
                if x.is_ascii_alphabetic() {
                    x.to_ascii_uppercase()
                } else {
                    '_'
                }
            })
            .collect();
        /* Extract File Contents */
        let contents = match std::fs::read_to_string(&entry) {
            Ok(x) => x.escape_default().to_string(),
            Err(e) => {
                println!("{:?}", e);
                panic!("Failed to read the contents of {}", entry.display());
            }
        };
        /* Add to final file */
        template_rs_content.push_str(&format!(
            "pub const {}_FILE_NAME: &str = \"{}\";\n",
            formatted_name, filename
        ));
        template_rs_content.push_str(&format!(
            "pub const {}: &str = \"{}\";\n",
            formatted_name, contents
        ));
    }

    let mut out_file = match std::fs::File::create(&template_file) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            panic!(
                "Failed to create the template .rs file at {}",
                template_file.display()
            );
        }
    };

    if let Err(e) = out_file.write_all(template_rs_content.as_bytes()) {
        println!("{:?}", e);
        panic!(
            "Failed to write the template content into {}",
            template_file.display()
        );
    }
}
