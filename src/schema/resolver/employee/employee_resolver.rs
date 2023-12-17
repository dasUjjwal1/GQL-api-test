use async_graphql::{Context, Object, Result};
use jsonwebtoken::errors::Error;

use crate::{
    schema::models::employee::employee_model::{Employee, EmployeeCreateInput, Gender},
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
    ) -> Result<String> {
        let employee: Option<bool> =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(body.email.to_string().to_ascii_lowercase())
                .fetch_one(&ctx.data::<DbState>()?.db)
                .await
                .map_err(|er| er)?;
        if let Some(employee_exist) = employee {
            if employee_exist {
                return Err("Email already registered".to_string().into());
            }
        }

        let gender: Option<i16> = match body.gender {
            Some(data) => match data.as_str() {
                "M" => Some(Gender::M as i16),
                "F" => Some(Gender::F as i16),
                "O" => Some(Gender::O as i16),
                _ => None,
            },
            None => None,
        };
        let save_employee: Employee = sqlx::query_as(
            r#"
        INSERT INTO
    users(
        name email,
        password,
        photo,
        mobile,
        personal_email,
        gender,
        dob,
        employee_id,
        address,
        qualification,
        role
    )
VALUES
    ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) RETURNING *
        "#,
        )
        .bind(body.name.to_string())
        .bind(body.email.to_string().to_ascii_lowercase())
        .bind(body.password.to_string())
        .bind(Some(body.photo))
        .bind(body.mobile.to_string())
        .bind(
            body.personal_email
                .unwrap_or_default()
                .to_string()
                .to_ascii_lowercase(),
        )
        .bind(Some(gender))
        .bind(Some(body.dob))
        .bind(body.employee_id.to_string())
        .bind(Some(body.address))
        .bind(Some(body.qualification))
        .bind(Some(body.role))
        .fetch_one(&ctx.data::<DbState>()?.db)
        .await
        .map_err(|er| er)?;
        Ok("Cre".to_string())
    }
}
