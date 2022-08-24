<template>
  <el-container class="v-main">
    <el-aside class="left-side" width="200px" data-tauri-drag-region>
      <Logo />
      <HostList />
      <Menu />
    </el-aside>
    <el-main class="right-container">
      <Content />
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useSystemStore } from './stores/sys';
import hotkeys from "hotkeys-js";
import { os } from '@tauri-apps/api';
import { WebviewWindow } from '@tauri-apps/api/window';
import { emit, listen } from '@tauri-apps/api/event'
import { useHostStore } from './stores/host';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process';

onMounted(async () => {
  const appSetting = useSystemStore();
  await appSetting.getSysSetting();
  const type = await os.type();
  let hotKey = 'command+W';
  switch (type) {
    case 'Darwin':
      hotKey = 'command+W';
      break;
    case 'Linux':
    case 'Windows_NT':
      hotKey = 'ctrl+W';
      break;
  }
  hotkeys(hotKey, () => {
    const win = WebviewWindow.getByLabel('main');
    win?.hide();
  });
  // 添加系统托盘时间监听
  listen('change-used', async (event: { payload: { id: string, used: boolean } }) => {
    const { id, used } = event.payload;
    const host = useHostStore();
    await host.updateHostUsed(id, !used);
  })
  // 检查更新
  try {
    const { shouldUpdate, manifest } = await checkUpdate();
    console.log('检查更新', shouldUpdate);
    if (shouldUpdate) {
      console.log('manifest ->', manifest);
      // display dialog
      await installUpdate()
      // install complete, restart app
      await relaunch()
    }
  } catch (error) {
    console.error('错误信息 ->', error)
  }
});
</script>

<style lang="less" scoped>
.v-main {
  height: 100vh;

  .left-side {
    background-color: @white;
    position: relative;
  }

  .right-container {
    height: 100vh;
    --el-main-padding: 0;
  }
}
</style>