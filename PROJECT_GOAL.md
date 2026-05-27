# Rust 学习项目：静态博客生成器

## 🎯 项目目标
学习 Rust 编程，从零开始实现一个静态博客生成器。

## 📚 学习重点
1. **Rust 基础语法** - 所有权、借用、生命周期
2. **CLI 工具开发** - 使用 clap 库
3. **文件系统操作** - 读写文件、目录遍历
4. **Markdown 解析** - 使用 pulldown-cmark
5. **模板引擎** - 使用 Tera
6. **错误处理** - Result 和 Error 类型
7. **异步编程** - 使用 tokio

## 📁 项目结构
rust-blog/
├── Cargo.toml # Rust 项目配置
├── src/ # Rust 源代码
│ ├── main.rs # CLI 入口
│ ├── lib.rs # 核心库
│ ├── blog/ # 博客逻辑
│ ├── parser/ # Markdown 解析
│ ├── generator/ # HTML 生成
│ └── server/ # 本地服务器
├── blog.toml # 博客配置
├── content/ # 博客内容
├── templates/ # HTML 模板
└── static/ # 静态资源

text

## 🗓️ 学习计划
### 第1周：基础 CLI 和文件操作
- 实现 `new` 命令创建文章
- 实现 `build` 命令生成基本 HTML

### 第2周：Markdown 解析和模板
- 解析 Markdown 文件
- 使用 Tera 模板引擎

### 第3周：高级功能和优化
- 添加 RSS 生成
- 添加搜索功能
- 优化性能

### 第4周：部署和完善
- 添加部署功能
- 错误处理和日志
- 编写测试
