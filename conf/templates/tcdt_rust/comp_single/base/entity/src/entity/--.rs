use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "{{ rootInfo.tableName }}")]
pub struct Model {
    #[sea_orm(primary_key, comment = "{{ rootInfo.pkAttributeInfo.displayName }}")]
    pub {{ rootInfo.pkAttributeInfo.columnName }}: String,

{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    /// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
    #[sea_orm(comment = "{{ attributeInfo.displayName }}")]
    pub {{ attributeInfo.columnName }}: Option<{{ attributeInfo.objectType }}>,
{%- endfor %}
{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
    #[sea_orm(comment = "{{ attributeInfo.displayName }}")]
    pub {{ attributeInfo.columnName }}: Option<String>,
{%- endfor %}
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

{%- for attributeInfo in rootInfo.upAttributeInfoList %}
/// {{ attributeInfo.pascalCaseName }}Linked
pub struct {{ attributeInfo.pascalCaseName }}Linked;
impl Linked for {{ attributeInfo.pascalCaseName }}Linked {
    type FromEntity = Entity;

    type ToEntity = super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Entity)
            .from(Column::{{ attributeInfo.innerInfo.pascalCaseName }})
            .to(super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Column::{{attributeInfo.outEntityInfo.pkAttributeInfo.pascalCaseName }})
            .into()]
    }
}
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
/// {{ attributeInfo.pascalCaseName }}Linked
pub struct {{ attributeInfo.pascalCaseName }}Linked;
impl Linked for {{ attributeInfo.pascalCaseName }}Linked {
    type FromEntity = Entity;

    type ToEntity = super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Entity)
            .from(Column::{{ attributeInfo.innerInfo.pascalCaseName }})
            .to(super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Column::{{attributeInfo.outEntityInfo.pkAttributeInfo.pascalCaseName }})
            .into()]
    }
}
{%- endfor %}
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
/// {{ attributeInfo.pascalCaseName }}Linked
pub struct {{ attributeInfo.pascalCaseName }}Linked;
impl Linked for {{ attributeInfo.pascalCaseName }}Linked {
    type FromEntity = Entity;

    type ToEntity = super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Entity)
            .from(Column::{{ rootInfo.pkAttributeInfo.pascalCaseName }})
            .to(super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Column::{{attributeInfo.outerFkInfo.pascalCaseName }})
            .into()]
    }
}
{%- endfor %}

{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
/// {{ attributeInfo.pascalCaseName }}Linked
pub struct {{ attributeInfo.pascalCaseName }}Linked;
impl Linked for {{ attributeInfo.pascalCaseName }}Linked {
    type FromEntity = Entity;

    type ToEntity = super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Entity)
            .from(Column::{{ rootInfo.pkAttributeInfo.pascalCaseName }})
            .to(super::{{ attributeInfo.outEntityInfo.snakeCaseName }}::Column::{{attributeInfo.outerFkInfo.pascalCaseName }})
            .into()]
    }
}
{%- endfor %}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model({{ rootInfo.snakeCaseName }}_model: Model) -> ActiveModel {
    ActiveModel {
        {{ rootInfo.pkAttributeInfo.columnName }}: Set({{ rootInfo.snakeCaseName }}_model.{{ rootInfo.pkAttributeInfo.columnName }}.clone()),
    
    {%- for attributeInfo in rootInfo.baseAttributeInfoList %}
        {{ attributeInfo.columnName }}: Set({{ rootInfo.snakeCaseName }}_model.{{ attributeInfo.columnName }}.clone()),
    {%- endfor %}
    {%- for attributeInfo in rootInfo.fkAttributeInfoList %}
        {{ attributeInfo.columnName }}: Set({{ rootInfo.snakeCaseName }}_model.{{ attributeInfo.columnName }}.clone()),
    {%- endfor %}
    }
}