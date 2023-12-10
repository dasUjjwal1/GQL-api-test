use async_graphql::{Context, Object, Result};
use jsonwebtoken::errors::Error;

use crate::{
    schema::models::employee::employee_model::{Employee, EmployeeCreateInput},
    DbState,
};

#[derive(Default)]
pub struct EmployeeMutation;

#[Object]
impl EmployeeMutation {
    async fn create_employee(
        &self,
        ctx: &Context<'_>,
        body: EmployeeCreateInput,
    ) -> Result<Employee> {
        let employee: Option<bool> =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(body.email.to_string().to_ascii_lowercase())
                .fetch_one(&ctx.data::<DbState>()?.db)
                .await
                .map_err(|er| er);
        if let Some(employee_exist) = employee {
            if employee_exist {
                return Err("Email registered".to_string());
            }
        }
    }
}
