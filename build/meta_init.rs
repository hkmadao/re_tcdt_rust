use tera::Context;
use tera::Tera;

use crate::util::{scan_dir_and_file, write_to_file, GenerateInfo};

/// generate entity meta_init.rs by scan
pub(crate) fn entity_meta_init_generate(tera: &Tera) {
    let template_file_name = "meta_init.tera";
    let generate_file_path = "entity/src/conf/meta_init.rs";
    let mut context = Context::new();
    let entity_file_name_list = scan_dir_and_file("entity/src/entity", false);
    context.insert("name_info_list", &entity_file_name_list);
    let generate_info = GenerateInfo {
        context,
        generate_file_path: generate_file_path.to_owned(),
        template_file_name: template_file_name.to_owned(),
    };
    write_to_file(generate_info, tera);
}
