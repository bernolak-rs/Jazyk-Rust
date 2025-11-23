use chrono::{NaiveDate, TimeDelta};
use diesel::{Insertable, Queryable, Selectable};

use crate::task::Task;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TaskDb {
    pub id: i32,
    pub nazov: String,
    pub popis: String,
    pub priorita: i32,
    pub planovany_zaciatok: String,
    pub skutocny_zaciatok: Option<String>,
    pub planovane_trvanie: i32,
    pub skutocne_trvanie: Option<i32>,
}

impl From<Task> for TaskDb {
    fn from(value: Task) -> Self {
        TaskDb {
            id: value.id as i32,
            nazov: value.nazov,
            popis: value.popis,
            priorita: value.priorita,
            planovany_zaciatok: value.planovany_zaciatok.format("%d.%m.%Y").to_string(),
            skutocny_zaciatok: match value.skutocny_zaciatok {
                Some(t) => Some(t.format("%d.%m.%Y").to_string()),
                None => None,
            },
            planovane_trvanie: value.planovane_trvanie.num_days().try_into().unwrap(),
            skutocne_trvanie: match value.skutocne_trvanie {
                Some(t) => Some(t.num_days().try_into().unwrap()),
                None => None,
            },
        }
    }
}

impl From<&TaskDb> for Task {
    fn from(value: &TaskDb) -> Self {
        Task {
            id: value.id as usize,
            nazov: value.nazov.clone(),
            popis: value.popis.clone(),
            priorita: value.priorita,
            planovany_zaciatok: NaiveDate::parse_from_str(&value.planovany_zaciatok, "%d.%m.%Y")
                .expect("Cannot parse from string"),
            skutocny_zaciatok: match &value.skutocny_zaciatok {
                Some(t) => Some(
                    NaiveDate::parse_from_str(t, "%d.%m.%Y").expect("Cannot parse from string"),
                ),
                None => None,
            },
            planovane_trvanie: TimeDelta::days(value.planovane_trvanie.into()),
            skutocne_trvanie: match value.skutocne_trvanie {
                Some(t) => Some(TimeDelta::days(t.into())),
                None => None,
            },
        }
    }
}
