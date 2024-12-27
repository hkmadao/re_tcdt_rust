use crate::dto::po::ext::generate::component_enum::{ComponentInfoPO, EnumInfoPO};
use crate::dto::vo::ext::generate::generate_result::GenerateResult;
use crate::service::base::component_service::ComponentQuery;
use crate::service::ext::generator::component_enum::param_build::build;
use crate::service::ext::generator::genrate_util::{
    create_folder_strut_by_template_folder, generator,
};
use crate::service::ext::generator::write_dir::folder_zip;
use crate::util::file_util::recursion_get_file_by_folder;
use ::entity::entity::{component, component_module, project, sub_project};
use sea_orm::*;
use tcdt_common::file_util::get_file_separator;
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tera::{Context, Tera};
use crate::util::id_util::generate_id;

const DIR_TEMPLATE_FULL: &str = "comp_enum_full";
const DIR_TEMPLATE_PART: &str = "comp_enum_part";

pub async fn generate_full(
    db: &DbConn,
    id_coll: String,
) -> Result<GenerateResult, TcdtServiceError> {
    let coll_entity = ComponentQuery::find_by_id(db, id_coll).await?;
    let project_entity = get_project(db, &coll_entity).await?;
    let template_code =
        project_entity
            .template_code
            .clone()
            .ok_or(TcdtServiceError::build_internal_msg(
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

    let component_info = build(db, coll_entity).await?;
    let component_info = fill_component_info_po_param_json(component_info)?;
    let target_common_path = format!(
        "{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        nanoid_dir
    );
    // base common relative path
    let base_common_relative_path = format!("{}{}", DIR_TEMPLATE_FULL, get_file_separator());

    // base target generate common path
    let base_target_generate_common_path = format!(
        "{}{}",
        target_common_path,
        get_file_separator(),
    );

    // base template common path
    let base_template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        base_common_relative_path
    );
    generate_component_code(
        &base_target_generate_common_path,
        &base_template_common_path,
        &component_info,
    )?;

    let zip_file_name = format!(
        "{}.zip",
        component_info
            .display_name
            .clone()
            .unwrap_or(String::from("unknown")),
    );
    let zip_file_full_name = format!(
        "{}{}{}.zip",
        target_common_path,
        get_file_separator(),
        zip_file_name,
    );
    log::info!("zip file full name is: {}", zip_file_full_name);
    folder_zip(&base_target_generate_common_path, &zip_file_full_name)?;

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
    let coll_entity = ComponentQuery::find_by_id(db, id_coll).await?;
    let project_entity = get_project(db, &coll_entity).await?;
    let template_code =
        project_entity
            .template_code
            .clone()
            .ok_or(TcdtServiceError::build_internal_msg(
                "project entity template code is Empty",
            ))?;

    let component_info = build(db, coll_entity).await?;
    let nanoid_dir = generate_id();
    let target_code_dir = component_info
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
    // base common relative path
    let base_common_relative_path = format!("{}{}", DIR_TEMPLATE_PART, get_file_separator());

    // base target generate common path
    let base_target_generate_common_path =
        format!("{}{}", target_common_path, get_file_separator());

    // base template common path
    let base_template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        base_common_relative_path
    );

    for enum_po in component_info.enum_info_list {
        let enum_po = fill_entity_info_param_json(enum_po)?;
        generate_entity_code(
            &base_target_generate_common_path,
            &base_template_common_path,
            &enum_po,
            &get_base_file_name(&project_entity, &enum_po),
        )?;
    }

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
    enum_info: &EnumInfoPO,
    entity_name: &str,
) -> Result<(), TcdtServiceError> {
    let mut context = Context::new();
    context.insert("rootInfo", &enum_info);
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
        generator(
            &target_path,
            &template_file_path,
            &template_file_full_name,
            entity_name,
            &context,
            &tera,
        )?;
    }
    Ok(())
}

fn generate_component_code(
    target_path: &str,
    template_file_path: &str,
    component_info_po: &ComponentInfoPO,
) -> Result<(), TcdtServiceError> {
    let mut context = Context::new();
    context.insert("rootInfo", &component_info_po);
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
        generator(
            &target_path,
            &template_file_path,
            &template_file_full_name,
            &component_info_po.display_name.clone().unwrap(),
            &context,
            &tera,
        )?;
    }
    Ok(())
}

fn get_base_file_name(project_entity: &project::Model, enum_info: &EnumInfoPO) -> String {
    let entity_name: String;
    if project_entity.file_name_type == Some("SnakeCase".to_string()) {
        entity_name = enum_info.snake_case_name.clone().unwrap_or_default();
    } else {
        entity_name = enum_info.pascal_case_name.clone().unwrap_or_default();
    }
    entity_name
}

fn fill_entity_info_param_json(enum_info: EnumInfoPO) -> Result<EnumInfoPO, TcdtServiceError> {
    let mut enum_info = enum_info;
    if TCDT_CONF.enable_code_generate_debug_mode {
        let param_json = serde_json::to_string_pretty(&enum_info).map_err(|err| {
            log::error!("enum_info Serialize to param_json error");
            TcdtServiceError::build_internal_msg_error(
                "enum_info Serialize to param_json error",
                err,
            )
        })?;
        enum_info.param_json = Some(param_json);
    }
    Ok(enum_info)
}

fn fill_component_info_po_param_json(
    component_info_po: ComponentInfoPO,
) -> Result<ComponentInfoPO, TcdtServiceError> {
    let mut application_po = component_info_po;
    if TCDT_CONF.enable_code_generate_debug_mode {
        let param_json = serde_json::to_string_pretty(&application_po).map_err(|err| {
            log::error!("componentInfo_po Serialize to param_json error");
            TcdtServiceError::build_internal_msg_error(
                "componentInfo_po Serialize to param_json error",
                err,
            )
        })?;
        application_po.param_json = Some(param_json);
    }
    Ok(application_po)
}
