<template>
    <el-button v-if="haveButton" size="small" type="primary" @click="handleCheckUpdate">检查更新</el-button>
    <el-dialog v-model="updateVisible" title="版本更新" width="40%" center :close-on-click-modal="false">
        <div class="font-size text-title">版本：<span class="font-weight-bold">v{{ updateInfo?.version }}</span></div>
        <div class="font-size text-title margin-top">发布时间：<span class="font-weight-bold">{{ updateInfo?.date }}</span>
        </div>
        <div class="font-size text-title margin-top">更新内容：<span class="font-weight-bold">{{ updateInfo?.body }}</span>
        </div>
        <template #footer>
            <span class="dialog-footer">
                <el-button @click="updateVisible = false">取消</el-button>
                <el-button type="primary" @click="updateAction">安装</el-button>
            </span>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { onMounted, ref, toRefs } from 'vue';
import { checkUpdate, installUpdate, UpdateManifest } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process';
import { listen } from '@tauri-apps/api/event'

const props = defineProps<{ haveButton: boolean, isAuto: boolean }>();
const { haveButton, isAuto } = toRefs(props);

const updateVisible = ref(false);
const updateInfo = ref<UpdateManifest | undefined>();

onMounted(async () => {
    console.log('是否自动更新 -> ', isAuto.value);
    if (isAuto.value) {
        await handleCheckUpdate();
    }
    listen('tauri://update-status', function (res) {
        console.log('New status: ', res)
    })
});

const handleCheckUpdate = async () => {
    // 检查更新
    try {
        const { shouldUpdate, manifest } = await checkUpdate();
        console.log('检查更新', shouldUpdate);
        if (shouldUpdate) {
            updateVisible.value = true;
            updateInfo.value = manifest;
            console.log('manifest ->', manifest);
            // // display dialog
            // await installUpdate()
            // // install complete, restart app
            // await relaunch()
        }
    } catch (error) {
        console.error('错误信息 ->', error)
    }
}
const updateAction = async () => {
    updateVisible.value = false;
    await installUpdate();
    await relaunch()
};
</script>

<style lang="less" scoped>
</style>