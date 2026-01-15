# Services / Config

- 责任边界：配置加载与装配入口说明。
- 不负责：配置存储与读取策略细节（由 common/storage 模块承担）。
- 目录映射：`crates/services/src/config/`。
- 交付物：配置装配入口说明与相关链接。
- 验收指标：2 分钟内定位配置装配入口。

## 1) 范围说明

services::config 负责运行期配置加载与装配入口。

## 2) 关联文档

- Services 概览：`../services.md`
- Common 模块：`../common.md`
