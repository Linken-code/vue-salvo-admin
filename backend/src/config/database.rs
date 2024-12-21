use crate::utils::password::hash_password;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::fs;
use std::path::Path;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // 确保数据目录存在
    let db_path = Path::new("../data");
    if !db_path.exists() {
        fs::create_dir_all(db_path).expect("Failed to create database directory");
    }

    let db_file = db_path.join("data.db");
    let db_url = format!("sqlite:{}", db_file.display());

    // 如果数据库文件不存在，创建一个空文件
    if !db_file.exists() {
        fs::File::create(&db_file).expect("Failed to create database file");
    }

    let pool = SqlitePool::connect(&db_url).await?;

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
    .execute(&pool)
    .await?;

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
    .execute(&pool)
    .await?;

    // 创建角色表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            code TEXT NOT NULL UNIQUE,
            description TEXT,
            status INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // 创建权限表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            code TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

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
    .execute(&pool)
    .await?;

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
    .execute(&pool)
    .await?;

    // 检查是否已有菜单数据
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM menus")
        .fetch_one(&pool)
        .await?;

    // 如果没有数据，添加初始菜单
    if count == 0 {
        // 添加仪表盘菜单
        sqlx::query(
            r#"
            INSERT INTO menus (name, path, component, title, icon, sort, is_hidden)
            VALUES ('Dashboard', '/', '../views/Dashboard.vue', '仪表盘', 'Histogram', 0, false)
            "#,
        )
        .execute(&pool)
        .await?;

        // 添加系统管理菜单
        let system_id = sqlx::query(
            r#"
            INSERT INTO menus (name, path, component, title, icon, sort, is_hidden)
            VALUES ('System', '/system', 'LAYOUT', '系统管理', 'Setting', 1, false)
            RETURNING id
            "#,
        )
        .fetch_one(&pool)
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
        .execute(&pool)
        .await?;

        // 添加用户管理子菜单
        sqlx::query(
            r#"
            INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
            VALUES (?, 'UserList', '/users', '../views/system/UserList.vue', '用户管理', 'User', 1, false)
            "#,
        )
        .bind(system_id)
        .execute(&pool)
        .await?;

        // 添加角色管理子菜单
        sqlx::query(
            r#"
            INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
            VALUES (?, 'RoleList', '/roles', '../views/system/RoleList.vue', '角色管理', 'UserFilled', 2, false)
            "#,
        )
        .bind(system_id)
        .execute(&pool)
        .await?;

        // 添加权限管理子菜单
        sqlx::query(
            r#"
            INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
            VALUES (?, 'PermissionList', '/permissions', '../views/system/PermissionList.vue', '权限管理', 'Lock', 3, false)
            "#,
        )
        .bind(system_id)
        .execute(&pool)
        .await?;

        // 添加个人信息设置菜单
        sqlx::query(
            r#"
            INSERT INTO menus (name, path, component, title, icon, sort, is_hidden)
            VALUES ('Profile', '/profile', '../views/user/Profile.vue', '个人信息', 'User', 99, true)
            "#,
        )
        .execute(&pool)
        .await?;
    }

    // 检查是否已有用户数据
    let user_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?;

    // 如果没有数据，添加默认管理员账号
    if user_count == 0 {
        let password_hash = hash_password("admin123");
        sqlx::query(
            r#"
            INSERT INTO users (username, password, nickname, status)
            VALUES ('admin', ?, '管理员', 1)
            "#,
        )
        .bind(&password_hash)
        .execute(&pool)
        .await?;
    }

    // 检查是否已有权限数据
    let permission_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM permissions")
        .fetch_one(&pool)
        .await?;

    // 如果没有数据，添加初始权限
    if permission_count == 0 {
        // 系统管理权限
        sqlx::query(
            r#"
            INSERT INTO permissions (name, code, description)
            VALUES 
                ('系统管理', 'system:manage', '系统管理相关权限'),
                ('用户管理', 'user:manage', '用户的增删改查权限'),
                ('角色管理', 'role:manage', '角色的增删改查权限'),
                ('菜单管理', 'menu:manage', '菜单的增删改查权限'),
                ('权限管理', 'permission:manage', '权限的增删改查权限')
            "#,
        )
        .execute(&pool)
        .await?;
    }

    // 检查是否已有角色数据
    let role_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM roles")
        .fetch_one(&pool)
        .await?;

    // 如果没有数据，添加超级管理员角色
    if role_count == 0 {
        // 添加超级管理员角色
        let role_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO roles (name, code, description, status)
            VALUES ('超级管理员', 'super_admin', '系统超级管理员，拥有所有权限', 1)
            RETURNING id
            "#,
        )
        .fetch_one(&pool)
        .await?;

        // 获取所有权限ID
        let permission_ids = sqlx::query_scalar::<_, i64>("SELECT id FROM permissions")
            .fetch_all(&pool)
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
            .execute(&pool)
            .await?;
        }

        // 为admin用户分配超级管理员角色
        let admin_id =
            sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE username = 'admin'")
                .fetch_one(&pool)
                .await?;

        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            VALUES (?, ?)
            "#,
        )
        .bind(admin_id)
        .bind(role_id)
        .execute(&pool)
        .await?;
    }

    Ok(pool)
}
