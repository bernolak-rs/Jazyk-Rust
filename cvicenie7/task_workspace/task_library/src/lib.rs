pub mod control;

pub mod task {

    pub trait ReadTaskFromUser {
        fn read_id(&self, message: &str) -> usize;
        fn read_nazov(&self, message: &str) -> String;
        fn read_popis(&self, message: &str) -> String;
        fn read_priorita(&self, message: &str) -> i32;
        fn read_planovany_zaciatok(&self, message: &str) -> NaiveDate;
        fn read_planovane_trvanie(&self, message: &str) -> TimeDelta;
        fn read_skutocny_zaciatok(&self, message: &str) -> Option<NaiveDate>;
        fn read_skutocne_trvanie(&self, message: &str) -> Option<TimeDelta>;
    }

    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
        task,
    };

    use chrono::{NaiveDate, TimeDelta};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Task {
        pub id: usize,
        pub nazov: String,
        pub popis: String,
        pub priorita: i32,
        pub planovany_zaciatok: NaiveDate,
        pub skutocny_zaciatok: Option<NaiveDate>,
        pub planovane_trvanie: TimeDelta,
        pub skutocne_trvanie: Option<TimeDelta>,
    }

    //1. upravme strukturu Task tak, aby skutocny zaciatok a skutocne trvanie nebolo nutne zadat pri vytvarani tasku
    //2. vytvorte metodu pre typ Task, ktora vypocita a vrati koncovy datum (na zaklade planovaneho zaciatku a trvania)
    impl Task {
        pub fn vypocitaj_planovany_koniec(&self) -> NaiveDate {
            self.planovany_zaciatok + self.planovane_trvanie
        }
        pub fn vypocitaj_skutocny_koniec(&self) -> Option<NaiveDate> {
            match self.skutocny_zaciatok {
                Some(zaciatok) => match self.skutocne_trvanie {
                    Some(trvanie) => Option::from(zaciatok + trvanie),
                    None => None,
                },
                None => None,
            }
        }

        pub fn format_task(&self) -> String {
            let skutocny_zaciatok = match self.skutocny_zaciatok {
                Some(s) => s.to_string(),
                None => "-".to_string(),
            };
            let skutocny_koniec = match self.vypocitaj_skutocny_koniec() {
                Some(s) => s.to_string(),
                None => "-".to_string(),
            };
            format!("{}: {}", self.id, self.nazov,)
        }

        pub fn print_task(&self) {
            println!("{}", self.format_task());
        } //spravte metodu pre TaskManager, ktora vypise vsetky tasky (vyuzite funkciu print_task)

        pub fn get_print_string(&self) -> String {
            let real_from = match self.skutocny_zaciatok {
                Some(val) => val.to_string(),
                None => String::from("-"),
            };

            let real_to = match self.vypocitaj_skutocny_koniec() {
                Some(val) => val.to_string(),
                None => String::from("-"),
            };

            let real_duration = match self.skutocne_trvanie {
                Some(val) => val.num_days().to_string(),
                None => String::from("-"),
            };

            format!(
                "Id: {}\nName: {}\nPriority: {}\nPlanned start: {}\nPlanned end: {}\nPlanned duration: {}\nReal start: {}\nReal end: {}\nReal duration: {}\nDescription: {}",
                self.id,
                self.nazov,
                self.priorita,
                self.planovany_zaciatok,
                self.vypocitaj_planovany_koniec(),
                self.planovany_zaciatok,
                real_from,
                real_to,
                real_duration,
                self.popis
            )
        }
    }

    impl Clone for Task {
        fn clone(&self) -> Self {
            Self {
                id: self.id,
                nazov: self.nazov.clone(),
                popis: self.popis.clone(),
                priorita: self.priorita,
                planovany_zaciatok: self.planovany_zaciatok.clone(),
                skutocny_zaciatok: self.skutocny_zaciatok.clone(),
                planovane_trvanie: self.planovane_trvanie,
                skutocne_trvanie: self.skutocne_trvanie,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TaskManager {
        tasks: Vec<Task>,
    }

    impl TaskManager {
        pub fn new() -> TaskManager {
            TaskManager { tasks: vec![] }
        }

        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        //metoda, ktora pozicia task na zaklade id tasku
        pub fn get_task_by_id(&self, id: usize) -> Option<&Task> {
            let result = self.tasks.iter().find(|t| t.id == id);
            result
        }

        pub fn remove_by_id(&mut self, task_id: usize) -> Option<Task> {
            let index = self.tasks.iter().position(|t| t.id == task_id);
            match index {
                Some(index) => Some(self.tasks.remove(index)),
                None => None,
            }
        }

        pub fn format_all_tasks(&self) -> Vec<String> {
            self.tasks.iter().map(|t| t.format_task()).collect()
        }

        pub fn print_all_tasks(&self) {
            for task in &self.format_all_tasks() {
                println!("{task}");
            }
        }

        pub fn get_tasks(&self) -> &Vec<Task> {
            &self.tasks
        }

        // 1. metoda, ktora zoradi tasky podla planovanej dlzky
        pub fn sort_tasks_by_planned_duration(&mut self) {
            self.tasks
                .sort_by(|t1, t2| t1.planovane_trvanie.cmp(&t2.planovane_trvanie));
        }
        // 2. metoda, ktora zoradi tasky podla planovaneho zaciatku
        pub fn sort_tasks_by_planned_start(&mut self) {
            self.tasks
                .sort_by(|t1, t2| t1.planovany_zaciatok.cmp(&t2.planovany_zaciatok));
        }

        pub fn read_from_txt_file(&mut self, file_path: &Path) {
            let file = File::open(file_path).expect("Cannot open file!");
            let file_lines = BufReader::new(file).lines();
            for line in file_lines {
                match line {
                    Ok(line) => {
                        let task = Self::_process_line(&line);
                        self.add_task(task);
                        println!("{line}")
                    }
                    Err(e) => println!("{e}"),
                }
            }
        }

        fn _process_line(line: &String) -> Task {
            let split: Vec<&str> = line.split(',').collect();
            let task = Task {
                id: split[0].parse().unwrap(),
                nazov: String::from(split[1]),
                popis: String::from(split[2]),
                priorita: split[3].parse().unwrap(),
                planovany_zaciatok: NaiveDate::parse_from_str(split[4], "%d.%m.%Y")
                    .expect("Cannot parse from string"),
                skutocny_zaciatok: Option::from(
                    NaiveDate::parse_from_str(split[5], "%d.%m.%Y")
                        .expect("Cannot parse from string"),
                ),
                planovane_trvanie: TimeDelta::days(split[6].parse().unwrap()),
                skutocne_trvanie: Option::from(TimeDelta::days(split[7].parse().unwrap())),
            };
            task
        }
    }
}
