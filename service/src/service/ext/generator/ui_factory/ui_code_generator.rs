use crate::dto::po::ext::generate::ui_code::UIFactoryParamPO;
use crate::dto::vo::ext::generate::generate_result::GenerateResult;
use crate::service::base::factory_service::FactoryQuery;
use crate::service::ext::generator::genrate_util::{
    create_folder_strut_by_template_folder, generator,
};
use crate::service::ext::generator::write_dir::folder_zip;
use crate::util::file_util::{recursion_get_file_by_folder, rename_file_placeholder};
use ::entity::entity::{bill_form, button_action, factory, project, query, tree};
use sea_orm::*;
use tcdt_common::file_util::get_file_separator;
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tera::{Context, Tera};
use crate::util::id_util::generate_id;

const DIR_TEMPLATE: &str = "web_ui";

pub async fn ui_code_generate(
    db: &DbConn,
    id_coll: String,
) -> Result<GenerateResult, TcdtServiceError> {
    let factory_entity = FactoryQuery::find_by_id(db, id_coll).await?;
    let project_entity = get_project(db, &factory_entity).await?;
    let template_code =
        project_entity
            .web_template_code
            .clone()
            .ok_or(TcdtServiceError::build_internal_msg(
                "project entity template code is Empty",
            ))?;

    let ui_factory_po = build(db, &factory_entity).await?;
    let nanoid_dir = generate_id();
    let target_code_dir = ui_factory_po
        .name
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
    //生成代码公共路径
    let target_common_path = format!(
        "{}{}{}",
        TCDT_CONF.code_generate_path,
        get_file_separator(),
        target_folder_name
    );
    // common relative path
    let common_relative_path = format!("{}{}", DIR_TEMPLATE, get_file_separator());
    // target  generate common path
    let target_generate_common_path = format!("{}{}", target_common_path, get_file_separator());
    // template common path
    let template_common_path = format!(
        "{}{}{}{}{}",
        TCDT_CONF.code_template_path,
        get_file_separator(),
        template_code,
        get_file_separator(),
        common_relative_path
    );
    generate_ui_code(
        &target_generate_common_path,
        &template_common_path,
        &ui_factory_po,
    )?;

    let zip_file_name = format!(
        "{}.zip",
        ui_factory_po
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
        generate_target_dir: format!("{}{}", TCDT_CONF.code_generate_path, get_file_separator()),
    })
}

async fn build(
    db: &DbConn,
    factory_entity: &factory::Model,
) -> Result<UIFactoryParamPO, TcdtServiceError> {
    let mut ui_factory_po = UIFactoryParamPO {
        name: factory_entity.name.clone(),
        display_name: factory_entity.display_name.clone(),
        ..Default::default()
    };
    let ref_id_content =
        factory_entity
            .ref_id_content
            .clone()
            .ok_or(TcdtServiceError::build_custom_msg(
                "ref_id_content is Empty",
            ))?;
    let ref_id_josn =
        serde_json::from_str::<serde_json::Value>(&ref_id_content).map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ref_id_content json Deserialize failed",
                err,
            )
        })?;
    let ui_content = factory_entity
        .content
        .clone()
        .ok_or(TcdtServiceError::build_custom_msg("content is Empty"))?;
    let ui_content_json =
        serde_json::from_str::<serde_json::Value>(&ui_content).map_err(|err| {
            TcdtServiceError::build_internal_msg_error("ui_content json Deserialize failed", err)
        })?;
    // let ui_content = serde_json::to_string_pretty(&ui_content_json).map_err(|err| {
    //     TcdtServiceError::build_internal_msg_error("ui_content pretty failed", err)
    // })?;
    ui_factory_po.ui_json = Some(ui_content_json);
    ui_factory_po.ui_content = Some(ui_content);
    if let Some(id_view_billform) = ref_id_josn.get("idViewBillform") {
        let bill_form_entity = bill_form::Entity::find_by_id(id_view_billform.as_str().unwrap())
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error("can not find bill_form", err)
            })?
            .ok_or(TcdtServiceError::build_custom_msg("can not find bill_form"))?;
        let bill_form_content =
            bill_form_entity
                .content
                .clone()
                .ok_or(TcdtServiceError::build_custom_msg(
                    "bill_form_content is Empty",
                ))?;
        let bill_form_content_json = serde_json::from_str::<serde_json::Value>(&bill_form_content)
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "bill_form_content json Deserialize failed",
                    err,
                )
            })?;
        // let bill_form_content =
        //     serde_json::to_string_pretty(&bill_form_content).map_err(|err| {
        //         TcdtServiceError::build_internal_msg_error("ui_content pretty failed", err)
        //     })?;
        ui_factory_po.b_table_json = Some(bill_form_content_json);
        ui_factory_po.b_table_content = Some(bill_form_content);
        let bill_form_meta_data_content =
            bill_form_entity
                .meta_data
                .clone()
                .ok_or(TcdtServiceError::build_custom_msg(
                    "meta_data_content is Empty",
                ))?;
        let bill_form_meta_data_json = serde_json::from_str::<serde_json::Value>(
            &bill_form_meta_data_content,
        )
        .map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "bill_form_content json Deserialize failed",
                err,
            )
        })?;
        ui_factory_po.b_m_d_json = Some(bill_form_meta_data_json);
    }

    if let Some(id_edit_billform) = ref_id_josn.get("idEditBillform") {
        let bill_form_entity = bill_form::Entity::find_by_id(id_edit_billform.as_str().unwrap())
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error("can not find bill_form", err)
            })?
            .ok_or(TcdtServiceError::build_custom_msg("can not find bill_form"))?;
        let bill_form_content =
            bill_form_entity
                .content
                .clone()
                .ok_or(TcdtServiceError::build_custom_msg(
                    "edit_bill_form_content is Empty",
                ))?;
        let bill_form_content_json = serde_json::from_str::<serde_json::Value>(&bill_form_content)
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "edit_bill_form json Deserialize failed",
                    err,
                )
            })?;
        // let bill_form_content =
        //     serde_json::to_string_pretty(&bill_form_content).map_err(|err| {
        //         TcdtServiceError::build_internal_msg_error(
        //             "edit_bill_form_content pretty failed",
        //             err,
        //         )
        //     })?;
        ui_factory_po.b_json = Some(bill_form_content_json);
        ui_factory_po.b_content = Some(bill_form_content);
    }

    if let Some(id_tree) = ref_id_josn.get("idTree") {
        let tree_entity = tree::Entity::find_by_id(id_tree.as_str().unwrap())
            .one(db)
            .await
            .map_err(|err| TcdtServiceError::build_internal_msg_error("can not find tree", err))?
            .ok_or(TcdtServiceError::build_custom_msg("can not find tree"))?;
        let tree_content = tree_entity
            .content
            .clone()
            .ok_or(TcdtServiceError::build_custom_msg("tree_content is Empty"))?;
        let tree_content_json =
            serde_json::from_str::<serde_json::Value>(&tree_content).map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "tree_content json Deserialize failed",
                    err,
                )
            })?;
        // let tree_content = serde_json::to_string_pretty(&tree_content).map_err(|err| {
        //     TcdtServiceError::build_internal_msg_error("tree_content pretty failed", err)
        // })?;
        ui_factory_po.t_json = Some(tree_content_json);
        ui_factory_po.t_content = Some(tree_content);
    }

    if let Some(id_query) = ref_id_josn.get("idQuery") {
        let query_entity = query::Entity::find_by_id(id_query.as_str().unwrap())
            .one(db)
            .await
            .map_err(|err| TcdtServiceError::build_internal_msg_error("can not find query", err))?
            .ok_or(TcdtServiceError::build_custom_msg("can not find query"))?;
        let query_content = query_entity
            .content
            .clone()
            .ok_or(TcdtServiceError::build_custom_msg("query_content is Empty"))?;
        let query_content_json = serde_json::from_str::<serde_json::Value>(&query_content)
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "query_content json Deserialize failed",
                    err,
                )
            })?;
        // let query_content = serde_json::to_string_pretty(&query_content).map_err(|err| {
        //     TcdtServiceError::build_internal_msg_error("query_content pretty failed", err)
        // })?;
        ui_factory_po.q_json = Some(query_content_json);
        ui_factory_po.q_content = Some(query_content);
    }

    if let Some(id_view_button_action) = ref_id_josn.get("idViewButtonAction") {
        let view_button_action_entity =
            button_action::Entity::find_by_id(id_view_button_action.as_str().unwrap())
                .one(db)
                .await
                .map_err(|err| {
                    TcdtServiceError::build_internal_msg_error(
                        "can not find view_button_action",
                        err,
                    )
                })?
                .ok_or(TcdtServiceError::build_custom_msg(
                    "can not find view_button_action",
                ))?;
        let view_button_action_content =
            view_button_action_entity
                .content
                .clone()
                .ok_or(TcdtServiceError::build_custom_msg(
                    "view_button_action_content is Empty",
                ))?;
        let view_button_action_content_json = serde_json::from_str::<serde_json::Value>(
            &view_button_action_content,
        )
        .map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "view_button_action_content json Deserialize failed",
                err,
            )
        })?;
        // let view_button_action_content = serde_json::to_string_pretty(&view_button_action_content)
        //     .map_err(|err| {
        //         TcdtServiceError::build_internal_msg_error(
        //             "view_button_action_content pretty failed",
        //             err,
        //         )
        //     })?;
        ui_factory_po.v_button_json = Some(view_button_action_content_json);
        ui_factory_po.v_button_content = Some(view_button_action_content);
    }

    if let Some(id_edit_button_action) = ref_id_josn.get("idEditButtonAction") {
        let edit_button_action_entity =
            button_action::Entity::find_by_id(id_edit_button_action.as_str().unwrap())
                .one(db)
                .await
                .map_err(|err| {
                    TcdtServiceError::build_internal_msg_error(
                        "can not find edit_button_action",
                        err,
                    )
                })?
                .ok_or(TcdtServiceError::build_custom_msg(
                    "can not find edit_button_action",
                ))?;
        let edit_button_action_content =
            edit_button_action_entity
                .content
                .clone()
                .ok_or(TcdtServiceError::build_custom_msg(
                    "edit_button_action_content is Empty",
                ))?;
        let edit_button_action_content_json = serde_json::from_str::<serde_json::Value>(
            &edit_button_action_content,
        )
        .map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "edit_button_action_content json Deserialize failed",
                err,
            )
        })?;
        // let edit_button_action_content = serde_json::to_string_pretty(&edit_button_action_content)
        //     .map_err(|err| {
        //         TcdtServiceError::build_internal_msg_error(
        //             "edit_button_action_content pretty failed",
        //             err,
        //         )
        //     })?;
        ui_factory_po.button_json = Some(edit_button_action_content_json);
        ui_factory_po.button_content = Some(edit_button_action_content);
    }
    let ui_factory_po = fill_factory_info_param_json(ui_factory_po)?;
    Ok(ui_factory_po)
}

async fn get_project(
    db: &DbConn,
    factory_entity: &factory::Model,
) -> Result<project::Model, TcdtServiceError> {
    let id_project = factory_entity
        .id_project
        .clone()
        .ok_or(TcdtServiceError::build_custom_msg("id_project is Empty"))?;
    let project_entity = project::Entity::find_by_id(id_project.clone())
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find project failed");
            TcdtServiceError::build_internal_msg_error("find project failed", err)
        })?
        .ok_or(TcdtServiceError::build_custom_msg(
            "can not find project entity",
        ))?;
    Ok(project_entity)
}

fn generate_ui_code(
    target_path: &str,
    template_file_path: &str,
    ui_factory_po: &UIFactoryParamPO,
) -> Result<(), TcdtServiceError> {
    let mut context = Context::new();
    context.insert("rootInfo", ui_factory_po);
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
        let new_file_name = rename_file_placeholder(&generate_file_path, "_{displayName}_", &ui_factory_po.display_name.clone().unwrap())?;
    }
    Ok(())
}

fn fill_factory_info_param_json(
    application_info: UIFactoryParamPO,
) -> Result<UIFactoryParamPO, TcdtServiceError> {
    let mut factory_info = application_info;
    if TCDT_CONF.enable_code_generate_debug_mode {
        let param_json = serde_json::to_string_pretty(&factory_info).map_err(|err| {
            log::error!("entity_po Serialize to param_json error");
            TcdtServiceError::build_internal_msg_error(
                "entity_po Serialize to param_json error",
                err,
            )
        })?;
        factory_info.param_json = Some(param_json);
    }
    Ok(factory_info)
}
