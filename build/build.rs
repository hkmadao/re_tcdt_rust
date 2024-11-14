use tera::Tera;
mod api_lib;
mod api_register;
mod entity_lib;
mod meta_init;
mod service_lib;
mod util;

use api_lib::api_lib_generate;
use api_register::api_register_generate;
use entity_lib::entity_lib_generate;
use meta_init::entity_meta_init_generate;
use service_lib::service_lib_generate;

fn main() {
    println!("cargo:rerun-if-changed=NULL");
    lib_generate();
}

fn lib_generate() {
    let mut tera = match Tera::new("build/templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            return;
        }
    };
    entity_lib_generate(&tera);
    entity_meta_init_generate(&tera);
    service_lib_generate(&mut tera);
    api_lib_generate(&mut tera);
    api_register_generate(&mut tera);
}
