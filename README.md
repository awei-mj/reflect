# 載せ

基于postgresql的图床 图片可用bytea表示

用户(要不要和隔壁同步?)、图片(UUID?)、容量限制

sm.ms: <https://i.loli.net/2020/04/05/5MFP9CtOsBuazlr.png> 时间+ID+后缀

前端提供html、md、url、bbcode?

[![20200328000851.png](https://i.loli.net/2020/04/05/5MFP9CtOsBuazlr.png)](https://sm.ms/image/5MFP9CtOsBuazlr)

主要问题:

pics表: uuid 文件名 大小 宽 高 上传时间 内容

基于uuid增删查，在前端预览时缩放，另研究token/cookie/authentication
