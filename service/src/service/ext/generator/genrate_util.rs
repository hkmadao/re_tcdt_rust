use std::fs;
use tera::Context;
use tera::Tera;
use tcdt_common::file_util::{folder_exists, get_file_separator};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use crate::util::file_util::copy_folder_struct;

pub(crate) fn create_folder_strut_by_template_folder(
    code_template_path: &str,
    top_code_folder: &str,
    second_code_folder: &str,
    code_generate_path: &str,
    target_folder_name: &str,
) -> Result<(), TcdtServiceError> {
    let template_dir = format!(
        "{}{}{}{}{}",
        code_template_path,
        get_file_separator(),
        top_code_folder,
        get_file_separator(),
        second_code_folder,
    );
    let template_dir_fg_exist = folder_exists(&template_dir);
    if !template_dir_fg_exist {
        return Err(TcdtServiceError::build_internal_msg(
            "template dir not exist",
        ));
    }

    let target_dir = format!(
        "{}{}{}",
        code_generate_path,
        get_file_separator(),
        target_folder_name
    );
    let target_dir_fg_exist = folder_exists(&target_dir);
    if !target_dir_fg_exist {
        log::info!("target dir: '{}' not exist, will be create.", target_dir);
        fs::create_dir_all(&target_dir).map_err(|err| {
            log::error!("target dir: '{}' create_dir_all error", target_dir);
            TcdtServiceError::build_internal_msg_error(
                &format!("target dir: '{}' create_dir_all error", target_dir),
                err,
            )
        })?;
    }

    copy_folder_struct(&template_dir, &target_dir)?;

    Ok(())
}

/// generator
pub(crate) fn generator(
    target_path: &str,
    template_file_prefix: &str,
    template_file_full_name: &str,
    entity_name: &str,
    context: &Context,
    tera: &Tera,
) -> Result<String, TcdtServiceError> {
    let generate_file_full_name = format!(
        "{}{}",
        target_path,
        template_file_full_name.replace(&template_file_prefix, "")
    )
    .replace("--", &entity_name);
    write_to_file(
        tera,
        context,
        template_file_full_name,
        &generate_file_full_name,
    )?;
    Ok(generate_file_full_name.to_owned())
}

/// write to file
fn write_to_file(
    tera: &Tera,
    context: &Context,
    template_file_name: &str,
    generate_file_full_name: &str,
) -> Result<(), TcdtServiceError> {
    let write = fs::File::create(generate_file_full_name).map_err(|err| {
        TcdtServiceError::build_internal_msg_error(
            &format!("file '{}' create error: ", generate_file_full_name),
            err,
        )
    })?;
    log::debug!(
        "render template file '{}' to '{}'",
        template_file_name,
        generate_file_full_name
    );
    tera.render_to(template_file_name, context, write)
        .map_err(|err| {
            TcdtServiceError::build_custom_msg_error(
                &format!("render template file: '{}' error", template_file_name),
                err,
            )
        })?;
    Ok(())
}
