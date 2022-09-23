use tauri::{self};
use crate::db::{SqliteDB,Record};


#[tauri::command]
pub fn clear_data()->bool{
    match SqliteDB::new().clear_data(){
        Ok(())=>{
            true
        },
        Err(e)=>{
            println!("{}",format!("清除失败:{}",e));
            false
        }
    }
}

#[tauri::command]
pub fn insert_record(r:Record)->bool{
    match SqliteDB::new().insert_record(r){
        Ok(_i)=>{true},
        Err(e)=>{
            println!("err:{}",e);
            false
        }
    }
}
#[tauri::command]
pub fn insert_if_not_exist(r:Record)->bool{
    match SqliteDB::new().insert_if_not_exist(r){
        Ok(_i)=>{true},
        Err(e)=>{
            println!("err:{}",e);
            false
        }
    }
}

#[tauri::command]
pub fn batch_get_record()->Vec<Record>{
    SqliteDB::new().batch_get_record().unwrap()
}

#[tauri::command]
pub fn delete_record(id:i32){
    SqliteDB::new().delete_record(id).unwrap();
}