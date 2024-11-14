use std::collections::HashMap;
use tera::from_value;
use tera::to_value;
use tera::Context;
use tera::Function;
use tera::Result as TeraResult;
use tera::Tera;
use tera::Value;

use crate::util::{
    recursion_use_mod_str, scan_dir_and_file, write_to_file, GenerateInfo, NameInfo,
};

/// genrate api register api_register.rs by scan
pub(crate) fn api_register_generate(tera: &mut Tera) {
    let template_file_name = "api_register.tera";
    let generate_file_path = "api/src/api_register.rs";
    let context = Context::new();
    let api_file_name_list = scan_dir_and_file("api/src/api", true);
    let generate_info = GenerateInfo {
        context,
        generate_file_path: generate_file_path.to_owned(),
        template_file_name: template_file_name.to_owned(),
    };
    let mut param = HashMap::new();
    param.insert("api_name_info_list_key".to_string(), api_file_name_list);
    tera.register_function("recursion_use_mod", recursion_use_mod(param.clone()));
    tera.register_function(
        "recursion_api_route_register",
        recursion_api_route_register(param),
    );
    write_to_file(generate_info, tera);
}

fn recursion_use_mod(name_info_list_map: HashMap<String, Vec<NameInfo>>) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => {
                    if v == "api_name_info_list_key" {
                        let mod_str =
                            recursion_use_mod_str(name_info_list_map.get(&v).unwrap(), "    ");
                        return Ok(to_value(mod_str).unwrap());
                    }
                    let mod_str =
                        recursion_use_mod_str(name_info_list_map.get(&v).unwrap(), "    ");
                    Ok(to_value(mod_str).unwrap())
                }
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}

fn recursion_api_route_register(
    name_info_list_map: HashMap<String, Vec<NameInfo>>,
) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => {
                    if v == "api_name_info_list_key" {
                        let mod_str = recursion_register_str(name_info_list_map.get(&v).unwrap());
                        return Ok(to_value(mod_str).unwrap());
                    }
                    let mod_str = recursion_register_str(name_info_list_map.get(&v).unwrap());
                    Ok(to_value(mod_str).unwrap())
                }
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}

fn recursion_register_str(name_info_list: &Vec<NameInfo>) -> String {
    let render_str_list: Vec<String> = name_info_list
        .iter()
        .map(|name_info| {
            if name_info.fg_dir {
                return recursion_register_str(&name_info.children);
            } else {
                let mut register_methods: Vec<String> = vec![];
                for route_method in name_info.route_methods.clone() {
                    register_methods.push(format!(
                        "    cfg.service({}::{});\r\n",
                        name_info.pascal_name, route_method,
                    ));
                }
                return register_methods.join("");
            }
        })
        .collect();
    render_str_list.join("")
}
