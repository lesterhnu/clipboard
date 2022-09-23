use crate::utils;
use rusqlite::{Connection, OpenFlags, Result};
use std::fs::File;
// use std::io;
use std::path::Path;
#[derive(serde::Serialize, serde::Deserialize, Debug, Default,PartialEq)]
pub struct Record{
    pub id: i32,
    pub content: String,
    pub timestamp: i64,
    pub hash_string: String,
    pub is_del: usize,
}

pub struct SqliteDB{
    conn:Connection,
}
const SQLITE_FILE:&str = "../data.sqlite";

#[allow(unused)]
impl SqliteDB {
    pub fn new()->Self{
        let c = Connection::open_with_flags(SQLITE_FILE, OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
        SqliteDB{
            conn:c,
        }
    }
    pub fn add(&self)->i64{
        self.conn.last_insert_rowid()
        
    }
    pub fn init(){
        if !Path::new(SQLITE_FILE).exists() {
            File::create(SQLITE_FILE).unwrap();
        }
        let c = Connection::open_with_flags(SQLITE_FILE, OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
        let sql = r#"
        create table if not exists record(
            "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            "content" TEXT,
            "hash_string" TEXT,
            "timestamp" INTEGER NOT NULL,
            "is_del" INTEGER DEFAULT 0
        );
        "#;
        c.execute(sql, ()).unwrap();
    }
    pub fn insert_record(&self,r:Record)->Result<i64>{
        let sql = "INSERT INTO record(content,hash_string,timestamp) values (?1,?2,?3)";
        let hash_string = utils::md5(r.content.as_str());
        self.conn.execute(sql,(&r.content,hash_string, &r.timestamp))?;
        Ok(self.conn.last_insert_rowid())
    }
    fn find_record_by_hash(&self,hash_string: String)->Result<Record>{
        let sql = "SELECT id,content,timestamp,hash_string,is_del from record where hash_string=?1 and is_del = 0";
        let r = self.conn.query_row(sql, [hash_string], |row|{
            Ok(Record{
                id: row.get(0)?,
                ..Default::default()
            })
        })?;
        Ok(r)
    }
    // 更新时间
    fn update_record_timestamp(&self,r:Record)->Result<()>{
        let sql = "update record set timestamp = ?1 where hash_string = ?2 and is_del = 0";
        let hash_string = utils::md5(r.content.as_str());
        self.conn.execute(sql, (&r.timestamp, hash_string))?;
        Ok(())
    }
    pub fn insert_if_not_exist(&self,r:Record)->Result<()>{
        let hash_string = utils::md5(r.content.as_str());
        match self.find_record_by_hash(hash_string){
            Ok(res) => {
                self.update_record_timestamp(r)?;
            },
            Err(_e) =>{
                self.insert_record(r)?;
            }
        }
        Ok(())
    }

    // 清除数据
    pub fn clear_data(&self)->Result<()>{
        let sql = "delete from record";
        self.conn.execute(sql, ())?;
        Ok(())
    }

    // 批量查询
    pub fn batch_get_record(&self)->Result<Vec<Record>>{
        let mut l = Vec::new();
        let sql = "select id,content,hash_string,timestamp,is_del from record where is_del = 0 order by timestamp desc limit 10";
        let mut stmt = self.conn.prepare(sql)?;
        let record_iter = stmt.query_map([],|row|{
            let record = Record{
                id: row.get(0)?,
                content: row.get(1)?, 
                hash_string: row.get(2)?,
                timestamp: row.get(3)?,
                is_del: row.get(4)?,
            };
            Ok(record)
        })?;
        for item in record_iter{
            l.push(item?);
        }
        Ok(l)
    }
    pub fn delete_record(&self,id:i32)->Result<()>{
        let sql = "update record set is_del = 1 where id = ?1";
        self.conn.execute(sql, [id]);
        Ok(())
    }


}

#[test]
fn test_batch_query(){
    match SqliteDB::new().batch_get_record(){
        Ok(s) =>{
            println!("batch:{:?}",s);
        },
        Err(e) => {
            println!("err:{}",e)
        }
    }
}
#[test]
fn test_vec(){
    let s = Record{
        ..Default::default()
    };
    let v = SqliteDB::new().batch_get_record().unwrap();
    // assert_eq!(SqliteDB::new().batch_get_record().unwrap().get(0),r);
    assert_eq!(v[0],s);
}


#[test]
fn test_sqlite_insert(){
    SqliteDB::init();
    let r = Record{
        content:"123456".to_string(),
        hash_string:"e10adc3949ba59abbe56e057f20f883e".to_string(),
        timestamp:1234567,
        ..Default::default()
    };
    assert_eq!(SqliteDB::new().insert_record(r).unwrap(),1_i64)
}