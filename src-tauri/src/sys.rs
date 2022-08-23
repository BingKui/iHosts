use fs_err as fs;

use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::AppHandle;

#[derive(Serialize, Deserialize)]
pub struct SysInfo {
    pub auto_start: bool,
    pub auto_restart_wifi: bool,
    pub auto_save: bool,
    pub auto_update: bool,
    pub dock_show: bool,
}

pub struct SysApp {
    pub path: String,
}

impl SysApp {
    pub fn new(app_handle: &AppHandle) -> SysApp {
        let res_dir = app_handle.path_resolver().app_dir().unwrap();
        let db_path = res_dir.join("sys.json");
        let is_folder = Path::new(res_dir.as_os_str()).exists();
        if !is_folder {
            fs::create_dir(res_dir.as_os_str().to_str().unwrap().to_string()).unwrap();
        }
        let is_exist = Path::new(&db_path).exists();
        if !is_exist {
            let info: SysInfo = SysInfo {
                auto_start: false,
                auto_restart_wifi: false,
                auto_save: false,
                auto_update: false,
                dock_show: false,
            };
            let serialized = serde_json::to_string(&info).unwrap();
            println!("处理结果为 -> {}", serialized);
            // 创建文件并初始化
            fs::write(db_path.as_os_str().to_str().unwrap().to_string(), serialized).unwrap();
        }
        println!("文件是否存在 -> {:?}", Path::new(&db_path).exists());
        let path = db_path.as_os_str().to_str().unwrap().to_string();
        println!("设置的存储位置为 -> {}", path);
        SysApp {
            path,
        }
    }
    pub fn get_sys(&self) -> SysInfo {
        let contents = fs::read_to_string(self.path.to_string()).unwrap();
        let setting: SysInfo = serde_json::from_str(&contents).unwrap();
        setting
    }
    pub fn set_sys(&self, setting: SysInfo) -> bool {
        let content = serde_json::to_string(&setting).unwrap();
        fs::write(self.path.to_string(), content).unwrap();
        true
    }
}
