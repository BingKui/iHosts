<template>
    <div class="v-mouse-right">
        <div class="mouse-content" v-contextmenu:contextmenu>
            <slot></slot>
        </div>
        <Contextmenu v-if="mouseData && mouseData.length > 0" ref="contextmenu">
            <ContextmenuItem v-for="(item, i) in mouseData" :key="i" @click="() => item.action(mouseKey)">{{ item.text
            }}
            </ContextmenuItem>
        </Contextmenu>
    </div>
</template>

<script setup lang="ts">
import { toRefs } from 'vue';
// 基础版右键菜单封装，支持单层右键菜单，不可分组，不存在层级
import { Contextmenu, ContextmenuItem } from 'v-contextmenu';
type MouseProps = {
    mouseData: { text: string, action: (mouseKey: any) => {} }[];
    mouseKey: String | Number | Object; // 鼠标事件返回需要的数据，选中相应操作都会把相应的数据返回
};
const props = defineProps<MouseProps>();
const { mouseData, mouseKey } = toRefs(props);
</script>

<style lang="less" scoped>
.v-contextmenu {
    padding: 0;
    border: none;
    overflow: hidden;

    .v-contextmenu-divider {
        margin: 0;
    }

    .v-contextmenu-item {
        min-width: 50px;
        padding: @gap @gap-md;
    }
}
</style>