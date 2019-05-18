#[macro_use] 
extern crate serde_derive;
extern crate reqwest;
extern crate pinyin;
// extern crate time;

use reqwest::Error;

use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};
use time::Timespec;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    page_name: String,  
    pinyin: String,
    english_name: String,
    pinyin_short: String,
    nickname: String,
    nickname1: String,
    nickname2: String,
    nickname3: String,
    age: i32,
    description: String
}


#[derive(Deserialize)]
struct Ip {
    // origin: String,
    // ret: Vec<String>
    status: String,
    #[serde(rename = "ret")]
    ret: Vec<String>
    // ref: String
}

fn run() -> Result<Ip, Error> {
    
    // let wikilink = "http://shuyantech.com/api/cndbpedia/avpair?q=范冰冰";
    // let wikilink = "http://shuyantech.com/api/cndbpedia/value?q=乔振宇（中国内地男演员）&attr=别名";
    // let wikilink = "http://shuyantech.com/api/cndbpedia/value?q=乔振宇（中国内地男演员）&attr=别名";
    let attrname = "别名";
    let baseurl = "http://shuyantech.com/api/cndbpedia/value?q=乔振宇（中国内地男演员）&attr=";
    // let wikilink = baseurl + attrname;
    let wikilink = &[baseurl, attrname].join("");


        

    // let categ

    let json: Ip = reqwest::get(wikilink)?.json()?;
    // println!("status = {:?}", json.status);
    // println!("body = {:?}", json.ret);

    // let hanss = json.ret;
    // let args = pinyin::Args::new();
    // for hans in hanss
    // {
    //     println!("{:?}",  pinyin::lazy_pinyin(&hans, &args));
    // }

    Ok(json)
}

fn main()
{
    // let body = reqwest::get(wikilink);//?.text();



// let json: Ip = reqwest::get("http://httpbin.org/ip")?.json()?;

    let result = run();
    println!("body = {:?}", result.unwrap().ret);
    println!("main logic");

    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  page_name       TEXT NOT NULL,  
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  pinyin          TEXT NOT NULL,
                  english_name    TEXT NOT NULL,
                  pinyin_short    TEXT NOT NULL,
                  nickname        TEXT NOT NULL,
                  nickname1       TEXT,
                  nickname2       TEXT,
                  nickname3       TEXT,
                  age             INTEGER,
                  description     TEXT
                  )",
        NO_PARAMS,
    ).unwrap();
    
    let me = Person {
        id: 0,
        name: "测试".to_string(),
        time_created: time::get_time(),
        page_name: "Steven".to_string(),  
        pinyin: "Steven".to_string(),
        english_name: "Steven".to_string(),
        pinyin_short: "Steven".to_string(),
        nickname: "Steven".to_string(),
        nickname1: "Steven".to_string(),
        nickname2: "Steven".to_string(),
        nickname3: "Steven".to_string(),
        age: 15,
        description: "Steven".to_string()
    };
    conn.execute(
        "INSERT INTO person (name, time_created, nickname, page_name, pinyin, english_name, pinyin_short, nickname1, nickname2, nickname3, age, dscription)
                  VALUES (?1, ?2, ?3)",
        &[&me.name as &ToSql, &me.time_created, &me.nickname, &me.page_name, &me.pinyin, &me.english_name, &me.pinyin_short, &me.nickname1, &me.nickname2, &me.nickname3, &me.age, &me.description],
    ).unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, time_created, nickname FROM person")
        .unwrap();
    let person_iter = stmt
        .query_map(NO_PARAMS, |row| Person {
            id: row.get(0),
            name: row.get(1),
            time_created: row.get(2),
            nickname: row.get(3),
        }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}

// 