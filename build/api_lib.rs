use std::collections::HashMap;
use tera::from_value;
use tera::to_value;
use tera::Context;
use tera::Function;
use tera::Result as TeraResult;
use tera::Tera;
use tera::Value;

use crate::util::{
    recursion_export_mod_str, scan_dir_and_file, write_to_file, GenerateInfo, NameInfo,
};

/// genrate api lib.rs by scan
pub(crate) fn api_lib_generate(tera: &mut Tera) {
    let template_file_name = "api_lib.tera";
    let generate_file_path = "api/src/lib.rs";
    let context = Context::new();
    let api_file_name_list = scan_dir_and_file("api/src/api", false);
    let generate_info = GenerateInfo {
        context,
        generate_file_path: generate_file_path.to_owned(),
        template_file_name: template_file_name.to_owned(),
    };
    let mut param = HashMap::new();
    param.insert("api_name_info_list_key".to_string(), api_file_name_list);
    tera.register_function("recursion_export_mod", recursion_export_mod(param));
    write_to_file(generate_info, tera);
}

fn recursion_export_mod(name_info_list_map: HashMap<String, Vec<NameInfo>>) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => {
                    if v == "api_name_info_list_key" {
                        let mod_str =
                            recursion_export_mod_str(name_info_list_map.get(&v).unwrap(), 2);
                        return Ok(to_value(mod_str).unwrap());
                    }
                    let mod_str = recursion_export_mod_str(name_info_list_map.get(&v).unwrap(), 2);
                    Ok(to_value(mod_str).unwrap())
                }
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}
