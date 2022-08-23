#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod host;
mod sys;

use std::path::Path;

use auto_launch::AutoLaunch;
use fs_err as fs;
use host::{HostApp, HostItem};
use sys::{SysApp, SysInfo};
use tauri::{
  AppHandle, CustomMenuItem, Manager, Menu, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
  SystemTrayMenuItem, WindowEvent,
};

#[derive(Clone, serde::Serialize)]
struct Payload {
  id: String,
  used: bool,
}

fn main() {
  // 解决路径问题，No such file or directory
  fix_path_env::fix().unwrap();
  // 内容
  let context = tauri::generate_context!();
  // 系统托盘
  let tray_menu = SystemTrayMenu::new();
  let system_tray = SystemTray::new().with_menu(tray_menu);
  let mut app = tauri::Builder::default()
    .menu(Menu::os_default("iHosts"))
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "open" => {
          let main_win = app.get_window("main").unwrap();
          main_win.set_focus().unwrap();
          main_win.show().unwrap();
        }
        "exit" => {
          std::process::exit(0);
        }
        _ => {
          let host_app = HostApp::new(app).unwrap();
          let host = host_app.get_item(id).unwrap();
          app
            .emit_all(
              "change-used",
              Payload {
                id: host.id.to_string(),
                used: host.used,
              },
            )
            .unwrap();
        }
      },
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      get_current_host,
      set_current_host,
      get_back_host,
      get_setting,
      set_setting,
      add_host,
      del_host,
      change_name,
      change_content,
      change_sort,
      change_used,
      get_host_list,
      reset_tray,
    ])
    .build(context)
    .expect("iHosts 运行失败！");
  // 处理系统托盘
  deal_tray(&app.handle());
  // 处理设置
  deal_setting(&app.handle());
  // 备份host
  back_host(&app.handle());
  // 设置为无dock
  #[cfg(target_os = "macos")]
  app.set_activation_policy(tauri::ActivationPolicy::Accessory);
  // 设置关闭保留系统托盘
  app.run(|app_handle, e| match e {
    // 禁用原生关闭事件，改为隐藏
    RunEvent::WindowEvent {
      label,
      event: WindowEvent::CloseRequested { api, .. },
      ..
    } => {
      if label == "main" {
        let app_handle = app_handle.clone();
        let window = app_handle.get_window(&label).unwrap();
        api.prevent_close();
        window.hide().unwrap();
      }
    }
    RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  });
}

fn deal_setting(app_handle: &AppHandle) {
  let sys_app = SysApp::new(&app_handle);
  let sys_setting = sys_app.get_sys();
  let app_name = app_handle.package_info().name.to_string();
  let mut target_path = "/Applications/".to_string();
  target_path.push_str(&app_name);
  target_path.push_str(".app");
  println!("app 名字为 -> {:?}", app_name);
  println!("app 地址为 -> {:?}", target_path);
  #[cfg(target_os = "macos")]
  let auto = AutoLaunch::new(&app_name, &target_path, true, true);
  if sys_setting.auto_start {
    // enable the auto launch
    if auto.enable().is_ok() {
      auto.is_enabled().unwrap();
    }
  } else {
    // disable the auto launch
    if auto.disable().is_ok() {
      auto.is_enabled().unwrap();
    }
  }
}

fn back_host(app_handle: &AppHandle) {
  let res_dir = app_handle.path_resolver().app_dir().unwrap();
  let back_path = res_dir.join("hosts_back");
  let is_folder = Path::new(res_dir.as_os_str()).exists();
  if !is_folder {
    fs::create_dir(res_dir.as_os_str().to_str().unwrap().to_string()).unwrap();
  }
  let is_exist = Path::new(back_path.as_os_str()).exists();
  if !is_exist {
    let contents = fs::read_to_string("/etc/hosts").unwrap();
    fs::write(
      back_path.as_os_str().to_str().unwrap().to_string(),
      contents,
    )
    .unwrap();
  }
}

fn deal_tray(app_handle: &AppHandle) {
  let tray = app_handle.tray_handle();
  let app_name = app_handle.package_info().name.to_string();
  let mut open_title = "打开 ".to_string();
  open_title.push_str(&app_name);
  let mut exit_title = "打开 ".to_string();
  exit_title.push_str(&app_name);
  let open = CustomMenuItem::new("open".to_string(), open_title);
  let list_title = CustomMenuItem::new("list_title".to_string(), "Host 列表").disabled();
  let exit = CustomMenuItem::new("exit".to_string(), exit_title);
  let mut tray_menu = SystemTrayMenu::new();
  tray_menu = tray_menu.add_item(open);
  tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
  tray_menu = tray_menu.add_item(list_title);
  // 获取所有列表，循环添加
  let host_app = HostApp::new(&app_handle).unwrap();
  let host_list = host_app.get_list().unwrap();
  for host in host_list {
    let mut item = CustomMenuItem::new(host.id.to_string(), host.name.to_string());
    if host.used {
      item = item.selected();
    }
    tray_menu = tray_menu.add_item(item);
  }
  tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
  tray_menu = tray_menu.add_item(exit);
  tray.set_menu(tray_menu).unwrap();
}

// 获取当前的系统host内容
#[tauri::command]
fn reset_tray(app_handle: AppHandle) -> bool {
  println!("更新 tray 菜单");
  deal_tray(&app_handle);
  true
}
// 获取当前的系统host内容
#[tauri::command]
fn get_current_host() -> String {
  let contents = fs::read_to_string("/etc/hosts").unwrap();
  contents
}

// 设置当前系统的host内容
#[tauri::command]
fn set_current_host(content: String) -> bool {
  fs::write("/etc/hosts", content).unwrap();
  true
}

// 获取当前的系统host内容
#[tauri::command]
fn get_back_host(app_handle: AppHandle) -> String {
  let res_dir = app_handle.path_resolver().app_dir().unwrap();
  let back_path = res_dir.join("hosts_back");
  let contents = fs::read_to_string(back_path).unwrap();
  contents
}

// // 设置当前系统的host内容
// #[tauri::command]
// fn set_back_host(content: String) -> bool {
//   fs::write("/etc/hosts_back", content).unwrap();
//   true
// }

// 系统设置相关
#[tauri::command]
fn get_setting(app_handle: AppHandle) -> SysInfo {
  let sys_app = SysApp::new(&app_handle);
  let setting = sys_app.get_sys();
  setting
}
#[tauri::command]
fn set_setting(app_handle: AppHandle, setting: SysInfo) -> bool {
  let sys_app = SysApp::new(&app_handle);
  let result = sys_app.set_sys(setting);
  deal_setting(&app_handle);
  result
}

// host 相关操作
// 增加
#[tauri::command]
fn add_host(app_handle: AppHandle, item: HostItem) -> bool {
  let host_app = HostApp::new(&app_handle).unwrap();
  let result = host_app.add_item(item);
  result
}
// 删除
#[tauri::command]
fn del_host(app_handle: AppHandle, id: String) -> bool {
  let host_app = HostApp::new(&app_handle).unwrap();
  let result = host_app.del_item(id);
  result
}
// 修改名字
#[tauri::command]
fn change_name(app_handle: AppHandle, id: String, name: String) -> bool {
  let host_app = HostApp::new(&app_handle).unwrap();
  let result = host_app.update_name(id, name);
  result
}
// 修改内容
#[tauri::command]
fn change_content(app_handle: AppHandle, id: String, content: String) -> bool {
  let host_app = HostApp::new(&app_handle).unwrap();
  let result = host_app.update_content(id, content);
  result
}
// 排序
#[tauri::command]
fn change_sort(app_handle: AppHandle, id: String, sort: i32, is_add: bool) -> bool {
  let host_app = HostApp::new(&app_handle).unwrap();
  let result = host_app.update_sort(id, sort, is_add);
  result
}
// 启用、停用
#[tauri::command]
fn change_used(app_handle: AppHandle, id: String, used: bool) -> bool {
  let host_app = HostApp::new(&app_handle).unwrap();
  let result = host_app.update_used(id, used);
  result
}
// 获取所有的数据
#[tauri::command]
fn get_host_list(app_handle: AppHandle) -> Vec<HostItem> {
  let host_app = HostApp::new(&app_handle).unwrap();
  let result = host_app.get_list().unwrap();
  result
}
