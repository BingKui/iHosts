<template>
    <div class="v-drawer-header padding-h-md">
        <div class="font-size-md text-title font-weight-bold">{{ title }}</div>
        <div class="header-close" @click="closeAction">
            <Close theme="outline" size="20" fill="#333" :strokeWidth="2"/>
        </div>
    </div>
</template>

<script setup lang="ts">
import { toRefs } from 'vue';
import { Close } from '@icon-park/vue-next';
const props = defineProps<{ title: string; }>();
const { title } = toRefs(props);
const emit = defineEmits(['close']);
const closeAction = () => {
    emit('close');
}
</script>

<style lang="less" scoped>
.v-drawer-header {
    height: @headerHeight;
    border-bottom: 1px solid rgba(0, 0, 0, 0.08);
    display: flex;
    align-items: center;
    justify-content: space-between;
    .header-close {
        cursor: pointer;
    }
}
</style>