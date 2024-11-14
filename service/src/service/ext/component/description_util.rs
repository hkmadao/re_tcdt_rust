use crate::dto::vo::ext::component::description_info::{
    ComponentInfo, DescriptionInfo, EntityInfo, EnumAttributeInfo, EnumInfo,
};
use crate::service::ext::generator::genrate_const::{
    DOWN_TYPE_ONE_TO_ONE, DOWN_TYPE_ZERO_TO_MANY, DOWN_TYPE_ZERO_TO_ONE, INTERNAL_ARRAY,
    INTERNAL_REF, INTERNAL_SINGLE, INTERNAL_SINGLE_REF,
};
use ::entity::entity::{
    component, component_entity, component_entity_associate, component_module,
    computation_attribute, data_type, dd_entity, dd_enum, entity_associate, entity_attribute,
    enum_associate, enum_attribute, ext_attribute, sub_project,
};
use nanoid::nanoid;
use sea_orm::*;
use tcdt_common::name_switch_util::{
    camel_case_to_pascal_case, pascal_case_to_snake_case, snake_case_to_camel_case,
    snake_case_to_macro_case,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;

pub struct DescriptionUtil<'a> {
    db: &'a DbConn,
    component_entity_entity: component_entity::Model,
    ext_attribute_list: Vec<ext_attribute::Model>,
    computation_attribute_list: Vec<computation_attribute::Model>,
    comp_entity: component::Model,
    comp_module_entity: component_module::Model,
    sub_project_entity: sub_project::Model,
    entities: Vec<dd_entity::Model>,
    attributes: Vec<entity_attribute::Model>,
    enum_list: Vec<dd_enum::Model>,
    enum_attribute_list: Vec<enum_attribute::Model>,
    component_entity_associate_list: Vec<component_entity_associate::Model>,
    entity_associate_list: Vec<entity_associate::Model>,
    enum_associate_list: Vec<enum_associate::Model>,
    attribute_type_list: Vec<data_type::Model>,
}

impl<'a> DescriptionUtil<'a> {
    pub async fn build_description_info(
        &self,
        prefix_attr: Option<String>,
    ) -> Result<DescriptionInfo, TcdtServiceError> {
        let component_entity_entity = self.component_entity_entity.clone();
        let entity_entity = self
            .entities
            .iter()
            .find(|enti| Some(enti.id_entity.clone()) == component_entity_entity.id_entity)
            .ok_or(TcdtServiceError::build_internal_msg(&format!(
                "component_entity: {} can not find dd_entity error",
                component_entity_entity.id_component_entity
            )))?;

        let class_name =
            entity_entity
                .class_name
                .clone()
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "entity: {} class_name is empty",
                    entity_entity.id_entity
                )))?;

        let pascal_case_name = class_name;
        let snake_case_name = pascal_case_to_snake_case(&pascal_case_name);
        let macro_case_name = snake_case_to_macro_case(&snake_case_name);
        let camel_case_name = snake_case_to_camel_case(&snake_case_name);
        let base_attr_list = self
            .get_description_info_list_from_attr_list(
                &self.component_entity_entity.clone(),
                &self.ext_attribute_list.clone(),
                prefix_attr.clone(),
            )
            .await?;

        let up_cp_associate_list = self
            .component_entity_associate_list
            .iter()
            .filter(|cp_asso| {
                self.entity_associate_list
                    .iter()
                    .find(|asso| {
                        Some(asso.id_entity_associate.clone()) == cp_asso.id_entity_associate
                            && asso.id_down == Some(entity_entity.id_entity.clone())
                    })
                    .is_some()
            })
            .collect::<Vec<_>>();

        let up_description_info_list = self
            .build_ref_description_info_list(up_cp_associate_list, prefix_attr.clone())
            .await?;

        let down_cp_associate_list = self
            .component_entity_associate_list
            .iter()
            .filter(|cp_asso| {
                self.entity_associate_list
                    .iter()
                    .find(|asso| {
                        Some(asso.id_entity_associate.clone()) == cp_asso.id_entity_associate
                            && asso.id_up == Some(entity_entity.id_entity.clone())
                    })
                    .is_some()
            })
            .collect::<Vec<_>>();

        let down_description_info_list = self
            .build_down_description_info_list(down_cp_associate_list, prefix_attr.clone())
            .await?;

        let all_attr_list = base_attr_list
            .iter()
            .chain(up_description_info_list.iter())
            .chain(down_description_info_list.iter())
            .map(|attr| attr.clone())
            .collect::<Vec<_>>();

        let entity_info = self
            .get_entity_info_from_entity(
                &self.component_entity_entity.clone(),
                &self.ext_attribute_list.clone(),
            )
            .await?;

        let description_info = DescriptionInfo {
            id: nanoid!(),
            attribute_name: Some(camel_case_name.clone()),
            display_name: entity_entity.display_name.clone(),
            camel_case_name: Some(camel_case_name.clone()),
            pascal_case_name: Some(pascal_case_name.clone()),
            snake_case_name: Some(snake_case_name.clone()),
            macro_case_name: Some(macro_case_name.clone()),
            fg_computation: None,
            fg_partner: None,
            // ref_type: None,
            web_input_type: None,
            type_script_type: None,
            full_attribute_name: Some(camel_case_name.clone()),
            id_attribute_type: None,
            attribute_type_code: None,
            attribute_type_display_name: None,
            object_type_package: None,
            object_type: entity_info.class_name.clone(),
            entity_info: Some(entity_info),
            enum_info: None,
            outer_fk_info: None,
            outer_info: None,
            inner_info: None,
            children: all_attr_list,
            ..Default::default()
        };

        Ok(description_info)
    }

    async fn build_ref_description_info_list(
        &self,
        up_cp_associate_list: Vec<&component_entity_associate::Model>,
        prefix_attr: Option<String>,
    ) -> Result<Vec<DescriptionInfo>, TcdtServiceError> {
        let mut up_description_info_list = vec![];
        for up_cp_associate in up_cp_associate_list {
            let up_associate = self
                .entity_associate_list
                .iter()
                .find(|asso| {
                    Some(asso.id_entity_associate.clone()) == up_cp_associate.id_entity_associate
                })
                .unwrap();
            let up_entity = self
                .entities
                .iter()
                .find(|entity| Some(entity.id_entity.clone()) == up_associate.id_up)
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "up_associate: {} up_entity empty",
                    up_associate.id_entity_associate,
                )))?;
            let up_component_entity_list = up_entity
                .find_linked(dd_entity::ComponentEntitiesLinked)
                .filter(component_entity::Column::FgVirtual.eq(false))
                .all(self.db)
                .await
                .map_err(|err| {
                    log::error!(
                        "up_entity: {} find up_component_entity error: {}",
                        up_entity.id_entity,
                        err
                    );
                    TcdtServiceError::build_internal_msg_error(
                        &format!(
                            "up_entity: {} find up_component_entity error",
                            up_entity.id_entity,
                        ),
                        err,
                    )
                })?;
            if up_component_entity_list.len() > 1 {
                log::warn!(
                    "up_entity: {} component_entity more than one",
                    up_entity.id_entity,
                );
                continue;
            }
            if up_component_entity_list.len() == 0 {
                log::warn!(
                    "up_entity: {} no component_entity find",
                    up_entity.id_entity,
                );
                continue;
            }
            let up_component_entity = up_component_entity_list.first().unwrap().clone();
            let up_ext_attribute = up_component_entity
                .find_linked(component_entity::ExtAttributesLinked)
                .all(self.db)
                .await
                .map_err(|err| {
                    log::error!(
                        "up_component_entity: {} find up_component error: {}",
                        up_entity.id_entity,
                        err
                    );
                    TcdtServiceError::build_internal_msg_error(
                        &format!(
                            "up_component_entity: {} find up_component error",
                            up_entity.id_entity,
                        ),
                        err,
                    )
                })?;
            if up_associate.ref_attribute_name.is_none()
                || up_associate
                    .ref_attribute_name
                    .clone()
                    .unwrap()
                    .trim()
                    .is_empty()
            {
                log::warn!(
                    "associate: {} ref_attribute_name is empty",
                    up_associate.id_entity_associate,
                );
                continue;
            }
            let attribute_name = up_associate.ref_attribute_name.clone().unwrap();
            let camel_case_name = attribute_name.clone();
            let pascal_case_name = camel_case_to_pascal_case(&attribute_name);
            let snake_case_name = pascal_case_to_snake_case(&attribute_name);
            let macro_case_name = snake_case_to_macro_case(&attribute_name);
            let entity_info = self
                .get_entity_info_from_entity(&up_component_entity, &up_ext_attribute)
                .await?;
            let prefix_attr_option: Option<String>;
            if let Some(prefix_attr) = prefix_attr.clone() {
                prefix_attr_option = Some(format!("{}.{}", prefix_attr, attribute_name));
            } else {
                prefix_attr_option = Some(attribute_name.clone());
            }
            let children = self
                .get_description_info_list_from_attr_list(
                    &up_component_entity,
                    &up_ext_attribute,
                    prefix_attr_option,
                )
                .await?;
            let mut fk_description_info_option: Option<Box<DescriptionInfo>> = None;
            if up_associate.fk_attribute_name.is_some()
                && !up_associate
                    .fk_attribute_name
                    .clone()
                    .unwrap()
                    .trim()
                    .is_empty()
            {
                let fk_attribute_name = up_associate.fk_attribute_name.clone().unwrap();
                let fk_camel_case_name = fk_attribute_name.clone();
                let fk_pascal_case_name = camel_case_to_pascal_case(&fk_attribute_name);
                let fk_snake_case_name = pascal_case_to_snake_case(&fk_attribute_name);
                let fk_macro_case_name = snake_case_to_macro_case(&fk_attribute_name);
                let fk_description_info = DescriptionInfo {
                    id: nanoid!(),
                    attribute_name: Some(fk_camel_case_name.clone()),
                    display_name: up_associate.fk_attribute_display_name.clone(),
                    camel_case_name: Some(fk_camel_case_name.clone()),
                    pascal_case_name: Some(fk_pascal_case_name.clone()),
                    snake_case_name: Some(fk_snake_case_name.clone()),
                    macro_case_name: Some(fk_macro_case_name.clone()),
                    ..Default::default()
                };
                fk_description_info_option = Some(Box::new(fk_description_info));
            }
            let mut down_description_info_option: Option<Box<DescriptionInfo>> = None;
            if up_associate.down_attribute_name.is_some()
                && !up_associate
                    .down_attribute_name
                    .clone()
                    .unwrap()
                    .trim()
                    .is_empty()
            {
                let down_attribute_name = up_associate.down_attribute_name.clone().unwrap();
                let down_camel_case_name = down_attribute_name.clone();
                let down_pascal_case_name = camel_case_to_pascal_case(&down_attribute_name);
                let down_snake_case_name = pascal_case_to_snake_case(&down_attribute_name);
                let down_macro_case_name = snake_case_to_macro_case(&down_attribute_name);
                let down_info = DescriptionInfo {
                    id: nanoid!(),
                    attribute_name: Some(down_camel_case_name.clone()),
                    display_name: up_associate.fk_attribute_display_name.clone(),
                    camel_case_name: Some(down_camel_case_name.clone()),
                    pascal_case_name: Some(down_pascal_case_name.clone()),
                    snake_case_name: Some(down_snake_case_name.clone()),
                    macro_case_name: Some(down_macro_case_name.clone()),
                    ..Default::default()
                };
                down_description_info_option = Some(Box::new(down_info));
            }
            let down_associate_type = up_associate
                .down_associate_type
                .clone()
                .unwrap_or(DOWN_TYPE_ZERO_TO_MANY.to_string());
            let ref_domain_type_code: Option<String>;
            if down_associate_type == DOWN_TYPE_ONE_TO_ONE
                || down_associate_type == DOWN_TYPE_ZERO_TO_ONE
            {
                ref_domain_type_code = Some(String::from(INTERNAL_SINGLE_REF));
            } else {
                ref_domain_type_code = Some(String::from(INTERNAL_REF));
            }
            let description_info = DescriptionInfo {
                id: nanoid!(),
                attribute_name: Some(camel_case_name.clone()),
                display_name: up_associate.ref_attribute_display_name.clone(),
                camel_case_name: Some(camel_case_name.clone()),
                pascal_case_name: Some(pascal_case_name.clone()),
                snake_case_name: Some(snake_case_name.clone()),
                macro_case_name: Some(macro_case_name.clone()),
                attribute_type_code: ref_domain_type_code,
                fg_computation: None,
                fg_partner: None,
                // ref_type: None,
                web_input_type: None,
                type_script_type: None,
                full_attribute_name: Some(camel_case_name.clone()),
                id_attribute_type: None,
                // attribute_type_code: None,
                attribute_type_display_name: None,
                object_type_package: None,
                object_type: entity_info.class_name.clone(),
                entity_info: Some(entity_info),
                enum_info: None,
                outer_fk_info: None,
                outer_info: down_description_info_option,
                inner_info: fk_description_info_option,
                children,
                ..Default::default()
            };

            up_description_info_list.push(description_info);
        }
        Ok(up_description_info_list)
    }

    async fn build_down_description_info_list(
        &self,
        down_cp_associate_list: Vec<&component_entity_associate::Model>,
        prefix_attr: Option<String>,
    ) -> Result<Vec<DescriptionInfo>, TcdtServiceError> {
        let mut down_description_info_list = vec![];
        for down_cp_associate in down_cp_associate_list {
            let down_associate = self
                .entity_associate_list
                .iter()
                .find(|asso| {
                    Some(asso.id_entity_associate.clone()) == down_cp_associate.id_entity_associate
                })
                .unwrap();
            let down_entity = self
                .entities
                .iter()
                .find(|entity| Some(entity.id_entity.clone()) == down_associate.id_down)
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "down_associate: {} down_entity empty",
                    down_associate.id_entity_associate,
                )))?;
            let down_component_entity_list = down_entity
                .find_linked(dd_entity::ComponentEntitiesLinked)
                .filter(component_entity::Column::FgVirtual.eq(false))
                .all(self.db)
                .await
                .map_err(|err| {
                    log::error!(
                        "down_entity: {} find down_component_entity error: {}",
                        down_entity.id_entity,
                        err
                    );
                    TcdtServiceError::build_internal_msg_error(
                        &format!(
                            "down_entity: {} find down_component_entity error",
                            down_entity.id_entity,
                        ),
                        err,
                    )
                })?;
            if down_component_entity_list.len() > 1 {
                log::warn!(
                    "down_entity: {} component_entity more than one",
                    down_entity.id_entity,
                );
                continue;
            }
            if down_component_entity_list.len() == 0 {
                log::warn!(
                    "down_entity: {} no component_entity find",
                    down_entity.id_entity,
                );
                continue;
            }
            let down_component_entity = down_component_entity_list.first().unwrap().clone();
            let down_ext_attribute = down_component_entity
                .find_linked(component_entity::ExtAttributesLinked)
                .all(self.db)
                .await
                .map_err(|err| {
                    log::error!(
                        "down_component_entity: {} find down_component error: {}",
                        down_entity.id_entity,
                        err
                    );
                    TcdtServiceError::build_internal_msg_error(
                        &format!(
                            "down_component_entity: {} find down_component error",
                            down_entity.id_entity,
                        ),
                        err,
                    )
                })?;
            if down_associate.down_attribute_name.is_none()
                || down_associate
                    .down_attribute_name
                    .clone()
                    .unwrap()
                    .trim()
                    .is_empty()
            {
                log::warn!(
                    "associate: {} down_attribute_name is empty",
                    down_associate.id_entity_associate,
                );
                continue;
            }
            let attribute_name = down_associate.down_attribute_name.clone().unwrap();
            let camel_case_name = attribute_name.clone();
            let pascal_case_name = camel_case_to_pascal_case(&attribute_name);
            let snake_case_name = pascal_case_to_snake_case(&attribute_name);
            let macro_case_name = snake_case_to_macro_case(&attribute_name);
            let entity_info = self
                .get_entity_info_from_entity(&down_component_entity, &down_ext_attribute)
                .await?;
            let prefix_attr_option: Option<String>;
            if let Some(prefix_attr) = prefix_attr.clone() {
                prefix_attr_option = Some(format!("{}.{}", prefix_attr, attribute_name));
            } else {
                prefix_attr_option = Some(attribute_name.clone());
            }
            let children = self
                .get_description_info_list_from_attr_list(
                    &down_component_entity,
                    &down_ext_attribute,
                    prefix_attr_option,
                )
                .await?;
            let mut fk_description_info_option: Option<Box<DescriptionInfo>> = None;
            if down_associate.fk_attribute_name.is_some()
                && !down_associate
                    .fk_attribute_name
                    .clone()
                    .unwrap()
                    .trim()
                    .is_empty()
            {
                let fk_attribute_name = down_associate.fk_attribute_name.clone().unwrap();
                let fk_camel_case_name = fk_attribute_name.clone();
                let fk_pascal_case_name = camel_case_to_pascal_case(&fk_attribute_name);
                let fk_snake_case_name = pascal_case_to_snake_case(&fk_attribute_name);
                let fk_macro_case_name = snake_case_to_macro_case(&fk_attribute_name);
                let fk_description_info = DescriptionInfo {
                    id: nanoid!(),
                    attribute_name: Some(fk_camel_case_name.clone()),
                    display_name: down_associate.fk_attribute_display_name.clone(),
                    camel_case_name: Some(fk_camel_case_name.clone()),
                    pascal_case_name: Some(fk_pascal_case_name.clone()),
                    snake_case_name: Some(fk_snake_case_name.clone()),
                    macro_case_name: Some(fk_macro_case_name.clone()),
                    ..Default::default()
                };
                fk_description_info_option = Some(Box::new(fk_description_info));
            }
            let mut out_ref_description_info_option: Option<Box<DescriptionInfo>> = None;
            if down_associate.ref_attribute_name.is_some()
                && !down_associate
                    .ref_attribute_name
                    .clone()
                    .unwrap()
                    .trim()
                    .is_empty()
            {
                let out_ref_attribute_name = down_associate.ref_attribute_name.clone().unwrap();
                let out_ref_camel_case_name = out_ref_attribute_name.clone();
                let out_ref_pascal_case_name = camel_case_to_pascal_case(&out_ref_attribute_name);
                let out_ref_snake_case_name = pascal_case_to_snake_case(&out_ref_attribute_name);
                let out_ref_macro_case_name = snake_case_to_macro_case(&out_ref_attribute_name);
                let out_ref_info = DescriptionInfo {
                    id: nanoid!(),
                    attribute_name: Some(out_ref_camel_case_name.clone()),
                    display_name: down_associate.ref_attribute_name.clone(),
                    camel_case_name: Some(out_ref_camel_case_name.clone()),
                    pascal_case_name: Some(out_ref_pascal_case_name.clone()),
                    snake_case_name: Some(out_ref_snake_case_name.clone()),
                    macro_case_name: Some(out_ref_macro_case_name.clone()),
                    ..Default::default()
                };
                out_ref_description_info_option = Some(Box::new(out_ref_info));
            }
            let down_associate_type = down_associate
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
            let description_info = DescriptionInfo {
                id: nanoid!(),
                attribute_name: Some(camel_case_name.clone()),
                display_name: down_associate.ref_attribute_display_name.clone(),
                camel_case_name: Some(camel_case_name.clone()),
                pascal_case_name: Some(pascal_case_name.clone()),
                snake_case_name: Some(snake_case_name.clone()),
                macro_case_name: Some(macro_case_name.clone()),
                attribute_type_code: ref_domain_type_code,
                fg_computation: None,
                fg_partner: None,
                // ref_type: None,
                web_input_type: None,
                type_script_type: None,
                full_attribute_name: Some(camel_case_name.clone()),
                id_attribute_type: None,
                // attribute_type_code: None,
                attribute_type_display_name: None,
                object_type_package: None,
                object_type: entity_info.class_name.clone(),
                entity_info: Some(entity_info),
                enum_info: None,
                outer_fk_info: fk_description_info_option,
                outer_info: out_ref_description_info_option,
                inner_info: None,
                children,
                ..Default::default()
            };

            down_description_info_list.push(description_info);
        }
        Ok(down_description_info_list)
    }

    async fn get_entity_info_from_entity(
        &self,
        component_entity_entity: &component_entity::Model,
        ext_attribute_list: &Vec<ext_attribute::Model>,
    ) -> Result<EntityInfo, TcdtServiceError> {
        let comp_entity = component_entity_entity
            .find_linked(component_entity::ComponentLinked)
            .one(self.db)
            .await
            .map_err(|err| {
                log::error!(
                    "component_entity: {} find component error: {}",
                    component_entity_entity.id_component_entity,
                    err
                );
                TcdtServiceError::build_internal_msg_error(
                    &format!(
                        "component_entity: {} find component error",
                        component_entity_entity.id_component_entity
                    ),
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!(
                "component_entity: {} component empty",
                component_entity_entity.id_component_entity
            )))?;
        let comp_module = comp_entity
            .find_linked(component::ComponentModuleLinked)
            .one(self.db)
            .await
            .map_err(|err| {
                log::error!(
                    "component: {} find component_module error: {}",
                    comp_entity.id_component,
                    err
                );
                TcdtServiceError::build_internal_msg_error(
                    &format!(
                        "component: {} find component_module error",
                        comp_entity.id_component
                    ),
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!(
                "component: {} component_module empty",
                comp_entity.id_component
            )))?;
        let entity_entity = self
            .entities
            .iter()
            .find(|enti| Some(enti.id_entity.clone()) == component_entity_entity.id_entity)
            .ok_or(TcdtServiceError::build_internal_msg(&format!(
                "component_entity: {} can not find dd_entity error",
                component_entity_entity.id_component_entity
            )))?;

        let class_name =
            entity_entity
                .class_name
                .clone()
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "entity: {} class_name is empty",
                    entity_entity.id_entity
                )))?;

        let pascal_case_name = class_name;
        let snake_case_name = pascal_case_to_snake_case(&pascal_case_name);
        let macro_case_name = snake_case_to_macro_case(&snake_case_name);
        let camel_case_name = snake_case_to_camel_case(&snake_case_name);

        let attributes = self
            .get_description_info_list_from_attr_list(
                component_entity_entity,
                ext_attribute_list,
                None,
            )
            .await?;
        let pk_attribute_info = attributes
            .iter()
            .find(|attr| attr.fg_primary_key.is_some() && attr.fg_primary_key.unwrap())
            .ok_or(TcdtServiceError::build_internal_msg(&format!(
                "entity: {} pk not set",
                pascal_case_name.clone()
            )))?
            .clone();

        let component_info = ComponentInfo {
            id_component: comp_entity.id_component.clone(),
            base_path: comp_module.path.clone(),
            package_name: comp_entity.package_name.clone(),
            display_name: comp_entity.display_name.clone(),
        };
        let entity_info = EntityInfo {
            id_component_entity: component_entity_entity.id_component_entity.clone(),
            id_entity: component_entity_entity.id_entity.clone(),
            display_name: entity_entity.display_name.clone(),
            class_name: entity_entity.class_name.clone(),
            table_name: entity_entity.table_name.clone(),
            pk_attribute_code: entity_entity.pk_attribute_code.clone(),
            pk_attribute_name: entity_entity.pk_attribute_name.clone(),
            pk_attribute_type_name: entity_entity.pk_attribute_type_name.clone(),
            camel_case_name: Some(camel_case_name),
            pascal_case_name: Some(pascal_case_name),
            snake_case_name: Some(snake_case_name),
            macro_case_name: Some(macro_case_name),
            base_path: comp_module.path.clone(),
            package_name: comp_entity.package_name.clone(),
            component: Some(component_info),
            attributes,
            pk_attribute_info: Some(Box::new(pk_attribute_info)),
        };

        Ok(entity_info)
    }

    async fn get_description_info_list_from_attr_list(
        &self,
        component_entity_entity: &component_entity::Model,
        ext_attribute_list: &Vec<ext_attribute::Model>,
        prefix_attr: Option<String>,
    ) -> Result<Vec<DescriptionInfo>, TcdtServiceError> {
        let mut description_info_list: Vec<DescriptionInfo> = vec![];
        for ext_attribute_entity in ext_attribute_list {
            let id_attribute = ext_attribute_entity.id_attribute.clone().ok_or(
                TcdtServiceError::build_internal_msg(&format!(
                    "ext_attribute: {} can not find id_attribute error",
                    ext_attribute_entity.id_ext_attribute
                )),
            )?;
            let entity_attr = self
                .attributes
                .iter()
                .find(|attr| attr.id_attribute == id_attribute)
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "ext_attribute: {} can not find id_attribute error",
                    ext_attribute_entity.id_ext_attribute
                )))?;
            let attribute_name =
                entity_attr
                    .attribute_name
                    .clone()
                    .ok_or(TcdtServiceError::build_internal_msg(&format!(
                        "ext_attribute: {} attribute_name is not set",
                        ext_attribute_entity.id_ext_attribute
                    )))?;
            let pascal_case_name = camel_case_to_pascal_case(&attribute_name);
            let snake_case_name = pascal_case_to_snake_case(&attribute_name);
            let macro_case_name = snake_case_to_macro_case(&attribute_name);
            let mut full_attribute_name = attribute_name.clone();
            if let Some(prefix_attr) = prefix_attr.clone() {
                full_attribute_name = format!("{}.{}", prefix_attr, attribute_name)
            }
            let enum_info = self.get_enum_info(&entity_attr.id_attribute)?;
            let description_info = DescriptionInfo {
                id: nanoid!(),
                attribute_name: entity_attr.attribute_name.clone(),
                display_name: entity_attr.display_name.clone(),
                column_name: entity_attr.column_name.clone(),
                fg_primary_key: entity_attr.fg_primary_key.clone(),
                fg_mandatory: entity_attr.fg_mandatory.clone(),
                default_value: entity_attr.default_value.clone(),
                len: entity_attr.len.clone(),
                pcs: entity_attr.pcs.clone(),
                sn: entity_attr.sn.clone(),
                note: entity_attr.note.clone(),
                category: entity_attr.category.clone(),
                id_attribute_type: entity_attr.id_attribute_type.clone(),
                camel_case_name: Some(attribute_name.clone()),
                pascal_case_name: Some(pascal_case_name.clone()),
                snake_case_name: Some(snake_case_name.clone()),
                macro_case_name: Some(macro_case_name.clone()),
                full_attribute_name: Some(full_attribute_name),
                enum_info,
                ..Default::default()
            };
            description_info_list.push(description_info);
        }

        let description_info_list = self.build_attribute_type_info(description_info_list)?;
        Ok(description_info_list)
    }

    fn get_description_info_list_from_computation_attr_list(
        &self,
        component_entity_entity: &component_entity::Model,
        computation_attribute_list: &Vec<computation_attribute::Model>,
        prefix_attr: String,
    ) -> Result<Vec<DescriptionInfo>, TcdtServiceError> {
        let mut description_info_list: Vec<DescriptionInfo> = vec![];
        for computation_attribute_entity in computation_attribute_list {
            let attribute_name = computation_attribute_entity.attribute_name.clone().ok_or(
                TcdtServiceError::build_internal_msg(&format!(
                    "ext_attribute: {} attribute_name is not set",
                    computation_attribute_entity.id_computation_attribute
                )),
            )?;
            let camel_case_name = attribute_name.clone();
            let pascal_case_name = camel_case_to_pascal_case(&attribute_name);
            let snake_case_name = pascal_case_to_snake_case(&attribute_name);
            let macro_case_name = snake_case_to_macro_case(&attribute_name);
            let description_info = DescriptionInfo {
                id: nanoid!(),
                attribute_name: computation_attribute_entity.attribute_name.clone(),
                display_name: computation_attribute_entity.display_name.clone(),
                fg_computation: Some(true),
                fg_mandatory: computation_attribute_entity.fg_mandatory.clone(),
                default_value: computation_attribute_entity.default_value.clone(),
                len: computation_attribute_entity.len.clone(),
                pcs: computation_attribute_entity.pcs.clone(),
                sn: computation_attribute_entity.sn.clone(),
                id_attribute_type: computation_attribute_entity.id_attribute_type.clone(),
                camel_case_name: Some(camel_case_name.clone()),
                pascal_case_name: Some(pascal_case_name.clone()),
                snake_case_name: Some(snake_case_name.clone()),
                macro_case_name: Some(macro_case_name.clone()),
                full_attribute_name: Some(format!("{}.{}", prefix_attr, attribute_name)),
                enum_info: None,
                ..Default::default()
            };
            description_info_list.push(description_info);
        }

        Ok(description_info_list)
    }

    fn get_enum_info(&self, id_attribute: &str) -> Result<Option<EnumInfo>, TcdtServiceError> {
        let enum_associate_entity = self
            .enum_associate_list
            .iter()
            .find(|asso| asso.id_attribute == Some(id_attribute.to_string()));
        if enum_associate_entity.is_none() {
            return Ok(None);
        }
        let enum_associate_entity = enum_associate_entity.unwrap();
        let dd_enum = self
            .enum_list
            .iter()
            .find(|dd_enum| Some(dd_enum.id_enum.clone()) == enum_associate_entity.id_enum)
            .ok_or(TcdtServiceError::build_internal_msg(""))?;
        let enum_attribute_list = self
            .enum_attribute_list
            .iter()
            .filter(|enum_attribute_entity| {
                enum_attribute_entity.id_enum == enum_associate_entity.id_enum
            })
            .collect::<Vec<_>>();
        let mut attributes = vec![];
        for enum_attribute_entity in enum_attribute_list {
            let enum_attribute_info = EnumAttributeInfo {
                id_enum_attribute: enum_attribute_entity.id_enum_attribute.clone(),
                display_name: enum_attribute_entity.display_name.clone(),
                code: enum_attribute_entity.code.clone(),
                enum_value: enum_attribute_entity.enum_value.clone(),
            };
            attributes.push(enum_attribute_info);
        }
        let enum_info = EnumInfo {
            id_enum: dd_enum.id_enum.clone(),
            class_name: dd_enum.class_name.clone(),
            display_name: dd_enum.display_name.clone(),
            enum_value_type: dd_enum.enum_value_type.clone(),
            camel_case_name: None,
            pascal_case_name: None,
            snake_case_name: None,
            macro_case_name: None,
            attributes,
        };

        Ok(Some(enum_info))
    }

    fn build_attribute_type_info(
        &self,
        all_attr_info_list: Vec<DescriptionInfo>,
    ) -> Result<Vec<DescriptionInfo>, TcdtServiceError> {
        let mut all_attr_info_list_new: Vec<DescriptionInfo> = vec![];
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
                        "description_info: {} id_attribute_type is None",
                        attr_info.camel_case_name.clone().unwrap_or_default()
                    )))?;
            let attribute_type_vo = self
                .attribute_type_list
                .iter()
                .find(|attr_type| attr_type.id_data_type == id_attribute_type)
                .ok_or(TcdtServiceError::build_internal_msg(&format!(
                    "id_attribute_type: {} can not get info",
                    id_attribute_type
                )))?;
            attr_info.attribute_type_code = attribute_type_vo.code.clone();
            attr_info.attribute_type_display_name = attribute_type_vo.display_name.clone();
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
            if attr_info.web_input_type.is_none()
                || attr_info.web_input_type.clone().unwrap().trim().is_empty()
            {
                attr_info.web_input_type = attribute_type_vo.web_input_type.clone();
            }
            if attr_info.type_script_type.is_none()
                || attr_info
                    .type_script_type
                    .clone()
                    .unwrap()
                    .trim()
                    .is_empty()
            {
                attr_info.type_script_type = attribute_type_vo.type_script_type.clone();
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

    pub async fn load_data(
        db: &'a DbConn,
        id_component_entity: String,
    ) -> Result<Self, TcdtServiceError> {
        let component_entity_entity = component_entity::Entity::find_by_id(id_component_entity)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("find component_entity error: {}", err);
                TcdtServiceError::build_internal_msg_error("find component_entity error", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg(
                "component_entity is empty",
            ))?;
        let comp_entity = component_entity_entity
            .find_linked(component_entity::ComponentLinked)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("find component error: {}", err);
                TcdtServiceError::build_internal_msg_error("find component error", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("component is empty"))?;
        let comp_module_entity = comp_entity
            .find_linked(component::ComponentModuleLinked)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("find component module error: {}", err);
                TcdtServiceError::build_internal_msg_error("find component module error", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg(
                "component module is empty",
            ))?;
        let sub_project_entity = comp_module_entity
            .find_linked(component_module::SubProjectLinked)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("find sub_project error: {}", err);
                TcdtServiceError::build_internal_msg_error("find sub_project error", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("sub_project is empty"))?;
        let id_project = sub_project_entity
            .id_project
            .clone()
            .ok_or(TcdtServiceError::build_internal_msg("id_project is empty"))?;
        let attribute_type_list = data_type::Entity::find()
            .filter(data_type::Column::IdProject.eq(id_project))
            .order_by_asc(data_type::Column::Sn)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find attribute_type_list error: {}", err);
                TcdtServiceError::build_internal_msg_error("find attribute_type_list error", err)
            })?;

        let dd_entity_entity = component_entity_entity
            .find_linked(component_entity::DdEntityLinked)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("find dd_entity error: {}", err);
                TcdtServiceError::build_internal_msg_error("find dd_entity error", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("dd_entity is empty"))?;

        let ext_attribute_list = component_entity_entity
            .find_linked(component_entity::ExtAttributesLinked)
            .order_by_asc(ext_attribute::Column::Sn)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find ext_attribute error: {}", err);
                TcdtServiceError::build_internal_msg_error("find ext_attribute error", err)
            })?;

        let computation_attribute_list = component_entity_entity
            .find_linked(component_entity::ComputationAttributesLinked)
            .order_by_asc(computation_attribute::Column::Sn)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find computation_attribute error: {}", err);
                TcdtServiceError::build_internal_msg_error("find computation_attribute error", err)
            })?;
        let component_entity_associate_list = comp_entity
            .find_linked(component::ComponentEntityAssociatesLinked)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find component_entity_associates error: {}", err);
                TcdtServiceError::build_internal_msg_error(
                    "find component_entity_associates error",
                    err,
                )
            })?;
        let asso_id_list = component_entity_associate_list
            .iter()
            .map(|cp_asso| cp_asso.id_entity_associate.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        let mut entity_associate_list = vec![];
        if !asso_id_list.is_empty() {
            entity_associate_list = entity_associate::Entity::find()
                .filter(entity_associate::Column::IdEntityAssociate.is_in(asso_id_list))
                .all(db)
                .await
                .map_err(|err| {
                    log::error!("find entity_associates error: {}", err);
                    TcdtServiceError::build_internal_msg_error("find entity_associates error", err)
                })?;
        }
        let entity_associate_list = entity_associate_list;

        let mut entity_id_list = vec![component_entity_entity.id_entity.clone().ok_or(
            TcdtServiceError::build_internal_msg("component entity attribute id_entity is empty"),
        )?];
        for asso in entity_associate_list.clone() {
            let id_up = asso
                .id_up
                .clone()
                .ok_or(TcdtServiceError::build_internal_msg(
                    "entity_associate attribute id_up is empty",
                ))?;
            if !entity_id_list.contains(&id_up) {
                entity_id_list.push(id_up);
            }
            let id_down = asso
                .id_down
                .clone()
                .ok_or(TcdtServiceError::build_internal_msg(
                    "entity_associate attribute id_down is empty",
                ))?;
            if !entity_id_list.contains(&id_down) {
                entity_id_list.push(id_down);
            }
        }
        let entities = dd_entity::Entity::find()
            .filter(dd_entity::Column::IdEntity.is_in(entity_id_list.clone()))
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find entities error: {}", err);
                TcdtServiceError::build_internal_msg_error("find entities error", err)
            })?;
        let attributes = entity_attribute::Entity::find()
            .filter(entity_attribute::Column::IdEntity.is_in(entity_id_list))
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find attributes error: {}", err);
                TcdtServiceError::build_internal_msg_error("find attributes error", err)
            })?;

        let enum_associate_list = dd_entity_entity
            .find_linked(dd_entity::EnumAssociatesLinked)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find enum_associate_list error: {}", err);
                TcdtServiceError::build_internal_msg_error("find enum_associate_list error", err)
            })?;
        let enum_id_list = enum_associate_list
            .iter()
            .map(|asso| asso.id_enum.clone().unwrap())
            .collect::<Vec<_>>();
        let mut enum_list = vec![];
        let mut enum_attribute_list = vec![];
        if !enum_id_list.is_empty() {
            enum_list = dd_enum::Entity::find()
                .filter(dd_enum::Column::IdEnum.is_in(enum_id_list.clone()))
                .all(db)
                .await
                .map_err(|err| {
                    log::error!("find enum_list error: {}", err);
                    TcdtServiceError::build_internal_msg_error("find enum_list error", err)
                })?;
            enum_attribute_list = enum_attribute::Entity::find()
                .filter(enum_attribute::Column::IdEnum.is_in(enum_id_list))
                .all(db)
                .await
                .map_err(|err| {
                    log::error!("find enum_attributes error: {}", err);
                    TcdtServiceError::build_internal_msg_error("find enum_attributes error", err)
                })?;
        }
        let description_util = DescriptionUtil {
            db,
            component_entity_entity,
            ext_attribute_list,
            computation_attribute_list,
            comp_entity,
            comp_module_entity,
            sub_project_entity,
            entities,
            attributes,
            enum_list,
            enum_attribute_list,
            component_entity_associate_list,
            entity_associate_list,
            enum_associate_list,
            attribute_type_list,
        };
        Ok(description_util)
    }
}

fn ref_type_check(attr_info: &DescriptionInfo) -> bool {
    attr_info.attribute_type_code == Some(INTERNAL_REF.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_SINGLE_REF.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_ARRAY.to_string())
        || attr_info.attribute_type_code == Some(INTERNAL_SINGLE.to_string())
}
