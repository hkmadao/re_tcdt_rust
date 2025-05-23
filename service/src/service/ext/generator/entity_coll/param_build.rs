use super::coll_cache::CollCache;
use crate::dto::po::ext::generate::collection::{
    ApplicationInfoPO, AttributeInfoPO, BasePackageInfo, EntityInfoPO, EnumAttributeInfoPO,
    EnumInfoPO,
};
use crate::service::ext::generator::genrate_const::{
    DOWN_TYPE_ONE_TO_MANY, DOWN_TYPE_ONE_TO_ONE, DOWN_TYPE_ZERO_TO_MANY, DOWN_TYPE_ZERO_TO_ONE,
    INTERNAL_ARRAY, INTERNAL_FK, INTERNAL_PK, INTERNAL_REF, INTERNAL_SINGLE, INTERNAL_SINGLE_REF,
};
use ::entity::entity::{
    dd_entity, dd_enum, entity_associate, entity_attribute, entity_collection, enum_associate,
    enum_attribute,
};
use sea_orm::*;
use tcdt_common::name_switch_util::{
    camel_case_to_pascal_case, pascal_case_to_snake_case, snake_case_to_camel_case,
    snake_case_to_macro_case,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;

pub(crate) struct ParamBuild {
    pub coll_cache: CollCache,
}

impl ParamBuild {
    pub async fn new(db: &DbConn, id_coll: String) -> Result<ParamBuild, TcdtServiceError> {
        Ok(ParamBuild {
            coll_cache: CollCache::load_coll_data(db, id_coll).await?,
        })
    }
    pub(crate) fn build(
        self: &Self,
        id_coll: String,
    ) -> Result<ApplicationInfoPO, TcdtServiceError> {
        let mut entity_vo_list: Vec<EntityInfoPO> = vec![];
        let coll_entity = self
            .coll_cache
            .coll_list
            .iter()
            .find(|coll| coll.id_entity_collection == id_coll)
            .ok_or(TcdtServiceError::build_internal_msg("coll is not found"))?;
        let enum_info_list = self.make_inner_enum_info_list()?;

        let out_enum_info_list = self.make_out_enum_info_list()?;

        let mut all_enum_info_list = enum_info_list.clone();

        all_enum_info_list.append(&mut out_enum_info_list.clone());

        let enum_associate_list = self.coll_cache.enum_associater_list.clone();

        let entities = self.coll_cache.inner_entity_list.clone();
        for dd_entity_entity in entities {
            let entity_attrs = self
                .coll_cache
                .attr_list
                .clone()
                .into_iter()
                .filter(|attr| attr.id_entity == Some(dd_entity_entity.id_entity.clone()))
                .collect::<Vec<_>>();
            let mut entity_info = self.make_base_entity_info(
                &coll_entity,
                &dd_entity_entity,
                &entity_attrs,
                &enum_associate_list,
                &all_enum_info_list,
                false,
            )?;
            let mut attr_info_list = entity_info.attribute_info_list.clone();
            let up_associates = self
                .coll_cache
                .entity_associate_list
                .clone()
                .into_iter()
                .filter(|asso| asso.id_down == Some(dd_entity_entity.id_entity.clone()))
                .collect::<Vec<_>>();
            for up_asso in up_associates {
                let fk_attribute_info =
                    self.build_fk_attribute(&up_asso, &dd_entity_entity, &entity_attrs)?;
                if let Some(fk_attribute_info) = fk_attribute_info {
                    attr_info_list.push(fk_attribute_info);
                }
                let ref_attribute_info =
                    self.build_ref_attribute(&up_asso, &dd_entity_entity, &entity_attrs)?;
                if let Some(ref_attribute_info) = ref_attribute_info {
                    attr_info_list.push(ref_attribute_info);
                }
            }

            let down_associates = self
                .coll_cache
                .entity_associate_list
                .clone()
                .into_iter()
                .filter(|asso| asso.id_up == Some(dd_entity_entity.id_entity.clone()))
                .collect::<Vec<_>>();
            for down_asso in down_associates {
                let array_attribute_info =
                    self.build_array_attribute(&down_asso, &dd_entity_entity, &entity_attrs)?;
                if let Some(array_attribute_info) = array_attribute_info {
                    attr_info_list.push(array_attribute_info);
                }
            }
            entity_info.attribute_info_list = attr_info_list;
            entity_info = self.build_desc_info(entity_info)?;

            entity_vo_list.push(entity_info);
        }

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

        let app_info = ApplicationInfoPO {
            package_name: coll_entity.package_name.clone(),
            param_json: None,
            display_name: coll_entity.display_name.clone(),
            base_path: coll_entity.package_name.clone(),
            entities: entity_vo_list,
            enums: enum_info_list,
            out_base_package_list: base_package_name_list,
        };
        Ok(app_info)
    }

    fn make_inner_enum_info_list(self: &Self) -> Result<Vec<EnumInfoPO>, TcdtServiceError> {
        let enums = self.coll_cache.inner_enum_list.clone();
        let enum_id_list = enums
            .iter()
            .map(|enum_entity| enum_entity.id_enum.clone())
            .collect::<Vec<String>>();
        let enum_attr_list = self
            .coll_cache
            .enum_attr_list
            .clone()
            .into_iter()
            .filter(|attr| enum_id_list.contains(&attr.id_enum.clone().unwrap()))
            .collect::<Vec<_>>();
        let enum_info_list = make_enum_info_list(enums, enum_attr_list)?;
        Ok(enum_info_list)
    }

    fn make_out_enum_info_list(self: &Self) -> Result<Vec<EnumInfoPO>, TcdtServiceError> {
        let out_enum_list = self.coll_cache.out_enum_list.clone();
        let out_enum_id_list = out_enum_list
            .iter()
            .map(|out_enum| out_enum.id_enum.clone())
            .collect::<Vec<_>>();
        let out_enum_attr_list = self
            .coll_cache
            .enum_attr_list
            .clone()
            .into_iter()
            .filter(|attr| out_enum_id_list.contains(&attr.id_enum.clone().unwrap()))
            .collect::<Vec<_>>();
        let out_enum_info_list = make_enum_info_list(out_enum_list, out_enum_attr_list)?;
        Ok(out_enum_info_list)
    }

    fn build_fk_attribute(
        self: &Self,
        associate_entity: &entity_associate::Model,
        down_entity: &dd_entity::Model,
        down_entity_attrs: &Vec<entity_attribute::Model>,
    ) -> Result<Option<AttributeInfoPO>, TcdtServiceError> {
        let fk_attribute_name = associate_entity
            .fk_attribute_name
            .clone()
            .unwrap_or_default();
        if fk_attribute_name.trim() == String::default() {
            log::warn!(
                "entity_associate: '{}' fk attribute name is empty",
                associate_entity.id_entity_associate
            );
            return Ok(None);
        }
        let fk_camel_case_name = fk_attribute_name;
        let fk_pascal_case_name = camel_case_to_pascal_case(&fk_camel_case_name);
        let fk_snake_case_name = pascal_case_to_snake_case(&fk_pascal_case_name);
        let fk_macro_case_name = snake_case_to_macro_case(&fk_snake_case_name);
        let fk_fg_mandatory: Option<bool>;
        let down_associate_type = associate_entity
            .down_associate_type
            .clone()
            .unwrap_or(DOWN_TYPE_ZERO_TO_MANY.to_string());
        if down_associate_type == DOWN_TYPE_ONE_TO_ONE
            || down_associate_type == DOWN_TYPE_ONE_TO_MANY
        {
            fk_fg_mandatory = Some(true);
        } else {
            fk_fg_mandatory = Some(false);
        }
        let mut fk_attr_info = AttributeInfoPO {
            attribute_name: associate_entity.fk_attribute_name.clone(),
            display_name: associate_entity.fk_attribute_display_name.clone(),
            column_name: associate_entity.fk_column_name.clone(),
            fg_primary_key: Some(false),
            fg_mandatory: fk_fg_mandatory,
            fg_foreign_key: associate_entity.fg_foreign_key.clone(),
            attribute_type_code: Some(INTERNAL_FK.to_owned()),
            camel_case_name: Some(fk_camel_case_name.clone()),
            pascal_case_name: Some(fk_pascal_case_name),
            snake_case_name: Some(fk_snake_case_name),
            macro_case_name: Some(fk_macro_case_name),
            ..Default::default()
        };

        let ref_attribute_name = associate_entity.ref_attribute_name.clone();
        if let Some(ref_attribute_name) = ref_attribute_name {
            let ref_camel_case_name = ref_attribute_name;
            let ref_pascal_case_name = camel_case_to_pascal_case(&ref_camel_case_name);
            let ref_snake_case_name = pascal_case_to_snake_case(&ref_pascal_case_name);
            let ref_macro_case_name = snake_case_to_macro_case(&ref_snake_case_name);
            let inner_attribute_info = AttributeInfoPO {
                attribute_name: Some(ref_camel_case_name.clone()),
                display_name: associate_entity.ref_attribute_display_name.clone(),
                note: associate_entity.ref_attribute_display_name.clone(),
                object_type: down_entity.class_name.clone(),
                camel_case_name: Some(ref_camel_case_name.clone()),
                pascal_case_name: Some(ref_pascal_case_name.clone()),
                snake_case_name: Some(ref_snake_case_name.clone()),
                macro_case_name: Some(ref_macro_case_name.clone()),
                ..Default::default()
            };
            fk_attr_info.inner_info = Some(Box::new(inner_attribute_info));
        }

        let out_entity = self
            .coll_cache
            .all_entity_list
            .iter()
            .find(|enti| associate_entity.id_up == Some(enti.id_entity.clone()))
            .ok_or(TcdtServiceError::build_custom_msg(&format!(
                "entity_associate: '{}' can not get down_entity",
                associate_entity.id_entity_associate
            )))?
            .clone();

        let out_attribute_name_option = associate_entity.down_attribute_name.clone();
        if let Some(out_attribute_name) = out_attribute_name_option {
            let out_camel_case_name = out_attribute_name;
            let out_pascal_case_name = camel_case_to_pascal_case(&out_camel_case_name);
            let out_snake_case_name = pascal_case_to_snake_case(&out_pascal_case_name);
            let out_macro_case_name = snake_case_to_macro_case(&out_snake_case_name);
            let out_attribute_info = AttributeInfoPO {
                attribute_name: Some(out_camel_case_name.clone()),
                display_name: associate_entity.down_attribute_display_name.clone(),
                note: associate_entity.down_attribute_display_name.clone(),
                object_type: out_entity.class_name.clone(),
                camel_case_name: Some(out_camel_case_name.clone()),
                pascal_case_name: Some(out_pascal_case_name.clone()),
                snake_case_name: Some(out_snake_case_name.clone()),
                macro_case_name: Some(out_macro_case_name.clone()),
                ..Default::default()
            };
            fk_attr_info.outer_info = Some(Box::new(out_attribute_info));
        }

        let out_entity_attrs = self
            .coll_cache
            .attr_list
            .clone()
            .into_iter()
            .filter(|attr| attr.id_entity == Some(out_entity.id_entity.clone()))
            .collect::<Vec<_>>();

        let out_coll = self
            .coll_cache
            .coll_list
            .iter()
            .find(|coll| out_entity.id_entity_collection == Some(coll.id_entity_collection.clone()))
            .ok_or(TcdtServiceError::build_internal_msg("out_collection empty"))?
            .clone();

        let out_entity_info = self.make_base_entity_info(
            &out_coll,
            &out_entity,
            &out_entity_attrs,
            &vec![],
            &vec![],
            true,
        )?;
        fk_attr_info.out_entity_info = Some(Box::new(out_entity_info.clone()));

        let out_pk_attribute_info = out_entity_info.pk_attribute_info.clone().unwrap();

        fk_attr_info.id_attribute_type = out_pk_attribute_info.id_attribute_type.clone();
        fk_attr_info.len = out_pk_attribute_info.len.clone();
        fk_attr_info.pcs = out_pk_attribute_info.pcs.clone();
        fk_attr_info.default_value = out_pk_attribute_info.default_value.clone();
        fk_attr_info.fg_mandatory = out_pk_attribute_info.fg_mandatory.clone();

        Ok(Some(fk_attr_info))
    }

    fn build_ref_attribute(
        self: &Self,
        associate_entity: &entity_associate::Model,
        down_entity: &dd_entity::Model,
        down_entity_attrs: &Vec<entity_attribute::Model>,
    ) -> Result<Option<AttributeInfoPO>, TcdtServiceError> {
        let ref_attribute_name = associate_entity
            .ref_attribute_name
            .clone()
            .unwrap_or_default();
        if ref_attribute_name.trim() == String::default() {
            log::warn!(
                "entity_associate: '{}' ref attribute name is empty",
                associate_entity.id_entity_associate
            );
            return Ok(None);
        }
        let ref_camel_case_name = ref_attribute_name;
        let ref_pascal_case_name = camel_case_to_pascal_case(&ref_camel_case_name);
        let ref_snake_case_name = pascal_case_to_snake_case(&ref_pascal_case_name);
        let ref_macro_case_name = snake_case_to_macro_case(&ref_snake_case_name);
        let down_associate_type = associate_entity
            .down_associate_type
            .clone()
            .unwrap_or(DOWN_TYPE_ZERO_TO_MANY.to_string());
        let fg_fg_mandatory: Option<bool>;
        if down_associate_type == DOWN_TYPE_ONE_TO_ONE
            || down_associate_type == DOWN_TYPE_ONE_TO_MANY
        {
            fg_fg_mandatory = Some(true);
        } else {
            fg_fg_mandatory = Some(false);
        }
        let ref_domain_type_code: Option<String>;
        if down_associate_type == DOWN_TYPE_ONE_TO_ONE
            || down_associate_type == DOWN_TYPE_ZERO_TO_ONE
        {
            ref_domain_type_code = Some(String::from(INTERNAL_SINGLE_REF));
        } else {
            ref_domain_type_code = Some(String::from(INTERNAL_REF));
        }
        let mut ref_attr_info = AttributeInfoPO {
            attribute_name: associate_entity.ref_attribute_name.clone(),
            display_name: associate_entity.ref_attribute_display_name.clone(),
            fg_mandatory: fg_fg_mandatory,
            attribute_type_code: ref_domain_type_code,
            camel_case_name: Some(ref_camel_case_name.clone()),
            pascal_case_name: Some(ref_pascal_case_name),
            snake_case_name: Some(ref_snake_case_name),
            macro_case_name: Some(ref_macro_case_name),
            ..Default::default()
        };

        let fk_attribute_name = associate_entity.fk_attribute_name.clone();
        if let Some(fk_attribute_name) = fk_attribute_name {
            let fk_camel_case_name = fk_attribute_name;
            let fk_pascal_case_name = camel_case_to_pascal_case(&fk_camel_case_name);
            let fk_snake_case_name = pascal_case_to_snake_case(&fk_pascal_case_name);
            let fk_macro_case_name = snake_case_to_macro_case(&fk_snake_case_name);
            let fk_attribute_info = AttributeInfoPO {
                fg_foreign_key: associate_entity.fg_foreign_key.clone(),
                fg_mandatory: fg_fg_mandatory,
                column_name: associate_entity.fk_column_name.clone(),
                attribute_name: Some(fk_camel_case_name.clone()),
                display_name: associate_entity.fk_attribute_display_name.clone(),
                note: associate_entity.fk_attribute_display_name.clone(),
                camel_case_name: Some(fk_camel_case_name.clone()),
                pascal_case_name: Some(fk_pascal_case_name.clone()),
                snake_case_name: Some(fk_snake_case_name.clone()),
                macro_case_name: Some(fk_macro_case_name.clone()),
                ..Default::default()
            };
            ref_attr_info.inner_info = Some(Box::new(fk_attribute_info));
        }

        let out_entity = self
            .coll_cache
            .all_entity_list
            .iter()
            .find(|enti| associate_entity.id_up == Some(enti.id_entity.clone()))
            .ok_or(TcdtServiceError::build_custom_msg(&format!(
                "entity_associate: '{}' can not get up_entity",
                associate_entity.id_entity_associate
            )))?
            .clone();

        let out_attribute_name_option = associate_entity.down_attribute_name.clone();
        if let Some(out_attribute_name) = out_attribute_name_option {
            let out_camel_case_name = out_attribute_name;
            let out_pascal_case_name = camel_case_to_pascal_case(&out_camel_case_name);
            let out_snake_case_name = pascal_case_to_snake_case(&out_pascal_case_name);
            let out_macro_case_name = snake_case_to_macro_case(&out_snake_case_name);
            let out_attribute_info = AttributeInfoPO {
                attribute_name: Some(out_camel_case_name.clone()),
                display_name: associate_entity.down_attribute_display_name.clone(),
                note: associate_entity.down_attribute_display_name.clone(),
                object_type: out_entity.class_name.clone(),
                camel_case_name: Some(out_camel_case_name.clone()),
                pascal_case_name: Some(out_pascal_case_name.clone()),
                snake_case_name: Some(out_snake_case_name.clone()),
                macro_case_name: Some(out_macro_case_name.clone()),
                ..Default::default()
            };
            ref_attr_info.outer_info = Some(Box::new(out_attribute_info));
        }

        let out_entity_attrs = self
            .coll_cache
            .attr_list
            .clone()
            .into_iter()
            .filter(|attr| attr.id_entity == Some(out_entity.id_entity.clone()))
            .collect::<Vec<_>>();

        let out_coll = self
            .coll_cache
            .coll_list
            .iter()
            .find(|coll| out_entity.id_entity_collection == Some(coll.id_entity_collection.clone()))
            .ok_or(TcdtServiceError::build_internal_msg("out_collection empty"))?
            .clone();

        let out_entity_info = self.make_base_entity_info(
            &out_coll,
            &out_entity,
            &out_entity_attrs,
            &vec![],
            &vec![],
            true,
        )?;
        ref_attr_info.object_type = out_entity_info.class_name.clone();
        let out_object_type_package =
            format!("{}", out_entity_info.base_path.clone().unwrap_or_default());
        ref_attr_info.object_type_package = Some(out_object_type_package);
        ref_attr_info.out_entity_info = Some(Box::new(out_entity_info.clone()));

        let inner_info_option = ref_attr_info.inner_info.clone();
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

    fn build_array_attribute(
        self: &Self,
        associate_entity: &entity_associate::Model,
        up_entity: &dd_entity::Model,
        up_entity_attrs: &Vec<entity_attribute::Model>,
    ) -> Result<Option<AttributeInfoPO>, TcdtServiceError> {
        let down_attribute_name = associate_entity
            .down_attribute_name
            .clone()
            .unwrap_or_default();
        if down_attribute_name.trim() == String::default() {
            log::warn!(
                "entity_associate: '{}' down attribute name is empty",
                associate_entity.id_entity_associate
            );
            return Ok(None);
        }
        let down_camel_case_name = down_attribute_name;
        let down_pascal_case_name = camel_case_to_pascal_case(&down_camel_case_name);
        let down_snake_case_name = pascal_case_to_snake_case(&down_pascal_case_name);
        let down_macro_case_name = snake_case_to_macro_case(&down_snake_case_name);
        let down_associate_type = associate_entity
            .down_associate_type
            .clone()
            .unwrap_or(DOWN_TYPE_ZERO_TO_MANY.to_string());
        let ref_domain_type_code: Option<String>;
        if down_associate_type == DOWN_TYPE_ONE_TO_ONE
            || down_associate_type == DOWN_TYPE_ZERO_TO_ONE
        {
            ref_domain_type_code = Some(String::from(INTERNAL_SINGLE));
        } else {
            ref_domain_type_code = Some(String::from(INTERNAL_ARRAY));
        }
        let mut attr_info = AttributeInfoPO {
            attribute_name: associate_entity.down_attribute_name.clone(),
            display_name: associate_entity.down_attribute_display_name.clone(),
            note: associate_entity.down_attribute_display_name.clone(),
            attribute_type_code: ref_domain_type_code,
            object_type: up_entity.class_name.clone(),
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

        let out_entity = self
            .coll_cache
            .all_entity_list
            .iter()
            .find(|enti| associate_entity.id_down == Some(enti.id_entity.clone()))
            .ok_or(TcdtServiceError::build_custom_msg(&format!(
                "build_array_attribute: entity_associate: '{}' can not get down_entity",
                associate_entity.id_entity_associate
            )))?
            .clone();

        let out_ref_attribute_name_option = associate_entity.ref_attribute_name.clone();
        if let Some(out_ref_attribute_name) = out_ref_attribute_name_option {
            let out_camel_case_name = out_ref_attribute_name;
            let out_pascal_case_name = camel_case_to_pascal_case(&out_camel_case_name);
            let out_snake_case_name = pascal_case_to_snake_case(&out_pascal_case_name);
            let out_macro_case_name = snake_case_to_macro_case(&out_snake_case_name);
            let out_attribute_info = AttributeInfoPO {
                attribute_name: Some(out_camel_case_name.clone()),
                display_name: associate_entity.ref_attribute_display_name.clone(),
                note: associate_entity.ref_attribute_display_name.clone(),
                object_type: out_entity.class_name.clone(),
                camel_case_name: Some(out_camel_case_name.clone()),
                pascal_case_name: Some(out_pascal_case_name.clone()),
                snake_case_name: Some(out_snake_case_name.clone()),
                macro_case_name: Some(out_macro_case_name.clone()),
                ..Default::default()
            };
            attr_info.outer_info = Some(Box::new(out_attribute_info));
        }

        let out_entity_attrs = self
            .coll_cache
            .attr_list
            .clone()
            .into_iter()
            .filter(|attr| attr.id_entity == Some(out_entity.id_entity.clone()))
            .collect::<Vec<_>>();

        let out_coll = self
            .coll_cache
            .coll_list
            .iter()
            .find(|coll| {
                Some(coll.id_entity_collection.clone()) == out_entity.id_entity_collection.clone()
            })
            .ok_or(TcdtServiceError::build_internal_msg("out_collection empty"))?
            .clone();

        let out_entity_info = self.make_base_entity_info(
            &out_coll,
            &out_entity,
            &out_entity_attrs,
            &vec![],
            &vec![],
            true,
        )?;

        attr_info.object_type = out_entity_info.class_name.clone();
        let out_object_type_package =
            format!("{}", out_entity_info.base_path.clone().unwrap_or_default());
        attr_info.object_type_package = Some(out_object_type_package);
        attr_info.out_entity_info = Some(Box::new(out_entity_info));

        let out_fk_attribute_name = associate_entity.fk_attribute_name.clone();
        if let Some(out_fk_attribute_name) = out_fk_attribute_name {
            let out_fk_camel_case_name = out_fk_attribute_name;
            let out_fk_pascal_case_name = camel_case_to_pascal_case(&out_fk_camel_case_name);
            let out_fk_snake_case_name = pascal_case_to_snake_case(&out_fk_pascal_case_name);
            let out_fk_macro_case_name = snake_case_to_macro_case(&out_fk_snake_case_name);
            let out_fk_attribute_info = AttributeInfoPO {
                fg_foreign_key: associate_entity.fg_foreign_key.clone(),
                column_name: associate_entity.fk_column_name.clone(),
                attribute_name: Some(out_fk_camel_case_name.clone()),
                display_name: associate_entity.fk_attribute_display_name.clone(),
                note: associate_entity.fk_attribute_display_name.clone(),
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

    fn make_base_entity_info(
        self: &Self,
        coll_enity: &entity_collection::Model,
        dd_entity_entity: &dd_entity::Model,
        entity_attrs: &Vec<entity_attribute::Model>,
        enum_associate_list: &Vec<enum_associate::Model>,
        all_enum_info_list: &Vec<EnumInfoPO>,
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
            make_attributes_from_entity(entity_attrs, enum_associate_list, all_enum_info_list)?;
        if fg_out_entity {
            entity_attr_info_list = self.build_attribute_type_info(entity_attr_info_list)?;
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

        let base_path = coll_enity.package_name.clone();

        let entity_info = EntityInfoPO {
            id_entity: dd_entity_entity.id_entity.clone(),
            display_name: dd_entity_entity.display_name.clone(),
            class_name: dd_entity_entity.class_name.clone(),
            table_name: dd_entity_entity.table_name.clone(),
            pk_attribute_code: dd_entity_entity.pk_attribute_code.clone(),
            pk_attribute_name: dd_entity_entity.pk_attribute_name.clone(),
            pk_attribute_type_name: dd_entity_entity.pk_attribute_type_name.clone(),
            base_path: base_path,
            camel_case_name: Some(camel_case_name),
            pascal_case_name: Some(pascal_case_name),
            snake_case_name: Some(snake_case_name),
            macro_case_name: Some(macro_case_name),
            attribute_info_list: entity_attr_info_list,
            base_attribute_info_list: base_attr_info_list,
            pk_attribute_info: Some(pk_attr_info.clone()),
            ..Default::default()
        };
        Ok(entity_info)
    }

    fn build_desc_info(
        self: &Self,
        entity_info: EntityInfoPO,
    ) -> Result<EntityInfoPO, TcdtServiceError> {
        let all_attr_info_list = entity_info.attribute_info_list.clone();
        let all_attr_info_list = self.build_attribute_type_info(all_attr_info_list)?;
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
        let base_attribute_info_list = all_attr_info_list
            .clone()
            .into_iter()
            .filter(|attr_info| {
                if attr_info.attribute_type_code == Some(INTERNAL_PK.to_string())
                    || attr_info.attribute_type_code == Some(INTERNAL_FK.to_string())
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
            param_json: entity_info.param_json.clone(),
            display_name: entity_info.display_name.clone(),
            class_name: entity_info.class_name.clone(),
            table_name: entity_info.table_name.clone(),
            pk_attribute_code: entity_info.pk_attribute_code.clone(),
            pk_attribute_name: entity_info.pk_attribute_name.clone(),
            pk_attribute_type_name: entity_info.pk_attribute_type_name.clone(),
            base_path: entity_info.base_path.clone(),
            camel_case_name: entity_info.camel_case_name.clone(),
            pascal_case_name: entity_info.pascal_case_name.clone(),
            snake_case_name: entity_info.snake_case_name.clone(),
            macro_case_name: entity_info.macro_case_name.clone(),
            out_base_package_list: entity_info.out_base_package_list.clone(),
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
        };
        Ok(result_entity_info)
    }

    fn build_attribute_type_info(
        self: &Self,
        all_attr_info_list: Vec<AttributeInfoPO>,
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
                        "attribute: {} id_attribute_type is None",
                        attr_info.id_attribute
                    )))?;
            let attribute_type_vo = self
                .coll_cache
                .column_domain_type_map
                .get(&id_attribute_type)
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "id_attribute_type: {} can not get info",
                    id_attribute_type
                )))?;
            if attr_info.len.is_none() {
                attr_info.len = attribute_type_vo.len.clone();
            }
            if attr_info.pcs.is_none() {
                attr_info.pcs = attribute_type_vo.pcs.clone();
            }
            if attr_info.column_type.is_none()
                || attr_info.column_type.clone().unwrap().trim().is_empty()
            {
                attr_info.column_type = attribute_type_vo.column_type.clone();
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
fn make_enum_info_list(
    enums: Vec<dd_enum::Model>,
    all_enum_attr_list: Vec<enum_attribute::Model>,
) -> Result<Vec<EnumInfoPO>, TcdtServiceError> {
    let mut enum_info_list: Vec<EnumInfoPO> = vec![];
    for enum_entity in enums {
        let enum_attr_list = all_enum_attr_list
            .iter()
            .filter(|enum_attr| enum_attr.id_enum == Some(enum_entity.id_enum.clone()))
            .collect::<Vec<&enum_attribute::Model>>();
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
            return Err(TcdtServiceError::build_custom_msg("class name is empty"));
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
        enum_info_list.push(enum_info);
    }
    Ok(enum_info_list)
}
fn make_attributes_from_entity(
    attrs: &Vec<entity_attribute::Model>,
    enum_associate_list: &Vec<enum_associate::Model>,
    all_enum_info_list: &Vec<EnumInfoPO>,
) -> Result<Vec<AttributeInfoPO>, TcdtServiceError> {
    let mut attr_info_list: Vec<AttributeInfoPO> = vec![];
    for attr in attrs {
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
            enum_info_option = all_enum_info_list
                .clone()
                .into_iter()
                .find(|enum_info| Some(enum_info.id_enum.clone()) == enum_associate.id_enum);
        }
        let attr_info = AttributeInfoPO {
            id_attribute: attr.id_attribute.clone(),
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
