use sea_query::{expr::SimpleExpr::Column, Expr, SimpleExpr};
use std::collections::HashMap;

use crate::common::{
    aq::*,
    aq_const::{
        LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_EQUAL, OPERATOR_CODE_LIKE, OPERATOR_CODE_GT,
        OPERATOR_CODE_GTE, OPERATOR_CODE_IN, OPERATOR_CODE_LEFT_LIKE, OPERATOR_CODE_LT,
        OPERATOR_CODE_LTE, OPERATOR_CODE_NOT_EQUAL, OPERATOR_CODE_RIGHT_LIKE, ORDER_DIRECTION_ASC,
    },
};

use ::entity::{
    common::desc::{
        AttributeInfo, EntityDesc, DATA_TYPE_AGG_REF, DATA_TYPE_AGG_SINGLE_REF, DATA_TYPE_REF,
        DATA_TYPE_SINGLE_REF,
    },
    conf::meta_init::DESC_MAP,
};
use crypto::{digest::Digest, md5::Md5};
use sea_orm::*;
use sea_query::{Alias, ColumnRef, ConditionType};
use tcdt_common::tcdt_service_error::TcdtServiceError;

pub fn md5(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.input_str(input);
    md5.result_str()
}

fn get_join_names_from_logic_nodes(
    logic_node_option: Option<Box<AqLogicNode>>,
) -> HashMap<String, u32> {
    let mut result_map = HashMap::new();
    if let Some(logic_node) = logic_node_option {
        let filter_nodes = logic_node.filter_nodes;
        for filter_node in filter_nodes {
            let mut names: Vec<&str> = filter_node.name.split('.').collect();
            if names.len() > 1 {
                names.truncate(names.len() - 1);
                result_map.entry(names.join(".")).or_insert(1);
            }
        }

        let child_result = get_join_names_from_logic_nodes(logic_node.logic_node.clone());
        result_map.extend(child_result);
    }
    result_map
}

fn get_join_names_from_orders(
    mut result_map: HashMap<String, u32>,
    orders: Vec<AqOrder>,
) -> HashMap<String, u32> {
    for order in orders {
        let mut names: Vec<&str> = order.property.split('.').collect();
        if names.len() > 1 {
            names.truncate(names.len() - 1);
            result_map.entry(names.join(".")).or_insert(1);
        }
    }
    result_map
}

fn get_desc_attr(
    attrs: Vec<&str>,
    parent_name: &str,
    index: usize,
) -> Result<AttributeInfo, TcdtServiceError> {
    let parent_desc: &EntityDesc = DESC_MAP.get(parent_name).ok_or_else(|| {
        TcdtServiceError::build_internal_msg(&format!(
            "can not get description info: {}",
            parent_name
        ))
    })?;
    let attr_info: &AttributeInfo = parent_desc
        .attribute_info_map
        .get(attrs[index])
        .ok_or_else(|| {
            TcdtServiceError::build_internal_msg(&format!(
                "{} can not get attribute info: {}",
                parent_name, attrs[index]
            ))
        })?;
    if attrs.len() - 1 == index {
        Ok(attr_info.clone())
    } else {
        get_desc_attr(attrs, &attr_info.out_entity_name, index + 1)
    }
}

fn make_join_select<T: EntityTrait>(
    sql_build: &mut Select<T>,
    attrs: Vec<String>,
    parent_name: &str,
    index: usize,
) -> Result<(), TcdtServiceError> {
    if attrs.len() < 1 {
        return Ok(());
    }
    let parent_desc: &EntityDesc = DESC_MAP.get(parent_name).ok_or_else(|| {
        TcdtServiceError::build_internal_msg(&format!(
            "can not get description info: {}",
            parent_name
        ))
    })?;
    let attr_info: &AttributeInfo = parent_desc
        .attribute_info_map
        .get(&attrs[index])
        .ok_or_else(|| {
            TcdtServiceError::build_internal_msg(&format!(
                "{} can not get attribute info: {}",
                parent_name, attrs[index]
            ))
        })?;
    let ref_desc: &EntityDesc = DESC_MAP.get(&attr_info.out_entity_name).ok_or_else(|| {
        TcdtServiceError::build_internal_msg(&format!(
            "can not get description info: {}",
            attr_info.out_entity_name
        ))
    })?;
    let entity_ref_option: Option<RelationDef>;
    if attr_info.data_type == DATA_TYPE_REF
        || attr_info.data_type == DATA_TYPE_SINGLE_REF
        || attr_info.data_type == DATA_TYPE_AGG_REF
        || attr_info.data_type == DATA_TYPE_AGG_SINGLE_REF
    {
        let fk_attr_info: &AttributeInfo = parent_desc
            .attribute_info_map
            .get(&attr_info.inner_attribute_name)
            .ok_or_else(|| {
                TcdtServiceError::build_internal_msg(&format!(
                    "entity desc: {} can not get inner attribute info: {}",
                    parent_desc.entity_info.class_name, attr_info.inner_attribute_name
                ))
            })?;
        let entity_ref: RelationDef = RelationDef {
            rel_type: RelationType::HasMany,
            from_tbl: sea_query::TableRef::Table(DynIden::new(Alias::new(
                parent_desc.entity_info.table_name.clone(),
            ))),
            to_tbl: sea_query::TableRef::Table(DynIden::new(Alias::new(
                ref_desc.entity_info.table_name.clone(),
            ))),
            from_col: Identity::Unary(DynIden::new(Alias::new(fk_attr_info.column_name.clone()))),
            to_col: Identity::Unary(DynIden::new(Alias::new(
                ref_desc.pk_attribute_info.column_name.clone(),
            ))),
            is_owner: true,
            on_delete: None,
            on_update: None,
            on_condition: None,
            fk_name: None,
            condition_type: ConditionType::All,
        };
        entity_ref_option = Some(entity_ref);
    } else {
        let out_entity_id_reversal_attr_info: &AttributeInfo = ref_desc
            .attribute_info_map
            .get(&attr_info.out_entity_id_reversal_attribute_name)
            .ok_or_else(|| {
                TcdtServiceError::build_internal_msg(&format!(
                    "entity desc: {} can not get out entity id reversal attribute info: {}",
                    ref_desc.entity_info.class_name,
                    attr_info.out_entity_id_reversal_attribute_name
                ))
            })?;
        let entity_ref: RelationDef = RelationDef {
            rel_type: RelationType::HasMany,
            from_tbl: sea_query::TableRef::Table(DynIden::new(Alias::new(
                parent_desc.entity_info.table_name.clone(),
            ))),
            to_tbl: sea_query::TableRef::Table(DynIden::new(Alias::new(
                ref_desc.entity_info.table_name.clone(),
            ))),
            from_col: Identity::Unary(DynIden::new(Alias::new(
                parent_desc.pk_attribute_info.column_name.clone(),
            ))),
            to_col: Identity::Unary(DynIden::new(Alias::new(
                out_entity_id_reversal_attr_info.column_name.clone(),
            ))),
            is_owner: true,
            on_delete: None,
            on_update: None,
            on_condition: None,
            fk_name: None,
            condition_type: ConditionType::All,
        };
        entity_ref_option = Some(entity_ref);
    }
    let alias_name = attrs[..index].join(".");
    *sql_build = sql_build.clone().join_as(
        JoinType::LeftJoin,
        entity_ref_option.unwrap(),
        DynIden::new(Alias::new(md5(&alias_name))),
    );
    let next_index = index + 1;
    if attrs.len() > next_index {
        make_join_select(sql_build, attrs, &attr_info.out_entity_name, next_index)?;
    }
    Ok(())
}

fn make_column_order_by<T: EntityTrait>(
    sql_build: &mut Select<T>,
    orders: Vec<AqOrder>,
    main_entity_name: &str,
) -> Result<(), TcdtServiceError> {
    for order in orders {
        if order.direction.to_lowercase() == ORDER_DIRECTION_ASC.to_lowercase() {
            let names: Vec<&str> = order.property.split('.').collect();
            if names.len() < 2 {
                let attr_info = get_desc_attr(names, main_entity_name, 0)?;
                *sql_build = sql_build.clone().order_by(
                    Column(ColumnRef::Column(DynIden::new(Alias::new(
                        attr_info.column_name,
                    )))),
                    Order::Asc,
                );
            } else {
                let alias_name = names[..names.len() - 2].join(".");
                let attr_info = get_desc_attr(names, main_entity_name, 0)?;
                *sql_build = sql_build.clone().order_by(
                    Column(ColumnRef::TableColumn(
                        DynIden::new(Alias::new(md5(&alias_name))),
                        DynIden::new(Alias::new(attr_info.column_name)),
                    )),
                    Order::Asc,
                );
            }
        } else {
            let names: Vec<&str> = order.property.split('.').collect();
            if names.len() < 2 {
                let attr_info = get_desc_attr(names, main_entity_name, 0)?;
                *sql_build = sql_build.clone().order_by(
                    Column(ColumnRef::Column(DynIden::new(Alias::new(
                        attr_info.column_name,
                    )))),
                    Order::Desc,
                );
            } else {
                let alias_name = names[..names.len() - 2].join(".");
                let attr_info = get_desc_attr(names, main_entity_name, 0)?;
                *sql_build = sql_build.clone().order_by(
                    Column(ColumnRef::TableColumn(
                        DynIden::new(Alias::new(md5(&alias_name))),
                        DynIden::new(Alias::new(attr_info.column_name)),
                    )),
                    Order::Desc,
                );
            }
        }
    }
    Ok(())
}

fn make_condition(
    logic_node_option: Option<Box<AqLogicNode>>,
    condition: &mut Condition,
    main_table_alias: &str,
    main_entity_name: &str,
) -> Result<(), TcdtServiceError> {
    if let Some(logic_node) = logic_node_option {
        if logic_node.filter_nodes.len() == 0 {
            return Ok(());
        }
        // parent level node
        let mut simple_expr = make_simple_expr(
            &logic_node,
            logic_node.logic_operator_code == LOGIC_OPERATOR_CODE_AND,
            main_table_alias,
            main_entity_name,
        )?
        .ok_or_else(|| TcdtServiceError::build_internal_msg("simple expr no data"))?;

        // children level node
        let sub_logic_node_option = logic_node.logic_node;
        recursion_make_simple_expr(
            sub_logic_node_option,
            main_table_alias,
            main_entity_name,
            &mut simple_expr,
        )?;
        *condition = condition.clone().add(simple_expr);
    }
    Ok(())
}

fn recursion_make_simple_expr(
    sub_logic_node_option: Option<Box<AqLogicNode>>,
    main_table_alias: &str,
    main_entity_name: &str,
    simple_expr: &mut SimpleExpr,
) -> Result<(), TcdtServiceError> {
    //has children logic node
    if let Some(sub_logic_node) = sub_logic_node_option {
        if sub_logic_node.filter_nodes.len() == 0 {
            return Ok(());
        }
        let sub_simple_expr = make_simple_expr(
            &sub_logic_node,
            sub_logic_node.logic_operator_code == LOGIC_OPERATOR_CODE_AND,
            main_table_alias,
            main_entity_name,
        )?
        .ok_or_else(|| TcdtServiceError::build_internal_msg("simple expr no data"))?;
        *simple_expr = simple_expr.clone().and(sub_simple_expr);
        recursion_make_simple_expr(
            sub_logic_node.logic_node,
            main_table_alias,
            main_entity_name,
            simple_expr,
        )?;
    }
    Ok(())
}

fn make_simple_expr(
    logic_node: &Box<AqLogicNode>,
    fg_and: bool,
    main_table_alias: &str,
    main_entity_name: &str,
) -> Result<Option<SimpleExpr>, TcdtServiceError> {
    let mut simple_expr_option: Option<SimpleExpr> = None;
    for filter_node in &logic_node.filter_nodes {
        build_filter(
            filter_node,
            main_entity_name,
            &mut simple_expr_option,
            fg_and,
            main_table_alias,
            &filter_node.operator_code,
        )?;
    }
    Ok(simple_expr_option)
}

fn build_filter(
    filter_node: &AqFilterNode,
    main_entity_name: &str,
    simple_expr_option: &mut Option<SimpleExpr>,
    fg_and: bool,
    main_table_alias: &str,
    operator_code: &str,
) -> Result<(), TcdtServiceError> {
    if filter_node.name.contains(".") {
        let names: Vec<&str> = filter_node.name.split('.').collect();
        let alias_name = names[..names.len() - 2].join(".");
        let attr_info = get_desc_attr(names, main_entity_name, 0)?;
        let expr_temp = build_filter_by_operator_code(
            operator_code,
            filter_node,
            &alias_name,
            attr_info,
            true,
        )?;
        match simple_expr_option {
            Some(simple_expr) => {
                if fg_and {
                    *simple_expr_option = Some(simple_expr.clone().and(expr_temp));
                } else {
                    *simple_expr_option = Some(simple_expr.clone().or(expr_temp));
                }
            }
            None => {
                *simple_expr_option = Some(expr_temp);
            }
        }
    } else {
        let attr_info = get_desc_attr(vec![&filter_node.name], main_entity_name, 0)?;
        let expr_temp = build_filter_by_operator_code(
            operator_code,
            filter_node,
            main_table_alias,
            attr_info,
            false,
        )?;
        match simple_expr_option {
            Some(simple_expr) => {
                if fg_and {
                    *simple_expr_option = Some(simple_expr.clone().and(expr_temp));
                } else {
                    *simple_expr_option = Some(simple_expr.clone().or(expr_temp));
                }
            }
            None => {
                *simple_expr_option = Some(expr_temp);
            }
        }
    }
    Ok(())
}

fn build_filter_by_operator_code(
    operator_code: &str,
    filter_node: &AqFilterNode,
    alias_name: &str,
    attr_info: AttributeInfo,
    fg_md5_alias_name: bool,
) -> Result<SimpleExpr, TcdtServiceError> {
    let expr_temp: SimpleExpr;
    if operator_code == OPERATOR_CODE_EQUAL {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_eq_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        );
    } else if operator_code == OPERATOR_CODE_NOT_EQUAL {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_ne_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        );
    } else if operator_code == OPERATOR_CODE_LT {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_lt_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        );
    } else if operator_code == OPERATOR_CODE_LTE {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_lte_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        );
    } else if operator_code == OPERATOR_CODE_GT {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_gt_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        );
    } else if operator_code == OPERATOR_CODE_GTE {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_gte_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        );
    } else if operator_code == OPERATOR_CODE_IN {
        let params = filter_node
            .filter_params
            .clone()
            .iter()
            .map(|param| <EFilterParam as Into<sea_orm::Value>>::into(param.to_owned()))
            .collect();
        expr_temp = make_in_condition(&alias_name, &attr_info, params, fg_md5_alias_name);
    } else if operator_code == OPERATOR_CODE_LIKE {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_like_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        )?;
    } else if operator_code == OPERATOR_CODE_LEFT_LIKE {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_left_like_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        )?;
    } else if operator_code == OPERATOR_CODE_RIGHT_LIKE {
        let param = filter_node.filter_params[0].clone();
        expr_temp = make_right_like_condition(
            &alias_name,
            &attr_info,
            <EFilterParam as Into<sea_orm::Value>>::into(param),
            fg_md5_alias_name,
        )?;
    } else {
        return Err(TcdtServiceError::build_internal_msg(&format!(
            "unsupport operator code: '{}'",
            operator_code
        )));
    }
    Ok(expr_temp)
}

fn make_eq_condition<T: Into<sea_orm::Value>>(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: T,
    fg_md5_alias_name: bool,
) -> SimpleExpr {
    let mut alias_name = alias_name.to_string();
    if fg_md5_alias_name {
        alias_name = md5(&alias_name);
    }
    let expr_temp = Expr::col(ColumnRef::TableColumn(
        DynIden::new(Alias::new(alias_name)),
        DynIden::new(Alias::new(&attr_info.column_name)),
    ))
    .eq(v);
    expr_temp
}

fn make_ne_condition<T: Into<sea_orm::Value>>(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: T,
    fg_md5_alias_name: bool,
) -> SimpleExpr {
    let mut alias_name = alias_name.to_string();
    if fg_md5_alias_name {
        alias_name = md5(&alias_name);
    }
    let expr_temp = Expr::col(ColumnRef::TableColumn(
        DynIden::new(Alias::new(alias_name)),
        DynIden::new(Alias::new(&attr_info.column_name)),
    ))
    .ne(v);
    expr_temp
}

fn make_lt_condition<T: Into<sea_orm::Value>>(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: T,
    fg_md5_alias_name: bool,
) -> SimpleExpr {
    let mut alias_name = alias_name.to_string();
    if fg_md5_alias_name {
        alias_name = md5(&alias_name);
    }
    let expr_temp = Expr::col(ColumnRef::TableColumn(
        DynIden::new(Alias::new(alias_name)),
        DynIden::new(Alias::new(&attr_info.column_name)),
    ))
    .lt(v);
    expr_temp
}

fn make_lte_condition<T: Into<sea_orm::Value>>(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: T,
    fg_md5_alias_name: bool,
) -> SimpleExpr {
    let mut alias_name = alias_name.to_string();
    if fg_md5_alias_name {
        alias_name = md5(&alias_name);
    }
    let expr_temp = Expr::col(ColumnRef::TableColumn(
        DynIden::new(Alias::new(alias_name)),
        DynIden::new(Alias::new(&attr_info.column_name)),
    ))
    .lte(v);
    expr_temp
}

fn make_gt_condition<T: Into<sea_orm::Value>>(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: T,
    fg_md5_alias_name: bool,
) -> SimpleExpr {
    let mut alias_name = alias_name.to_string();
    if fg_md5_alias_name {
        alias_name = md5(&alias_name);
    }
    let expr_temp = Expr::col(ColumnRef::TableColumn(
        DynIden::new(Alias::new(alias_name)),
        DynIden::new(Alias::new(&attr_info.column_name)),
    ))
    .gt(v);
    expr_temp
}

fn make_gte_condition<T: Into<sea_orm::Value>>(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: T,
    fg_md5_alias_name: bool,
) -> SimpleExpr {
    let mut alias_name = alias_name.to_string();
    if fg_md5_alias_name {
        alias_name = md5(&alias_name);
    }
    let expr_temp = Expr::col(ColumnRef::TableColumn(
        DynIden::new(Alias::new(alias_name)),
        DynIden::new(Alias::new(&attr_info.column_name)),
    ))
    .gte(v);
    expr_temp
}

fn make_in_condition<T: Into<sea_orm::Value>>(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: Vec<T>,
    fg_md5_alias_name: bool,
) -> SimpleExpr {
    let mut alias_name = alias_name.to_string();
    if fg_md5_alias_name {
        alias_name = md5(&alias_name);
    }
    let expr_temp = Expr::col(ColumnRef::TableColumn(
        DynIden::new(Alias::new(alias_name)),
        DynIden::new(Alias::new(&attr_info.column_name)),
    ))
    .is_in(v);
    expr_temp
}

fn make_like_condition(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: sea_orm::Value,
    fg_md5_alias_name: bool,
) -> Result<SimpleExpr, TcdtServiceError> {
    match v {
        Value::String(v) => {
            let mut alias_name = alias_name.to_string();
            if fg_md5_alias_name {
                alias_name = md5(&alias_name);
            }
            let expr_temp = Expr::col(ColumnRef::TableColumn(
                DynIden::new(Alias::new(alias_name)),
                DynIden::new(Alias::new(&attr_info.column_name)),
            ))
            .like(format!("%{}%", v.unwrap()));
            Ok(expr_temp)
        }
        _ => Err(TcdtServiceError::build_internal_msg(
            "like expr only support String",
        )),
    }
}

fn make_left_like_condition(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: sea_orm::Value,
    fg_md5_alias_name: bool,
) -> Result<SimpleExpr, TcdtServiceError> {
    match v {
        Value::String(v) => {
            let mut alias_name = alias_name.to_string();
            if fg_md5_alias_name {
                alias_name = md5(&alias_name);
            }
            let expr_temp = Expr::col(ColumnRef::TableColumn(
                DynIden::new(Alias::new(alias_name)),
                DynIden::new(Alias::new(&attr_info.column_name)),
            ))
            .like(format!("%{}", v.unwrap()));
            Ok(expr_temp)
        }
        _ => Err(TcdtServiceError::build_internal_msg(
            "like expr only support String",
        )),
    }
}

fn make_right_like_condition(
    alias_name: &str,
    attr_info: &AttributeInfo,
    v: sea_orm::Value,
    fg_md5_alias_name: bool,
) -> Result<SimpleExpr, TcdtServiceError> {
    match v {
        Value::String(v) => {
            let mut alias_name = alias_name.to_string();
            if fg_md5_alias_name {
                alias_name = md5(&alias_name);
            }
            let expr_temp = Expr::col(ColumnRef::TableColumn(
                DynIden::new(Alias::new(alias_name)),
                DynIden::new(Alias::new(&attr_info.column_name)),
            ))
            .like(format!("{}%", v.unwrap()));
            Ok(expr_temp)
        }
        _ => Err(TcdtServiceError::build_internal_msg(
            "like expr only support String",
        )),
    }
}

pub fn make_select_by_condition<T: sea_orm::EntityTrait>(
    _t: T,
    aq_condition: AqCondition,
    main_table_alias: &str,
    main_entity_name: &str,
) -> Result<Select<T>, TcdtServiceError> {
    let root_logic_node = aq_condition.logic_node;
    let orders = aq_condition.orders;
    let join_name_map = get_join_names_from_logic_nodes(root_logic_node.clone());
    let join_name_map = get_join_names_from_orders(join_name_map, orders.clone());
    let mut join_names: Vec<String> = Vec::new();
    for (key, _value) in join_name_map {
        join_names.push(key);
    }
    join_names.sort_by(|a, b| a.split('.').count().cmp(&b.split('.').count()));

    let mut condition = Condition::all();

    make_condition(
        root_logic_node,
        &mut condition,
        main_table_alias,
        main_entity_name,
    )?;

    let mut sql_build = T::find();
    make_join_select(&mut sql_build, join_names, main_entity_name, 0)?;

    sql_build = sql_build.filter(condition);

    make_column_order_by(&mut sql_build, orders, main_entity_name)?;
    Ok(sql_build)
}
