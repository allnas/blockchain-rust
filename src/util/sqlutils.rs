use sqlite::{Connection, State};

pub fn connect() -> Connection {
    let connection = sqlite::open("block_chain.db").unwrap();
    return connection;
}

pub fn init_table(connection: &mut Connection) {
    connection.execute(
        "
             CREATE TABLE IF NOT EXISTS t_block(id INTEGER PRIMARY KEY,json TEXT NOT NULL);
             ", ).unwrap();
}

pub fn save_block(connection: &mut Connection, id: i64, json: &str) -> bool {
    let mut statement = connection
        .prepare("INSERT INTO t_block VALUES (?, ?)")
        .unwrap();

    statement.bind(1, id).unwrap();
    statement.bind(2, json).unwrap();
    let stm = statement.next();
    return stm.is_ok();
}

pub fn search(connection: &mut Connection, id: i64) {
    let mut statement = connection
        .prepare("SELECT * FROM t_block WHERE id > ?")
        .unwrap();

    statement.bind(1, id).unwrap();

    while let State::Row = statement.next().unwrap() {
        println!("id = {}", statement.read::<i64>(0).unwrap());
        println!("json = {}", statement.read::<String>(1).unwrap());
    }
}

pub fn max_id(connection: &mut Connection) -> i64 {
    let statement = "SELECT MAX(id)+1 AS id FROM t_block";
    let mut cursor = connection.prepare(statement).unwrap().cursor();
    return cursor.next().unwrap().unwrap()[0].as_integer().unwrap();
}