use std::collections::HashMap;
use tera::from_value;
use tera::to_value;
use tera::Context;
use tera::Function;
use tera::Result as TeraResult;
use tera::Tera;
use tera::Value;

use crate::util::{
    recursion_export_mod_str, scan_dir_and_file, write_to_file,
    GenerateInfo, NameInfo,
};

/// generate service lib.rs by scan
pub(crate) fn service_lib_generate(tera: &mut Tera) {
    let template_file_name = "service_lib.tera";
    let generate_file_path = "service/src/lib.rs";
    let context = Context::new();
    let service_file_name_list = scan_dir_and_file("service/src/service", false);
    let dto_file_name_list = scan_dir_and_file("service/src/dto", false);

    let generate_info = GenerateInfo {
        context,
        generate_file_path: generate_file_path.to_owned(),
        template_file_name: template_file_name.to_owned(),
    };
    let mut param: HashMap<String, Vec<NameInfo>> = HashMap::new();
    param.insert("service_file_name_list_key".to_string(), service_file_name_list);
    param.insert("dto_file_name_list_key".to_string(), dto_file_name_list);
    tera.register_function("recursion_export_mod", recursion_export_mod(param));
    write_to_file(generate_info, tera);
}

fn recursion_export_mod(name_info_list_map: HashMap<String, Vec<NameInfo>>) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => {
                    if v == "service_file_name_list_key" {
                        let mod_str =
                            recursion_export_mod_str(name_info_list_map.get(&v).unwrap(), 1);
                        return Ok(to_value(mod_str).unwrap());
                    }
                    let mod_str = recursion_export_mod_str(name_info_list_map.get(&v).unwrap(), 1);
                    Ok(to_value(mod_str).unwrap())
                }
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}
