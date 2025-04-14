use crate::dto::po::ext::generate::collection::{ApplicationInfoPO, EntityInfoPO};
use crate::dto::vo::ext::generate::generate_result::{GenerateResult, GenerateSingleFileResult};
use crate::service::ext::generator::entity_coll::param_build::ParamBuild;
use crate::service::ext::generator::genrate_util::{
    create_folder_strut_by_template_folder, generator,
};
use crate::service::ext::generator::write_dir::folder_zip;
use crate::util::file_util::{get_file_extension, recursion_get_file_by_folder, rename_file_placeholder};
use ::entity::entity::project;
use sea_orm::*;
use tcdt_common::file_util::get_file_separator;
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tera::{Context, Tera};
use crate::util::id_util::generate_id;

const DIR_TEMPLATE_SINGLE_FILE: &str = "app_single_file";
const DIR_TEMPLATE_FULL: &str = "app_full";
const DIR_TEMPLATE_PART: &str = "app_part";

pub async fn generate_single_file(
    db: &DbConn,
    id_coll: String,
) -> Result<GenerateSingleFileResult, TcdtServiceError> {
    let param_build = ParamBuild::new(db, id_coll.clone()).await?;
    let application_info = param_build.build(id_coll.clone())?;
    let project_entity = param_build.coll_cache.project_entity.clone();
    let template_code =
        project_entity
            .template_code
            .clone()
            .ok_or(TcdtServiceError::build_custom_msg(
                "project entity template code is Empty",
            ))?;
    let nanoid_dir = generate_id();
    create_folder_strut_by_template_folder(
        &TCDT_CONF.code_template_path.to_string(),
        &template_code,
        DIR_TEMPLATE_SINGLE_FILE,
        &TCDT_CONF.code_generate_path.to_string(),
        &nanoid_dir,
    )?;

    //生成代码公共路径
    let target_common_path = format!(
        "{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        nanoid_dir
    );
    // common relative path
    let common_relative_path = format!("{}{}", DIR_TEMPLATE_SINGLE_FILE, get_file_separator());
    // target  generate common path
    let target_generate_common_path = format!(
        "{}{}",
        target_common_path,
        get_file_separator(),
    );
    // template common path
    let template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        common_relative_path
    );
    let generate_file_full_name = generate_singe_file(
        &target_generate_common_path,
        &template_common_path,
        &application_info,
    )?;

    let file_extension = get_file_extension(&generate_file_full_name).unwrap_or("unknown");
    let file_name = format!(
        "{}.{}",
        application_info
            .display_name
            .clone()
            .unwrap_or(String::from("unknown")),
        file_extension,
    );
    Ok(GenerateSingleFileResult {
        file_name,
        file_full_name: generate_file_full_name,
        generate_target_dir: target_common_path,
    })
}

pub async fn generate_full(
    db: &DbConn,
    id_coll: String,
) -> Result<GenerateResult, TcdtServiceError> {
    let param_build = ParamBuild::new(db, id_coll.clone()).await?;
    let application_info = param_build.build(id_coll.clone())?;
    let application_info = fill_application_info_param_json(application_info)?;
    let project_entity = param_build.coll_cache.project_entity.clone();
    let template_code =
        project_entity
            .template_code
            .clone()
            .ok_or(TcdtServiceError::build_custom_msg(
                "project entity template code is Empty",
            ))?;

    let nanoid_dir = generate_id();
    create_folder_strut_by_template_folder(
        &TCDT_CONF.code_template_path.to_string(),
        &template_code,
        DIR_TEMPLATE_FULL,
        &TCDT_CONF.code_generate_path.to_string(),
        &nanoid_dir,
    )?;
    //生成代码公共路径
    let target_common_path = format!(
        "{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        nanoid_dir
    );
    // common relative path
    let common_relative_path = format!("{}{}", DIR_TEMPLATE_FULL, get_file_separator());
    // target  generate common path
    let target_generate_common_path = format!(
        "{}{}",
        target_common_path,
        get_file_separator(),
    );
    // template common path
    let template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        common_relative_path
    );
    generate_application_code(
        &target_generate_common_path,
        &template_common_path,
        &application_info,
    )?;

    let zip_file_name = format!(
        "{}.zip",
        application_info
            .display_name
            .clone()
            .unwrap_or(String::from("unknown")),
    );
    let zip_file_full_name = format!(
        "{}{}{}",
        target_common_path,
        get_file_separator(),
        zip_file_name,
    );
    log::info!("zip file full name is: {}", zip_file_full_name);
    folder_zip(&target_generate_common_path, &zip_file_full_name)?;

    Ok(GenerateResult {
        zip_file_name,
        zip_file_full_name,
        generate_target_dir: target_common_path,
    })
}

pub async fn generate_part(
    db: &DbConn,
    id_coll: String,
) -> Result<GenerateResult, TcdtServiceError> {
    let param_build = ParamBuild::new(db, id_coll.clone()).await?;
    let application_info = param_build.build(id_coll.clone())?;
    let project_entity = param_build.coll_cache.project_entity.clone();
    let template_code =
        project_entity
            .template_code
            .clone()
            .ok_or(TcdtServiceError::build_custom_msg(
                "project entity template code is Empty",
            ))?;
    let nanoid_dir = generate_id();
    let target_code_dir = application_info
        .package_name
        .clone()
        .unwrap_or(String::from("unknown"));
    let target_folder_name = format!("{}{}{}", nanoid_dir, get_file_separator(), target_code_dir);
    create_folder_strut_by_template_folder(
        &TCDT_CONF.code_template_path.to_string(),
        &template_code,
        DIR_TEMPLATE_PART,
        &TCDT_CONF.code_generate_path.to_string(),
        &target_folder_name,
    )?;

    let target_common_path = format!(
        "{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        target_folder_name
    );
    // common relative path
    let common_relative_path = format!("{}{}", DIR_TEMPLATE_PART, get_file_separator());
    // target generate common path
    let target_generate_common_path = format!("{}{}", target_common_path, get_file_separator(),);
    // template common path
    let template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        common_relative_path
    );

    for entity_po in application_info.entities {
        let entity_po = fill_entity_info_param_json(entity_po)?;
        generate_entity_code(
            &target_generate_common_path,
            &template_common_path,
            &entity_po,
        )?;
    }

    let zip_file_name = format!(
        "{}.zip",
        application_info
            .display_name
            .clone()
            .unwrap_or(String::from("unknown")),
    );
    let zip_file_full_name = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        nanoid_dir,
        get_file_separator(),
        zip_file_name,
    );
    log::info!("zip file full name is: {}", zip_file_full_name);
    let download_path = format!(
        "{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        nanoid_dir,
    );
    folder_zip(&download_path, &zip_file_full_name)?;

    Ok(GenerateResult {
        zip_file_name,
        zip_file_full_name,
        generate_target_dir: target_common_path,
    })
}

fn generate_singe_file(
    target_path: &str,
    template_file_path: &str,
    application_info: &ApplicationInfoPO,
) -> Result<String, TcdtServiceError> {
    let mut context = Context::new();
    context.insert("rootInfo", application_info);
    let mut tera = Tera::default();
    let template_file_full_name_list = recursion_get_file_by_folder(&template_file_path)?;
    if template_file_full_name_list.len() > 1 {
        return Err(TcdtServiceError::build_internal_msg(&format!(
            "{} folder not exists template file",
            DIR_TEMPLATE_SINGLE_FILE
        )));
    }
    if template_file_full_name_list.len() > 1 {
        return Err(TcdtServiceError::build_internal_msg(&format!(
            "{} folder more than one template file",
            DIR_TEMPLATE_SINGLE_FILE
        )));
    }
    let template_file_full_name = template_file_full_name_list.first().unwrap().to_owned();
    tera.add_template_file(&template_file_full_name, Some(&template_file_full_name))
        .map_err(|err| {
            TcdtServiceError::build_custom_msg_error(
                &format!("add template file: '{}' error", template_file_full_name),
                err,
            )
        })?;
    let generate_file_full_name = generator(
        &target_path,
        &template_file_path,
        &template_file_full_name,
        &context,
        &tera,
    )?;
    let new_file_name = rename_file_placeholder(&generate_file_full_name, "_{displayName}_", &application_info.display_name.clone().unwrap())?;
    Ok(generate_file_full_name)
}

fn generate_application_code(
    target_path: &str,
    template_file_path: &str,
    application_info: &ApplicationInfoPO,
) -> Result<(), TcdtServiceError> {
    let mut context = Context::new();
    context.insert("rootInfo", application_info);
    let mut tera = Tera::default();
    let template_file_full_name_list = recursion_get_file_by_folder(&template_file_path)?;
    for template_file_full_name in template_file_full_name_list.clone() {
        tera.add_template_file(&template_file_full_name, Some(&template_file_full_name))
            .map_err(|err| {
                TcdtServiceError::build_custom_msg_error(
                    &format!("add template file: '{}' error", template_file_full_name),
                    err,
                )
            })?;
    }
    for template_file_full_name in template_file_full_name_list {
        let generate_file_path = generator(
            &target_path,
            &template_file_path,
            &template_file_full_name,
            &context,
            &tera,
        )?;
        rename_file_placeholder(&generate_file_path, "_{displayName}_", &application_info.display_name.clone().unwrap())?;
    }
    Ok(())
}

fn generate_entity_code(
    target_path: &str,
    template_file_path: &str,
    entity_info_po: &EntityInfoPO,
) -> Result<(), TcdtServiceError> {
    let mut context = Context::new();
    context.insert("rootInfo", &entity_info_po);
    let mut tera = Tera::default();
    let template_file_full_name_list = recursion_get_file_by_folder(&template_file_path)?;
    for template_file_full_name in template_file_full_name_list.clone() {
        tera.add_template_file(&template_file_full_name, Some(&template_file_full_name))
            .map_err(|err| {
                TcdtServiceError::build_custom_msg_error(
                    &format!("add template file: '{}' error", template_file_full_name),
                    err,
                )
            })?;
    }
    for template_file_full_name in template_file_full_name_list {
        let generate_file_path = generator(
            &target_path,
            &template_file_path,
            &template_file_full_name,
            &context,
            &tera,
        )?;
        rename_file_placeholder(&generate_file_path, "_{camelCase}_", &entity_info_po.camel_case_name.clone().unwrap())?;
        rename_file_placeholder(&generate_file_path, "_{pascalCase}_", &entity_info_po.pascal_case_name.clone().unwrap())?;
        rename_file_placeholder(&generate_file_path, "_{snakeCase}_", &entity_info_po.snake_case_name.clone().unwrap())?;
        rename_file_placeholder(&generate_file_path, "_{macroCase}_", &entity_info_po.macro_case_name.clone().unwrap())?;
        rename_file_placeholder(&generate_file_path, "_{displayName}_", &entity_info_po.display_name.clone().unwrap())?;
    }
    Ok(())
}

fn get_base_file_name(project_entity: &project::Model, entity_po: &EntityInfoPO) -> String {
    let entity_name: String;
    if project_entity.file_name_type == Some("SnakeCase".to_string()) {
        entity_name = entity_po.snake_case_name.clone().unwrap_or_default();
    } else {
        entity_name = entity_po.pascal_case_name.clone().unwrap_or_default();
    }
    entity_name
}

fn fill_entity_info_param_json(entity_po: EntityInfoPO) -> Result<EntityInfoPO, TcdtServiceError> {
    let mut entity_po = entity_po;
    if TCDT_CONF.enable_code_generate_debug_mode {
        let param_json = serde_json::to_string_pretty(&entity_po).map_err(|err| {
            log::error!("application_info Serialize to param_json error");
            TcdtServiceError::build_internal_msg_error(
                "application_info Serialize to param_json error",
                err,
            )
        })?;
        entity_po.param_json = Some(param_json);
    }
    Ok(entity_po)
}

fn fill_application_info_param_json(
    application_info: ApplicationInfoPO,
) -> Result<ApplicationInfoPO, TcdtServiceError> {
    let mut application_po = application_info;
    if TCDT_CONF.enable_code_generate_debug_mode {
        let param_json = serde_json::to_string_pretty(&application_po).map_err(|err| {
            log::error!("entity_po Serialize to param_json error");
            TcdtServiceError::build_internal_msg_error(
                "entity_po Serialize to param_json error",
                err,
            )
        })?;
        application_po.param_json = Some(param_json);
    }
    Ok(application_po)
}
