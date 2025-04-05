# reflect

基于postgresql的图床 图片可用bytea表示

用户(要不要和隔壁同步?)、图片(UUID?)、容量限制

- **SeaORM**
- totp
- jwt

sm.ms: <https://i.loli.net/2020/04/05/5MFP9CtOsBuazlr.png> 时间+ID+后缀

前端提供html、md、url、bbcode?

[![20200328000851.png](https://i.loli.net/2020/04/05/5MFP9CtOsBuazlr.png)](https://sm.ms/image/5MFP9CtOsBuazlr)

主要问题:

pics表: uuid 文件名 大小 宽 高 上传时间 内容

基于uuid增删查，在前端预览时缩放，另研究token/cookie/authentication

如果希望完全仅使用 TOTP（无需账号密码），并结合 Cookie 等机制实现身份认证和会话管理，以下是具体方案和实现细节：

核心思路
抛弃传统账号密码，直接以 TOTP 作为唯一认证方式。

用户首次访问时绑定 TOTP（通过共享密钥），后续登录仅需输入动态验证码。

通过安全的 Cookie 或 Token 维持会话状态，避免重复输入 TOTP。

实现步骤
1. 初始化 TOTP 密钥
生成密钥
在服务端预先生成一个唯一的 TOTP 密钥（Base32 编码，如 JBSWY3DPEHPK3PXP），并将其加密存储（如使用 AES 或环境变量保护）。

绑定验证器
用户首次访问时，服务端返回一个二维码（包含 otpauth:// URL），用户用 Google Authenticator 等 App 扫描绑定。

2. 登录认证流程
步骤 1：用户访问登录页面
前端展示一个表单，仅要求输入 6 位 TOTP 动态码（无账号密码字段）。

步骤 2：提交动态码
用户输入当前 TOTP 码，前端将其发送到服务端验证。

步骤 3：服务端验证
服务端使用预存的密钥和 pyotp.TOTP(secret_key).verify(code) 验证动态码是否有效。

步骤 4：发放会话 Cookie
验证通过后，服务端生成一个加密的会话 Cookie（如 JWT），标记用户已登录。

3. 会话管理（Cookie 安全设置）
Cookie 属性

HttpOnly: 禁止 JavaScript 读取，防御 XSS 窃取。

Secure: 仅通过 HTTPS 传输。

SameSite=Strict: 禁止跨站携带 Cookie，防御 CSRF。

Max-Age: 设置较短有效期（如 1-24 小时），超时后需重新验证。

服务端校验
每次请求时，验证 Cookie 中的 Token 是否有效（如 JWT 签名校验）。
4. 安全增强措施
IP 绑定
在发放 Cookie 时记录用户的 IP 地址，后续请求校验 IP 是否一致（防御 Cookie 劫持）。

备用验证码
生成一组一次性备用码（如 10 个），加密存储，供用户在丢失 TOTP 设备时紧急登录。

限流与监控

限制同一 IP 的 TOTP 尝试频率（如 5 次/小时）。

记录所有登录请求，异常时触发告警（如频繁失败尝试）。

5. 密钥备份与恢复
离线备份密钥
将 TOTP 密钥明文（或加密后）存储在离线介质（如纸质笔记），避免服务器数据丢失后无法登录。

多设备同步
允许用户将同一 TOTP 密钥绑定到多个设备（如手机 + 平板），提高可用性。

优点与风险
✅ 优点
极简认证：无需记忆密码，适合个人专属场景。

防爆破：TOTP 动态码一次性有效，无法重放。

无密码泄露风险：服务端不存储密码，攻击面减少。

⚠️ 风险
单点故障：若 TOTP 设备丢失且无备用码，将永久失去访问权。

密钥泄露：若服务端存储的 TOTP 密钥被窃取，攻击者可生成有效动态码。

缓解方案：加密存储密钥，定期轮换（需重新绑定）。

部署建议
强制 HTTPS
使用 Let's Encrypt 证书，避免 Cookie 和 TOTP 码被中间人截获。

隔离上传接口
仅对 /upload 等敏感接口要求 Cookie 认证，公开内容无需鉴权。

定期轮换密钥
每隔 3-6 个月更换一次 TOTP 密钥，并重新绑定验证器。

技术栈参考
TOTP 库

rs: cargo add totp-rs

Python: pyotp + qrcode

Node.js: speakeasy + qrcode

会话管理

JWT 令牌: python-jose (Python) / jsonwebtoken (Node.js)

Cookie 安全设置: 框架原生支持（如 Flask、Express）

总结
纯 TOTP + Cookie 的方案适合个人专属服务，能简化操作并保障安全性，但需严格实施密钥管理、IP 绑定和备用码机制。若你希望彻底避免密码，这是一个可行的选择！
