use crate::dto::{
    po::ext::generate::component_combination::{
        AttributeInfoPO, BasePackageInfo, ComponentInfoPO, EntityInfoPO, EnumAttributeInfoPO,
        EnumInfoPO,
    },
    vo::base::data_type_vo::DataTypeVO,
};
use crate::service::ext::generator::genrate_const::{
    DOWN_TYPE_ONE_TO_MANY, DOWN_TYPE_ONE_TO_ONE, DOWN_TYPE_ZERO_TO_MANY, DOWN_TYPE_ZERO_TO_ONE,
    INTERNAL_AGG_ARRAY, INTERNAL_AGG_FK, INTERNAL_AGG_REF, INTERNAL_AGG_SINGLE,
    INTERNAL_AGG_SINGLE_REF, INTERNAL_ARRAY, INTERNAL_FK, INTERNAL_PK, INTERNAL_REF,
    INTERNAL_SINGLE, INTERNAL_SINGLE_REF,
};
use ::entity::entity::{
    component, component_entity, component_entity_associate, component_module, dd_entity, dd_enum,
    entity_associate, entity_attribute, enum_attribute, ext_attribute,
};
use sea_orm::*;
use std::collections::HashMap;
use tcdt_common::name_switch_util::{
    camel_case_to_pascal_case, pascal_case_to_snake_case, snake_case_to_camel_case,
    snake_case_to_macro_case,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;

pub(crate) async fn build(
    db: &DbConn,
    com_entity: component::Model,
    column_domain_type_map: HashMap<String, DataTypeVO>,
) -> Result<ComponentInfoPO, TcdtServiceError> {
    let mut entity_vo_list: Vec<EntityInfoPO> = vec![];

    let component_module_entity = com_entity
        .find_linked(component::ComponentModuleLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find component_module_entity failed");
            TcdtServiceError::build_internal_msg_error("find component_module_entity failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find component_module",
        ))?;

    let base_path = build_base_path(&component_module_entity);

    let component_entity_entities = com_entity
        .find_linked(component::ComponentEntitiesLinked)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find component_entity_entities failed");
            TcdtServiceError::build_internal_msg_error("find component_entity_entities failed", err)
        })?;

    let cp_associates = com_entity
        .find_linked(component::ComponentEntityAssociatesLinked)
        .order_by_asc(component_entity_associate::Column::IdComponentEntityAssociate)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find cp_associates failed");
            TcdtServiceError::build_internal_msg_error("find cp_associates failed", err)
        })?;
    let entity_associate_id_list = cp_associates
        .clone()
        .into_iter()
        .map(|cp_asso| cp_asso.id_entity_associate.unwrap_or_default())
        .collect::<Vec<_>>();
    let entity_associate_list = entity_associate::Entity::find()
        .filter(entity_associate::Column::IdEntityAssociate.is_in(entity_associate_id_list))
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find entity_associate_list failed");
            TcdtServiceError::build_internal_msg_error("find entity_associate_list failed", err)
        })?;

    let agg_cp_associates = cp_associates
        .clone()
        .into_iter()
        .filter(|cp_asso| cp_asso.fg_agg_asso == Some(true))
        .collect::<Vec<_>>();
    let agg_entity_associate_id_list = agg_cp_associates
        .clone()
        .into_iter()
        .map(|cp_asso| cp_asso.id_entity_associate.unwrap_or_default())
        .collect::<Vec<_>>();
    let agg_entity_associate_list = entity_associate_list
        .clone()
        .into_iter()
        .filter(|asso| agg_entity_associate_id_list.contains(&asso.id_entity_associate))
        .collect::<Vec<_>>();
    for component_entity_entity in component_entity_entities {
        let ext_attrs = component_entity_entity
            .find_linked(component_entity::ExtAttributesLinked)
            .order_by_asc(ext_attribute::Column::Sn)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find ext_attrs failed");
                TcdtServiceError::build_internal_msg_error("find ext_attrs failed", err)
            })?;
        let dd_entity_entity = component_entity_entity
            .find_linked(component_entity::DdEntityLinked)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("find dd_entity_entity failed");
                TcdtServiceError::build_internal_msg_error("find dd_entity_entity failed", err)
            })?
            .ok_or(TcdtServiceError::build_custom_msg("entity is empty"))?;
        let entity_attrs = dd_entity_entity
            .find_linked(dd_entity::AttributesLinked)
            .order_by_asc(entity_attribute::Column::Sn)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find entity_attrs failed");
                TcdtServiceError::build_internal_msg_error("find entity_attrs failed", err)
            })?;
        let mut entity_info = make_base_entity_info(
            db,
            &component_module_entity,
            &com_entity,
            &component_entity_entity,
            &ext_attrs,
            &dd_entity_entity,
            &entity_attrs,
            &column_domain_type_map,
            false,
        )
        .await?;

        build_real_path(db, &dd_entity_entity, &mut entity_info).await?;

        let mut attr_info_list = entity_info.attribute_info_list.clone();

        let up_cp_associates = entity_associate_list
            .iter()
            .filter(|asso| asso.id_down == Some(dd_entity_entity.id_entity.clone()))
            .collect::<Vec<_>>();
        for up_asso in up_cp_associates {
            let up_cp_asso = cp_associates.iter().find(|cp_asso| {
                cp_asso.id_entity_associate == Some(up_asso.id_entity_associate.clone())
            });
            let fk_attribute_info = build_fk_attribute(
                db,
                up_cp_asso.unwrap(),
                up_asso,
                &component_entity_entity,
                &ext_attrs,
                &dd_entity_entity,
                &entity_attrs,
                &column_domain_type_map,
            )
            .await?;
            if let Some(fk_attribute_info) = fk_attribute_info {
                attr_info_list.push(fk_attribute_info);
            }
            let ref_attribute_info = build_ref_attribute(
                db,
                up_cp_asso.unwrap(),
                up_asso,
                &component_entity_entity,
                &ext_attrs,
                &dd_entity_entity,
                &entity_attrs,
                &column_domain_type_map,
            )
            .await?;
            if let Some(ref_attribute_info) = ref_attribute_info {
                attr_info_list.push(ref_attribute_info);
            }
        }

        let down_cp_associates = entity_associate_list
            .iter()
            .filter(|asso| asso.id_up == Some(dd_entity_entity.id_entity.clone()))
            .collect::<Vec<_>>();
        for down_asso in down_cp_associates {
            let down_cp_asso = cp_associates.iter().find(|cp_asso| {
                cp_asso.id_entity_associate == Some(down_asso.id_entity_associate.clone())
            });
            let array_attribute_info = build_array_attribute(
                db,
                down_cp_asso.unwrap(),
                down_asso,
                &component_entity_entity,
                &ext_attrs,
                &dd_entity_entity,
                &entity_attrs,
                &column_domain_type_map,
            )
            .await?;
            if let Some(array_attribute_info) = array_attribute_info {
                attr_info_list.push(array_attribute_info);
            }
        }
        entity_info.attribute_info_list = attr_info_list;
        entity_info = build_desc_info(
            entity_info,
            &agg_entity_associate_list,
            &com_entity,
            &column_domain_type_map,
        )?;

        entity_vo_list.push(entity_info);
    }

    // find and set main entity
    entity_vo_list.iter_mut().for_each(|entity_vo| {
        if Some(entity_vo.id_component_entity.clone()) == com_entity.id_main_component_entity {
            entity_vo.fg_main = true;
        }
    });

    let entity_vo_list = entity_vo_list;

    let up_entity_info_list = entity_vo_list
        .iter()
        .flat_map(|entity_vo| entity_vo.up_entity_info_list.clone())
        .collect::<Vec<_>>();
    let up_entity_info_list = distinct(up_entity_info_list);

    let down_entity_info_list = entity_vo_list
        .iter()
        .flat_map(|entity_vo| entity_vo.down_entity_info_list.clone())
        .collect::<Vec<_>>();
    let down_entity_info_list = distinct(down_entity_info_list);

    let entity_vo_list = distinct_out_entity_info(entity_vo_list);

    let entity_vo_list = entity_vo_list
        .into_iter()
        .map(|mut entity_vo| {
            let package_name_list = make_base_package_name_list(&entity_vo.attribute_info_list);
            entity_vo.out_base_package_list = package_name_list;
            entity_vo
        })
        .collect::<Vec<_>>();

    let all_out_base_package_list = entity_vo_list
        .iter()
        .flat_map(|entity_vo| entity_vo.out_base_package_list.clone())
        .collect::<Vec<_>>();

    let base_package_name_list = distinct_base_package_info(all_out_base_package_list);

    let mut main_entity_info = entity_vo_list
        .clone()
        .iter()
        .find(|entity_vo| entity_vo.fg_main)
        .unwrap()
        .clone();
    let mut child_entity_info_list = entity_vo_list
        .clone()
        .into_iter()
        .filter(|entity_vo| !entity_vo.fg_main)
        .collect::<Vec<_>>();

    child_entity_info_list.iter_mut().for_each(|entity_info| {
        entity_info.main_entity_info = Some(Box::new(main_entity_info.clone()));
    });

    main_entity_info.child_entity_info_list = child_entity_info_list.clone();

    let ref_component_info = ComponentInfoPO {
        id_component: String::new(),
        package_name: com_entity.package_name.clone(),
        param_json: None,
        display_name: com_entity.display_name.clone(),
        base_path: Some(base_path),
        up_entity_info_list: up_entity_info_list.clone(),
        down_entity_info_list: down_entity_info_list.clone(),
        main_entity_info: main_entity_info.clone(),
        child_entity_info_list: child_entity_info_list.clone(),
        out_base_package_list: base_package_name_list,
    };

    main_entity_info.comp_info = Some(Box::new(ref_component_info.clone()));

    let mut component_info_result = ref_component_info.clone();
    component_info_result.main_entity_info = main_entity_info.clone();

    Ok(component_info_result)
}

fn make_enum_info_list(
    enums: Vec<dd_enum::Model>,
    all_enum_attr_list: Vec<enum_attribute::Model>,
) -> Result<Vec<EnumInfoPO>, TcdtServiceError> {
    let mut enum_info_list: Vec<EnumInfoPO> = vec![];
    for enum_entity in enums {
        let enum_attr_list = all_enum_attr_list
            .clone()
            .into_iter()
            .filter(|enum_attr| enum_attr.id_enum == Some(enum_entity.id_enum.clone()))
            .collect::<Vec<enum_attribute::Model>>();
        let enum_info = make_enum_info(enum_entity, enum_attr_list)?;
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
    enum_entity: dd_enum::Model,
    enum_attr_list: Vec<enum_attribute::Model>,
) -> Result<Option<EnumInfoPO>, TcdtServiceError> {
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
        id_enum: enum_entity.id_enum.clone(),
        class_name: enum_entity.class_name.clone(),
        display_name: enum_entity.display_name.clone(),
        enum_value_type: enum_entity.enum_value_type.clone(),
        camel_case_name: Some(pascal_case_name),
        pascal_case_name: Some(snake_case_name),
        snake_case_name: Some(macro_case_name),
        macro_case_name: Some(camel_case_name),
        enum_attribute_info_list: enum_attr_info_list,
    };
    Ok(Some(enum_info))
}

async fn build_fk_attribute(
    db: &DbConn,
    cp_associate: &component_entity_associate::Model,
    associate_entity_entity: &entity_associate::Model,
    down_component_entity_entity: &component_entity::Model,
    down_ext_attrs: &Vec<ext_attribute::Model>,
    down_entity: &dd_entity::Model,
    down_entity_attrs: &Vec<entity_attribute::Model>,
    column_domain_type_map: &HashMap<String, DataTypeVO>,
) -> Result<Option<AttributeInfoPO>, TcdtServiceError> {
    let fk_attribute_name = associate_entity_entity
        .fk_attribute_name
        .clone()
        .unwrap_or_default();
    if fk_attribute_name.trim() == String::default() {
        log::warn!(
            "entity_associate: '{}' fk attribute name is empty",
            associate_entity_entity.id_entity_associate
        );
        return Ok(None);
    }
    let fk_camel_case_name = fk_attribute_name;
    let fk_pascal_case_name = camel_case_to_pascal_case(&fk_camel_case_name);
    let fk_snake_case_name = pascal_case_to_snake_case(&fk_pascal_case_name);
    let fk_macro_case_name = snake_case_to_macro_case(&fk_snake_case_name);
    let fk_fg_mandatory: Option<bool>;
    let down_associate_type = associate_entity_entity
        .down_associate_type
        .clone()
        .unwrap_or(DOWN_TYPE_ZERO_TO_MANY.to_string());
    if down_associate_type == DOWN_TYPE_ONE_TO_ONE || down_associate_type == DOWN_TYPE_ONE_TO_MANY {
        fk_fg_mandatory = Some(true);
    } else {
        fk_fg_mandatory = Some(false);
    }
    let mut fk_attr_info = AttributeInfoPO {
        attribute_name: associate_entity_entity.fk_attribute_name.clone(),
        display_name: associate_entity_entity.fk_attribute_display_name.clone(),
        column_name: associate_entity_entity.fk_column_name.clone(),
        fg_primary_key: Some(false),
        fg_mandatory: fk_fg_mandatory,
        attribute_type_code: Some(INTERNAL_FK.to_owned()),
        camel_case_name: Some(fk_camel_case_name.clone()),
        pascal_case_name: Some(fk_pascal_case_name),
        snake_case_name: Some(fk_snake_case_name),
        macro_case_name: Some(fk_macro_case_name),
        ..Default::default()
    };

    let ref_attribute_name = associate_entity_entity.ref_attribute_name.clone();
    if let Some(ref_attribute_name) = ref_attribute_name {
        let ref_camel_case_name = ref_attribute_name;
        let ref_pascal_case_name = camel_case_to_pascal_case(&ref_camel_case_name);
        let ref_snake_case_name = pascal_case_to_snake_case(&ref_pascal_case_name);
        let ref_macro_case_name = snake_case_to_macro_case(&ref_snake_case_name);
        let inner_attribute_info = AttributeInfoPO {
            attribute_name: Some(ref_camel_case_name.clone()),
            display_name: associate_entity_entity.ref_attribute_display_name.clone(),
            note: associate_entity_entity.ref_attribute_display_name.clone(),
            object_type: down_entity.class_name.clone(),
            camel_case_name: Some(ref_camel_case_name.clone()),
            pascal_case_name: Some(ref_pascal_case_name.clone()),
            snake_case_name: Some(ref_snake_case_name.clone()),
            macro_case_name: Some(ref_macro_case_name.clone()),
            ..Default::default()
        };
        fk_attr_info.inner_info = Some(Box::new(inner_attribute_info));
    }

    let out_entity = associate_entity_entity
        .find_linked(entity_associate::UpEntityLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_entity failed");
            TcdtServiceError::build_internal_msg_error("find out_entity failed", err)
        })?
        .ok_or(TcdtServiceError::build_custom_msg(&format!(
            "entity_associate: '{}' can not get down_entity",
            associate_entity_entity.id_entity_associate
        )))?;

    let down_attribute_name = associate_entity_entity.down_attribute_name.clone();
    if let Some(down_attribute_name) = down_attribute_name {
        let out_camel_case_name = down_attribute_name;
        let out_pascal_case_name = camel_case_to_pascal_case(&out_camel_case_name);
        let out_snake_case_name = pascal_case_to_snake_case(&out_pascal_case_name);
        let out_macro_case_name = snake_case_to_macro_case(&out_snake_case_name);
        let out_attribute_info = AttributeInfoPO {
            attribute_name: Some(out_camel_case_name.clone()),
            display_name: associate_entity_entity.down_attribute_display_name.clone(),
            note: associate_entity_entity.down_attribute_display_name.clone(),
            object_type: out_entity.class_name.clone(),
            camel_case_name: Some(out_camel_case_name.clone()),
            pascal_case_name: Some(out_pascal_case_name.clone()),
            snake_case_name: Some(out_snake_case_name.clone()),
            macro_case_name: Some(out_macro_case_name.clone()),
            ..Default::default()
        };
        fk_attr_info.outer_info = Some(Box::new(out_attribute_info));
    }

    let out_entity_attrs = out_entity
        .find_linked(dd_entity::AttributesLinked)
        .order_by_asc(entity_attribute::Column::Sn)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find out_entity_attrs failed");
            TcdtServiceError::build_internal_msg_error("find out_entity_attrs failed", err)
        })?;

    let out_component_entity_option = out_entity
        .find_linked(dd_entity::ComponentEntitiesLinked)
        .filter(component_entity::Column::FgVirtual.ne(true))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component_entity failed");
            TcdtServiceError::build_internal_msg_error("find out_component_entity failed", err)
        })?;

    if out_component_entity_option.is_none() {
        log::warn!(
            "entity: '{}' not instance component",
            out_entity.class_name.unwrap_or_default()
        );
        return Ok(None);
    }
    let out_component_entity = out_component_entity_option.unwrap();
    let out_ext_attr_list = out_component_entity
        .find_linked(component_entity::ExtAttributesLinked)
        .order_by_asc(ext_attribute::Column::Sn)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find out_component_entity failed");
            TcdtServiceError::build_internal_msg_error("find out_component_entity failed", err)
        })?;
    let out_component = out_component_entity
        .find_linked(component_entity::ComponentLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component failed");
            TcdtServiceError::build_internal_msg_error("find out_component failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find down component",
        ))?;
    let out_component_module_entity = out_component
        .find_linked(component::ComponentModuleLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component_module_entity failed");
            TcdtServiceError::build_internal_msg_error(
                "find out_component_module_entity failed",
                err,
            )
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find down component_module",
        ))?;
    let out_entity_info = make_base_entity_info(
        db,
        &out_component_module_entity,
        &out_component,
        &out_component_entity,
        &out_ext_attr_list,
        &out_entity,
        &out_entity_attrs,
        column_domain_type_map,
        true,
    )
    .await?;
    fk_attr_info.out_entity_info = Some(Box::new(out_entity_info.clone()));

    let out_pk_attribute_info = out_entity_info.pk_attribute_info.clone().unwrap();

    fk_attr_info.id_attribute_type = out_pk_attribute_info.id_attribute_type.clone();
    fk_attr_info.len = out_pk_attribute_info.len.clone();
    fk_attr_info.pcs = out_pk_attribute_info.pcs.clone();
    fk_attr_info.default_value = out_pk_attribute_info.default_value.clone();
    fk_attr_info.fg_mandatory = out_pk_attribute_info.fg_mandatory.clone();

    Ok(Some(fk_attr_info))
}

async fn build_ref_attribute(
    db: &DbConn,
    cp_associate: &component_entity_associate::Model,
    associate_entity_entity: &entity_associate::Model,
    down_component_entity_entity: &component_entity::Model,
    down_ext_attrs: &Vec<ext_attribute::Model>,
    down_entity: &dd_entity::Model,
    down_entity_attrs: &Vec<entity_attribute::Model>,
    column_domain_type_map: &HashMap<String, DataTypeVO>,
) -> Result<Option<AttributeInfoPO>, TcdtServiceError> {
    let ref_attribute_name = associate_entity_entity
        .ref_attribute_name
        .clone()
        .unwrap_or_default();
    if ref_attribute_name.trim() == String::default() {
        log::warn!(
            "entity_associate: '{}' ref attribute name is empty",
            associate_entity_entity.id_entity_associate
        );
        return Ok(None);
    }
    let ref_camel_case_name = ref_attribute_name;
    let ref_pascal_case_name = camel_case_to_pascal_case(&ref_camel_case_name);
    let ref_snake_case_name = pascal_case_to_snake_case(&ref_pascal_case_name);
    let ref_macro_case_name = snake_case_to_macro_case(&ref_snake_case_name);
    let down_associate_type = associate_entity_entity
        .down_associate_type
        .clone()
        .unwrap_or(DOWN_TYPE_ZERO_TO_MANY.to_string());
    let fg_fg_mandatory: Option<bool>;
    if down_associate_type == DOWN_TYPE_ONE_TO_ONE || down_associate_type == DOWN_TYPE_ONE_TO_MANY {
        fg_fg_mandatory = Some(true);
    } else {
        fg_fg_mandatory = Some(false);
    }
    let ref_domain_type_code: Option<String>;
    if down_associate_type == DOWN_TYPE_ONE_TO_ONE || down_associate_type == DOWN_TYPE_ZERO_TO_ONE {
        ref_domain_type_code = Some(String::from(INTERNAL_SINGLE_REF));
    } else {
        ref_domain_type_code = Some(String::from(INTERNAL_REF));
    }
    let mut ref_attr_info = AttributeInfoPO {
        attribute_name: associate_entity_entity.ref_attribute_name.clone(),
        display_name: associate_entity_entity.ref_attribute_display_name.clone(),
        fg_mandatory: fg_fg_mandatory,
        attribute_type_code: ref_domain_type_code,
        camel_case_name: Some(ref_camel_case_name.clone()),
        pascal_case_name: Some(ref_pascal_case_name),
        snake_case_name: Some(ref_snake_case_name),
        macro_case_name: Some(ref_macro_case_name),
        ..Default::default()
    };

    let fk_attribute_name = associate_entity_entity.fk_attribute_name.clone();
    if let Some(fk_attribute_name) = fk_attribute_name {
        let fk_camel_case_name = fk_attribute_name;
        let fk_pascal_case_name = camel_case_to_pascal_case(&fk_camel_case_name);
        let fk_snake_case_name = pascal_case_to_snake_case(&fk_pascal_case_name);
        let fk_macro_case_name = snake_case_to_macro_case(&fk_snake_case_name);
        let fk_attribute_info = AttributeInfoPO {
            fg_mandatory: fg_fg_mandatory,
            attribute_name: Some(fk_camel_case_name.clone()),
            display_name: associate_entity_entity.fk_attribute_display_name.clone(),
            note: associate_entity_entity.fk_attribute_display_name.clone(),
            camel_case_name: Some(fk_camel_case_name.clone()),
            pascal_case_name: Some(fk_pascal_case_name.clone()),
            snake_case_name: Some(fk_snake_case_name.clone()),
            macro_case_name: Some(fk_macro_case_name.clone()),
            ..Default::default()
        };
        ref_attr_info.inner_info = Some(Box::new(fk_attribute_info));
    }

    let out_entity = associate_entity_entity
        .find_linked(entity_associate::UpEntityLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_entity failed");
            TcdtServiceError::build_internal_msg_error("find out_entity failed", err)
        })?
        .ok_or(TcdtServiceError::build_custom_msg(
            "can not get down_entity",
        ))?;

    let down_attribute_name = associate_entity_entity.down_attribute_name.clone();
    if let Some(down_attribute_name) = down_attribute_name {
        let out_camel_case_name = down_attribute_name;
        let out_pascal_case_name = camel_case_to_pascal_case(&out_camel_case_name);
        let out_snake_case_name = pascal_case_to_snake_case(&out_pascal_case_name);
        let out_macro_case_name = snake_case_to_macro_case(&out_snake_case_name);
        let out_attribute_info = AttributeInfoPO {
            attribute_name: Some(out_camel_case_name.clone()),
            display_name: associate_entity_entity.down_attribute_display_name.clone(),
            note: associate_entity_entity.down_attribute_display_name.clone(),
            object_type: out_entity.class_name.clone(),
            camel_case_name: Some(out_camel_case_name.clone()),
            pascal_case_name: Some(out_pascal_case_name.clone()),
            snake_case_name: Some(out_snake_case_name.clone()),
            macro_case_name: Some(out_macro_case_name.clone()),
            ..Default::default()
        };
        ref_attr_info.outer_info = Some(Box::new(out_attribute_info));
    }

    let out_entity_attrs = out_entity
        .find_linked(dd_entity::AttributesLinked)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find out_entity_attrs failed");
            TcdtServiceError::build_internal_msg_error("find out_entity failed", err)
        })?;

    let out_component_entity_option = out_entity
        .find_linked(dd_entity::ComponentEntitiesLinked)
        .filter(component_entity::Column::FgVirtual.ne(true))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component_entity failed");
            TcdtServiceError::build_internal_msg_error("find out_entity failed", err)
        })?;

    if out_component_entity_option.is_none() {
        log::warn!(
            "entity: '{}' not instance component entity",
            out_entity.class_name.unwrap_or_default()
        );
        return Ok(None);
    }
    let out_component_entity = out_component_entity_option.unwrap();
    let out_ext_attr_list = out_component_entity
        .find_linked(component_entity::ExtAttributesLinked)
        .order_by_asc(ext_attribute::Column::Sn)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find out_ext_attr_list failed");
            TcdtServiceError::build_internal_msg_error("find out_ext_attr_list failed", err)
        })?;
    let out_component = out_component_entity
        .find_linked(component_entity::ComponentLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find down component failed");
            TcdtServiceError::build_internal_msg_error("find down component failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find down component",
        ))?;
    let out_component_module_entity = out_component
        .find_linked(component::ComponentModuleLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find down component_module failed");
            TcdtServiceError::build_internal_msg_error("find down component_module failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find down component_module",
        ))?;
    let out_entity_info = make_base_entity_info(
        db,
        &out_component_module_entity,
        &out_component,
        &out_component_entity,
        &out_ext_attr_list,
        &out_entity,
        &out_entity_attrs,
        column_domain_type_map,
        true,
    )
    .await?;

    ref_attr_info.object_type = out_entity_info.class_name.clone();
    let out_object_type_package = format!(
        "{}.{}",
        out_entity_info.base_path.clone().unwrap_or_default(),
        out_entity_info.package_name.clone().unwrap_or_default()
    );
    ref_attr_info.object_type_package = Some(out_object_type_package);
    ref_attr_info.out_entity_info = Some(Box::new(out_entity_info.clone()));

    let mut inner_info_option = ref_attr_info.inner_info.clone();
    if let Some(mut fk_attr_info) = inner_info_option {
        let out_pk_attribute_info = out_entity_info.pk_attribute_info.clone().unwrap();
        fk_attr_info.id_attribute_type = out_pk_attribute_info.id_attribute_type.clone();
        fk_attr_info.len = out_pk_attribute_info.len.clone();
        fk_attr_info.pcs = out_pk_attribute_info.pcs.clone();
        fk_attr_info.default_value = out_pk_attribute_info.default_value.clone();
        fk_attr_info.fg_mandatory = out_pk_attribute_info.fg_mandatory.clone();

        ref_attr_info.inner_info = Some(fk_attr_info);
    }

    Ok(Some(ref_attr_info))
}

async fn build_array_attribute(
    db: &DbConn,
    cp_associate: &component_entity_associate::Model,
    associate_entity_entity: &entity_associate::Model,
    up_component_entity_entity: &component_entity::Model,
    up_ext_attrs: &Vec<ext_attribute::Model>,
    up_entity: &dd_entity::Model,
    up_entity_attrs: &Vec<entity_attribute::Model>,
    column_domain_type_map: &HashMap<String, DataTypeVO>,
) -> Result<Option<AttributeInfoPO>, TcdtServiceError> {
    let down_attribute_name = associate_entity_entity
        .down_attribute_name
        .clone()
        .unwrap_or_default();
    if down_attribute_name.trim() == String::default() {
        log::warn!(
            "entity_associate: '{}' down attribute name is empty",
            associate_entity_entity.id_entity_associate
        );
        return Ok(None);
    }
    let down_camel_case_name = down_attribute_name.clone();
    let down_pascal_case_name = camel_case_to_pascal_case(&down_camel_case_name);
    let down_snake_case_name = pascal_case_to_snake_case(&down_pascal_case_name);
    let down_macro_case_name = snake_case_to_macro_case(&down_snake_case_name);
    let down_associate_type = associate_entity_entity
        .down_associate_type
        .clone()
        .unwrap_or(DOWN_TYPE_ZERO_TO_MANY.to_string());
    let ref_domain_type_code: Option<String>;
    if down_associate_type == DOWN_TYPE_ONE_TO_ONE || down_associate_type == DOWN_TYPE_ZERO_TO_ONE {
        ref_domain_type_code = Some(String::from(INTERNAL_SINGLE));
    } else {
        ref_domain_type_code = Some(String::from(INTERNAL_ARRAY));
    }
    let mut attr_info = AttributeInfoPO {
        attribute_name: associate_entity_entity.down_attribute_name.clone(),
        display_name: associate_entity_entity.down_attribute_display_name.clone(),
        note: associate_entity_entity.down_attribute_display_name.clone(),
        attribute_type_code: ref_domain_type_code,
        camel_case_name: Some(down_camel_case_name.clone()),
        pascal_case_name: Some(down_pascal_case_name),
        snake_case_name: Some(down_snake_case_name),
        macro_case_name: Some(down_macro_case_name),
        ..Default::default()
    };

    let up_pk_attr = up_entity_attrs
        .iter()
        .find(|attr| attr.fg_primary_key.unwrap_or(false))
        .ok_or(TcdtServiceError::build_custom_msg(
            "can not get up_entity pk attribute",
        ))?;

    let out_entity = associate_entity_entity
        .find_linked(entity_associate::DownEntityLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find up_entity failed");
            TcdtServiceError::build_internal_msg_error("find up_entity failed", err)
        })?
        .ok_or(TcdtServiceError::build_custom_msg("can not get up_entity"))?;

    let out_ref_attribute_name = associate_entity_entity.ref_attribute_name.clone();
    if let Some(out_ref_attribute_name) = out_ref_attribute_name {
        let out_camel_case_name = out_ref_attribute_name;
        let out_pascal_case_name = camel_case_to_pascal_case(&out_camel_case_name);
        let out_snake_case_name = pascal_case_to_snake_case(&out_pascal_case_name);
        let out_macro_case_name = snake_case_to_macro_case(&out_snake_case_name);
        let out_attribute_info = AttributeInfoPO {
            attribute_name: Some(out_camel_case_name.clone()),
            display_name: associate_entity_entity.ref_attribute_display_name.clone(),
            note: associate_entity_entity.ref_attribute_display_name.clone(),
            object_type: out_entity.class_name.clone(),
            camel_case_name: Some(out_camel_case_name.clone()),
            pascal_case_name: Some(out_pascal_case_name.clone()),
            snake_case_name: Some(out_snake_case_name.clone()),
            macro_case_name: Some(out_macro_case_name.clone()),
            ..Default::default()
        };
        attr_info.outer_info = Some(Box::new(out_attribute_info));
    }

    let out_entity_attrs = out_entity
        .find_linked(dd_entity::AttributesLinked)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find out_entity_attrs failed");
            TcdtServiceError::build_internal_msg_error("find out_entity_attrs failed", err)
        })?;

    let out_component_entity_option = out_entity
        .find_linked(dd_entity::ComponentEntitiesLinked)
        .filter(component_entity::Column::FgVirtual.ne(true))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component_entity_option failed");
            TcdtServiceError::build_internal_msg_error(
                "find out_component_entity_option failed",
                err,
            )
        })?;

    if out_component_entity_option.is_none() {
        log::warn!(
            "entity: '{}' not instance component",
            out_entity.class_name.unwrap_or_default()
        );
        return Ok(None);
    }
    let out_component_entity = out_component_entity_option.unwrap();
    let out_ext_attr_list = out_component_entity
        .find_linked(component_entity::ExtAttributesLinked)
        .order_by_asc(ext_attribute::Column::Sn)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find out_ext_attr_list failed");
            TcdtServiceError::build_internal_msg_error("find out_ext_attr_list failed", err)
        })?;
    let out_component = out_component_entity
        .find_linked(component_entity::ComponentLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component failed");
            TcdtServiceError::build_internal_msg_error("find out_component failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find up component",
        ))?;
    let out_component_module_entity = out_component
        .find_linked(component::ComponentModuleLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component_module_entity failed");
            TcdtServiceError::build_internal_msg_error(
                "find out_component_module_entity failed",
                err,
            )
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find up component_module",
        ))?;
    let out_entity_info = make_base_entity_info(
        db,
        &out_component_module_entity,
        &out_component,
        &out_component_entity,
        &out_ext_attr_list,
        &out_entity,
        &out_entity_attrs,
        column_domain_type_map,
        true,
    )
    .await?;

    attr_info.object_type = out_entity_info.class_name.clone();
    let out_object_type_package = format!(
        "{}.{}",
        out_entity_info.base_path.clone().unwrap_or_default(),
        out_entity_info.package_name.clone().unwrap_or_default()
    );
    attr_info.object_type_package = Some(out_object_type_package);
    attr_info.out_entity_info = Some(Box::new(out_entity_info.clone()));

    let out_fk_attribute_name = associate_entity_entity.fk_attribute_name.clone();
    if let Some(out_fk_attribute_name) = out_fk_attribute_name {
        let out_fk_camel_case_name = out_fk_attribute_name;
        let out_fk_pascal_case_name = camel_case_to_pascal_case(&out_fk_camel_case_name);
        let out_fk_snake_case_name = pascal_case_to_snake_case(&out_fk_pascal_case_name);
        let out_fk_macro_case_name = snake_case_to_macro_case(&out_fk_snake_case_name);
        let out_fk_attribute_info = AttributeInfoPO {
            column_name: associate_entity_entity.fk_column_name.clone(),
            attribute_name: Some(out_fk_camel_case_name.clone()),
            display_name: associate_entity_entity.fk_attribute_display_name.clone(),
            note: associate_entity_entity.fk_attribute_display_name.clone(),
            camel_case_name: Some(out_fk_camel_case_name.clone()),
            pascal_case_name: Some(out_fk_pascal_case_name.clone()),
            snake_case_name: Some(out_fk_snake_case_name.clone()),
            macro_case_name: Some(out_fk_macro_case_name.clone()),
            id_attribute_type: up_pk_attr.id_attribute_type.clone(),
            len: up_pk_attr.len.clone(),
            pcs: up_pk_attr.pcs.clone(),
            default_value: up_pk_attr.default_value.clone(),
            fg_mandatory: up_pk_attr.fg_mandatory.clone(),
            ..Default::default()
        };
        attr_info.outer_fk_info = Some(Box::new(out_fk_attribute_info));
    }

    Ok(Some(attr_info))
}

async fn make_attributes_from_entity(
    db: &DbConn,
    ext_attrs: &Vec<ext_attribute::Model>,
    dd_entity_entity: &dd_entity::Model,
    attrs: &Vec<entity_attribute::Model>,
) -> Result<Vec<AttributeInfoPO>, TcdtServiceError> {
    let enum_associate_list = dd_entity_entity
        .find_linked(dd_entity::EnumAssociatesLinked)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find enum_associate_list failed");
            TcdtServiceError::build_internal_msg_error("find enum_associate_list failed", err)
        })?;
    let enum_id_list = enum_associate_list
        .iter()
        .map(|asso| asso.id_enum.clone().unwrap_or_default())
        .collect::<Vec<_>>();
    let enum_entity_list = dd_enum::Entity::find()
        .filter(dd_enum::Column::IdEnum.is_in(enum_id_list.clone()))
        .order_by_asc(dd_enum::Column::ClassName)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find enum_entity_list failed");
            TcdtServiceError::build_internal_msg_error("find enum_entity_list failed", err)
        })?;
    let enum_attr_list = enum_attribute::Entity::find()
        .filter(enum_attribute::Column::IdEnum.is_in(enum_id_list.clone()))
        .order_by_asc(enum_attribute::Column::Sn)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("find enum_attr_list failed");
            TcdtServiceError::build_internal_msg_error("find enum_attr_list failed", err)
        })?;
    let enum_info_list = make_enum_info_list(enum_entity_list, enum_attr_list)?;
    let mut attr_info_list: Vec<AttributeInfoPO> = vec![];
    for ext_attr in ext_attrs {
        let attr = ext_attr
            .find_linked(ext_attribute::AttributeLinked)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("find attribute failed");
                TcdtServiceError::build_internal_msg_error("find attribute failed", err)
            })?
            .ok_or(TcdtServiceError::build_custom_msg("can not find attribute"))?;
        let attribute_name =
            attr.attribute_name
                .clone()
                .ok_or(TcdtServiceError::build_custom_msg(
                    "attribute name is empty",
                ))?;
        if attribute_name.trim() == "" {
            return Err(TcdtServiceError::build_custom_msg(
                "attribute name is empty",
            ));
        }
        let camel_case_name = attribute_name;
        let pascal_case_name = camel_case_to_pascal_case(&camel_case_name);
        let snake_case_name = pascal_case_to_snake_case(&pascal_case_name);
        let macro_case_name = snake_case_to_macro_case(&snake_case_name);
        let domain_type_code: Option<String>;
        if attr.fg_primary_key.is_some() && attr.fg_primary_key.unwrap() {
            domain_type_code = Some(INTERNAL_PK.to_owned());
        } else {
            domain_type_code = None;
        }
        let mut enum_info_option: Option<EnumInfoPO> = None;
        let enum_associate_option = enum_associate_list
            .iter()
            .find(|enum_associate| enum_associate.id_attribute == Some(attr.id_attribute.clone()));
        if let Some(enum_associate) = enum_associate_option {
            enum_info_option = enum_info_list
                .clone()
                .into_iter()
                .find(|enum_info| Some(enum_info.id_enum.clone()) == enum_associate.id_enum);
        }
        let attr_info = AttributeInfoPO {
            id_ext_attribute: ext_attr.id_ext_attribute.clone(),
            id_attribute_type: attr.id_attribute_type.clone(),
            attribute_name: attr.attribute_name.clone(),
            display_name: attr.display_name.clone(),
            column_name: attr.column_name.clone(),
            fg_primary_key: Some(attr.fg_primary_key.unwrap_or_default()),
            fg_mandatory: attr.fg_mandatory.clone(),
            default_value: attr.default_value.clone(),
            len: attr.len.clone(),
            pcs: attr.pcs.clone(),
            sn: attr.sn.clone(),
            note: attr.note.clone(),
            category: attr.category.clone(),
            camel_case_name: Some(camel_case_name),
            pascal_case_name: Some(pascal_case_name),
            snake_case_name: Some(snake_case_name),
            macro_case_name: Some(macro_case_name),
            attribute_type_code: domain_type_code,
            enum_info: enum_info_option,
            ..Default::default()
        };
        attr_info_list.push(attr_info);
    }
    Ok(attr_info_list)
}

async fn make_base_entity_info(
    db: &DbConn,
    component_module_entity: &component_module::Model,
    com_entity: &component::Model,
    component_entity_entity: &component_entity::Model,
    ext_attrs: &Vec<ext_attribute::Model>,
    dd_entity_entity: &dd_entity::Model,
    attrs: &Vec<entity_attribute::Model>,
    column_domain_type_map: &HashMap<String, DataTypeVO>,
    fg_out_entity: bool,
) -> Result<EntityInfoPO, TcdtServiceError> {
    let class_name = dd_entity_entity
        .class_name
        .clone()
        .ok_or(TcdtServiceError::build_custom_msg("class name is empty"))?;
    if class_name.trim() == "" {
        return Err(TcdtServiceError::build_custom_msg("class name is empty"));
    }
    let pascal_case_name = class_name;
    let snake_case_name = pascal_case_to_snake_case(&pascal_case_name);
    let macro_case_name = snake_case_to_macro_case(&snake_case_name);
    let camel_case_name = snake_case_to_camel_case(&snake_case_name);

    let mut entity_attr_info_list =
        make_attributes_from_entity(db, ext_attrs, dd_entity_entity, attrs).await?;
    if fg_out_entity {
        entity_attr_info_list =
            build_attribute_type_info(entity_attr_info_list, column_domain_type_map)?;
    }
    let entity_attr_info_list = entity_attr_info_list;
    let pk_attr_info = entity_attr_info_list
        .clone()
        .into_iter()
        .find(|attr| attr.fg_primary_key.unwrap_or_default())
        .ok_or(TcdtServiceError::build_internal_msg(&format!(
            "class: '{}' pk attribute not set",
            pascal_case_name
        )))?;

    let base_attr_info_list = entity_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| !attr.fg_primary_key.unwrap_or_default())
        .collect();

    let base_path = build_base_path(component_module_entity);

    let entity_info = EntityInfoPO {
        id_component_entity: component_entity_entity.id_component_entity.clone(),
        id_entity: dd_entity_entity.id_entity.clone(),
        fg_main: false,
        param_json: None,
        real_base_path: Some(base_path.clone()),
        real_package_name: com_entity.package_name.clone(),
        base_path: Some(base_path),
        package_name: com_entity.package_name.clone(),
        display_name: dd_entity_entity.display_name.clone(),
        class_name: dd_entity_entity.class_name.clone(),
        table_name: dd_entity_entity.table_name.clone(),
        pk_attribute_code: dd_entity_entity.pk_attribute_code.clone(),
        pk_attribute_name: dd_entity_entity.pk_attribute_name.clone(),
        pk_attribute_type_name: dd_entity_entity.pk_attribute_type_name.clone(),
        camel_case_name: Some(camel_case_name),
        pascal_case_name: Some(pascal_case_name),
        snake_case_name: Some(snake_case_name),
        macro_case_name: Some(macro_case_name),
        comp_info: None,
        out_base_package_list: vec![],
        fk_attribute_info_list: vec![],
        attribute_info_list: entity_attr_info_list,
        base_attribute_info_list: base_attr_info_list,
        agg_fk_attribute_info_list: vec![],
        agg_up_attribute_info_list: vec![],
        agg_up_single_attribute_info_list: vec![],
        agg_down_attribute_info_list: vec![],
        pk_attribute_info: Some(pk_attr_info.clone()),
        down_single_attribute_info_list: vec![],
        down_entity_info_list: vec![],
        up_single_attribute_info_list: vec![],
        up_attribute_info_list: vec![],
        up_entity_info_list: vec![],
        down_attribute_info_list: vec![],
        agg_down_single_attribute_info_list: vec![],
        child_entity_info_list: vec![],
        main_entity_info: None,
    };
    Ok(entity_info)
}

async fn build_real_path(
    db: &DbConn,
    dd_entity_entity: &dd_entity::Model,
    entity_info: &mut EntityInfoPO,
) -> Result<(), TcdtServiceError> {
    let out_component_entity = dd_entity_entity
        .find_linked(dd_entity::ComponentEntitiesLinked)
        .filter(component_entity::Column::FgVirtual.ne(true))
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component_entity failed");
            TcdtServiceError::build_internal_msg_error("find out_component_entity failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(&format!(
            "entity: '{}' not instance component",
            dd_entity_entity.class_name.clone().unwrap_or_default()
        )))?;

    let out_component = out_component_entity
        .find_linked(component_entity::ComponentLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component failed");
            TcdtServiceError::build_internal_msg_error("find out_component failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find down component",
        ))?;
    let out_component_module_entity = out_component
        .find_linked(component::ComponentModuleLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find out_component_module_entity failed");
            TcdtServiceError::build_internal_msg_error(
                "find out_component_module_entity failed",
                err,
            )
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find down component_module",
        ))?;
    entity_info.real_base_path = out_component_module_entity.path.clone();
    entity_info.real_package_name = out_component.package_name.clone();
    Ok(())
}

fn build_base_path(component_module_entity: &component_module::Model) -> String {
    component_module_entity.path.clone().unwrap_or_default()
}

fn build_desc_info(
    entity_info: EntityInfoPO,
    agg_entity_associate_list: &Vec<entity_associate::Model>,
    comp_entity: &component::Model,
    column_domain_type_map: &HashMap<String, DataTypeVO>,
) -> Result<EntityInfoPO, TcdtServiceError> {
    let all_attr_info_list =
        build_agg_domain_type_code(&entity_info, agg_entity_associate_list, comp_entity);
    let all_attr_info_list = build_attribute_type_info(all_attr_info_list, column_domain_type_map)?;
    let up_entity_info_list = get_ref_entity_info(all_attr_info_list.clone());
    let down_entity_info_list = get_down_entity_info(all_attr_info_list.clone());
    let fk_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_FK.to_string()))
        .collect();
    let ref_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_REF.to_string()))
        .collect();
    let single_ref_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_SINGLE_REF.to_string()))
        .collect();
    let single_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_SINGLE.to_string()))
        .collect();
    let array_ref_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_ARRAY.to_string()))
        .collect();
    let agg_fk_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_AGG_FK.to_string()))
        .collect();
    let agg_ref_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_AGG_REF.to_string()))
        .collect();
    let agg_single_ref_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_AGG_SINGLE_REF.to_string()))
        .collect();
    let agg_single_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_AGG_SINGLE.to_string()))
        .collect();
    let agg_array_ref_attr_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr| attr.attribute_type_code == Some(INTERNAL_AGG_ARRAY.to_string()))
        .collect();
    let base_attribute_info_list = all_attr_info_list
        .clone()
        .into_iter()
        .filter(|attr_info| {
            if attr_info.attribute_type_code == Some(INTERNAL_PK.to_string())
                || attr_info.attribute_type_code == Some(INTERNAL_FK.to_string())
                || attr_info.attribute_type_code == Some(INTERNAL_AGG_FK.to_string())
                || ref_type_check(attr_info)
            {
                return false;
            }
            return true;
        })
        .collect::<Vec<_>>();
    let pk_attribute_info = all_attr_info_list
        .clone()
        .into_iter()
        .find(|attr| attr.attribute_type_code == Some(INTERNAL_PK.to_string()));
    let result_entity_info = EntityInfoPO {
        id_entity: entity_info.id_entity.clone(),
        id_component_entity: entity_info.id_component_entity.clone(),
        fg_main: entity_info.fg_main.clone(),
        param_json: entity_info.param_json.clone(),
        real_base_path: entity_info.real_base_path.clone(),
        real_package_name: entity_info.real_package_name.clone(),
        base_path: entity_info.base_path.clone(),
        package_name: entity_info.package_name.clone(),
        display_name: entity_info.display_name.clone(),
        class_name: entity_info.class_name.clone(),
        table_name: entity_info.table_name.clone(),
        pk_attribute_code: entity_info.pk_attribute_code.clone(),
        pk_attribute_name: entity_info.pk_attribute_name.clone(),
        pk_attribute_type_name: entity_info.pk_attribute_type_name.clone(),
        camel_case_name: entity_info.camel_case_name.clone(),
        pascal_case_name: entity_info.pascal_case_name.clone(),
        snake_case_name: entity_info.snake_case_name.clone(),
        macro_case_name: entity_info.macro_case_name.clone(),
        out_base_package_list: entity_info.out_base_package_list.clone(),
        child_entity_info_list: entity_info.child_entity_info_list.clone(),
        main_entity_info: entity_info.main_entity_info.clone(),
        attribute_info_list: all_attr_info_list.clone(),
        base_attribute_info_list: base_attribute_info_list.clone(),
        pk_attribute_info: pk_attribute_info.clone(),
        fk_attribute_info_list: fk_attr_info_list,
        up_single_attribute_info_list: single_ref_attr_info_list,
        up_attribute_info_list: ref_attr_info_list,
        down_attribute_info_list: array_ref_attr_info_list,
        down_single_attribute_info_list: single_attr_info_list,
        up_entity_info_list: up_entity_info_list,
        down_entity_info_list: down_entity_info_list,
        comp_info: None,
        agg_fk_attribute_info_list: agg_fk_attr_info_list,
        agg_down_single_attribute_info_list: agg_single_attr_info_list,
        agg_up_attribute_info_list: agg_ref_attr_info_list,
        agg_up_single_attribute_info_list: agg_single_ref_attr_info_list,
        agg_down_attribute_info_list: agg_array_ref_attr_info_list,
    };
    Ok(result_entity_info)
}

fn build_attribute_type_info(
    all_attr_info_list: Vec<AttributeInfoPO>,
    column_domain_type_map: &HashMap<String, DataTypeVO>,
) -> Result<Vec<AttributeInfoPO>, TcdtServiceError> {
    let mut all_attr_info_list_new: Vec<AttributeInfoPO> = vec![];
    for mut attr_info in all_attr_info_list {
        if ref_type_check(&attr_info) {
            all_attr_info_list_new.push(attr_info);
            continue;
        }
        let id_attribute_type =
            attr_info
                .id_attribute_type
                .clone()
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "ext attribute: {} id_attribute_type is None",
                    attr_info.id_ext_attribute
                )))?;
        let attribute_type_vo = column_domain_type_map.get(&id_attribute_type).ok_or(
            TcdtServiceError::build_internal_msg(&format!(
                "id_attribute_type: {} can not get info",
                id_attribute_type
            )),
        )?;
        if attr_info.len.is_none() {
            attr_info.len = attribute_type_vo.len.clone();
        }
        if attr_info.pcs.is_none() {
            attr_info.pcs = attribute_type_vo.pcs.clone();
        }
        if attr_info.attribute_type_code.is_none()
            || attr_info
                .attribute_type_code
                .clone()
                .unwrap()
                .trim()
                .is_empty()
        {
            attr_info.attribute_type_code = attribute_type_vo.code.clone();
        }
        if attr_info.column_type.is_none()
            || attr_info.column_type.clone().unwrap().trim().is_empty()
        {
            attr_info.column_type = attribute_type_vo.column_type.clone();
        }
        if attr_info.object_type.is_none()
            || attr_info.object_type.clone().unwrap().trim().is_empty()
        {
            attr_info.object_type = attribute_type_vo.object_type.clone();
        }
        if attr_info.default_value.is_none()
            || attr_info.default_value.clone().unwrap().trim().is_empty()
        {
            attr_info.default_value = attribute_type_vo.default_value.clone();
        }
        if attr_info.fg_mandatory.is_none() {
            attr_info.fg_mandatory = attribute_type_vo.fg_mandatory.clone();
        }
        if attr_info.ext1.is_none() || attr_info.ext1.clone().unwrap().trim().is_empty() {
            attr_info.ext1 = attribute_type_vo.ext1.clone();
        }
        if attr_info.ext2.is_none() || attr_info.ext2.clone().unwrap().trim().is_empty() {
            attr_info.ext2 = attribute_type_vo.ext2.clone();
        }
        if attr_info.ext3.is_none() || attr_info.ext3.clone().unwrap().trim().is_empty() {
            attr_info.ext3 = attribute_type_vo.ext3.clone();
        }
        if attr_info.ext4.is_none() || attr_info.ext4.clone().unwrap().trim().is_empty() {
            attr_info.ext4 = attribute_type_vo.ext4.clone();
        }
        if attr_info.ext5.is_none() || attr_info.ext5.clone().unwrap().trim().is_empty() {
            attr_info.ext5 = attribute_type_vo.ext5.clone();
        }
        if attr_info.ext6.is_none() || attr_info.ext6.clone().unwrap().trim().is_empty() {
            attr_info.ext6 = attribute_type_vo.ext6.clone();
        }

        all_attr_info_list_new.push(attr_info);
    }
    Ok(all_attr_info_list_new)
}

fn build_agg_domain_type_code(
    entity_info: &EntityInfoPO,
    agg_entity_associate_list: &Vec<entity_associate::Model>,
    comp_entity: &component::Model,
) -> Vec<AttributeInfoPO> {
    let mut all_attr_info_list = entity_info.attribute_info_list.clone();
    let fk_attr_name_list = agg_entity_associate_list
        .iter()
        .map(|asso| asso.fk_attribute_name.clone().unwrap_or_default())
        .filter(|fk_attr| !fk_attr.trim().is_empty())
        .collect::<Vec<_>>();
    let ref_attr_name_list = agg_entity_associate_list
        .iter()
        .map(|asso| asso.ref_attribute_name.clone().unwrap_or_default())
        .filter(|fk_attr| !fk_attr.trim().is_empty())
        .collect::<Vec<_>>();
    let down_attr_name_list = agg_entity_associate_list
        .iter()
        .map(|asso| asso.down_attribute_name.clone().unwrap_or_default())
        .filter(|fk_attr| !fk_attr.trim().is_empty())
        .collect::<Vec<_>>();
    if comp_entity.id_main_component_entity == Some(entity_info.id_component_entity.clone()) {
        all_attr_info_list.iter_mut().for_each(|attr_info| {
            if attr_info.attribute_type_code == Some(INTERNAL_ARRAY.to_string()) {
                if down_attr_name_list.contains(&attr_info.attribute_name.clone().unwrap()) {
                    attr_info.attribute_type_code = Some(INTERNAL_AGG_ARRAY.to_string());
                }
            }
            if attr_info.attribute_type_code == Some(INTERNAL_SINGLE.to_string()) {
                if down_attr_name_list.contains(&attr_info.attribute_name.clone().unwrap()) {
                    attr_info.attribute_type_code = Some(INTERNAL_AGG_SINGLE.to_string());
                }
            }
        });
    } else {
        all_attr_info_list.iter_mut().for_each(|attr_info| {
            if attr_info.attribute_type_code == Some(INTERNAL_FK.to_string()) {
                if fk_attr_name_list.contains(&attr_info.attribute_name.clone().unwrap()) {
                    attr_info.attribute_type_code = Some(INTERNAL_AGG_FK.to_string());
                }
            }
            if attr_info.attribute_type_code == Some(INTERNAL_REF.to_string()) {
                if ref_attr_name_list.contains(&attr_info.attribute_name.clone().unwrap()) {
                    attr_info.attribute_type_code = Some(INTERNAL_AGG_REF.to_string());
                }
            }
            if attr_info.attribute_type_code == Some(INTERNAL_SINGLE_REF.to_string()) {
                if ref_attr_name_list.contains(&attr_info.attribute_name.clone().unwrap()) {
                    attr_info.attribute_type_code = Some(INTERNAL_AGG_SINGLE_REF.to_string());
                }
            }
        });
    }
    all_attr_info_list
}

fn get_ref_entity_info(attr_info_list: Vec<AttributeInfoPO>) -> Vec<EntityInfoPO> {
    let ref_entity_info_list = attr_info_list
        .into_iter()
        .filter(|attr| {
            attr.attribute_type_code == Some(INTERNAL_REF.to_string())
                || attr.attribute_type_code == Some(INTERNAL_SINGLE_REF.to_string())
        })
        .map(|attr| *attr.out_entity_info.unwrap())
        .collect::<Vec<_>>();

    distinct(ref_entity_info_list)
}

fn get_down_entity_info(attr_info_list: Vec<AttributeInfoPO>) -> Vec<EntityInfoPO> {
    let ref_entity_info_list = attr_info_list
        .into_iter()
        .filter(|attr| {
            attr.attribute_type_code == Some(INTERNAL_SINGLE.to_string())
                || attr.attribute_type_code == Some(INTERNAL_ARRAY.to_string())
        })
        .map(|attr| *attr.out_entity_info.unwrap())
        .collect::<Vec<_>>();

    distinct(ref_entity_info_list)
}

fn distinct(ref_entity_info_list: Vec<EntityInfoPO>) -> Vec<EntityInfoPO> {
    let mut entity_id_list: Vec<String> = vec![];
    let mut result: Vec<EntityInfoPO> = vec![];
    for ref_entity_info in ref_entity_info_list {
        if entity_id_list.contains(&ref_entity_info.id_entity) {
            continue;
        }
        entity_id_list.push(ref_entity_info.id_entity.clone());
        result.push(ref_entity_info.clone());
    }
    result
}

fn ref_type_check(attr_info: &AttributeInfoPO) -> bool {
    attr_info.attribute_type_code == Some(INTERNAL_REF.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_SINGLE_REF.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_ARRAY.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_SINGLE.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_AGG_SINGLE_REF.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_AGG_REF.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_AGG_ARRAY.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_AGG_SINGLE.to_string())
}

fn distinct_out_entity_info(entity_vo_list: Vec<EntityInfoPO>) -> Vec<EntityInfoPO> {
    let self_entity_id_list = entity_vo_list
        .iter()
        .map(|entity_vo| entity_vo.id_entity.clone())
        .collect::<Vec<_>>();
    let entity_vo_list = entity_vo_list
        .into_iter()
        .map(|mut entity_vo| {
            entity_vo.up_entity_info_list = entity_vo
                .up_entity_info_list
                .into_iter()
                .filter(|up_enti| !self_entity_id_list.contains(&up_enti.id_entity))
                .collect::<Vec<_>>();
            entity_vo.down_entity_info_list = entity_vo
                .down_entity_info_list
                .into_iter()
                .filter(|up_enti| !self_entity_id_list.contains(&up_enti.id_entity))
                .collect::<Vec<_>>();
            entity_vo
        })
        .collect::<Vec<_>>();
    entity_vo_list
}

fn make_base_package_name_list(attribute_info_list: &Vec<AttributeInfoPO>) -> Vec<BasePackageInfo> {
    let mut package_name_str_list: Vec<String> = vec![];
    let mut package_name_list: Vec<BasePackageInfo> = vec![];
    attribute_info_list.iter().for_each(|attr_info| {
        if ref_type_check(attr_info) {
            return;
        }
        if let Some(object_type_package) = attr_info.object_type_package.clone() {
            if object_type_package.trim() != String::default() {
                let package_name_str = format!(
                    "{}.{}",
                    object_type_package,
                    attr_info.object_type.clone().unwrap_or_default()
                );
                if package_name_str_list.contains(&package_name_str) {
                    return;
                }
                package_name_str_list.push(package_name_str);
                package_name_list.push(BasePackageInfo {
                    object_type_package: attr_info.object_type_package.clone().unwrap_or_default(),
                    object_type: attr_info.object_type.clone().unwrap_or_default(),
                });
            }
        }
    });
    package_name_list
}
fn distinct_base_package_info(
    all_out_base_package_list: Vec<BasePackageInfo>,
) -> Vec<BasePackageInfo> {
    let mut package_name_str_list: Vec<String> = vec![];
    let mut package_name_list: Vec<BasePackageInfo> = vec![];
    all_out_base_package_list
        .into_iter()
        .for_each(|base_package| {
            let package_name_str = format!(
                "{}.{}",
                base_package.object_type_package, base_package.object_type
            );
            if package_name_str_list.contains(&package_name_str) {
                return;
            }
            package_name_str_list.push(package_name_str);
            package_name_list.push(base_package);
        });
    package_name_list
}
