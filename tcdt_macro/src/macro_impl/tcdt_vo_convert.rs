use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::PathArguments::AngleBracketed;
use syn::{self, Data, DeriveInput, Field, Ident, Meta, Type};
use syn::{GenericArgument, LitStr};
use tcdt_common::name_switch_util::{
    camel_case_to_pascal_case, pascal_case_to_snake_case, snake_case_to_camel_case,
};

pub(crate) fn tcdt_vo_convert(ast: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        ident, data, attrs, ..
    } = ast;
    if !ident.to_string().ends_with("VO") {
        let my_err = syn::Error::new(
            Span::call_site(),
            format!(
                "Struct: {} incorrect, Struct name must ends with 'VO'",
                ident.to_string()
            ),
        );
        return Err(my_err);
    }
    let vo_pascal_case_name = format!("{}", ident.to_string());
    let vo_pascal_case_name_ident = format_ident!("{}", &vo_pascal_case_name);

    let vo_snake_name = format!("{}", pascal_case_to_snake_case(&vo_pascal_case_name));
    let vo_snake_name_ident = format_ident!("{}", &vo_snake_name);

    let base_pascal_case_name = ident.to_string()[0..ident.to_string().len() - 2].to_owned();
    let mut base_snake_case_name = pascal_case_to_snake_case(&base_pascal_case_name);

    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("tcdt_vo"))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("mod_name") {
                    base_snake_case_name = meta.value()?.parse::<LitStr>()?.value();
                }
                Ok(())
            })
        })?;

    let snake_case_name_ident = format_ident!("{}", &base_snake_case_name);

    let entity_name_ident = format_ident!("{}{}", base_snake_case_name, "_entity");

    let mut filed_ast = quote!();
    let mut single_find_ast = quote!();
    let mut array_find_ast = quote!();
    if let Data::Struct(item_strut) = data {
        for (_, field) in item_strut.fields.iter().enumerate() {
            //非匿名字段
            if field.ident.is_some() {
                let field_ty = &field.ty;
                if let Some(field_ident) = &field.ident {
                    if field_ident.to_string() == "action" {
                        filed_ast.extend(quote! {
                            #field_ident: 1,
                        });
                        continue;
                    }
                    // println!("//{:?}", field.attrs);
                    if field.attrs.len() == 0 {
                        filed_ast.extend(quote! {
                            #field_ident: #entity_name_ident.#field_ident.clone(),
                        });
                    } else {
                        let mut anos: Vec<String> = vec![];
                        for (_, attr) in field.attrs.iter().enumerate() {
                            if !attr.path().is_ident("tcdt_vo") {
                                continue;
                            }
                            let _ = attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("ignore") {
                                    anos.push("ignore".to_owned());
                                }
                                if meta.path.is_ident("vo_ref") {
                                    anos.push("vo_ref".to_owned());
                                }
                                if meta.path.is_ident("vo_ref_single") {
                                    anos.push("vo_ref_single".to_owned());
                                }
                                if meta.path.is_ident("vo_array_single") {
                                    anos.push("vo_array_single".to_owned());
                                }
                                if meta.path.is_ident("vo_array") {
                                    anos.push("vo_array".to_owned());
                                }
                                Ok(())
                            });
                        }
                        if anos.contains(&String::from("ignore")) {
                            if anos.contains(&String::from("vo_ref"))
                                || anos.contains(&String::from("vo_ref_single"))
                                || anos.contains(&String::from("vo_array_single"))
                            {
                                filed_ast.extend(quote! {
                                    #field_ident: None,
                                });
                            } else if anos.contains(&String::from("vo_array")) {
                                filed_ast.extend(quote! {
                                    #field_ident: vec![],
                                });
                            } else {
                                //加了tctd_dto标记，只有ingore参数
                                if let Type::Path(ty) = field_ty {
                                    if let Some(_p) = ty.path.get_ident() {
                                        filed_ast.extend(quote! {
                                            #field_ident: #field_ty::default(),
                                        });
                                    } else {
                                        //Option类型，待优化
                                        filed_ast.extend(quote! {
                                            #field_ident: None,
                                        });
                                    }
                                } else {
                                    filed_ast.extend(quote! {
                                        #field_ident: #field_ty::default(),
                                    });
                                }
                            }
                            continue;
                        }
                        if anos.contains(&String::from("vo_ref"))
                            || anos.contains(&String::from("vo_ref_single"))
                            || anos.contains(&String::from("vo_array_single"))
                        {
                            go_single(
                                field_ty,
                                &mut filed_ast,
                                field_ident,
                                &mut single_find_ast,
                                &entity_name_ident,
                                &snake_case_name_ident,
                            )?;
                        } else if anos.contains(&String::from("vo_array")) {
                            go_array(
                                field,
                                field_ty,
                                &mut filed_ast,
                                field_ident,
                                &mut array_find_ast,
                                &entity_name_ident,
                                &snake_case_name_ident,
                            );
                        } else {
                            //加了tctd_dto标记，但是没参数
                            filed_ast.extend(quote! {
                                #field_ident: #entity_name_ident.#field_ident.clone(),
                            });
                        }
                    }
                }
            }
        }
    };

    // println!("//{:?}", filed_ast);
    let gen = quote! {
        impl TcdtViewObjectTrait<#snake_case_name_ident::Model> for #vo_pascal_case_name_ident {
            async fn convert(db: &DbConn, #entity_name_ident: Option<#snake_case_name_ident::Model>) -> Result<Option<Self>, TcdtServiceError>{
                if let Some(#entity_name_ident) = #entity_name_ident {
                    #single_find_ast
                    #array_find_ast
                    let #vo_snake_name_ident: #vo_pascal_case_name_ident =  #vo_pascal_case_name_ident {
                        #filed_ast
                    };
                    return Ok(Some(#vo_snake_name_ident));
                }
                return Ok(None);
            }
        }
    };
    Ok(TokenStream::from(gen))
}

fn go_single(
    field_ty: &Type,
    filed_ast: &mut proc_macro2::TokenStream,
    field_ident: &Ident,
    ref_find_ast: &mut proc_macro2::TokenStream,
    entity_var_name_ident: &Ident,
    entity_snake_case_name_ident: &Ident,
) -> Result<(),syn::Error>{
    if let Type::Path(tp) = field_ty {
        // println!("{:?}",tp);
        for seg in tp.path.segments.iter() {
            if seg.ident.to_string() == "Option" {
                if let AngleBracketed(arg) = &seg.arguments {
                    for g_arg in arg.args.iter() {
                        if let GenericArgument::Type(g_ty) = g_arg {
                            if let Type::Path(gtp) = g_ty {
                                for gseg in gtp.path.segments.iter() {
                                    let field_snake_case_name =
                                        snake_case_to_camel_case(&field_ident.to_string());
                                    let field_pascal_case_name =
                                        camel_case_to_pascal_case(&field_snake_case_name);
                                    let field_pascal_case_name_ident =
                                        format_ident!("{}", &field_pascal_case_name);
                                    let linked_name_ident = format_ident!(
                                        "{}{}",
                                        &field_pascal_case_name_ident,
                                        "Linked"
                                    );
                                    // 引用实体类型
                                    let ref_vo_type_ident = gseg.ident.clone();
                                    filed_ast.extend(quote! {
                                        #field_ident: #ref_vo_type_ident::convert(db, #field_ident).await?,
                                    });
                                    ref_find_ast.extend(quote! {
                                        let #field_ident = #entity_var_name_ident
                                            .find_linked(#entity_snake_case_name_ident::#linked_name_ident)
                                            .one(db)
                                            .await
                                            .map_err(|err| {
                                                TcdtServiceError::build_internal_msg_error(
                                                    "find_linked failed",
                                                    err,
                                                )
                                            })?;
                                    });
                                }
                            }
                        }else {
                            let my_err = syn::Error::new(
                                Span::call_site(),
                                format!(
                                    "field_ident: {} is not GenericArgument::Type, value: {}",
                                    field_ident.to_string(),
                                    g_arg.to_token_stream().to_string()
                                ),
                            );
                            return Err(my_err);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn go_array(
    field: &Field,
    field_ty: &Type,
    filed_ast: &mut proc_macro2::TokenStream,
    field_ident: &Ident,
    ref_find_ast: &mut proc_macro2::TokenStream,
    entity_var_name_ident: &Ident,
    entity_snake_case_name_ident: &Ident,
) {
    let list_attr_list = field
        .attrs
        .iter()
        .filter(|attr| match &attr.meta {
            Meta::Path(_) => false,
            Meta::List(ml) => { 
                if ml.path.is_ident("tcdt_vo") && ml.tokens.to_string().contains("order_by") {
                    return true;
                }
                return false;
            },
            Meta::NameValue(_) => false,
        })
        .collect::<Vec<_>>();
    let order_by_attr = list_attr_list.first();
    let mut pv = String::default();
    if let Some(order_by_attr) = order_by_attr {
        match &order_by_attr.meta {
            Meta::Path(_) => {}
            Meta::List(ml) => {
                let tokens = &ml.tokens;
                let mut order = 0;
                tokens.clone().into_iter().for_each(|x| {
                    let t = x.to_string();
                    if order == 2 {
                        pv = t.clone();
                    }
                    if order == 1 || t == "order_by" {
                        order += 1;
                    }
                });
            }
            Meta::NameValue(_) => {}
        }
    }
    let mut order_property = String::default();
    let mut order_direction = String::from("asc");
    pv = pv.replace("\"","");
    if pv != String::default() {
        if !pv.contains(" ") {
            order_property = pv.clone();
        } else {
            let mut pp = pv.split(" ");
            order_property = pp.next().unwrap().to_string();
            order_direction = pp.next().unwrap().to_string();
        }
    }

    if let Type::Path(tp) = field_ty {
        // println!("{:?}",tp);
        for seg in tp.path.segments.iter() {
            if seg.ident.to_string() == "Vec" {
                if let AngleBracketed(arg) = &seg.arguments {
                    for g_arg in arg.args.iter() {
                        if let GenericArgument::Type(g_ty) = g_arg {
                            if let Type::Path(gtp) = g_ty {
                                for gseg in gtp.path.segments.iter() {
                                    let field_snake_case_name =
                                        format!("{}", field_ident.to_string());
                                    let field_camel_case_name =
                                        snake_case_to_camel_case(&field_snake_case_name);
                                    let field_pascal_case_name =
                                        camel_case_to_pascal_case(&field_camel_case_name);
                                    let field_pascal_case_name_ident =
                                        format_ident!("{}", &field_pascal_case_name);
                                    let linked_name_ident = format_ident!(
                                        "{}{}",
                                        &field_pascal_case_name_ident,
                                        "Linked"
                                    );
                                    // 引用实体类型
                                    let ref_vo_type_ident = gseg.ident.clone();
                                    //引用实体的基础名称
                                    let ref_entity_pascal_name = format!(
                                        "{}",
                                        &ref_vo_type_ident.to_string()
                                            [0..ref_vo_type_ident.to_string().len() - 2]
                                    );
                                    let ref_entity_snake_name =
                                        pascal_case_to_snake_case(&ref_entity_pascal_name);
                                    let ref_entity_var_name =
                                        format!("{}{}", ref_entity_snake_name, "_entity");
                                    let ref_entity_var_name_ident =
                                        format_ident!("{}", &ref_entity_var_name);
                                    let array_var_name =
                                        format!("{}{}", &field_snake_case_name, "_vos");
                                    let array_var_name_ident = format_ident!("{}", &array_var_name);
                                    filed_ast.extend(quote! {
                                        #field_ident: #array_var_name_ident,
                                    });
                                    //数组属性名称
                                    let ref_entity_arr_name =
                                        format!("{}{}", ref_entity_snake_name, "_entities");
                                    let _ref_entity_arr_name_ident =
                                        format_ident!("{}", &ref_entity_arr_name,);
                                    let find_linked_ast :proc_macro2::TokenStream;
                                    if order_property != String::default() {
                                        if order_direction == "asc" {
                                            find_linked_ast = quote! {
                                                let #field_ident = #entity_var_name_ident
                                                    .find_linked(#entity_snake_case_name_ident::#linked_name_ident)
                                                    .order_by(
                                                        Expr::col(ColumnRef::Column(DynIden::new(Alias::new(#order_property)))),
                                                        Order::Asc,
                                                    )
                                                    .all(db)
                                                    .await.map_err(|err| {
                                                        TcdtServiceError::build_internal_msg_error(
                                                            "find_linked failed",
                                                            err,
                                                        )
                                                    })?;
                                                };
                                        }else {
                                            find_linked_ast = quote! {
                                                let #field_ident = #entity_var_name_ident
                                                    .find_linked(#entity_snake_case_name_ident::#linked_name_ident)
                                                    .order_by(
                                                        Expr::col(ColumnRef::Column(DynIden::new(Alias::new(#order_property)))),
                                                        Order::Desc,
                                                    )
                                                    .all(db)
                                                    .await.map_err(|err| {
                                                        TcdtServiceError::build_internal_msg_error(
                                                            "find_linked failed",
                                                            err,
                                                        )
                                                    })?;
                                                };
                                        }
                                    }else {
                                        find_linked_ast = quote! {
                                            let #field_ident = #entity_var_name_ident
                                                .find_linked(#entity_snake_case_name_ident::#linked_name_ident)
                                                .all(db)
                                                .await.map_err(|err| {
                                                    TcdtServiceError::build_internal_msg_error(
                                                        "find_linked failed",
                                                        err,
                                                    )
                                                })?;
                                        };
                                    }
                                    ref_find_ast.extend(quote! {
                                        #find_linked_ast;
                                        
                                        let mut #array_var_name_ident: Vec<#ref_vo_type_ident> = vec![];
                                        for #ref_entity_var_name_ident in #field_ident {
                                            let vo = #ref_vo_type_ident::convert(db, Some(#ref_entity_var_name_ident)).await?;
                                            if let Some(vo) = vo {
                                                #array_var_name_ident.push(vo);
                                            }
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
