pub mod permissions {
    pub const UPDATE_WORKSPACE: &str = "update_workspace";
    pub const DELETE_WORKSPACE: &str = "delete_workspace";
    pub const MANAGE_ROLES: &str = "manage_roles";
    pub const MANAGE_PERMISSIONS: &str = "manage_permissions";
    pub const INVITE_MEMBERS: &str = "invite_members";
    pub const VIEW_MEMBERS: &str = "view_members";
    pub const VIEW_ROLES: &str = "view_roles";
    pub const VIEW_PERMISSIONS: &str = "view_permissions";
    pub const REMOVE_MEMBERS: &str = "remove_members";
    pub const ASSIGN_ROLES_TO_MEMBERS: &str = "assign_roles_to_members";

    pub const ALL: [&str; 10] = [
        UPDATE_WORKSPACE,
        DELETE_WORKSPACE,
        MANAGE_ROLES,
        MANAGE_PERMISSIONS,
        INVITE_MEMBERS,
        VIEW_MEMBERS,
        VIEW_ROLES,
        VIEW_PERMISSIONS,
        REMOVE_MEMBERS,
        ASSIGN_ROLES_TO_MEMBERS,
    ];
}
