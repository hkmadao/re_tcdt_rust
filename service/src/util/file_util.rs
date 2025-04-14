use std::fs;
use std::path::Path;
use tcdt_common::file_util::get_file_separator;
use tcdt_common::tcdt_service_error::TcdtServiceError;

pub fn recursion_get_file_by_folder(path: &str) -> Result<Vec<String>, TcdtServiceError> {
    let mut file_full_name_list: Vec<String> = vec![];
    let read_dir = fs::read_dir(path).map_err(|err| {
        log::error!("read_dir: {} failed", path);
        TcdtServiceError::build_internal_msg_error(&format!("read_dir: {} failed", path), err)
    })?;
    for dir_entry in read_dir {
        if let Ok(dir) = dir_entry {
            let fg_dir = dir
                .metadata()
                .map_err(|err| {
                    log::error!("dir metadata failed");
                    TcdtServiceError::build_internal_msg_error("dir metadata failed", err)
                })?
                .is_dir();
            let file_full_name = dir
                .path()
                .to_str()
                .ok_or(TcdtServiceError::build_internal_msg(
                    "cant not get file full name",
                ))?
                .to_string();
            if !fg_dir {
                file_full_name_list.push(file_full_name.to_owned());
            }
            if fg_dir {
                let mut sub_file_full_name_list = recursion_get_file_by_folder(&file_full_name)?;
                file_full_name_list.append(&mut sub_file_full_name_list);
            }
        }
    }
    Ok(file_full_name_list)
}

pub fn copy_folder_struct(source_path: &str, target_path: &str) -> Result<(), TcdtServiceError> {
    if target_path.contains(source_path) {
        return Err(TcdtServiceError::build_internal_msg(
            &format!("target dir: '{target_path}' can not contains source path: '{source_path}'"),
        ));
    }
    let sub_dir_read_dir = fs::read_dir(source_path).map_err(|err| {
        log::error!("source_dir_read_dir: '{source_path}' read_dir failed");
        TcdtServiceError::build_internal_msg_error("source_dir_read_dir read_dir failed", err)
    })?;
    for sub_dir_result in sub_dir_read_dir {
        if let Ok(sub_dir) = sub_dir_result {
            let fg_dir = sub_dir
                .metadata()
                .map(|meta| meta.is_dir())
                .unwrap_or(false);
            if fg_dir {
                let sub_dir_name = sub_dir
                    .file_name()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_owned();
                let new_sub_dir_name = format!("{}{}{}", target_path, get_file_separator(), sub_dir_name);
                let sub_path_dir_name =
                    format!("{}{}{}", source_path, get_file_separator(), sub_dir_name);
                if new_sub_dir_name.len() > 254 {
                    return Err(TcdtServiceError::build_internal_msg(&format!(
                        "target dir name: '{}' to long",
                        new_sub_dir_name
                    )));
                }
                fs::create_dir(&new_sub_dir_name).map_err(|err| {
                    log::error!("create_dir: '{new_sub_dir_name}' failed");
                    TcdtServiceError::build_internal_msg_error("create_dir failed", err)
                })?;
                copy_folder_struct(&sub_path_dir_name, &new_sub_dir_name)?;
            }
        }
    }
    Ok(())
}

pub fn rename_file_placeholder(source_path: &str, from: &str, to: &str) -> Result<String, TcdtServiceError> {
    let source_metadata = fs::metadata(source_path).map_err(|err| {
        log::error!("path: '{source_path}' metadata failed");
        TcdtServiceError::build_internal_msg_error("path read_dir failed", err)
    })?;
    if !source_metadata.is_file() {
        return Err(TcdtServiceError::build_internal_msg(&format!("path: '{source_path}' is not a file")));
    }
    if !source_path.contains(from) {
        return Ok(source_path.to_owned());
    }
    let file_path = Path::new(source_path);
    let file_name = file_path.file_name().ok_or_else(|| {
        log::error!("path: '{source_path}' get filename failed");
        TcdtServiceError::build_internal_msg("path get filename failed")
    })?.to_str().ok_or_else(|| {
        log::error!("path: '{source_path}' to_str failed");
        TcdtServiceError::build_internal_msg("path to_str failed")
    })?;

    let source_path = file_path.to_str().ok_or_else(|| {
        log::error!("path: '{source_path}' get path failed");
        TcdtServiceError::build_internal_msg("path get path failed")
    })?;
    let end = source_path.len() - file_name.len();
    let target_name = format!("{}{}", &source_path[..end], file_name.replace(from, to));
    fs::rename(source_path, &target_name).map_err(|err| {
        log::error!("rename file path: '{source_path}' to '{target_name}' failed");
        TcdtServiceError::build_internal_msg_error("rename filename failed", err)
    })?;
    Ok(target_name)
}

pub fn rename_sub_folder_placeholder(source_path: &str, from: &str, to: &str) -> Result<(), TcdtServiceError> {
    let sub_dir_read_dir = fs::read_dir(source_path).map_err(|err| {
        log::error!("source_dir_read_dir: '{source_path}' read_dir failed");
        TcdtServiceError::build_internal_msg_error("source_dir_read_dir read_dir failed", err)
    })?;
    for sub_dir_result in sub_dir_read_dir {
        if let Ok(sub_dir) = sub_dir_result {
            let fg_dir = sub_dir
                .metadata()
                .map(|meta| meta.is_dir())
                .unwrap_or(false);
            if fg_dir {
                let sub_path = sub_dir
                    .path()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_owned();
                rename_sub_folder_placeholder(&sub_path, from, to)?;
                let sub_dir_name = sub_dir
                    .file_name()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_owned();
                if !sub_dir_name.contains(from) {
                    continue;
                }
                let end = sub_path.len() - sub_dir_name.len();
                let target_name = format!("{}{}", &sub_path[..end], sub_dir_name.replace(from, to));
                fs::rename(&sub_dir.path(), &target_name).map_err(|err| {
                    log::error!("rename path: '{sub_path}' to '{target_name}' failed");
                    TcdtServiceError::build_internal_msg_error("rename dir failed", err)
                })?;
            }
        }
    }
    Ok(())
}

pub fn copy_folder_to_dest(source_path: &str, target_path: &str) -> Result<(), TcdtServiceError> {
    if target_path.contains(source_path) {
        return Err(TcdtServiceError::build_internal_msg(
            "target dir can not contains source path",
        ));
    }
    let sub_dir_read_dir = fs::read_dir(source_path).map_err(|err| {
        log::error!("source_dir_read_dir read_dir failed");
        TcdtServiceError::build_internal_msg_error("source_dir_read_dir read_dir failed", err)
    })?;
    for sub_dir_result in sub_dir_read_dir {
        if let Ok(sub_dir) = sub_dir_result {
            let fg_dir = sub_dir
                .metadata()
                .map(|meta| meta.is_dir())
                .unwrap_or(false);
            if fg_dir {
                let sub_dir_name = sub_dir
                    .file_name()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_owned();
                let new_sub_dir_name = format!("{}{}{}", target_path, get_file_separator(), sub_dir_name);
                let sub_path_name =
                    format!("{}{}{}", source_path, get_file_separator(), sub_dir_name);
                if new_sub_dir_name.len() > 254 {
                    return Err(TcdtServiceError::build_internal_msg(&format!(
                        "target dir name: '{}' to long",
                        new_sub_dir_name
                    )));
                }
                if !folder_exists(&new_sub_dir_name) {
                    fs::create_dir(&new_sub_dir_name).map_err(|err| {
                        log::error!("create_dir failed");
                        TcdtServiceError::build_internal_msg_error("create_dir failed", err)
                    })?;
                }
                copy_folder_to_dest(&sub_path_name, &new_sub_dir_name)?;
            } else {
                let file_name = sub_dir
                    .file_name()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_owned();
                let new_file_full_name =
                    format!("{}{}{}", target_path, get_file_separator(), file_name);
                let source_file_full_name =
                    format!("{}{}{}", source_path, get_file_separator(), file_name);
                if new_file_full_name.len() > 254 {
                    return Err(TcdtServiceError::build_internal_msg(&format!(
                        "target file full name: '{}' to long",
                        new_file_full_name
                    )));
                }
                if !file_exists(&new_file_full_name) {
                    fs::copy(&source_file_full_name, &new_file_full_name).map_err(|err| {
                        log::error!("copy_file failed");
                        TcdtServiceError::build_internal_msg_error("copy_file failed", err)
                    })?;
                }
            }
        }
    }
    Ok(())
}

pub fn get_file_extension(file_name: &str) -> Option<&str> {
    let path = Path::new(file_name);
    path.extension().and_then(|ext| ext.to_str())
}

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    let metadata_result = fs::metadata(path);

    match metadata_result {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}

pub fn folder_exists(folder_path: &str) -> bool {
    let path = Path::new(folder_path);
    let metadata_result = fs::metadata(path);

    match metadata_result {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

pub fn illegal_folder_name(name: &str) -> bool {
    name.is_empty()
        || name.contains("/")
        || name.contains("\\")
        || name.contains("*")
        || name.contains("?")
        || name.contains(">")
        || name.contains("<")
        || name.contains(":")
        || name.contains("'")
}