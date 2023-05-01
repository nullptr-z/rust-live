use proc_macros::sql;

fn main() {
    hello();
    sql!(select * from table1 where id = 10 and timestamp >100 order by timestamp desc limit 10);
}
