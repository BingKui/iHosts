<template>
    <MenuItem name="设置" @click="openSetting">
        <SettingTwo theme="outline" fill="#333" :strokeWidth="3" />
    </MenuItem>
    <el-drawer custom-class="v-setting" v-model="drawer" direction="rtl" :modal="true" size="450px" :show-close="false"
        :append-to-body="true">
        <template #header>
            <DrawerHeader title="设置" @close="closeSetting" />
        </template>
        <div class="setting-container">
            <div class="setting-item">
                <div class="font-size">跟随系统启动</div>
                <div class="setting-content">
                    <el-switch :width="30" :value="auto_start" @change="handleStartAction" />
                </div>
            </div>
            <div class="setting-item">
                <div class="font-size">自动重启WiFi网络</div>
                <div class="setting-content">
                    <el-switch :width="30" :value="auto_restart_wifi" @change="handleWifiAction" />
                </div>
            </div>
            <div class="font-size-sm text-sub">可以有效解决网站Keep-Alive请求导致的host不生效问题。</div>
            <div class="setting-item">
                <div class="font-size">自动保存</div>
                <div class="setting-content">
                    <el-switch :width="30" :value="auto_save" @change="handleSaveAction" />
                </div>
            </div>
            <!-- <div class="setting-item">
                <div class="font-size">Dock显示(Mac)</div>
                <div class="setting-content">
                    <el-switch :width="30" :value="dock_show" @change="handleDockAction" />
                </div>
            </div> -->
            <div class="setting-item">
                <div class="font-size">自动检查更新</div>
                <div class="setting-content">
                    <el-switch :width="30" :value="auto_update" @change="handleUpdateAction" />
                </div>
            </div>
            <!-- <div class="setting-item">
                <div class="font-size">快捷键</div>
                <div class="setting-content text-sub font-size text-right">
                    <div class="short-item">Mac: Shift+Cmd+E</div>
                    <div class="short-item">Windows&Linux: Ctrl+Shift+E</div>
                </div>
            </div> -->
            <div class="setting-item">
                <div class="font-size">软件更新</div>
                <div class="setting-content">
                    <el-button size="small" type="primary" @click="handleCheckUpdate">检查更新</el-button>
                </div>
            </div>
        </div>
    </el-drawer>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { SettingTwo } from '@icon-park/vue-next';
import { useSystemStore } from '../stores/sys';
import { storeToRefs } from 'pinia';
import { emit } from '@tauri-apps/api/event';
const drawer = ref(false);
const sys = useSystemStore();
const { auto_start, auto_restart_wifi, auto_save, auto_update, dock_show } = storeToRefs(sys);

const openSetting = () => {
    drawer.value = true;
}
const closeSetting = () => {
    drawer.value = false;
}

const handleStartAction = (flag: boolean) => handleAction(flag, 'auto_start');
const handleWifiAction = (flag: boolean) => handleAction(flag, 'auto_restart_wifi');
const handleSaveAction = (flag: boolean) => handleAction(flag, 'auto_save');
const handleUpdateAction = (flag: boolean) => handleAction(flag, 'auto_update');
const handleDockAction = (flag: boolean) => handleAction(flag, 'dock_show');

const handleCheckUpdate = () => {
    emit('update-action');
};

const handleAction = async (flag: boolean, settingKey: string) => {
    const info = {
        auto_start: auto_start.value,
        auto_restart_wifi: auto_restart_wifi.value,
        auto_save: auto_save.value,
        auto_update: auto_update.value,
        dock_show: dock_show.value,
        [settingKey]: flag,
    };
    console.log('修改设置 ->', info);
    await sys.setSysSetting(info);
}
</script>

<style lang="less">
.v-setting {
    --el-drawer-padding-primary: 0;
    .el-drawer__header {
        margin-bottom: 0;
    }
    .setting-container {
        padding: @gap-md;
    }
    .setting-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        height: 50px;
    }
}
</style>