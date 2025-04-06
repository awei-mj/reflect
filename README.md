# reflect

基于mariadb的图床 图片保存于文件系统

上传鉴权使用TOTP+JWT模式、图片(UUID)

- SeaORM
- totp
- jwt

sm.ms: <https://i.loli.net/2020/04/05/5MFP9CtOsBuazlr.png> 时间+ID+后缀

前端提供html、md、url、bbcode形式表达

[![20200328000851.png](https://i.loli.net/2020/04/05/5MFP9CtOsBuazlr.png)](https://sm.ms/image/5MFP9CtOsBuazlr)

基于uuid增删查，在前端预览时缩放

1. 会话管理（Cookie 安全设置）

    Cookie 属性:

    - HttpOnly: 禁止 JavaScript 读取，防御 XSS 窃取。
    - Secure: 仅通过 HTTPS 传输。
    - SameSite=Strict: 禁止跨站携带 Cookie，防御 CSRF。
    - Max-Age: 设置较短有效期（如 1-24 小时），超时后需重新验证。

    服务端校验:

    每次请求时，验证 Cookie 中的 Token 是否有效（如 JWT 签名校验）。

2. 安全增强措施

    IP 绑定: 在发放 Cookie 时记录用户的 IP 地址，后续请求校验 IP 是否一致（防御 Cookie 劫持）。

    备用验证码: 生成一组一次性备用码（如 10 个），加密存储，供用户在丢失 TOTP 设备时紧急登录。

    限流与监控: 限制同一 IP 的 TOTP 尝试频率（如 5 次/小时）。

    记录所有登录请求，异常时触发告警（如频繁失败尝试）。

3. 密钥备份与恢复

    离线备份密钥: 将 TOTP 密钥明文（或加密后）存储在离线介质（如纸质笔记），避免服务器数据丢失后无法登录。

    多设备同步: 允许用户将同一 TOTP 密钥绑定到多个设备（如手机 + 平板），提高可用性。

部署建议

强制 HTTPS
使用 Let's Encrypt 证书，避免 Cookie 和 TOTP 码被中间人截获。

隔离上传接口

仅对 /upload 等敏感接口要求 Cookie 认证，公开内容无需鉴权。

定期轮换密钥

每隔 3-6 个月更换一次 TOTP 密钥，并重新绑定验证器。

技术栈参考

TOTP 库

rs: totp-rs

Node.js: speakeasy + qrcode

会话管理

JWT 令牌: python-jose (Python) / jsonwebtoken (Node.js)

Cookie 安全设置: 框架原生支持（如 Flask、Express）

总结
纯 TOTP + Cookie 的方案适合个人专属服务，能简化操作并保障安全性，但需严格实施密钥管理、IP 绑定和备用码机制。若你希望彻底避免密码，这是一个可行的选择！
