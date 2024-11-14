use crate::tcdt_service_error::TcdtServiceError;
use sea_orm::{ConnectionTrait, DbConn, DeleteResult, ModelTrait};

pub trait TcdtViewObjectTrait<T: ModelTrait> {
    fn convert(
        db: &DbConn,
        entity_option: Option<T>,
    ) -> impl std::future::Future<Output = Result<Option<Self>, TcdtServiceError>> + Send
    where
        Self: Sized;
}

pub trait TcdtSaveParamObjectTrait<T: ModelTrait> {
    fn save<C: ConnectionTrait>(
        self,
        db: &C,
        id_parent: Option<String>,
    ) -> impl std::future::Future<Output = Result<Option<T>, TcdtServiceError>> + Send;
}

pub trait TcdtCudParamObjectTrait<T: ModelTrait> {
    fn insert<C: ConnectionTrait>(
        self,
        db: &C,
        id_parent: Option<String>,
    ) -> impl std::future::Future<Output = Result<T, TcdtServiceError>> + Send;
    fn update<C: ConnectionTrait>(
        self,
        db: &C,
        id_parent: Option<String>,
    ) -> impl std::future::Future<Output = Result<T, TcdtServiceError>> + Send;
    fn delete<C: ConnectionTrait>(
        self,
        db: &C,
        id_parent: Option<String>,
    ) -> impl std::future::Future<Output = Result<DeleteResult, TcdtServiceError>> + Send;
}
