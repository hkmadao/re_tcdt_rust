use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::LitStr;
use syn::{self, Data, DeriveInput, Ident};
use tcdt_common::name_switch_util::pascal_case_to_snake_case;

pub(crate) fn tcdt_po_cud(ast: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        ident, data, attrs, ..
    } = ast;
    if !ident.to_string().ends_with("PO") {
        let my_err = syn::Error::new(
            Span::call_site(),
            format!(
                "Struct: {} incorrect, Struct name must ends with 'PO'",
                ident.to_string()
            ),
        );
        return Err(my_err);
    }

    let base_pascal_case_name = ident.to_string()[0..ident.to_string().len() - 2].to_owned();
    let mut base_snake_case_name = pascal_case_to_snake_case(&base_pascal_case_name);

    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("tcdt_po"))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("mod_name") {
                    base_snake_case_name = meta.value()?.parse::<LitStr>()?.value();
                }
                Ok(())
            })
        })?;

    let snake_case_name_ident = format_ident!("{}", &base_snake_case_name);

    let _po_var_name_ident = format_ident!("{}{}", base_snake_case_name.to_lowercase(), "_po");

    let po_type_name_ident = ident.clone();

    let entity_name_ident = format_ident!("{}{}", base_snake_case_name.to_lowercase(), "_entity");

    let entity_save_name_ident =
        format_ident!("{}{}", base_snake_case_name.to_lowercase(), "_save");

    let mut filed_ast = quote!();
    let mut model_filed_ast = quote!();
    let mut primary_key_ast = quote!();
    let mut primary_key_ident_option: Option<Ident> = None;
    let single_find_ast = quote!();
    let array_find_ast = quote!();
    if let Data::Struct(item_strut) = data {
        for (_, field) in item_strut.fields.iter().enumerate() {
            //非匿名字段
            if field.ident.is_some() {
                let _field_ty = &field.ty;
                if let Some(field_ident) = &field.ident {
                    // println!("//{:?}", field.attrs);
                    if field.attrs.len() == 0 {
                        filed_ast.extend(quote! {
                            #field_ident: Set(self.#field_ident.clone()),
                        });
                    } else {
                        let mut anos: Vec<String> = vec![];
                        for (_, attr) in field.attrs.iter().enumerate() {
                            if !attr.path().is_ident("tcdt_po") {
                                continue;
                            }
                            let _ = attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("ignore") {
                                    anos.push("ignore".to_owned());
                                }
                                if meta.path.is_ident("po_children") {
                                    anos.push("po_children".to_owned());
                                }
                                if meta.path.is_ident("po_singleton") {
                                    anos.push("po_singleton".to_owned());
                                }
                                if meta.path.is_ident("po_parent_key") {
                                    anos.push("po_parent_key".to_owned());
                                }
                                if meta.path.is_ident("po_primary_key") {
                                    anos.push("po_primary_key".to_owned());
                                }
                                Ok(())
                            });
                        }
                        if anos.contains(&String::from("ignore")) {
                            //忽略的属性无需处理
                        } else {
                            if anos.contains(&String::from("po_singleton")) {
                            } else if anos.contains(&String::from("po_children")) {
                            } else if anos.contains(&String::from("po_parent_key")) {
                                filed_ast.extend(quote! {
                                    #field_ident: Set(id_parent.clone()),
                                });
                                model_filed_ast.extend(quote! {
                                    #field_ident: id_parent.clone(),
                                });
                            } else if anos.contains(&String::from("po_primary_key")) {
                                primary_key_ast.extend(quote! {
                                    #field_ident: Set(id.clone()),
                                });
                                primary_key_ident_option = Some(field_ident.clone());
                                model_filed_ast.extend(quote! {
                                    #field_ident: self.#field_ident.clone(),
                                });
                            } else {
                                //加了tctd_po标记，但是没参数
                                filed_ast.extend(quote! {
                                    #field_ident: Set(self.#field_ident.clone()),
                                });
                                model_filed_ast.extend(quote! {
                                    #field_ident: self.#field_ident.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    };

    if primary_key_ident_option == None {
        let my_err = syn::Error::new(
            Span::call_site(),
            format!("Struct: {}, primary_key not set", ident.to_string()),
        );
        return Err(my_err);
    }

    let primary_key_ident = primary_key_ident_option.unwrap();

    // println!("//{:?}", filed_ast);
    let gen = quote! {
        impl TcdtCudParamObjectTrait<#snake_case_name_ident::Model> for #po_type_name_ident {
            fn convert_po_to_model(self) -> #snake_case_name_ident::Model {
                #snake_case_name_ident::Model {
                    #model_filed_ast
                    ..Default::default()
                }
            }
            async fn insert<C: ConnectionTrait>(self, db: &C, id_parent: Option<String>) -> Result<#snake_case_name_ident::Model, TcdtServiceError> {
                let id = nanoid::nanoid!();
                let #entity_save_name_ident = #snake_case_name_ident::ActiveModel {
                    #primary_key_ident: Set(id.clone()),
                    #filed_ast
                    ..Default::default()
                };
                let _ = #snake_case_name_ident::Entity::insert(#entity_save_name_ident).exec(db)
                .await.map_err(|err| {
                    TcdtServiceError::build_internal_msg_error(
                        "insert failed",
                        err,
                    )
                })?;
                #single_find_ast
                #array_find_ast

                let #entity_save_name_ident = #snake_case_name_ident::Entity::find_by_id(id).one(db)
                .await.map_err(|err| {
                    TcdtServiceError::build_internal_msg_error(
                        "find_by_id failed",
                        err,
                    )
                })?
                .ok_or(TcdtServiceError::build_internal_msg("Cannot find entity"))?;
                Ok(#entity_save_name_ident)
            }
            async fn update<C: ConnectionTrait>(self, db: &C, id_parent: Option<String>) -> Result<#snake_case_name_ident::Model, TcdtServiceError> {
                let id = self.#primary_key_ident.clone();
                #single_find_ast
                #array_find_ast
                let #entity_name_ident: #snake_case_name_ident::ActiveModel = #snake_case_name_ident::Entity::find_by_id(&id)
                    .one(db)
                    .await
                    .map_err(|err| {
                        TcdtServiceError::build_internal_msg_error(
                            "find_by_id failed",
                            err,
                        )
                    })?
                    .ok_or(TcdtServiceError::build_internal_msg(&format!("Cannot find entity [{}].", stringify!(#entity_name_ident))))?
                    .into_active_model();

                let #entity_save_name_ident = #snake_case_name_ident::ActiveModel {
                    #primary_key_ast
                    #filed_ast
                    ..#entity_name_ident
                }
                .update(db)
                .await.map_err(|err| {
                    TcdtServiceError::build_internal_msg_error(
                        "update failed",
                        err,
                    )
                })?;

                Ok(#entity_save_name_ident)
            }
            async fn delete<C: ConnectionTrait>(self, db: &C, id_parent: Option<String>) -> Result<DeleteResult, TcdtServiceError> {
                let id = self.#primary_key_ident.clone();
                #single_find_ast
                #array_find_ast
                let delete_result = #snake_case_name_ident::ActiveModel {
                    #primary_key_ast
                    ..Default::default()
                }
                .delete(db)
                .await.map_err(|err| {
                    TcdtServiceError::build_internal_msg_error(
                        "delete failed",
                        err,
                    )
                })?;

                Ok(delete_result)
            }
        }
    };
    Ok(TokenStream::from(gen))
}
