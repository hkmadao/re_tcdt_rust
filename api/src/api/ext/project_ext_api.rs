use actix_web::{error, post, web, Error, HttpRequest, HttpResponse, Result};
use entity::entity::{
    component, component_module, dto_entity_collection, dto_module, entity_collection, sub_project,
};
use tcdt_service::{
    sea_orm::{EntityTrait, QueryFilter},
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::aq::*,
    dto::vo::{
        base::project_vo::ProjectVO,
        ext::project::{
            component::ProjectTreeVO as ComponentProectTreeVO,
            dto_collection::ProjectTreeVO as DtoProjectTreeVO, entity_collection::ProjectTreeVO,
        },
    },
    service::base::project_service::ProjectQuery,
};
use tcdt_service::sea_orm::prelude::Expr;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(aq_detail)]
#[post("/project/aqDetail")]
pub async fn aq_detail(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_condition: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition = aq_condition.into_inner();

    let projects = ProjectQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ProjectVO> = vec![];
    for project_entity in projects {
        let baker_vo = ProjectVO::convert(conn, Some(project_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(baker_vo) = baker_vo {
            vos.push(baker_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(sub_project_aq)]
#[post("/project/subProjectAq")]
pub async fn sub_project_aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json_option: Option<web::Json<AqCondition>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition;
    if let Some(aq_json) = aq_json_option {
        aq_condition = aq_json.into_inner();
    } else {
        aq_condition = AqCondition {
            logic_node: None,
            orders: vec![],
        };
    }
    let projects = ProjectQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut project_vo_list: Vec<ProjectTreeVO> = Vec::new();
    for project in projects {
        let mut project_vo = ProjectTreeVO {
            action: 0,
            code: project.code,
            display_name: project.display_name.clone(),
            path: project.path,
            template_code: project.template_code,
            note: project.note.clone(),
            file_name_type: project.file_name_type,
            level: "project".to_string(),
            children: vec![],
            id: project.id_project.clone(),
            id_parent: None,
        };
        let sub_project_list = sub_project::Entity::find()
            .filter(Expr::col(sub_project::Column::IdProject).eq(project_vo.id.clone()))
            .all(conn)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        let mut sub_project_vo_list: Vec<ProjectTreeVO> = vec![];
        for sub_project in sub_project_list {
            let sub_project_vo = ProjectTreeVO {
                action: 0,
                code: sub_project.name,
                display_name: sub_project.display_name.clone(),
                path: sub_project.path,
                template_code: Some(String::new()),
                note: sub_project.display_name.clone(),
                file_name_type: Some(String::new()),
                level: "subProject".to_string(),
                children: vec![],
                id: sub_project.id_sub_project.clone(),
                id_parent: sub_project.id_project,
            };
            sub_project_vo_list.push(sub_project_vo);
        }
        project_vo.children = sub_project_vo_list;
        project_vo_list.push(project_vo);
    }
    Ok(HttpResponse::Ok().json(project_vo_list))
}

#[tcdt_route(entity_collection_aq)]
#[post("/project/entityCollectionAq")]
pub async fn entity_collection_aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json_option: Option<web::Json<AqCondition>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition;
    if let Some(aq_json) = aq_json_option {
        aq_condition = aq_json.into_inner();
    } else {
        aq_condition = AqCondition {
            logic_node: None,
            orders: vec![],
        };
    }
    let projects = ProjectQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut project_vo_list: Vec<ProjectTreeVO> = Vec::new();
    for project in projects {
        let mut project_vo = ProjectTreeVO {
            action: 0,
            code: project.code,
            display_name: project.display_name.clone(),
            path: project.path,
            template_code: project.template_code,
            note: project.note.clone(),
            file_name_type: project.file_name_type,
            level: "project".to_string(),
            children: vec![],
            id: project.id_project.clone(),
            id_parent: None,
        };
        let sub_project_list = sub_project::Entity::find()
            .filter(Expr::col(sub_project::Column::IdProject).eq(project_vo.id.clone()))
            .all(conn)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        let mut sub_project_vo_list: Vec<ProjectTreeVO> = vec![];
        for sub_project in sub_project_list {
            let mut sub_project_vo = ProjectTreeVO {
                action: 0,
                code: sub_project.name,
                display_name: sub_project.display_name.clone(),
                path: sub_project.path,
                template_code: Some(String::new()),
                note: sub_project.display_name.clone(),
                file_name_type: Some(String::new()),
                level: "subProject".to_string(),
                children: vec![],
                id: sub_project.id_sub_project.clone(),
                id_parent: sub_project.id_project,
            };
            let entity_collection_list = entity_collection::Entity::find()
                .filter(
                    Expr::col(entity_collection::Column::IdSubProject)
                        .eq(sub_project_vo.id.clone()),
                )
                .all(conn)
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    error::ErrorInternalServerError("internal server error")
                })?;
            let mut entity_collection_vo_list: Vec<ProjectTreeVO> = vec![];
            for entity_collection_entity in entity_collection_list {
                let entity_collection_vo = ProjectTreeVO {
                    action: 0,
                    code: entity_collection_entity.package_name.clone(),
                    display_name: entity_collection_entity.display_name.clone(),
                    path: entity_collection_entity.package_name.clone(),
                    template_code: Some(String::new()),
                    note: entity_collection_entity.display_name.clone(),
                    file_name_type: Some(String::new()),
                    level: "entityCollection".to_string(),
                    children: vec![],
                    id: entity_collection_entity.id_entity_collection.clone(),
                    id_parent: entity_collection_entity.id_sub_project.clone(),
                };
                entity_collection_vo_list.push(entity_collection_vo);
            }
            sub_project_vo.children = entity_collection_vo_list;
            sub_project_vo_list.push(sub_project_vo);
        }
        project_vo.children = sub_project_vo_list;
        project_vo_list.push(project_vo);
    }
    Ok(HttpResponse::Ok().json(project_vo_list))
}

#[tcdt_route(component_aq)]
#[post("/project/componentAq")]
pub async fn component_aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json_option: Option<web::Json<AqCondition>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition;
    if let Some(aq_json) = aq_json_option {
        aq_condition = aq_json.into_inner();
    } else {
        aq_condition = AqCondition {
            logic_node: None,
            orders: vec![],
        };
    }
    let projects = ProjectQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut project_vo_list: Vec<ComponentProectTreeVO> = Vec::new();
    for project in projects {
        let mut project_vo = ComponentProectTreeVO {
            action: 0,
            code: project.code,
            display_name: project.display_name.clone(),
            path: project.path,
            template_code: project.template_code,
            note: project.note.clone(),
            file_name_type: project.file_name_type,
            level: "project".to_string(),
            children: vec![],
            id: project.id_project.clone(),
            id_parent: None,
            package_name: None,
            component_type: None,
        };
        let sub_project_list = sub_project::Entity::find()
            .filter(Expr::col(sub_project::Column::IdProject).eq(project_vo.id.clone()))
            .all(conn)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        let mut sub_project_vo_list: Vec<ComponentProectTreeVO> = vec![];
        for sub_project in sub_project_list {
            let mut sub_project_vo = ComponentProectTreeVO {
                action: 0,
                code: sub_project.name,
                display_name: sub_project.display_name.clone(),
                path: sub_project.path,
                template_code: Some(String::new()),
                note: sub_project.display_name.clone(),
                file_name_type: Some(String::new()),
                level: "subProject".to_string(),
                children: vec![],
                id: sub_project.id_sub_project.clone(),
                id_parent: sub_project.id_project,
                package_name: None,
                component_type: None,
            };
            let component_module_list = component_module::Entity::find()
                .filter(
                    Expr::col(component_module::Column::IdSubProject).eq(sub_project_vo.id.clone()),
                )
                .all(conn)
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    error::ErrorInternalServerError("internal server error")
                })?;
            let mut component_module_vo_list: Vec<ComponentProectTreeVO> = vec![];
            for component_module_entity in component_module_list {
                let mut component_module_vo = ComponentProectTreeVO {
                    action: 0,
                    code: component_module_entity.name,
                    display_name: component_module_entity.display_name.clone(),
                    path: component_module_entity.path,
                    template_code: Some(String::new()),
                    note: component_module_entity.display_name.clone(),
                    file_name_type: Some(String::new()),
                    level: "componentModule".to_string(),
                    children: vec![],
                    id: component_module_entity.id_component_module.clone(),
                    id_parent: component_module_entity.id_sub_project.clone(),
                    package_name: None,
                    component_type: None,
                };
                let component_list = component::Entity::find()
                    .filter(
                        Expr::col(component::Column::IdComponentModule)
                            .eq(component_module_vo.id.clone()),
                    )
                    .all(conn)
                    .await
                    .map_err(|e| {
                        log::error!("{:?}", e);
                        error::ErrorInternalServerError("internal server error")
                    })?;
                let mut component_vo_list: Vec<ComponentProectTreeVO> = vec![];
                for component_entity in component_list {
                    let component_vo = ComponentProectTreeVO {
                        action: 0,
                        code: None,
                        display_name: component_entity.display_name.clone(),
                        path: None,
                        template_code: Some(String::new()),
                        note: component_entity.display_name.clone(),
                        file_name_type: Some(String::new()),
                        level: "componentEntityCollection".to_string(),
                        children: vec![],
                        id: component_entity.id_component.clone(),
                        id_parent: component_entity.id_component_module.clone(),
                        package_name: component_entity.package_name.clone(),
                        component_type: component_entity.component_type.clone(),
                    };
                    component_vo_list.push(component_vo);
                }
                component_module_vo.children = component_vo_list;
                component_module_vo_list.push(component_module_vo);
            }
            sub_project_vo.children = component_module_vo_list;
            sub_project_vo_list.push(sub_project_vo);
        }
        project_vo.children = sub_project_vo_list;
        project_vo_list.push(project_vo);
    }
    Ok(HttpResponse::Ok().json(project_vo_list))
}

#[tcdt_route(dto_collection_aq)]
#[post("/project/dtoCollectionAq")]
pub async fn dto_collection_aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json_option: Option<web::Json<AqCondition>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition;
    if let Some(aq_json) = aq_json_option {
        aq_condition = aq_json.into_inner();
    } else {
        aq_condition = AqCondition {
            logic_node: None,
            orders: vec![],
        };
    }
    let projects = ProjectQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut project_vo_list: Vec<DtoProjectTreeVO> = Vec::new();
    for project in projects {
        let mut project_vo = DtoProjectTreeVO {
            action: 0,
            code: project.code,
            display_name: project.display_name.clone(),
            path: project.path,
            template_code: project.template_code,
            note: project.note.clone(),
            file_name_type: project.file_name_type,
            level: "project".to_string(),
            children: vec![],
            id: project.id_project.clone(),
            id_parent: None,
        };
        let sub_project_list = sub_project::Entity::find()
            .filter(Expr::col(sub_project::Column::IdProject).eq(project_vo.id.clone()))
            .all(conn)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        let mut sub_project_vo_list: Vec<DtoProjectTreeVO> = vec![];
        for sub_project in sub_project_list {
            let mut sub_project_vo = DtoProjectTreeVO {
                action: 0,
                code: sub_project.name,
                display_name: sub_project.display_name.clone(),
                path: sub_project.path,
                template_code: Some(String::new()),
                note: sub_project.display_name.clone(),
                file_name_type: Some(String::new()),
                level: "subProject".to_string(),
                children: vec![],
                id: sub_project.id_sub_project.clone(),
                id_parent: sub_project.id_project,
            };
            let dto_module_list = dto_module::Entity::find()
                .filter(
                    Expr::col(component_module::Column::IdSubProject).eq(sub_project_vo.id.clone()),
                )
                .all(conn)
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    error::ErrorInternalServerError("internal server error")
                })?;
            let mut dto_module_vo_list: Vec<DtoProjectTreeVO> = vec![];
            for dto_module_entity in dto_module_list {
                let mut dto_module_vo = DtoProjectTreeVO {
                    action: 0,
                    code: dto_module_entity.name,
                    display_name: dto_module_entity.display_name.clone(),
                    path: dto_module_entity.path,
                    template_code: Some(String::new()),
                    note: dto_module_entity.display_name.clone(),
                    file_name_type: Some(String::new()),
                    level: "dtoModule".to_string(),
                    children: vec![],
                    id: dto_module_entity.id_dto_module.clone(),
                    id_parent: dto_module_entity.id_sub_project.clone(),
                };
                let dto_component_list = dto_entity_collection::Entity::find()
                    .filter(
                        Expr::col(dto_entity_collection::Column::IdDtoModule)
                            .eq(dto_module_vo.id.clone()),
                    )
                    .all(conn)
                    .await
                    .map_err(|e| {
                        log::error!("{:?}", e);
                        error::ErrorInternalServerError("internal server error")
                    })?;
                let mut dto_component_vo_list: Vec<DtoProjectTreeVO> = vec![];
                for dto_component_entity in dto_component_list {
                    let dto_component_vo = DtoProjectTreeVO {
                        action: 0,
                        code: None,
                        display_name: dto_component_entity.display_name.clone(),
                        path: None,
                        template_code: Some(String::new()),
                        note: dto_component_entity.display_name.clone(),
                        file_name_type: Some(String::new()),
                        level: "dtoCollection".to_string(),
                        children: vec![],
                        id: dto_component_entity.id_dto_entity_collection.clone(),
                        id_parent: dto_component_entity.id_dto_module.clone(),
                    };
                    dto_component_vo_list.push(dto_component_vo);
                }
                dto_module_vo.children = dto_component_vo_list;
                dto_module_vo_list.push(dto_module_vo);
            }
            sub_project_vo.children = dto_module_vo_list;
            sub_project_vo_list.push(sub_project_vo);
        }
        project_vo.children = sub_project_vo_list;
        project_vo_list.push(project_vo);
    }
    Ok(HttpResponse::Ok().json(project_vo_list))
}
