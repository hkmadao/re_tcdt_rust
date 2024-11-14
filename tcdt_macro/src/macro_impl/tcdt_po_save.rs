use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::PathArguments::AngleBracketed;
use syn::{self, Data, DeriveInput, Ident, Type};
use syn::{GenericArgument, LitStr};
use tcdt_common::name_switch_util::pascal_case_to_snake_case;

pub(crate) fn tcdt_po_save(ast: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        ident, data, attrs, ..
    } = ast;
    if !ident.to_string().ends_with("AggPO") {
        let my_err = syn::Error::new(
            Span::call_site(),
            format!(
                "Struct: {} incorrect, Struct name must ends with 'AggPO'",
                ident.to_string()
            ),
        );
        return Err(my_err);
    }

    let base_pascal_case_name = ident.to_string()[0..ident.to_string().len() - 5].to_owned();
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

    let po_name_ident = format_ident!("{}{}", base_snake_case_name.to_lowercase(), "_po");

    let po_type_name_ident = ident.clone();

    let entity_name_ident = format_ident!("{}{}", base_snake_case_name.to_lowercase(), "_entity");

    let entity_save_name_ident =
        format_ident!("{}{}", base_snake_case_name.to_lowercase(), "_save");

    let mut filed_ast = quote!();
    let mut primary_key_ast = quote!();
    let mut primary_key_ident_option: Option<Ident> = None;
    let mut single_find_ast = quote!();
    let mut array_find_ast = quote!();
    if let Data::Struct(item_strut) = data {
        for (_, field) in item_strut.fields.iter().enumerate() {
            //非匿名字段
            if field.ident.is_some() {
                let field_ty = &field.ty;
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
                                go_singleton(
                                    field_ty,
                                    &mut filed_ast,
                                    field_ident,
                                    &mut single_find_ast,
                                    &po_name_ident,
                                );
                            } else if anos.contains(&String::from("po_children")) {
                                go_children(
                                    field_ty,
                                    &mut filed_ast,
                                    field_ident,
                                    &mut array_find_ast,
                                    &po_name_ident,
                                );
                            } else if anos.contains(&String::from("po_parent_key")) {
                                filed_ast.extend(quote! {
                                    #field_ident: Set(id_parent.clone()),
                                });
                            } else if anos.contains(&String::from("po_primary_key")) {
                                primary_key_ast.extend(quote! {
                                    #field_ident: Set(id.clone()),
                                });
                                primary_key_ident_option = Some(field_ident.clone());
                            } else {
                                //加了tctd_po标记，但是没参数
                                filed_ast.extend(quote! {
                                    #field_ident: Set(self.#field_ident.clone()),
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
        impl TcdtSaveParamObjectTrait<#snake_case_name_ident::Model> for #po_type_name_ident {
            async fn save<C: ConnectionTrait>(self, db: &C, id_parent: Option<String>) -> Result<Option<#snake_case_name_ident::Model>, TcdtServiceError> {
                if self.action == DO_UNCHANGE {
                    Ok(None)
                } else if self.action == DO_NEW {
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

                    let #entity_save_name_ident = #snake_case_name_ident::Entity::find_by_id(id)
                    .one(db)
                    .await
                    .map_err(|err| {
                        TcdtServiceError::build_internal_msg_error(
                            "find_by_id failed",
                            err,
                        )
                    })?
                    .ok_or(TcdtServiceError::build_internal_msg("Cannot find entity"))?;
                    Ok(Some(#entity_save_name_ident))
                } else if self.action == DO_UPDATE {
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

                    Ok(Some(#entity_save_name_ident))
                } else if self.action == DO_DELETE {
                    let id = self.#primary_key_ident.clone();
                    #single_find_ast
                    #array_find_ast
                    let entity_save = #snake_case_name_ident::ActiveModel {
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

                    Ok(None)
                } else {
                    Ok(None)
                }
            }
        }
    };
    Ok(TokenStream::from(gen))
}

fn go_singleton(
    field_ty: &Type,
    _filed_ast: &mut proc_macro2::TokenStream,
    field_ident: &Ident,
    ref_find_ast: &mut proc_macro2::TokenStream,
    _entity_po_name_ident: &Ident,
) {
    if let Type::Path(tp) = field_ty {
        // println!("{:?}",tp);
        for seg in tp.path.segments.iter() {
            if seg.ident.to_string() == "Option" {
                if let AngleBracketed(arg) = &seg.arguments {
                    for g_arg in arg.args.iter() {
                        if let GenericArgument::Type(g_ty) = g_arg {
                            if let Type::Path(gtp) = g_ty {
                                for gseg in gtp.path.segments.iter() {
                                    // 引用实体类型
                                    let ref_type_ident = gseg.ident.clone();
                                    ref_find_ast.extend(quote! {
                                        let _ = #ref_type_ident::save(self.#field_ident, db, Some(id.clone()));
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn go_children(
    field_ty: &Type,
    _filed_ast: &mut proc_macro2::TokenStream,
    field_ident: &Ident,
    ref_find_ast: &mut proc_macro2::TokenStream,
    _entity_po_name_ident: &Ident,
) {
    if let Type::Path(tp) = field_ty {
        // println!("{:?}",tp);
        for seg in tp.path.segments.iter() {
            if seg.ident.to_string() == "Vec" {
                if let AngleBracketed(arg) = &seg.arguments {
                    for g_arg in arg.args.iter() {
                        if let GenericArgument::Type(g_ty) = g_arg {
                            if let Type::Path(gtp) = g_ty {
                                for gseg in gtp.path.segments.iter() {
                                    // 引用实体类型
                                    let ref_type_ident = gseg.ident.clone();
                                    //引用实体的基础名称
                                    let ref_entity_pascal_name = format!(
                                        "{}",
                                        &ref_type_ident.to_string()
                                            [0..ref_type_ident.to_string().len() - 2]
                                    );
                                    let ref_entity_snake_name =
                                        pascal_case_to_snake_case(&ref_entity_pascal_name);
                                    let ref_po_name = format!("{}{}", ref_entity_snake_name, "_po");
                                    let ref_po_name_ident =
                                        Ident::new(&ref_po_name, Span::call_site());
                                    ref_find_ast.extend(quote! {
                                        for #ref_po_name_ident in self.#field_ident {
                                            let _ = #ref_type_ident::save(#ref_po_name_ident, db, Some(id.clone()));
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
