use std::path::Path;
use fs_err as fs;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize, Deserialize)]
pub struct HostItem {
    pub id: String,
    pub name: String,
    pub content: String,
    pub sort: i32,
    pub used: bool,
}

pub struct HostApp {
    pub conn: Connection,
}

impl HostApp {
    pub fn new(app_handle: &AppHandle) -> Result<HostApp> {
        let res_dir = app_handle.path_resolver().app_dir().unwrap();
        let db_path = res_dir.join("ihost.sqlite");
        let is_folder = Path::new(res_dir.as_os_str()).exists();
        if !is_folder {
            fs::create_dir(res_dir.as_os_str().to_str().unwrap().to_string()).unwrap();
        }
        let is_exist = Path::new(&db_path).exists();
        if !is_exist {
            // 创建文件并初始化
            fs::write(db_path.as_os_str().to_str().unwrap().to_string(), "").unwrap();
        }
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS HostList (
                id varchar(64) PRIMARY KEY,
                name text DEFAULT '',
                content text DEFAULT '',
                sort numeric DEFAULT 0,
                used numeric DEFAULT 0
            )",
            [],
        )?;
        Ok(HostApp { conn })
    }
    pub fn add_item(&self, item: HostItem) -> bool {
        let HostItem {
            id,
            name,
            content,
            sort,
            ..
        } = item;
        match self.conn.execute(
            "INSERT INTO HostList (id, name, content, sort) VALUES (?, ?, ?, ?)",
            [id, name, content, sort.to_string()],
        ) {
            Ok(insert) => {
                println!("{} row inserted", insert);
                true
            }
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn del_item(&self, id: String) -> bool {
        match self.conn.execute("DELETE FROM HostList WHERE id = ?", [id]) {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn update_name(&self, id: String, name: String) -> bool {
        match self
            .conn
            .execute("UPDATE HostList SET name = ? WHERE id = ?", [name, id])
        {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn update_content(&self, id: String, content: String) -> bool {
        match self.conn.execute(
            "UPDATE HostList SET content = ? WHERE id = ?",
            [content, id],
        ) {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn update_item_sort(&self, id: String, sort: i32) -> bool {
        match self.conn.execute(
            "UPDATE HostList SET sort = ? WHERE id = ?",
            [sort.to_string(), id],
        ) {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn update_sort(&self, id: String, sort: i32, is_add: bool) -> bool {
        // 查询需要修改索引的值
        let item = self.get_item(id).unwrap();
        let list = self.get_rang_list(item.sort, sort).unwrap();
        for host in list {
            if is_add {
                self.update_item_sort(host.id, host.sort + 1);
            } else {
                self.update_item_sort(host.id, host.sort - 1);
            }
        }
        match self.conn.execute(
            "UPDATE HostList SET sort = ? WHERE id = ?",
            [sort.to_string(), item.id],
        ) {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn update_used(&self, id: String, used: bool) -> bool {
        let used_value: i32;
        if used {
            used_value = 1
        } else {
            used_value = 0
        };
        match self.conn.execute(
            "UPDATE HostList SET used = ? WHERE id = ?",
            [used_value.to_string(), id],
        ) {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn get_item(&self, id: String) -> Result<HostItem> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM HostList WHERE id = ?")
            .unwrap();
        let item = stmt.query_row([id], |row| {
            let used = row.get::<usize, i32>(4).unwrap() == 1;
            Ok(HostItem {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                sort: row.get(3)?,
                used: used,
            })
        })?;
        Ok(item)
    }
    pub fn get_list(&self) -> Result<Vec<HostItem>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM HostList")
            .unwrap();
        let hosts_iter = stmt.query_map([], |row| {
            let used = row.get::<usize, i32>(4).unwrap() == 1;

            Ok(HostItem {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                sort: row.get(3)?,
                used: used,
            })
        })?;
        let mut hosts: Vec<HostItem> = Vec::new();
        for host in hosts_iter {
            hosts.push(host?);
        }
        Ok(hosts)
    }
    // 获取某一段的记录
    pub fn get_rang_list(&self, start_sort: i32, end_sort: i32) -> Result<Vec<HostItem>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM HostList WHERE ? < sort <= ?")
            .unwrap();
        let hosts_iter = stmt.query_map([start_sort.to_string(), end_sort.to_string()], |row| {
            let used = row.get::<usize, i32>(4).unwrap() == 1;

            Ok(HostItem {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                sort: row.get(3)?,
                used: used,
            })
        })?;
        let mut hosts: Vec<HostItem> = Vec::new();
        for host in hosts_iter {
            hosts.push(host?);
        }
        Ok(hosts)
    }
}
