use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use ::entity::entity::project;
use sea_orm::*;
use tcdt_common::file_util::{folder_exists, get_file_separator, file_exists};
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use crate::dto::vo::ext::template_file::template_file::TemplateFileStat;
use crate::util::file_util::illegal_folder_name;

pub struct TemplateFileExtQuery;

impl TemplateFileExtQuery {
    pub async fn fetch_tree_by_project_id(
        db: &DbConn,
        id_project: String,
    ) -> Result<TemplateFileStat, TcdtServiceError> {
        let project_entity =
            project::Entity::find_by_id(id_project.clone())
                .one(db)
                .await.map_err(|err| {
                log::error!("TemplateFileExtQuery project find_by_id failed");
                TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery project find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("TemplateFileExtQuery project cant not find data"))?;
        let template_code = project_entity.template_code
            .ok_or(TcdtServiceError::build_internal_msg("fetch_tree_by_project_id project template_code is empty"))?;
        if illegal_folder_name(&template_code) {
            return Err(TcdtServiceError::build_internal_msg("project template_code illegal"));
        }

        let project_template_file_path = format!("{}{}{}", TCDT_CONF.code_template_path.to_string(), get_file_separator(), template_code);
        let path = Path::new(&project_template_file_path);
        let children_file_list = traverse_folder(&id_project, &path)?;
        let template_file_stat = TemplateFileStat {
            id_project: id_project.clone(),
            parent_path_name: None,
            file_path_name: Some(project_template_file_path.clone()),
            file_name: template_code,
            fg_file: false,
            content: None,
            children: children_file_list,
            ..Default::default()
        };
        Ok(template_file_stat)
    }

    pub async fn fetch_file_by_path(
        db: &DbConn,
        id_project: String,
        file_path: String,
    ) -> Result<TemplateFileStat, TcdtServiceError> {
        let project_entity =
            project::Entity::find_by_id(id_project.clone())
                .one(db)
                .await.map_err(|err| {
                log::error!("TemplateFileExtQuery project find_by_id failed");
                TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery project find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("TemplateFileExtQuery project cant not find data"))?;
        let template_code = project_entity.template_code
            .ok_or(TcdtServiceError::build_internal_msg("fetch_tree_by_project_id project template_code is empty"))?;
        if illegal_folder_name(&template_code) {
            return Err(TcdtServiceError::build_internal_msg("project template_code illegal"));
        }

        if !file_path.starts_with(&format!("{}{}", template_code, "/")) {
            return Err(TcdtServiceError::build_internal_msg("file_path not start_with template_code"));
        }
        let file_full_path = format!("{}{}{}", TCDT_CONF.code_template_path.to_string(), get_file_separator(), file_path);
        let content = fs::read_to_string(file_full_path.clone()).map_err(|err| {
            TcdtServiceError::build_internal_msg_error("can not read file: %v", err)
        })?;
        let sp: Vec<&str> = file_path.split("/").collect();
        let parent_path_name = sp[0..sp.len() - 1].join("/");
        let template_file_stat = TemplateFileStat {
            id_project: id_project.clone(),
            parent_path_name: Some(parent_path_name),
            file_path_name: Some(file_path.clone()),
            file_name: sp[sp.len() - 1].to_string(),
            fg_file: true,
            content: Some(content),
            children: vec![],
            ..Default::default()
        };
        Ok(template_file_stat)
    }
}

fn traverse_folder(id_project: &String, path: &Path) -> Result<Vec<TemplateFileStat>, TcdtServiceError> {
    let mut template_files: Vec<TemplateFileStat> = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // 如果是文件夹则递归遍历子文件夹
            let children_files = traverse_folder(id_project, &entry_path)?;
            let template_file_stat = TemplateFileStat {
                id_project: id_project.clone(),
                parent_path_name: Some(trim_file_path(&path.to_str().unwrap().to_string())),
                file_path_name: Some(trim_file_path(&entry_path.to_str().unwrap().to_string())),
                file_name: entry_path.file_name().unwrap().to_str().unwrap().to_string(),
                fg_file: false,
                content: None,
                children: children_files,
                ..Default::default()
            };
            template_files.push(template_file_stat)
        } else {
            // 在这里处理文件路径
            let template_file_stat = TemplateFileStat {
                id_project: id_project.clone(),
                parent_path_name: Some(trim_file_path(&path.to_str().unwrap().to_string())),
                file_path_name: Some(trim_file_path(&entry_path.to_str().unwrap().to_string())),
                file_name: entry_path.file_name().unwrap().to_str().unwrap().to_string(),
                fg_file: true,
                content: Some("".to_string()),
                children: vec![],
                ..Default::default()
            };
            template_files.push(template_file_stat)
        }
    }
    Ok(template_files)
}

fn trim_file_path(path: &str) -> String {
    let path_pre = format!("{}{}", TCDT_CONF.code_template_path.to_string(), get_file_separator()).replace("\\", "/");
    path.replace("\\", "/").replace(&path_pre, "")
}

pub struct TemplateFileExtMutation;

impl TemplateFileExtMutation {
    pub async fn add_file(
        db: &DbConn,
        template_file_po: TemplateFileStat,
    ) -> Result<TemplateFileStat, TcdtServiceError> {
        let project_entity =
            project::Entity::find_by_id(template_file_po.id_project.clone())
                .one(db)
                .await.map_err(|err| {
                log::error!("TemplateFileExtMutation project find_by_id failed");
                TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery project find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("TemplateFileExtMutation project cant not find data"))?;
        let template_code = project_entity.template_code
            .ok_or(TcdtServiceError::build_internal_msg("fetch_tree_by_project_id project template_code is empty"))?;
        if illegal_folder_name(&template_code) {
            return Err(TcdtServiceError::build_internal_msg("project template_code illegal"));
        }

        if !template_file_po.file_path_name.clone().unwrap().starts_with(&format!("{}{}", template_code, "/")) {
            return Err(TcdtServiceError::build_internal_msg("file_path not start_with template_code"));
        }
        let file_full_path = format!("{}{}{}", TCDT_CONF.code_template_path.to_string(), get_file_separator(), template_file_po.clone().file_path_name.unwrap());
        if template_file_po.fg_file {
            if file_exists(&file_full_path) {
                return Err(TcdtServiceError::build_internal_msg(&format!("file: {} exists", file_full_path)));
            }
            File::create(file_full_path)
                .map_err(|err| TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery open file failed", err))?;
        } else {
            if folder_exists(&file_full_path) {
                return Err(TcdtServiceError::build_internal_msg(&format!("dir: {} exists", file_full_path)));
            }
            fs::create_dir(file_full_path)
                .map_err(|err| TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery create dir failed", err))?;
        }
        Ok(template_file_po)
    }

    pub async fn update_stat(
        db: &DbConn,
        template_file_po: TemplateFileStat,
    ) -> Result<TemplateFileStat, TcdtServiceError> {
        let project_entity =
            project::Entity::find_by_id(template_file_po.id_project.clone())
                .one(db)
                .await.map_err(|err| {
                log::error!("TemplateFileExtMutation project find_by_id failed");
                TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery project find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("TemplateFileExtMutation project cant not find data"))?;
        let template_code = project_entity.template_code
            .ok_or(TcdtServiceError::build_internal_msg("fetch_tree_by_project_id project template_code is empty"))?;
        if illegal_folder_name(&template_code) {
            return Err(TcdtServiceError::build_internal_msg("project template_code illegal"));
        }

        if !template_file_po.file_path_name.clone().unwrap().starts_with(&format!("{}{}", template_code, "/")) {
            return Err(TcdtServiceError::build_internal_msg("file_path not start_with template_code"));
        }
        if !template_file_po.old_file_path_name.clone().unwrap().starts_with(&format!("{}{}", template_code, "/")) {
            return Err(TcdtServiceError::build_internal_msg("new_file_path_name not start_with template_code"));
        }
        let old_file_full_path = format!("{}{}{}", TCDT_CONF.code_template_path.to_string(), get_file_separator(), template_file_po.clone().old_file_path_name.unwrap());
        let new_file_full_path = format!("{}{}{}", TCDT_CONF.code_template_path.to_string(), get_file_separator(), template_file_po.clone().file_path_name.unwrap());
        if template_file_po.fg_file {
            if !file_exists(&old_file_full_path) {
                return Err(TcdtServiceError::build_internal_msg(&format!("file: {} not exists", old_file_full_path)));
            }
            if file_exists(&new_file_full_path) {
                return Err(TcdtServiceError::build_internal_msg(&format!("file: {} exists", new_file_full_path)));
            }
        } else {
            if !folder_exists(&old_file_full_path) {
                return Err(TcdtServiceError::build_internal_msg(&format!("dir: {} not exists", old_file_full_path)));
            }
            if folder_exists(&new_file_full_path) {
                return Err(TcdtServiceError::build_internal_msg(&format!("dir: {} exists", new_file_full_path)));
            }
        }
        fs::rename(old_file_full_path, new_file_full_path)
            .map_err(|err| TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery rename file failed", err))?;
        Ok(template_file_po)
    }

    pub async fn update_content(
        db: &DbConn,
        template_file_po: TemplateFileStat,
    ) -> Result<TemplateFileStat, TcdtServiceError> {
        let project_entity =
            project::Entity::find_by_id(template_file_po.id_project.clone())
                .one(db)
                .await.map_err(|err| {
                log::error!("TemplateFileExtMutation project find_by_id failed");
                TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery project find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("TemplateFileExtMutation project cant not find data"))?;
        let template_code = project_entity.template_code
            .ok_or(TcdtServiceError::build_internal_msg("fetch_tree_by_project_id project template_code is empty"))?;
        if illegal_folder_name(&template_code) {
            return Err(TcdtServiceError::build_internal_msg("project template_code illegal"));
        }

        if !template_file_po.file_path_name.clone().unwrap().starts_with(&format!("{}{}", template_code, "/")) {
            return Err(TcdtServiceError::build_internal_msg("file_path not start_with template_code"));
        }
        let file_full_path = format!("{}{}{}", TCDT_CONF.code_template_path.to_string(), get_file_separator(), template_file_po.clone().file_path_name.unwrap());
        let mut content = String::new();
        if let Some(content_param) = template_file_po.content.clone() {
            content = content_param;
        }
        if !file_exists(&file_full_path) {
            return Err(TcdtServiceError::build_internal_msg("file not exists"));
        }
        let mut file = File::create(file_full_path)
            .map_err(|err| TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery open file failed", err))?;
        file.write_all(content.as_ref())
            .map_err(|err| TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery write file failed", err))?;
        Ok(template_file_po)
    }

    pub async fn remove_file(
        db: &DbConn,
        template_file_po: TemplateFileStat,
    ) -> Result<TemplateFileStat, TcdtServiceError> {
        let project_entity =
            project::Entity::find_by_id(template_file_po.id_project.clone())
                .one(db)
                .await.map_err(|err| {
                log::error!("TemplateFileExtMutation project find_by_id failed");
                TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery project find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("TemplateFileExtMutation project cant not find data"))?;
        let template_code = project_entity.template_code
            .ok_or(TcdtServiceError::build_internal_msg("fetch_tree_by_project_id project template_code is empty"))?;
        if illegal_folder_name(&template_code) {
            return Err(TcdtServiceError::build_internal_msg("project template_code illegal"));
        }

        if !template_file_po.file_path_name.clone().unwrap().starts_with(&format!("{}{}", template_code, "/")) {
            return Err(TcdtServiceError::build_internal_msg("file_path not start_with template_code"));
        }
        let file_full_path = format!("{}{}{}", TCDT_CONF.code_template_path.to_string(), get_file_separator(), template_file_po.clone().file_path_name.unwrap());
        if template_file_po.fg_file {
            if !file_exists(&file_full_path) {
                return Err(TcdtServiceError::build_internal_msg("file not exists"));
            }
            fs::remove_file(file_full_path)
                .map_err(|err| TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery remove file failed", err))?;
        } else {
            if !folder_exists(&file_full_path) {
                return Err(TcdtServiceError::build_internal_msg("dir not exists"));
            }
            fs::remove_dir(file_full_path)
                .map_err(|err| TcdtServiceError::build_internal_msg_error("TemplateFileExtQuery remove dir failed", err))?;
        }
        Ok(template_file_po)
    }
}