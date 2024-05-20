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
productbuild --component ./iHost.app /Applications ./iHost.pkg --sign "3rd Party Mac Developer Installer: Bingkui Kang (N8CP79P49X)"


sudo productbuild ‐‐component /Applications/iHost.app /Applications ~/Desktop/iHost.pkg


Asset validation failed (90296)
App sandbox not enabled. The following executables must include the "com.apple.security.app-sandbox" entitlement with a Boolean value of true in the entitlements property list: [( "cn.uiseed.ihost.pkg/Payload/iHost.app/Contents/MacOS/iHost" )] Refer to App Sandbox page at https://developer.apple.com/documentation/security/app_sandbox for more information on sandboxing your app. (ID: 254d6309-798b-4bc2-9bb4-6f3de762ab97)

Asset validation failed (90869)
Invalid bundle. The “iHost.app” bundle supports arm64 but not Intel-based Mac computers. Your build must include the x86_64 architecture to support Intel-based Mac computers. For details, view: https://developer.apple.com/documentation/xcode/building_a_universal_macos_binary. (ID: 67aae6a2-487c-4aee-9a49-bfb4683fc69f)