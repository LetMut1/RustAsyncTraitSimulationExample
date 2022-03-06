use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as UpdateResolverApplicationUserTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as UpdateResolverApplicationUser;
use redis_ref::aio::AsyncStream;
use redis_ref::aio::Connection as RConnection;
use std::boxed::Box;
use std::error::Error;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use std::pin::Pin;
use tokio_postgres::Client as PConnection;

pub struct Base;

impl Base {
    async fn p_create_<'a>(
        connection: &'a mut PConnection,
        application_user: &'a ApplicationUser
    ) -> Result<i64, BaseError> {
        // ...

        return Ok(1);
    }

    async fn r_create_<'a>(
        connection: &'a mut RConnection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        // ...
        
        return Ok(());
    }
}

pub trait PBaseTrait {
    type Error: Error;
    type UpdateResolverApplicationUser: UpdateResolverApplicationUserTrait;
    
    fn create<'a>(
        connection: &'a mut PConnection,
        application_user: &'a ApplicationUser
    ) -> Pin<Box<dyn Future<Output = Result<i64, Self::Error>> + Send + 'a>>;
}

impl PBaseTrait for Base {
    type Error = BaseError;
    type UpdateResolverApplicationUser = UpdateResolverApplicationUser;

    fn create<'a>(
        connection: &'a mut PConnection,
        application_user: &'a ApplicationUser
    ) -> Pin<Box<dyn Future<Output = Result<i64, Self::Error>> + Send + 'a>> {
        return Box::pin(Self::p_create_(connection, application_user));
    }
}

pub trait RBaseTrait {
    type Error: Error;

    fn create<'a>(
        connection: &'a mut RConnection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;
}


impl RBaseTrait for Base {
    type Error = BaseError;

    fn create<'a>(
        connection: &'a mut RConnection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        return Box::pin(Self::r_create_(connection, application_user_registration_confirmation_token));
    }
}