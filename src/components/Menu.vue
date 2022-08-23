<template>
    <div class="v-menu margin-h margin-bottom">
        <el-row class="menu-container">
            <el-col :span="6">
                <MenuItem name="添加" @click="handleAddItem">
                    <AddOne theme="outline" fill="#333" :strokeWidth="3"/>
                </MenuItem>
            </el-col>
            <el-col :span="6">
                <MenuItem name="还原" @click="handleRestore">
                    <Undo theme="outline" fill="#333" :strokeWidth="3"/>
                </MenuItem>
            </el-col>
            <el-col :span="6">
                <Setting />
            </el-col>
            <el-col :span="6">
                <About />
            </el-col>
        </el-row>
    </div>
</template>

<script setup lang="ts">
import { AddOne, Undo } from '@icon-park/vue-next';
import { useHostStore } from '../stores/host';

const host = useHostStore();

const handleAddItem = async () => {
    await host.addHostItem();
};

const handleRestore = async () => {
    await host.restoreCurrentHost();

};
</script>

<style lang="less" scoped>
.v-menu {
    background-color: #FFFFFF;
    z-index: 10;
    padding: @gap-sm;
    // border-radius: 8px;
    position: absolute;
    bottom: 0;
    right: 0;
    left: 0;
    box-shadow: 0 0 8px rgba(0, 0, 0, .2);
    .menu-container {
        width: 100%;
    }
}
// @media (prefers-color-scheme: dark) {
//     .v-menu {
//         background-color: @dark-mode-item-background;
//         box-shadow: 0 0 8px rgba(255, 255, 255, .1);
//     }
// }
</style>