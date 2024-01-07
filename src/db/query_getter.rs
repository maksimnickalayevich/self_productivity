use serde::{Deserialize, Serialize};


pub trait QueryGetter<'a, T> 
where T: Serialize + Deserialize<'a>
{
    fn get_table_init_query(table_name: &'a String) -> String;
}