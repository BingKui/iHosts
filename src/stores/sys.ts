import { HostItem } from './../common/types';
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { os } from '@tauri-apps/api';
import { defineStore } from 'pinia';
import { v4 as uuid } from 'uuid';
import { Command } from '@tauri-apps/api/shell';

export type SysState = {
    auto_start: boolean;
    auto_restart_wifi: boolean;
    auto_save: boolean;
    auto_update: boolean;
    dock_show: boolean;
};

export const useSystemStore = defineStore('sys', {
    state: () => <SysState>({
        auto_start: false,
        auto_restart_wifi: false,
        auto_save: false,
        auto_update: false,
        dock_show: false,
    }),
    getters: {},
    actions: {
        // 获取目前的host
        async getSysSetting() {
            const setting: SysState = await invoke('get_setting');
            console.log('获取的设置为 ->', setting);
            this.$state = { ...setting };
        },
        // 修改当前系统host
        async setSysSetting(info: SysState) {
            await invoke('set_setting', { setting: info })
            await this.getSysSetting();
            await emit('setting-change');
        },
        async restartWifi() {
            const type = await os.type();
            let stopCmd: Command;
            let startCmd: Command | null;
            switch(type) {
                case 'Darwin':
                    stopCmd = new Command('networksetup', ['-setnetworkserviceenabled', 'Wi-Fi', 'off']);
                    startCmd = new Command('networksetup', ['-setnetworkserviceenabled', 'Wi-Fi', 'on']);
                    break;
                case 'Linux':
                    stopCmd = new Command('service', ['network', 'restart']);
                    startCmd = null;
                    break;
                case 'Windows_NT':
                    stopCmd = new Command('netsh', ['wlan', 'stop']);
                    startCmd = new Command('netsh', ['wlan', 'start']);
                    break;
            }
            stopCmd.on('close', data => {
                console.log('已关闭，重新开启！');
                startCmd && startCmd.execute();
            });
            stopCmd.execute();
        },
    },
});