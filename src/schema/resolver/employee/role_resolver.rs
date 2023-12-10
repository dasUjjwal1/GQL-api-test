use crate::{
    schema::models::employee::role_model::{Role, RoleInput, RoleUpdateInput},
    DbState,
};
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct RoleMutation;

#[derive(Default)]
pub struct RoleQuery;

#[Object]
impl RoleMutation {
    async fn create_role(&self, ctx: &Context<'_>, body: RoleInput) -> Result<Role> {
        let role: Role =
            sqlx::query_as("INSERT INTO roles (role_name,parent) VALUES ($1,$2) RETURNING *")
                .bind(body.role_name.to_string())
                .bind(Some(body.parent))
                .fetch_one(&ctx.data::<DbState>()?.db)
                .await
                .map_err(|e| e)?;
        Ok(role)
    }
    async fn update_role(&self, ctx: &Context<'_>, body: RoleUpdateInput) -> Result<Role> {
        let check_role: Option<bool> =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM roles WHERE role_id = $1)")
                .bind(body.role_id)
                .fetch_one(&ctx.data::<DbState>()?.db)
                .await
                .map_err(|err| err)?;
        if let Some(role_exist) = check_role {
            if !role_exist {
                return Err("Role does not exists".to_string().into());
            }
        }
        let res: Role = sqlx::query_as(
            "UPDATE
            roles
        SET
            role_name = (
                CASE
                    when $1 IS NOT NULL THEN $1
                    ELSE role_name
                END
            ),
           
            parent = (
                CASE
                    when $2 IS NOT NULL THEN $3
                    ELSE parent
                END
            )
        WHERE
            role_id = $3 RETURNING *",
        )
        .bind(body.role_name)
        .bind(body.parent)
        .bind(body.role_id)
        .fetch_one(&ctx.data::<DbState>()?.db)
        .await
        .map_err(|err| err)?;
        Ok(res)
    }
}

#[Object]
impl RoleQuery {
    async fn get_all_role(&self, ctx: &Context<'_>) -> Result<Vec<Role>> {
        let roles: Vec<Role> = sqlx::query_as("SELECT * FROM roles")
            .fetch_all(&ctx.data::<DbState>()?.db)
            .await
            .map_err(|e| e)?;
        Ok(roles)
    }
}
