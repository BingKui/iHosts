<template>
    <div :class="`v-host-list-item ${active ? 'active' : ''}`" @click="handleActive">
        <el-input v-if="isEdit" ref="edit" size="small" class="item-input font-size" placeholder="输入名称" v-model="inputValue"
            :maxlength="15" @input="handleInput" @change="handleChange" @blur="handleBlur" />
        <div v-else @contextmenu="openContextMenu" class="item-content flex-row-between">
            <div class="font-size flex-item-one">{{ item.name }}</div>
            <div v-if="canEdit" class="item-switch">
                <el-switch :width="30" v-model="item.used" @change="handleChagneUsed" @click.stop />
            </div>
        </div>
        <context-menu :name="item.id">
            <context-menu-item @click="handleChangeName">修改名称</context-menu-item>
            <context-menu-item @click="handleDelete">删除</context-menu-item>
        </context-menu>
    </div>
</template>

<script setup lang="ts">
import { toRefs, ref, inject } from 'vue';
import { HostItem } from '../common/types';
import { useHostStore } from '../stores/host';

const props = defineProps<{ active: boolean; item: HostItem; canEdit: boolean; }>();
const { active, item, canEdit } = toRefs(props);
const edit = ref(null);
const isEdit = ref(false);
const inputValue = ref('');
const emit = defineEmits(['active']);
const host = useHostStore();

const emitContext = inject('emitContext') as (event: Event, dataId: Record<string, unknown>) => void

const openContextMenu = (e: any) => {
    emitContext(e, { name: item.value.id, id: [1, 2, 3] })
}

const handleActive = () => {
    emit('active', item.value);
};

const handleInput = (value: string) => {
    console.log('输入变化', value);
    inputValue.value = value;
};
const handleChange = async (value: string) => {
    isEdit.value = false;
    if (!value) return;
    // 修改名称
    await host.updateHostName(item.value.id, inputValue.value);
};
const handleBlur = () => {
    isEdit.value = false;
};

const handleChagneUsed = async (used: boolean) => {
    await host.updateHostUsed(item.value.id, used);
}

const handleChangeName = () => {
    isEdit.value = true;
    inputValue.value = item.value.name;
}
const handleDelete = async () => {
    await host.delHostItem(item.value.id, item.value.used);
}
</script>

<style lang="less" scoped>
.v-host-list-item {
    display: flex;
    height: 40px;
    width: 100%;
    cursor: pointer;
    color: @title-color;
    &.active {
        color: @primary;
        background-color: rgba(45, 140, 240, .1);
    }
    .item-content {
        user-select: none;
        height: 40px;
        // width: calc(100% - 20px);
        width: 100%;
        padding: 0 10px;
        // .flex-row-between();
        align-items: center;
    }
}
</style>