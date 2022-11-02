<template>
    <div class="v-logo flex-column-center" data-tauri-drag-region>
        <div class="logo" data-tauri-drag-region :style="`background-image: url(${logoSrc});`" />
        <div class="name" data-tauri-drag-region>iHost</div>
    </div>
</template>

<script setup lang="ts">
import logoSrc from '../assets/logo.png';
</script>

<style lang="less" scoped>
.v-logo {
    user-select: none;
    height: @asideWidth;
    .logo {
        background-repeat: no-repeat;
        background-size: 60px 60px;
        width: 60px;
        height: 60px;
        cursor: default;
    }
    .name {
        user-select: none;
        font-size: 14px;
        font-weight: bold;
        margin-top: @gap;
        cursor: default;
    }
}
</style>