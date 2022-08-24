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
  <Updater :isAuto="auto_update" :haveButton="false" />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useSystemStore } from './stores/sys';
import hotkeys from "hotkeys-js";
import { os } from '@tauri-apps/api';
import { WebviewWindow } from '@tauri-apps/api/window';
import { emit, listen } from '@tauri-apps/api/event'
import { useHostStore } from './stores/host';
import { storeToRefs } from 'pinia';
const appSetting = useSystemStore();
const { auto_update } = storeToRefs(appSetting);

onMounted(async () => {
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