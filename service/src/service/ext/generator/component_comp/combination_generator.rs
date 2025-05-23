use crate::dto::po::ext::generate::component_combination::EntityInfoPO;
use crate::dto::vo::ext::generate::generate_result::GenerateResult;
use crate::service::ext::generator::component_comp::param_build::build;
use crate::service::ext::generator::genrate_util::{
    create_folder_strut_by_template_folder, generator,
};
use crate::service::ext::generator::write_dir::folder_zip;
use crate::service::{
    base::component_service::ComponentQuery, ext::data_type_ext_service::DataTypeExtQuery,
};
use crate::util::file_util::{copy_folder_to_dest, recursion_get_file_by_folder, rename_file_placeholder, rename_sub_folder_placeholder};
use ::entity::entity::{component, component_module, project, sub_project};
use sea_orm::*;
use std::fs;
use tcdt_common::file_util::get_file_separator;
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tera::{Context, Tera};
use crate::util::id_util::generate_id;

const DIR_TEMPLATE: &str = "comp_combination";

const DIR_MASTER: &str = "master";
const DIR_CHILD: &str = "child";
const DIR_EXT: &str = "ext";

pub async fn generate(db: &DbConn, id_coll: String) -> Result<GenerateResult, TcdtServiceError> {
    let coll_entity = ComponentQuery::find_by_id(db, id_coll).await?;
    let project_entity = get_project(db, &coll_entity).await?;
    let template_code =
        project_entity
            .template_code
            .clone()
            .ok_or(TcdtServiceError::build_internal_msg(
                "project entity template code is Empty",
            ))?;
    let column_domain_type_map =
        DataTypeExtQuery::find_and_make_map_by_project_id(db, project_entity.id_project.clone())
            .await?;

    let component_info = build(db, coll_entity, column_domain_type_map).await?;
    let nanoid_dir = generate_id();
    let target_code_dir = component_info
        .package_name
        .clone()
        .unwrap_or(String::from("unknown"));
    let target_folder_name = format!("{}{}{}", nanoid_dir, get_file_separator(), target_code_dir);
    create_folder_strut_by_template_folder(
        &TCDT_CONF.code_template_path.to_string(),
        &template_code,
        DIR_TEMPLATE,
        &TCDT_CONF.code_generate_path.to_string(),
        &target_folder_name,
    )?;
    let target_common_path = format!(
        "{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        target_folder_name
    );
    let dir_template_base = format!(
        "{}{}{}{}",
        DIR_TEMPLATE,
        get_file_separator(),
        DIR_MASTER,
        get_file_separator()
    );
    // base common relative path
    let base_common_relative_path = format!("{}{}", DIR_MASTER, get_file_separator());

    // base target generate common path
    let base_target_generate_common_path = format!(
        "{}{}{}",
        target_common_path,
        get_file_separator(),
        base_common_relative_path
    );

    // base template common path
    let base_template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        dir_template_base
    );

    let dir_template_child = format!(
        "{}{}{}{}",
        DIR_TEMPLATE,
        get_file_separator(),
        DIR_CHILD,
        get_file_separator()
    );
    // child common relative path
    let child_common_relative_path = format!("{}{}", DIR_CHILD, get_file_separator());

    // child target generate common path
    let child_target_generate_common_path = format!(
        "{}{}{}",
        target_common_path,
        get_file_separator(),
        child_common_relative_path
    );

    // child template common path
    let child_template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        dir_template_child
    );

    let dir_template_ext = format!(
        "{}{}{}{}",
        DIR_TEMPLATE,
        get_file_separator(),
        DIR_EXT,
        get_file_separator()
    );
    // ext common relative path
    let ext_common_relative_path = format!("{}{}", DIR_EXT, get_file_separator());
    // ext target generate common path
    let ext_target_generate_common_path = format!(
        "{}{}{}",
        target_common_path,
        get_file_separator(),
        ext_common_relative_path
    );
    // ext template common path
    let ext_template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        dir_template_ext
    );
    // main entity generate
    let main_entity_info = component_info.main_entity_info;
    let main_entity_info = fill_entity_info_param_json(main_entity_info)?;
    generate_entity_code(
        &base_target_generate_common_path,
        &base_template_common_path,
        &main_entity_info,
    )?;
    // child entity generate
    generate_entity_code(
        &child_target_generate_common_path,
        &child_template_common_path,
        &main_entity_info,
    )?;
    let child_entity_info_list = component_info.child_entity_info_list;
    for child_entity_po in child_entity_info_list {
        let child_entity_po = fill_entity_info_param_json(child_entity_po)?;
        generate_entity_code(
            &child_target_generate_common_path,
            &child_template_common_path,
            &child_entity_po,
        )?;
    }
    // up ref entity generate
    generate_entity_code(
        &ext_target_generate_common_path,
        &ext_template_common_path,
        &main_entity_info,
    )?;
    let up_entity_info_list = component_info.up_entity_info_list;
    for up_entity_po in up_entity_info_list {
        let up_entity_po = fill_entity_info_param_json(up_entity_po)?;
        generate_entity_code(
            &ext_target_generate_common_path,
            &ext_template_common_path,
            &up_entity_po,
        )?;
    }

    //rename folder placeholder, must be render after run it
    rename_sub_folder_placeholder(&target_common_path, "_{camelCase}_", &main_entity_info.camel_case_name.clone().unwrap())?;
    rename_sub_folder_placeholder(&target_common_path, "_{snakeCase}_", &main_entity_info.snake_case_name.clone().unwrap())?;
    rename_sub_folder_placeholder(&target_common_path, "_{pascalCase}_", &main_entity_info.pascal_case_name.clone().unwrap())?;
    rename_sub_folder_placeholder(&target_common_path, "_{macroCase}_", &main_entity_info.macro_case_name.clone().unwrap())?;
    rename_sub_folder_placeholder(&target_common_path, "_{lowerCase}_", &main_entity_info.camel_case_name.clone().unwrap().to_lowercase())?;
    rename_sub_folder_placeholder(&target_common_path, "_{upperCase}_", &main_entity_info.camel_case_name.clone().unwrap().to_uppercase())?;

    // copy child dir content to base dir
    copy_folder_to_dest(
        &child_target_generate_common_path,
        &base_target_generate_common_path,
    )?;

    // copy ext dir content to base dir
    copy_folder_to_dest(
        &ext_target_generate_common_path,
        &base_target_generate_common_path,
    )?;

    // base common relative path
    let target_code_path = format!(
        "{}{}{}{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        nanoid_dir,
        get_file_separator(),
        target_code_dir,
        get_file_separator(),
    );
    // copy base dir content to finally dir
    copy_folder_to_dest(&base_target_generate_common_path, &target_code_path)?;
    fs::remove_dir_all(ext_target_generate_common_path).map_err(|err| {
        TcdtServiceError::build_internal_msg_error(
            "remove base_target_generate_common_path failed",
            err,
        )
    })?;
    fs::remove_dir_all(child_target_generate_common_path).map_err(|err| {
        TcdtServiceError::build_internal_msg_error(
            "remove base_target_generate_common_path failed",
            err,
        )
    })?;
    fs::remove_dir_all(base_target_generate_common_path).map_err(|err| {
        TcdtServiceError::build_internal_msg_error(
            "remove base_target_generate_common_path failed",
            err,
        )
    })?;

    let zip_file_name = format!(
        "{}.zip",
        component_info
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

async fn get_project(
    db: &DbConn,
    com_entity: &component::Model,
) -> Result<project::Model, TcdtServiceError> {
    let component_module_entity = com_entity
        .find_linked(component::ComponentModuleLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find component_module failed");
            TcdtServiceError::build_internal_msg_error("find component_module failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find component_module",
        ))?;
    let sub_project_entity = component_module_entity
        .find_linked(component_module::SubProjectLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find sub project failed");
            TcdtServiceError::build_internal_msg_error("find sub project failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find sub project",
        ))?;
    let project_entity = sub_project_entity
        .find_linked(sub_project::ProjectLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find project failed");
            TcdtServiceError::build_internal_msg_error("find project failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find project",
        ))?;
    Ok(project_entity)
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
        let new_file_name = rename_file_placeholder(&generate_file_path, "_{camelCase}_", &entity_info_po.camel_case_name.clone().unwrap())?;
        let new_file_name = rename_file_placeholder(&new_file_name, "_{pascalCase}_", &entity_info_po.pascal_case_name.clone().unwrap())?;
        let new_file_name = rename_file_placeholder(&new_file_name, "_{snakeCase}_", &entity_info_po.snake_case_name.clone().unwrap())?;
        let new_file_name = rename_file_placeholder(&new_file_name, "_{macroCase}_", &entity_info_po.macro_case_name.clone().unwrap())?;
        let new_file_name = rename_file_placeholder(&new_file_name, "_{lowerCase}_", &entity_info_po.camel_case_name.clone().unwrap().to_lowercase())?;
        let new_file_name = rename_file_placeholder(&new_file_name, "_{upperCase}_", &entity_info_po.camel_case_name.clone().unwrap().to_uppercase())?;
        let new_file_name = rename_file_placeholder(&new_file_name, "_{displayName}_", &entity_info_po.display_name.clone().unwrap())?;
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
            log::error!("entity_po Serialize to param_json error");
            TcdtServiceError::build_internal_msg_error(
                "entity_po Serialize to param_json error",
                err,
            )
        })?;
        entity_po.param_json = Some(param_json);
    }
    Ok(entity_po)
}
