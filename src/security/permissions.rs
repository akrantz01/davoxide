use crate::{
    database::{Action, Permission, User},
    error::{Error, Result},
};
use sqlx::PgPool;
use std::path::Path;

/// Check the user's permissions are sufficient for the requested action and path
pub async fn check_permissions(
    db: &PgPool,
    user: &User,
    path: &Path,
    required: Action,
) -> Result<()> {
    if user.default_access != Action::Admin {
        let permissions = user.permissions(db).await?;
        let effective = effective_permission(permissions, user.default_access, path);

        if effective < required {
            return Err(Error::InvalidPermissions);
        }
    }

    Ok(())
}

/// Find the effective permission for the given path by finding the most specific permission
/// applied to the user.
fn effective_permission(permissions: Vec<Permission>, default: Action, path: &Path) -> Action {
    let mut effective = default;

    for permission in permissions {
        if path.starts_with(&permission.path) && permission.affects_children {
            effective = permission.action;
        } else if path == Path::new(&permission.path) {
            effective = permission.action;
        }
    }

    effective
}

#[cfg(test)]
mod tests {
    use crate::database::Action;

    macro_rules! permission {
        (path = $path:expr, action = $action:expr, children = $children:expr $(,)?) => {
            crate::database::Permission {
                id: 1,
                applies_to: String::from("user"),
                path: $path.into(),
                action: $action,
                affects_children: $children,
            }
        };
    }

    macro_rules! evaluate_permissions {
        (default = $default:expr) => {
            evaluate_permissions!(default = $default, path = "/")
        };
        (default = $default:expr, path = $path:expr) => {
            evaluate_permissions!(default = $default, path = $path, permissions = [])
        };
        (
            default = $default:expr, path = $path:expr,
            permissions = [
                $( { path = $permission_path:expr, action = $permission_action:expr, children = $permission_children:expr } ),* $(,)?
            ]
        ) => {
            evaluate_permissions!(
                default = $default, path = $path,
                permissions = vec![ $(
                    permission!(
                        path = $permission_path,
                        action = $permission_action,
                        children = $permission_children
                    ),
                )* ]
            )
        };
        (default = $default:expr, path = $path:expr, permissions = $permissions:expr) => {
            {
                use std::path::Path;
                super::effective_permission($permissions, $default, Path::new($path))
            }
        };
    }

    #[test]
    fn default_action() {
        let action = evaluate_permissions!(default = Action::Modify);
        assert_eq!(action, Action::Modify);
    }

    #[test]
    fn single_file() {
        let action = evaluate_permissions!(
            default = Action::Modify, path = "/test",
            permissions = [
                { path = "/test", action = Action::Read, children = false },
            ]
        );
        assert_eq!(action, Action::Read);
    }

    #[test]
    fn folder_affecting_children() {
        let action = evaluate_permissions!(
            default = Action::Modify, path = "/abc/def",
            permissions = [
                { path = "/abc", action = Action::Read, children = true },
            ]
        );
        assert_eq!(action, Action::Read);
    }

    #[test]
    fn file_in_folder_override() {
        let action = evaluate_permissions!(
            default = Action::Modify, path = "/abc/def",
            permissions = [
                { path = "/abc", action = Action::Read, children = true },
                { path = "/abc/def", action = Action::Modify, children = false },
            ]
        );
        assert_eq!(action, Action::Modify);
    }

    #[test]
    fn deeply_nested_override() {
        let permissions = vec![
            permission!(path = "/folder", action = Action::Read, children = true),
            permission!(
                path = "/folder/sub/file",
                action = Action::Modify,
                children = false,
            ),
            permission!(
                path = "/folder/sub/no",
                action = Action::Deny,
                children = false,
            ),
        ];

        assert_eq!(
            evaluate_permissions!(
                default = Action::Modify,
                path = "/folder/abcdef",
                permissions = permissions.clone()
            ),
            Action::Read
        );
        assert_eq!(
            evaluate_permissions!(
                default = Action::Modify,
                path = "/folder/sub/no",
                permissions = permissions.clone()
            ),
            Action::Deny
        );
        assert_eq!(
            evaluate_permissions!(
                default = Action::Modify,
                path = "/folder/sub/file",
                permissions = permissions.clone()
            ),
            Action::Modify
        );
        assert_eq!(
            evaluate_permissions!(
                default = Action::Modify,
                path = "/folder/sub",
                permissions = permissions
            ),
            Action::Read
        );
    }
}
