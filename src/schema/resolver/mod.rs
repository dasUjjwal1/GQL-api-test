use async_graphql::MergedObject;

use self::employee::{
    employee_resolver::EmployeeMutation,
    role_resolver::{RoleMutation, RoleQuery},
};

pub mod employee;

#[derive(Default, MergedObject)]
pub struct Mutation(RoleMutation, EmployeeMutation);

#[derive(Default, MergedObject)]
pub struct QueryRoot(RoleQuery);
