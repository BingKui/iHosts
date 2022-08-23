import { HostItem } from "./types";

/**
 * 处理多个数据为一个
 * @param {Array} list
 */
export const dealHostContent = (list: HostItem[]) => {
    const header = (name: string) => {
        return `### iHost: ${name} - 开始(start) ###`;
    };
    const footer = (name: string) => {
        return `### iHost: ${name} - 结束(end) ###`;
    };
    let content = '';
    for (let i = 0; i < list.length; i++) {
        const item = list[i];
        if (!item.used) continue;
        content += `${header(item.name)}
${item.content}
${footer(item.name)}`;
    }
    return content;
};
