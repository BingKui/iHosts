<template>
    <div class="v-menu-item flex-column-center" @click="handleClick" :title="name">
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
import { toRefs } from "vue-demi";

const props = defineProps<{ name: string }>();
const { name } = toRefs(props);
const emit = defineEmits(['click']);
const handleClick = () => {
    emit('click');
}
</script>

<style lang="less">
.v-menu-item {
    background-color: #FFFFFF;
    height: 40px;
    width: 40px;
    font-size: 25px;
    cursor: pointer;
    .i-icon {
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    &:hover {
        color: @primary;
    }
}
// @media (prefers-color-scheme: dark) {
//     .v-menu-item {
//         color: @white;
//         background-color: @dark-mode-item-background;
//     }
// }
</style>
