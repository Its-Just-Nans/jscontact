use std::fs;
use std::path::Path;

const JSON_DEFAULT_CARD: &str = r#"
    "@type": "Card",
    "version": "1.0",
    "uid": "22B2C7DF-9120-4969-8460-05956FE6B065","#;

const SHOULD_ADD: [&str; 39] = [
    "figure_01.txt",
    "figure_07.txt",
    "figure_08.txt",
    "figure_09.txt",
    "figure_10.txt",
    "figure_11.txt",
    "figure_12.txt",
    "figure_13.txt",
    "figure_14.txt",
    "figure_15.txt",
    "figure_16.txt",
    "figure_17.txt",
    "figure_18.txt",
    "figure_19.txt",
    "figure_20.txt",
    "figure_21.txt",
    "figure_22.txt",
    "figure_23.txt",
    "figure_24.txt",
    "figure_25.txt",
    "figure_26.txt",
    "figure_27.txt",
    "figure_28.txt",
    "figure_29.txt",
    "figure_30.txt",
    "figure_31.txt",
    "figure_32.txt",
    "figure_33.txt",
    "figure_34.txt",
    "figure_35.txt",
    "figure_36.txt",
    "figure_37.txt",
    "figure_38.txt",
    "figure_39.txt",
    "figure_40.txt",
    "figure_41.txt",
    "figure_42.txt",
    "figure_43.txt",
    "figure_44.txt",
];

fn main() {
    let raws = fs::read_dir("./tests/rfc9553/raws").unwrap();

    let dest_path = Path::new("./tests/rfc9553/");
    for one_entry in raws {
        let entry = one_entry.unwrap();
        let path = entry.path();
        let mut contents = fs::read_to_string(&path).unwrap();
        let file_name = path.file_name().unwrap();
        let mut file_name = file_name.to_str().unwrap().to_string();
        let mut json_default = JSON_DEFAULT_CARD.to_string();
        match file_name.as_str() {
            // figure_06.txt is already a json file
            "figure_06.txt" => {
                file_name.replace_range(file_name.len() - 4.., ".json");
            }
            // figure_07.txt has already the `version` field
            "figure_07.txt" => {
                json_default = json_default.replace("\n    \"version\": \"1.0\",", "");
            }
            // figure_11.txt and figure_14.txt have already the `uid` field
            "figure_11.txt" | "figure_14.txt" => {
                json_default = json_default.replace(
                    "\n    \"uid\": \"22B2C7DF-9120-4969-8460-05956FE6B065\",",
                    "",
                );
            }
            // https://www.rfc-editor.org/errata/eid8265
            "figure_18.txt" => {
                contents = format!("\"name\": {{\n    {}\n}}", contents);
            }
            // https://www.rfc-editor.org/errata/eid8263
            "figure_20.txt" => {
                contents = contents.replace("{\n  \"@type\": \"Card\",\n", "");
                contents = contents[..contents.len() - 2].to_string();
            }
            // the rfc impose a max line length so a value is split in multiple lines
            "figure_35.txt" => {
                contents = contents.replace("\n            ", "");
            }
            // https://www.rfc-editor.org/errata/eid8266
            "figure_36.txt" => {
                contents = format!("{}\n}}", contents);
            }
            // https://www.rfc-editor.org/errata/eid8264
            "figure_39.txt" => {
                let mut chars = contents.chars();
                chars.next();
                chars.next();
                chars.next_back();
                contents = chars.collect();
                // remove first 4 spaces
                contents = contents
                    .lines()
                    .map(|line| line.chars().skip(2).collect())
                    .collect::<Vec<String>>()
                    .join("\n");
            }
            _ => {}
        }
        if SHOULD_ADD.contains(&file_name.as_str()) {
            let tabbed = contents
                .lines()
                .map(|line| format!("    {}", line))
                .collect::<Vec<String>>()
                .join("\n");
            contents = format!("{{{}\n{}\n}}", json_default, tabbed);
            file_name.replace_range(file_name.len() - 4.., ".json");
        }
        let out_path = dest_path.join(file_name);
        fs::write(&out_path, &contents).unwrap();
    }

    println!("cargo::rerun-if-changed=build.rs");
}
