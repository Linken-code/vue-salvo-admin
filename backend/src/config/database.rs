use crate::utils::password::hash_password;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    // 创建 data 目录
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).expect("Failed to create data directory");
    }

    // 创建数据库文件
    let db_path = data_dir.join("data.db");
    if !db_path.exists() {
        fs::File::create(&db_path).expect("Failed to create database file");
    }

    let database_url = format!("sqlite:{}", db_path.display());
    println!("Connected to database at: {}", database_url);

    let pool = SqlitePool::connect(&database_url).await?;
    init_database(&pool).await?;
    Ok(pool)
}

pub async fn init_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // 创建菜单表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS menus (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            parent_id INTEGER,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            component TEXT NOT NULL,
            title TEXT NOT NULL,
            icon TEXT,
            sort INTEGER NOT NULL DEFAULT 0,
            is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("Created menus table");

    // 创建用户表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            nickname TEXT NOT NULL,
            email TEXT,
            avatar TEXT,
            status INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("Created users table");

    // 创建角色表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            code TEXT NOT NULL UNIQUE,
            description TEXT,
            status INTEGER NOT NULL DEFAULT 1,
            color_start TEXT,
            color_end TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("Created roles table");

    // 创建权限表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            code TEXT NOT NULL UNIQUE,
            type_name TEXT NOT NULL DEFAULT 'PAGE',
            resource TEXT,
            action TEXT,
            parent_id INTEGER,
            sort INTEGER DEFAULT 0,
            description TEXT,
            color_start TEXT,
            color_end TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (parent_id) REFERENCES permissions(id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("Created permissions table");

    // 创建角色-权限关联表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS role_permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            role_id INTEGER NOT NULL,
            permission_id INTEGER NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE,
            FOREIGN KEY (permission_id) REFERENCES permissions (id) ON DELETE CASCADE,
            UNIQUE (role_id, permission_id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("Created role_permissions table");

    // 创建用户-角色关联表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS user_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            role_id INTEGER NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE,
            UNIQUE (user_id, role_id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("Created user_roles table");

    // 创建操作日志表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS operation_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER,
            username TEXT,
            module TEXT,
            operation TEXT,
            method TEXT,
            params TEXT,
            ip TEXT,
            status INTEGER,
            error TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("Created operation_logs table");

    // 检查是否已有菜单数据
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM menus")
        .fetch_one(pool)
        .await?;

    // 如果没有数据，添加初始菜单
    if count == 0 {
        println!("Adding initial menu data...");
        // 添加仪表盘菜单
        sqlx::query(
            r#"
            INSERT INTO menus (name, path, component, title, icon, sort, is_hidden)
            VALUES ('Dashboard', '/', '../views/Dashboard.vue', '仪表盘', 'Histogram', 0, false)
            "#,
        )
        .execute(pool)
        .await?;

        // 添加系统管理菜单
        let system_id = sqlx::query(
            r#"
            INSERT INTO menus (name, path, component, title, icon, sort, is_hidden)
            VALUES ('System', '/system', 'LAYOUT', '系统管理', 'Setting', 1, false)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await?
        .get::<i64, _>("id");

        // 添加菜单管理子菜单
        sqlx::query(
            r#"
            INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
            VALUES (?, 'MenuList', '/menus', '../views/system/MenuList.vue', '菜单管理', 'Menu', 0, false)
            "#,
        )
        .bind(system_id)
        .execute(pool)
        .await?;

        // 添加用户管理子菜单
        sqlx::query(
            r#"
            INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
            VALUES (?, 'UserList', '/users', '../views/system/UserList.vue', '用户管理', 'User', 1, false)
            "#,
        )
        .bind(system_id)
        .execute(pool)
        .await?;

        // 添加角色管理子菜单
        sqlx::query(
            r#"
            INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
            VALUES (?, 'RoleList', '/roles', '../views/system/RoleList.vue', '角色管理', 'UserFilled', 2, false)
            "#,
        )
        .bind(system_id)
        .execute(pool)
        .await?;

        // 添加权限管理子菜单
        sqlx::query(
            r#"
            INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
            VALUES (?, 'PermissionList', '/permissions', '../views/system/PermissionList.vue', '权限管理', 'Lock', 3, false)
            "#,
        )
        .bind(system_id)
        .execute(pool)
        .await?;

        // 添加操作日志子菜单
        sqlx::query(
            r#"
            INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
            VALUES (?, 'OperationLogList', '/operation-logs', '../views/system/OperationLogList.vue', '操作日志', 'Document', 4, false)
            "#,
        )
        .bind(system_id)
        .execute(pool)
        .await?;

        // 添加个人信息设置菜单
        sqlx::query(
            r#"
            INSERT INTO menus (name, path, component, title, icon, sort, is_hidden)
            VALUES ('Profile', '/profile', '../views/user/Profile.vue', '个人信息', 'User', 99, true)
            "#,
        )
        .execute(pool)
        .await?;
        println!("Added initial menu data");
    }

    // 检查是否已有用户数据
    let user_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    // 如果没有数据，添加默认管理员账号
    if user_count == 0 {
        println!("Adding default admin user...");
        let password_hash = hash_password("admin123");
        sqlx::query(
            r#"
            INSERT INTO users (username, password, nickname, status)
            VALUES ('admin', ?, '管理员', 1)
            "#,
        )
        .bind(&password_hash)
        .execute(pool)
        .await?;
        println!("Created admin user with password: admin123");
    }

    // 检查是否已有权限数据
    let permission_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM permissions")
        .fetch_one(pool)
        .await?;

    // 如果没有数据，添加初始权限
    if permission_count == 0 {
        println!("Adding initial permissions...");
        // 系统管理权限
        let system_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, sort, description, color_start, color_end)
            VALUES ('系统管理', 'system', 'MENU', '/system', 'VIEW', 1, '系统管理相关权限', '#9C27B0', '#BA68C8')
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await?;

        // 菜单管理
        let menu_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description, color_start, color_end)
            VALUES ('菜单管理', 'system:menu', 'PAGE', '/system/menus', 'VIEW', ?, 1, '菜单的增删改查权限', '#FF5722', '#FF7043')
            RETURNING id
            "#,
        )
        .bind(system_id)
        .fetch_one(pool)
        .await?;

        // 菜单管理API权限
        sqlx::query(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description)
            VALUES 
                ('查看菜单', 'system:menu:view', 'API', '/api/menus', 'GET', ?, 1, '查看菜单列表和详情'),
                ('创建菜单', 'system:menu:create', 'API', '/api/menus', 'POST', ?, 2, '创建新菜单'),
                ('编辑菜单', 'system:menu:edit', 'API', '/api/menus/*', 'PUT', ?, 3, '修改菜单信息'),
                ('删除菜单', 'system:menu:delete', 'API', '/api/menus/*', 'DELETE', ?, 4, '删除菜单')
            "#,
        )
        .bind(menu_id)
        .bind(menu_id)
        .bind(menu_id)
        .bind(menu_id)
        .execute(pool)
        .await?;

        // 用户管理
        let user_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description, color_start, color_end)
            VALUES ('用户管理', 'system:user', 'PAGE', '/system/users', 'VIEW', ?, 2, '用户的增删改查权限', '#1976D2', '#42A5F5')
            RETURNING id
            "#,
        )
        .bind(system_id)
        .fetch_one(pool)
        .await?;

        // 用户管理API权限
        sqlx::query(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description)
            VALUES 
                ('查看用户', 'system:user:view', 'API', '/api/users', 'GET', ?, 1, '查看用户列表和详情'),
                ('创建用户', 'system:user:create', 'API', '/api/users', 'POST', ?, 2, '创建新用户'),
                ('编辑用户', 'system:user:edit', 'API', '/api/users/*', 'PUT', ?, 3, '修改用户信息'),
                ('删除用户', 'system:user:delete', 'API', '/api/users/*', 'DELETE', ?, 4, '删除用户')
            "#,
        )
        .bind(user_id)
        .bind(user_id)
        .bind(user_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        // 角色管理
        let role_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description, color_start, color_end)
            VALUES ('角色管理', 'system:role', 'PAGE', '/system/roles', 'VIEW', ?, 3, '角色的增删改查权限', '#388E3C', '#66BB6A')
            RETURNING id
            "#,
        )
        .bind(system_id)
        .fetch_one(pool)
        .await?;

        // 角色管理API权限
        sqlx::query(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description)
            VALUES 
                ('查看角色', 'system:role:view', 'API', '/api/roles', 'GET', ?, 1, '查看角色列表和详情'),
                ('创建角色', 'system:role:create', 'API', '/api/roles', 'POST', ?, 2, '创建新角色'),
                ('编辑角色', 'system:role:edit', 'API', '/api/roles/*', 'PUT', ?, 3, '修改角色信息'),
                ('删除角色', 'system:role:delete', 'API', '/api/roles/*', 'DELETE', ?, 4, '删除角色')
            "#,
        )
        .bind(role_id)
        .bind(role_id)
        .bind(role_id)
        .bind(role_id)
        .execute(pool)
        .await?;

        // 权限管理
        let perm_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description, color_start, color_end)
            VALUES ('权限管理', 'system:permission', 'PAGE', '/system/permissions', 'VIEW', ?, 4, '权限的增删改查权限', '#0097A7', '#26C6DA')
            RETURNING id
            "#,
        )
        .bind(system_id)
        .fetch_one(pool)
        .await?;

        // 权限管理API权限
        sqlx::query(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description)
            VALUES 
                ('查看权限', 'system:permission:view', 'API', '/api/permissions', 'GET', ?, 1, '查看权限列表和详情'),
                ('创建权限', 'system:permission:create', 'API', '/api/permissions', 'POST', ?, 2, '创建新权限'),
                ('编辑权限', 'system:permission:edit', 'API', '/api/permissions/*', 'PUT', ?, 3, '修改权限信息'),
                ('删除权限', 'system:permission:delete', 'API', '/api/permissions/*', 'DELETE', ?, 4, '删除权限')
            "#,
        )
        .bind(perm_id)
        .bind(perm_id)
        .bind(perm_id)
        .bind(perm_id)
        .execute(pool)
        .await?;

        // 操作日志
        let log_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description, color_start, color_end)
            VALUES ('操作日志', 'system:log', 'PAGE', '/system/operation-logs', 'VIEW', ?, 5, '操作日志的查看和清空权限', '#5D4037', '#8D6E63')
            RETURNING id
            "#,
        )
        .bind(system_id)
        .fetch_one(pool)
        .await?;

        // 操作日志API权限
        sqlx::query(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description)
            VALUES 
                ('查看日志', 'system:log:view', 'API', '/api/operation-logs', 'GET', ?, 1, '查看操作日志列表'),
                ('清空日志', 'system:log:clear', 'API', '/api/operation-logs', 'DELETE', ?, 2, '清空操作日志')
            "#,
        )
        .bind(log_id)
        .bind(log_id)
        .execute(pool)
        .await?;

        // 个人信息
        sqlx::query(
            r#"
            INSERT INTO permissions (name, code, type_name, resource, action, parent_id, sort, description, color_start, color_end)
            VALUES ('个人信息', 'system:profile', 'PAGE', '/profile', 'VIEW', ?, 6, '个人信息的查看和修改权限', '#795548', '#A1887F')
            "#,
        )
        .bind(system_id)
        .execute(pool)
        .await?;

        println!("Added initial permissions");
    }

    // 检查是否已有角色数据
    let role_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM roles")
        .fetch_one(pool)
        .await?;

    // 如果没有数据，添加级管理员角色
    if role_count == 0 {
        println!("Adding super admin role...");
        // 添加超级管理员角色
        let role_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO roles (name, code, description, status)
            VALUES ('超级管理员', 'super_admin', '系统超级管理员，拥有所有权限', 1)
            RETURNING id
            "#,
        )
        .fetch_one(pool)
        .await?;

        // 获取所有权ID
        let permission_ids = sqlx::query_scalar::<_, i64>("SELECT id FROM permissions")
            .fetch_all(pool)
            .await?;

        // 为超级管理员分配所有权限
        for permission_id in permission_ids {
            sqlx::query(
                r#"
                INSERT INTO role_permissions (role_id, permission_id)
                VALUES (?, ?)
                "#,
            )
            .bind(role_id)
            .bind(permission_id)
            .execute(pool)
            .await?;
        }

        // 为admin用户分配超级管理员角色
        let admin_id =
            sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE username = 'admin'")
                .fetch_one(pool)
                .await?;

        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            VALUES (?, ?)
            "#,
        )
        .bind(admin_id)
        .bind(role_id)
        .execute(pool)
        .await?;
        println!("Added super admin role and assigned to admin user");
    }

    Ok(())
}
