use crate::dto::po::ext::generate::component_enum::{
    ComponentInfoPO, EnumAttributeInfoPO, EnumInfoPO,
};
use ::entity::entity::{component, component_module, dd_enum, enum_attribute};
use sea_orm::*;
use tcdt_common::name_switch_util::{
    pascal_case_to_snake_case, snake_case_to_camel_case, snake_case_to_macro_case,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;

pub(crate) async fn build(
    db: &DbConn,
    com_entity: component::Model,
) -> Result<ComponentInfoPO, TcdtServiceError> {
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

    let component_enum_entity_list = com_entity
        .find_linked(component::ComponentEnumsLinked)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find component_enum_entity_list failed");
            TcdtServiceError::build_internal_msg_error(
                "find component_enum_entity_list failed",
                err,
            )
        })?;

    let base_path = build_base_path(&component_module_entity, &com_entity);

    let ref_component_info = ComponentInfoPO {
        id_component: com_entity.id_component.clone(),
        param_json: None,
        package_name: com_entity.package_name.clone(),
        display_name: com_entity.display_name.clone(),
        base_path: Some(base_path),
        enum_info_list: vec![],
    };

    let enum_id_list = component_enum_entity_list
        .clone()
        .into_iter()
        .map(|com_enum| com_enum.id_enum.unwrap_or_default())
        .collect::<Vec<_>>();
    let enums = dd_enum::Entity::find()
        .filter(dd_enum::Column::IdEnum.is_in(enum_id_list.clone()))
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find enums failed");
            TcdtServiceError::build_internal_msg_error("find enums failed", err)
        })?;
    let enum_attr_list = enum_attribute::Entity::find()
        .filter(enum_attribute::Column::IdEnum.is_in(enum_id_list.clone()))
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find enum_attr_list failed");
            TcdtServiceError::build_internal_msg_error("find enum_attr_list failed", err)
        })?;
    let enum_info_list = make_enum_info_list(
        &component_module_entity,
        &com_entity,
        enums,
        enum_attr_list,
        &ref_component_info,
    )?;
    let mut component_info = ref_component_info.clone();
    component_info.enum_info_list = enum_info_list;
    Ok(component_info)
}

fn make_enum_info_list(
    component_module_entity: &component_module::Model,
    com_entity: &component::Model,
    enums: Vec<dd_enum::Model>,
    all_enum_attr_list: Vec<enum_attribute::Model>,
    ref_component_info: &ComponentInfoPO,
) -> Result<Vec<EnumInfoPO>, TcdtServiceError> {
    let mut enum_info_list: Vec<EnumInfoPO> = vec![];
    for enum_entity in enums {
        let enum_attr_list = all_enum_attr_list
            .clone()
            .into_iter()
            .filter(|enum_attr| enum_attr.id_enum == Some(enum_entity.id_enum.clone()))
            .collect::<Vec<enum_attribute::Model>>();
        let mut enum_info = make_enum_info(
            component_module_entity,
            com_entity,
            enum_entity,
            enum_attr_list,
            ref_component_info,
        )?;
        match enum_info {
            None => {
                log::warn!("can not get enum info");
            }
            Some(enum_info) => {
                enum_info_list.push(enum_info);
            }
        }
    }
    Ok(enum_info_list)
}

fn make_enum_info(
    component_module_entity: &component_module::Model,
    com_entity: &component::Model,
    enum_entity: dd_enum::Model,
    enum_attr_list: Vec<enum_attribute::Model>,
    ref_component_info: &ComponentInfoPO,
) -> Result<Option<EnumInfoPO>, TcdtServiceError> {
    let base_path = build_base_path(component_module_entity, com_entity);
    let mut enum_attr_info_list: Vec<EnumAttributeInfoPO> = vec![];
    for enum_attr in enum_attr_list {
        let enum_attr_info = EnumAttributeInfoPO {
            id_enum_attribute: enum_attr.id_enum_attribute.clone(),
            display_name: enum_attr.display_name.clone(),
            code: enum_attr.code.clone(),
            enum_value: enum_attr.enum_value.clone(),
        };
        enum_attr_info_list.push(enum_attr_info);
    }
    let class_name = enum_entity.class_name.clone().unwrap_or_default();
    if class_name.trim() == String::default() {
        return Err(TcdtServiceError::build_custom_msg(
            "enum class name is empty",
        ));
    }
    let pascal_case_name = class_name;
    let snake_case_name = pascal_case_to_snake_case(&pascal_case_name);
    let macro_case_name = snake_case_to_macro_case(&snake_case_name);
    let camel_case_name = snake_case_to_camel_case(&snake_case_name);
    let enum_info = EnumInfoPO {
        param_json: None,
        id_enum: enum_entity.id_enum.clone(),
        base_path: Some(base_path),
        package_name: com_entity.package_name.clone(),
        class_name: enum_entity.class_name.clone(),
        display_name: enum_entity.display_name.clone(),
        enum_value_type: enum_entity.enum_value_type.clone(),
        camel_case_name: Some(pascal_case_name),
        pascal_case_name: Some(snake_case_name),
        snake_case_name: Some(macro_case_name),
        macro_case_name: Some(camel_case_name),
        enum_attribute_info_list: enum_attr_info_list,
        comp_info: Some(Box::new(ref_component_info.clone())),
    };
    Ok(Some(enum_info))
}

fn build_base_path(
    component_module_entity: &component_module::Model,
    com_entity: &component::Model,
) -> String {
    let base_path = format!(
        "{}{}{}",
        component_module_entity.path.clone().unwrap_or_default(),
        ".",
        com_entity.package_name.clone().unwrap_or_default()
    );

    base_path
}
