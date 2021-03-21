#![allow(unused_variables)]
use chrono::NaiveDateTime;
use sqlx_helper::{Create, Delete, Get, Update};

#[derive(Debug, Clone, Get, Create, Delete, Update)]
pub struct Submission {
    #[get(pk)]
    pub id: i32,
    pub epoch_second: NaiveDateTime,
    pub problem_id: String,
    pub contest_id: String,
    pub result: String,
    pub atcoder_id: String,
    pub language: String,
    pub point: i32,
    pub length: i32,
    pub execution_time: i32,
    pub account_id: i64,
}

fn main() {}
