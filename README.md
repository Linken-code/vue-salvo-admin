# Jing Salvo Admin

一个基于 Vue 3 和 Rust Salvo 的后台管理系统。

## 功能特性

### 用户管理
- 用户的增删改查
- 用户状态管理（启用/禁用）
- 用户角色分配

### 角色管理
- 角色的增删改查
- 角色状态管理
- 角色权限分配

### 权限管理
- 权限的增删改查
- 权限与角色关联

### 系统功能
- 基础 Token 认证
- RESTful API
- 跨域支持
- 数据库事务处理

## 技术栈

### 前端
- Vue 3 (Setup语法)
- Vue Router 4
- Element Plus
- Vite
- Axios

### 后端
- Rust
- Salvo Web框架
- SQLx
- SQLite
- CORS

## 开发环境要求
- Node.js 16+
- Rust 1.70+
- pnpm 8+
- SQLite 3+

## 快速开始

### 前端
```bash
# 进入前端目录
cd frontend

# 安装依赖
pnpm install

# 启动开发服务器
pnpm dev
```

### 后端
```bash
# 进入后端目录
cd backend

# 运行服务器
cargo run
```

## 开发端口
- 前端开发服务器: http://localhost:5173
- 后端API服务器: http://localhost:3000

## 项目结构
```
frontend/          # 前端项目
├── src/          # 源码目录
│   ├── assets/   # 静态资源
│   ├── components/# 组件
│   ├── views/    # 页面视图
│   ├── router/   # 路由配置
│   └── main.js   # 入口文件
└── package.json  # 项目配置

backend/          # 后端项目
├── src/         # 源码目录
│   ├── config/  # 配置文件
│   ├── controllers/# 控制器
│   ├── models/  # 数据模型
│   ├── utils/   # 工具函数
│   └── main.rs  # 入口文件
└── Cargo.toml   # 项目配置
```

## API 接口

### 用户相关
- `POST /auth/login` - 用户登录
- `GET /auth/current-user` - 获取当前用户信息
- `GET /users?page=1&page_size=10` - 获取用户列表（分页）
- `POST /users` - 创建用户
- `PUT /users/:id` - 更新用户信息
- `DELETE /users/:id` - 删除用户
- `GET /users/:id/roles` - 获取用户角色
- `PUT /users/:id/roles` - 更新用户角色

### 角色相关
- `GET /roles` - 获取角色列表
- `POST /roles` - 创建角色
- `PUT /roles/:id` - 更新角色
- `DELETE /roles/:id` - 删除角色
- `GET /roles/:id/permissions` - 获取角色权限
- `PUT /roles/:id/permissions` - 更新角色权限

### 权限相关
- `GET /permissions` - 获取权限列表
- `POST /permissions` - 创建权限
- `PUT /permissions/:id` - 更新权限
- `DELETE /permissions/:id` - 删除权限