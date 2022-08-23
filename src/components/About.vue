<template>
    <MenuItem name="关于" @click="openAbout">
    <Attention theme="outline" fill="#333" :strokeWidth="3" />
    </MenuItem>
    <el-drawer custom-class="v-about" v-model="drawer" direction="rtl" :modal="true" size="450px" :show-close="false"
        :append-to-body="true">
        <template #header>
            <DrawerHeader title="关于" @close="closeAbout" />
        </template>
        <div class="about-container flex-column-center padding-all">
            <img src="../assets/appicon.png" width="80" height="80" alt="" />
            <div class="name font-size-28 margin-v font-weight-bold">{{ name }}</div>
            <div class="version font-size margin-bottom-sm">Version. v{{ version }} (v{{tauriVersion}})</div>
            <div class="copy-info font-size-sm">Copyright © 2022 {{ author }}. All rights reserved.</div>
            <Problem />
        </div>
    </el-drawer>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Attention } from '@icon-park/vue-next';
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';
const drawer = ref(false);

const name = ref('');
const version = ref('');
const tauriVersion = ref('');
const author = ref('康兵奎');

onMounted(async () => {
    name.value = await getName();
    version.value = await getVersion();
    tauriVersion.value = await getTauriVersion();
});

const openAbout = () => {
    drawer.value = true;
}
const closeAbout = () => {
    drawer.value = false;
}
</script>

<style lang="less">
.v-about {
    --el-drawer-padding-primary: 0;
    .el-drawer__header {
        margin-bottom: 0;
    }
    .about-container {
        .name {
            user-select: none;
            color: @black;
        }

        .version {
            color: @black;
        }

        .copy-info {
            color: @gray-dark;
        }
    }

    .problem-list {
        width: 100%;
    }
}
</style>