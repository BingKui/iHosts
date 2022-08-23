
export const PROBLEM_LIST = [{
    title: 'Mac 无权修改 hosts 文件',
    steps: [{
        name: '1.打开终端',
        values: [{
            text: '输入下面命令',
            type: 'text',
        }, {
            text: '> sudo /bin/chmod +a \'user:用户名:allow write\' /etc/hosts',
            type: 'shell',
        }, {
            text: '其中用户名，需要输入电脑的用户名。',
            type: 'text',
        }],
    }, {
        name: '2.输入密码，确认。',
        values: [{
            text: '用户即可获取 hosts 文件的修改权限。',
            type: 'text',
        }],
    }],
}, {
    title: '修改Host，没有生效',
    steps: [{
        name: '1.打开iHost设置',
        values: [{
            text: '勾选自动重启WiFi功能。',
            type: 'text',
        }, {
            text: '即可每次修改后立即生效。',
            type: 'text',
        }],
    }, {
        name: '2.使用浏览器隐私模式访问网站',
        values: [{
            text: '由于 Http 2.0 的keep-alive特性，可以通过隐私规避已打开的网站的影响。',
            type: 'text',
        }],
    }],
}];
