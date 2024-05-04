# 后端

使用axum+rbatis

TODO:

1. 数据库/日志配置信息解耦
2. log/slog与错误处理，使用info!error!warn!输出日志
3. 查全部图片的分页以及给数据库添加按时间排序的索引
4. 改为图片交给文件系统维护，数据库存储文件路径
5. token/cookie/authentcation开发 初步想法是基于microsoft authenticator
6. 添加SHA256摘要并在上传时检查
