<template>
    <div class="v-problem">
        <div class="font-size-lg font-weight-bold margin-v text-warning">常见问题</div>
        <div class="problem-item" v-for="item in PROBLEM_LIST" :key="item.title">
            <div class="font-size-md font-weight-bold margin-v">{{ item.title }}</div>
            <div class="margin-bottom-md" v-for="temp in item.steps" :key="temp.name">
                <div class="font-size-sm">{{ temp.name }}</div>
                <div class="step-value-item" v-for="value in temp.values" :key="value.text">
                    <div class="text-content font-size-sm margin-v-sm" v-if="value.type == 'text'">{{ value.text }}</div>
                    <div class="shell-content font-size-sm padding-all-sm margin-v-sm border-radius text-white" v-if="value.type == 'shell'">{{ value.text }}</div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { PROBLEM_LIST } from '../constants/common';

</script>

<style lang="less" scoped>
.v-problem {
    background-color: @white;
    .shell-content {
        background-color: @black;
    }
}
</style>