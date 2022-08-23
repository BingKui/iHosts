<template>
    <div class="v-host-list">
        <PerfectScrollbar class="list-container">
            <HostListItem :item="currentItem" :canEdit="false" :active="activeId == 'current'" @active="selectItem" />
            <Container
                class="drag-container"
                orientation="vertical"
                group-name="col-items"
                @drop="handleDrop($event)"
            >
                <Draggable class="drag-item" v-for="item in hostList" :key="item.id">
                    <HostListItem :item="item" :canEdit="true" :active="activeId == item.id" @active="selectItem" />
                </Draggable>
            </Container>
        </PerfectScrollbar>
    </div>
</template>

<script setup lang="ts">
import { inject, onMounted, Ref, ref } from 'vue';
import { storeToRefs } from 'pinia';
import { PerfectScrollbar } from 'vue3-perfect-scrollbar';
import { Container, Draggable } from 'vue3-smooth-dnd';
import { useHostStore } from '../stores/host';
import { HostItem } from '../common/types';

const host = useHostStore();
const { hostList } = storeToRefs(host);

const currentItem = {
    name: '当前系统Hosts',
    id: 'current',
};

const activeId = ref('current');

onMounted(async () => {
    // 获取当前内容
    await host.getCurrentHost();
    // 获取现在所有的列表
    await host.getHostList();
    // 获取备份
    await host.getBackHost();
    await host.setCurrentFlag(activeId.value === 'current');
});

const selectItem = async (item: HostItem) => {
    activeId.value = item.id;
    host.setHostInfo(item);
    // 更新右侧内容
    const isCurrent = item.id == 'current';
    host.setCurrentFlag(isCurrent);
    if (isCurrent) {
        await host.getCurrentHost();
    }
};

const handleDrop = (dropResult: any) => {
    console.log(dropResult);
}

const handleChangeName = () => {
    
}

</script>

<style lang="less">
.v-host-list {
    width: @asideWidth;
    padding-bottom: 50px;
    height: calc(100vh - @asideWidth);

    .list-container {
        height: calc(100vh - @asideWidth);
    }
}
</style>