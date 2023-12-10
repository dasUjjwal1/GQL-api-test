use async_graphql::MergedObject;

use self::employee::role_resolver::{RoleMutation, RoleQuery};

pub mod employee;

#[derive(Default, MergedObject)]
pub struct Mutation(RoleMutation);

#[derive(Default, MergedObject)]
pub struct QueryRoot(RoleQuery);
