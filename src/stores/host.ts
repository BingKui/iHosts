import { HostItem } from './../common/types';
import { invoke } from '@tauri-apps/api';
import { defineStore } from 'pinia';
import { v4 as uuid } from 'uuid';
import { useSystemStore } from './sys';
import { dealHostContent } from '../common/utils';
import { Notice } from '../common/notice';

type HostState =  {
    isCurrent: boolean;
    currentContent: string;
    backHostContent: string;
    hostList: HostItem[];
    hostInfo: HostItem | null;
    hostContent: string | null;
};

export const useHostStore = defineStore('host', {
    state: () => <HostState>({
        isCurrent: false,
        currentContent: '',
        backHostContent: '',
        hostList: [],
        hostInfo: null,
        hostContent: null,
    }),
    getters: {},
    actions: {
        setCurrentFlag(flag: boolean) {
            this.isCurrent = flag;
        },
        // 获取目前的 host
        async getCurrentHost() {
            const content: string = await invoke('get_current_host');
            this.currentContent = content;
        },
        // 获取备份的 host
        async getBackHost() {
            const content: string = await invoke('get_back_host');
            this.backHostContent = content;
        },
        // 修改当前系统host
        async setCurrentHost() {
            const { backHostContent, hostList } = this;
            const targetContent = dealHostContent(hostList);
            const content = `${backHostContent}
${targetContent}`;
            await invoke('set_current_host', { content });
            await this.dealSetting();
            Notice('修改成功', '修改系统 Host 成功！');
            this.getCurrentHost();
        },
        // 还原Host
        async restoreCurrentHost() {
            // 获取是否自动重启WiFi
            await invoke('set_current_host', { content: this.backHostContent });
            await this.dealSetting();
            Notice('还原成功', '系统 Host 还原成功！');
            this.getCurrentHost();
        },
        async dealSetting() {
            const appSetting = useSystemStore();
            const { auto_restart_wifi } = appSetting;
            if (auto_restart_wifi) {
                await appSetting.restartWifi();
                Notice('重启WiFi', '重启系统 WiFi 成功！');
            }
        },
        // 设置选中的host
        setHostInfo(item: HostItem) {
            this.hostInfo = item;
        },
        // 获取host列表
        async getHostList() {
            const list: HostItem[] = await invoke('get_host_list');
            this.hostList = list;
            this.hostInfo && this.setHostInfo(list.filter(item => item.id === this.hostInfo?.id)[0]);
        },
        // 添加host
        async addHostItem() {
            const item: HostItem = {
                id: uuid().replaceAll('-', ''),
                name: 'iHost New',
                content: '',
                sort: this.hostList.length + 1,
                used: false,
            };
            await invoke('add_host', { item });
            await this.getHostList();
            await this.updateTray();
        },
        // 删除host
        async delHostItem(id: string, used: boolean) {
            await invoke('del_host', { id });
            await this.getHostList();
            used && await this.setCurrentHost();
            await this.updateTray();
        },
        // 修改内容
        async updateHostContent(content: string) {
            await invoke('change_content', { id: this.hostInfo?.id, content });
            await this.getHostList();
            await this.setCurrentHost();
            await this.updateTray();
        },
        // 修改状态
        async updateHostUsed(id: string, used: boolean) {
            console.log('修改状态 -> ', id, used);
            await invoke('change_used', { id, used });
            await this.getHostList();
            await this.setCurrentHost();
            await this.updateTray();
        },
        // 修改名称
        async updateHostName(id: string, name: string) {
            await invoke('change_name', { id, name });
            await this.getHostList();
            await this.updateTray();
        },
        // 修改排序
        async updateHostSort(id: string, sort: number, isAdd: boolean) {
            await invoke('change_sort', { id, sort, is_add: isAdd });
            await this.getHostList();
            await this.updateTray();
        },
        // 获取单条host内容
        async getHostItemContent() {},
        // 更新tray信息
        async updateTray() {
            await invoke('reset_tray');
        },
    },
});