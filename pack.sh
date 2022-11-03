#!/bin/bash

TARGET_PATH_BASE=src-tauri/target/universal-apple-darwin/release/bundle/macos/iHost
APP_SIGN_IDENTITY="3rd Party Mac Developer Application: Bingkui Kang (N8CP79P49X)"
INSTALLER_SIGN_IDENTITY="3rd Party Mac Developer Installer: Bingkui Kang (N8CP79P49X)"
ENTITLEMENTS_PLIST=entitlements.plist

echo $TARGET_PATH_BASE.app
echo $ENTITLEMENTS_PLIST

plutil -convert xml1 $ENTITLEMENTS_PLIST
codesign --force --verbose --deep --sign $APP_SIGN_IDENTITY  ./$TARGET_PATH_BASE.app
codesign --force --verbose --deep --sign $APP_SIGN_IDENTITY --entitlements $ENTITLEMENTS_PLIST ./$TARGET_PATH_BASE.app
productbuild --product $ENTITLEMENTS_PLIST --component $TARGET_PATH_BASE.app /Applications $TARGET_PATH_BASE.pkg --sign $INSTALLER_SIGN_IDENTITY 
