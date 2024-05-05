# 后端

使用axum+rbatis

TODO:

1. log/slog与错误处理，使用info!error!warn!输出日志
2. 查全部图片的分页以及给数据库添加按时间排序的索引
3. 改为图片交给文件系统维护，数据库存储文件路径
4. token/cookie/authentcation开发 初步想法是基于microsoft authenticator
5. 添加SHA256摘要并在上传时检查
