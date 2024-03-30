# 載せ

基于postgresql的图床 图片可用bytea表示

用户(要不要和隔壁同步?)、图片(UUID?)、容量限制

sm.ms: <https://i.loli.net/2020/04/05/5MFP9CtOsBuazlr.png> 时间+ID+后缀

前端提供html、md、url、bbcode?

[![20200328000851.png](https://i.loli.net/2020/04/05/5MFP9CtOsBuazlr.png)](https://sm.ms/image/5MFP9CtOsBuazlr)

主要问题:

网站内显示图片及相关信息的路径 图片本身url(UUID) 数据库中图片存储结构(主键是UUID)、是否需要使用哈希检测相同图片(否)

总览页面: 图片名、预览(在前端裁剪)、大小、长宽、上传时间、查/删

图片url和站内图片信息url的参数一致，或许可以使用UUID

删除需要使用Deletion Hash(???)

另: 研究token/cookie/authentication怎么搞
