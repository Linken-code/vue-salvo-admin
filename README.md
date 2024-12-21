# Vue Salvo Admin

一个现代化的后台管理系统，基于 Vue 3 + Salvo (Rust) 开发。

## 技术栈

### 前端
- Vue 3 (Setup语法)
- Vue Router 4
- Pinia
- Element Plus
- Vite

### 后端
- Rust
- Salvo
- SQLx
- PostgreSQL
- Redis

## 开发环境要求
- Node.js 16+
- Rust 1.70+
- pnpm 8+
- PostgreSQL 14+
- Redis 6+

## 快速开始

### 前端
```bash
cd frontend
pnpm install
pnpm dev
```

### 后端
```bash
cd backend
cargo run
```

## 开发端口
- 前端开发服务器: http://localhost:5173
- 后端API服务器: http://localhost:3000

## 项目结构
```
frontend/          # 前端项目
backend/           # 后端项目
``` 