<template>
    <div class="v-content">
        <Codemirror
            class="content-mirror"
            v-model="content"
            :disabled="isCurrent"
            placeholder="输入host"
            :style="{ height: '100vh' }"
            :autofocus="true"
            :indent-with-tab="true"
            :tab-size="4"
            :extensions="[basicSetup, python(), search(), keyMap]"
            @change="handleContentChange"
        />
        <!-- @blur="updateAction" -->
    </div>
</template>

<script setup lang="ts">
import { onMounted, Ref, ref, watch, getCurrentInstance } from 'vue';
import { Codemirror } from 'vue-codemirror';
import { basicSetup } from 'codemirror'
import { EditorView, keymap } from '@codemirror/view'
import { python } from '@codemirror/lang-python';
import { search } from '@codemirror/search';
import { useHostStore } from '../stores/host';
import { useSystemStore } from '../stores/sys';
import { storeToRefs } from 'pinia';
import { debounce } from 'lodash';
import { ElNotification } from 'element-plus';
const content: Ref<string> = ref<string>('');
const host = useHostStore();
const setting = useSystemStore();
const { isCurrent, currentContent, hostInfo } = storeToRefs(host);

const keyMap = keymap.of([{
    key: 'Cmd-s',
    mac: 'Cmd-s',
    win: 'Ctrl-s',
    linux: 'Ctrl-s',
    run: (target: EditorView) => {
        updateAction();
        return true;
    },
}]);

watch([isCurrent, currentContent, hostInfo], ([val, ...ext], prevVal) => {
    console.log('监听变化', val, currentContent.value);
    console.log('监听变化 hostInfo', hostInfo);
    content.value = val ? currentContent.value : hostInfo.value?.content || '';
});

onMounted(async () => {
    console.log('onMounted 监听变化', isCurrent.value, currentContent.value);
    content.value = isCurrent.value ? currentContent.value : hostInfo.value?.content || '';
});

const updateAction = async () => {
    await host.updateHostContent(content.value);
    ElNotification.success({
        title: '保存成功',
        message: 'host 内容保存成功',
    })
}

const handleContentChange = debounce(() => {
    console.log('更新内容 ->', content.value);
    if (setting.auto_save) {
        updateAction()
    }
}, 500);



</script>

<style lang="less">
.v-content {
    width: 100%;
    height: 100vh;

    .content-mirror {
        height: 100vh;
    }

    .cm-gutters {
        display: none;
    }

    .cm-content {
        padding: 0 @gap;
        padding-top: @gap;
    }

    .cm-scroller {

        /*整个滚动条*/
        &::-webkit-scrollbar {
            width: 5px;
            height: 8px;
            background-color: @white;
        }

        /*定义滚动条轨道*/
        &::-webkit-scrollbar-track {
            background-color: @white;
        }

        /*定义滑块*/
        &::-webkit-scrollbar-thumb {
            background-color: @gray;
            border-radius: 0;
        }
    }
}
</style>