# iHosts

host 文件内容管理工具。

## 打包命令

Apple 芯片打包

```shell
# 打包
yarn tauri build --target aarch64-apple-darwin
# 生成 pkg 文件
productbuild --component /Users/luoye/MacPKG/iHost.app /Applications /Users/luoye/MacPKG/iHost.pkg --sign "3rd Party Mac Developer Installer: Bingkui Kang (N8CP79P49X)" --product /Users/luoye/iSoftWareWorkspace/iHosts/entitlements.plist
```

Intel 芯片打包

```shell
> yarn tauri build --target x86_64-apple-darwin

```