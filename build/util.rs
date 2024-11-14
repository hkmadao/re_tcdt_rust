use regex::bytes::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use tera::Context;
use tera::Tera;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct NameInfo {
    pub snake_name: String,
    pub pascal_name: String,
    pub children: Vec<NameInfo>,
    pub fg_dir: bool,
    pub route_methods: Vec<String>,
}

pub(crate) struct GenerateInfo {
    pub context: Context,
    pub generate_file_path: String,
    pub template_file_name: String,
}

/// snake case naming to camel case naming
pub(crate) fn snake_case_to_camel_case(underline: String) -> String {
    underline
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            if let Some(first_char) = chars.next() {
                first_char.to_uppercase().to_string()
            } else {
                "".to_owned()
            }
            .to_string()
                + &chars.as_str().to_lowercase()
        })
        .collect::<String>()
}

fn get_file_separator() -> char {
    if cfg!(target_os = "windows") {
        '\\'
    } else {
        '/'
    }
}

pub(crate) fn scan_dir_and_file(p: &str, fg_api_scan: bool) -> Vec<NameInfo> {
    let api_files_result = fs::read_dir(p);
    let mut file_name_into_list: Vec<NameInfo> = vec![];
    match api_files_result {
        Ok(api_files) => {
            for api_file in api_files {
                if let Ok(api_file) = api_file {
                    let file_name = api_file
                        .file_name()
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .to_owned();
                    let snake_name = file_name.replace(".rs", "");
                    let pascal_name = snake_case_to_camel_case(snake_name.clone());
                    let fg_dir = api_file
                        .metadata()
                        .map(|meta| meta.is_dir())
                        .unwrap_or(false);

                    let mut name_info = NameInfo {
                        snake_name: snake_name.clone(),
                        pascal_name: pascal_name.clone(),
                        children: vec![],
                        fg_dir: false,
                        route_methods: vec![],
                    };
                    if fg_dir {
                        let children_name_info_list = scan_dir_and_file(
                            &format!("{}{}{}", p, get_file_separator(), &snake_name),
                            fg_api_scan,
                        );
                        name_info.fg_dir = true;
                        name_info.children = children_name_info_list;
                    } else {
                        name_info.fg_dir = false;
                        if fg_api_scan {
                            let methods = scan_api_route_export(api_file);
                            name_info.route_methods = methods;
                        }
                    }
                    file_name_into_list.push(name_info);
                }
            }
        }
        Err(e) => {
            println!("scan api error(s): {}", e);
        }
    }
    file_name_into_list
}

fn scan_api_route_export(api_file: fs::DirEntry) -> Vec<String> {
    let file = File::open(api_file.path()).expect("Failed to open file");
    let reader = BufReader::new(file);
    let re = Regex::new(r".*#\[tcdt_route\((.+)\).*\].*").unwrap();
    let mut methods: Vec<String> = vec![];
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if let Some(cap) = re.captures(line.as_bytes()) {
            let (_full, [method_name]) = cap.extract();
            let method_name_vec = method_name.to_vec();
            methods.push(String::from_utf8(method_name_vec).unwrap());
        }
    }
    methods
}

/// write to file
pub(crate) fn write_to_file(generate_info: GenerateInfo, tera: &Tera) {
    let write_result = fs::File::create(generate_info.generate_file_path);
    match write_result {
        Ok(write) => {
            let render_to_result = tera.render_to(
                &generate_info.template_file_name,
                &generate_info.context,
                write,
            );
            match render_to_result {
                Ok(_) => {
                    // println!("generate file successful");
                }
                Err(err) => {
                    println!("render error, {:?}", err);
                    return;
                }
            }
        }
        Err(err) => {
            println!("generate file error, {:?}", err);
            return;
        }
    }
}

pub(crate) fn recursion_export_mod_str(name_info_list: &Vec<NameInfo>, count: usize) -> String {
    let render_str_list: Vec<String> = name_info_list
        .iter()
        .map(|name_info| {
            if name_info.children.len() == 0 {
                return format!("{}pub mod {};", "    ".repeat(count), name_info.snake_name);
            } else {
                return format!(
                    "{}pub mod {}{}{}{}{}{}",
                    "    ".repeat(count),
                    name_info.snake_name,
                    " {\r\n",
                    recursion_export_mod_str(&name_info.children, count + 1),
                    "\r\n",
                    "    ".repeat(count),
                    "}"
                );
            }
        })
        .collect();
    render_str_list.join("\r\n")
}

pub(crate) fn recursion_use_mod_str(name_info_list: &Vec<NameInfo>, parent_name: &str) -> String {
    let render_str_list: Vec<String> = name_info_list
        .iter()
        .map(|name_info| {
            if name_info.fg_dir {
                let full_name = format!("{}{}::", parent_name, name_info.snake_name);
                return format!("{}", recursion_use_mod_str(&name_info.children, &full_name));
            } else {
                return format!(
                    "{}{} as {},",
                    parent_name, name_info.snake_name, name_info.pascal_name,
                );
            }
        })
        .collect();
    render_str_list.join("\r\n")
}
