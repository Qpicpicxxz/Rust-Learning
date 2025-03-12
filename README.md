关键字Keywords
- `const` 常量/不变裸指针(直接使用内存地址的指针)
- `let` 绑定一个变量
- `match` 模式匹配
- `pub` 表示结构体字段

Crate是Rust编译器处理的基本编译单元, 分为Binary Crate和Library Crate(有点类似c的.so/.ao?)

更新完Cargo.toml之后`cargo build`会从registry上获取所有包的最新版本信息

语义化版本(semantic versioning, SemVer)是一种标准化的版本号格式
 - MAJOR.MINOR.PATCH
 - MAJOR -> 不兼容的API变更
 - MINOR -> 向后兼容的功能新增
 - PATCH -> 向后兼容的问题修正

Cargo.lock - 记录项目依赖的实际版本，永远别碰这个文件就好了

TOML格式(Tom's Obvious, Minimal Language)

Cargo是Rust的构建系统和包管理器

Rust 是一种预编译静态类型(ahead-of-time compiled)
