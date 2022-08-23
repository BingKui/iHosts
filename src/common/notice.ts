import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

/**
 * 发送系统通知
 * @param title 通知名称
 * @param message 通知信息
 */
export const Notice = async (title: string, message: string = '') => {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
        sendNotification({ title, body: message });
    }
}