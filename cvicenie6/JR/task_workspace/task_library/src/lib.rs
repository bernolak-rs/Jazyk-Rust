pub mod control;

pub mod task {
    use chrono::{NaiveDate, TimeDelta};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Task {
        pub id: i32,
        pub name: String,
        pub description: String,
        pub priority: i32,
        pub planned_from: NaiveDate,
        pub planned_duration: TimeDelta,
        pub real_from: Option<NaiveDate>,
        pub real_duration: Option<TimeDelta>,
        // dependency:
    }

    pub trait ReadTask {
        fn read_id(&self, message: &str) -> i32;
        fn read_name(&self, message: &str) -> String;
        fn read_description(&self, message: &str) -> String;
        fn read_priority(&self, message: &str) -> i32;
        fn read_planned_from(&self, message: &str) -> NaiveDate;
        fn read_planned_duration(&self, message: &str) -> TimeDelta;
        fn read_real_from(&self, message: &str) -> Option<NaiveDate>;
        fn read_real_duration(&self, message: &str) -> Option<TimeDelta>;
    }

    impl Task {
        pub fn calculate_planned_end(&self) -> NaiveDate {
            self.planned_from + self.planned_duration
        }

        pub fn calculate_real_end(&self) -> Option<NaiveDate> {
            match self.real_from {
                Some(from) => match self.real_duration {
                    Some(dur) => Option::from(from + dur),
                    None => None,
                },
                None => None,
            }
        }

        pub fn print(&self) {
            println!("{}", self.get_print_string());
        }

        pub fn get_print_string(&self) -> String {
            let real_from = match self.real_from {
                Some(val) => val.to_string(),
                None => String::from("-"),
            };

            let real_to = match self.calculate_real_end() {
                Some(val) => val.to_string(),
                None => String::from("-"),
            };

            let real_duration = match self.real_duration {
                Some(val) => val.num_days().to_string(),
                None => String::from("-"),
            };

            format!(
                "Id: {}\nName: {}\nPriority: {}\nPlanned start: {}\nPlanned end: {}\nPlanned duration: {}\nReal start: {}\nReal end: {}\nReal duration: {}\nDescription: {}",
                self.id,
                self.name,
                self.priority,
                self.planned_from,
                self.calculate_planned_end(),
                self.planned_duration.num_days(),
                real_from,
                real_to,
                real_duration,
                self.description
            )
        }

        pub fn print_header() {
            println!(
                "Task ID, Priority, Planned from, Planned to, Real from, Real to, Description
----------------------------------------------------------------------------"
            );
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
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

        pub fn get_tasks(&self) -> &Vec<Task> {
            &self.tasks
        }

        pub fn remove_task_by_id(&mut self, id: i32) {
            self.tasks.remove(id as usize);
        }

        pub fn list_task_by_id(&self, id: i32) -> Option<&Task> {
            self.tasks.iter().find(|t| t.id == id)
        }

        pub fn list_tasks(&self, show_header: bool) {
            if show_header {
                Task::print_header();
            }

            for i in self.tasks.iter() {
                i.print();
            }
        }

        pub fn sort_by_planned_duration(&mut self) {
            self.tasks
                .sort_by(|a, b| a.planned_duration.cmp(&b.planned_duration));
        }

        pub fn sort_by_planned_from(&mut self) {
            self.tasks
                .sort_by(|a, b| a.planned_from.cmp(&b.planned_from));
        }

        pub fn sort_by_priority(&mut self) {
            self.tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
        }

        // pub fn read_from_csv(&mut self, file_path: &Path, has_header: bool) {
        //     let file = File::open(file_path).expect("Cannot open file {file_path}");
        //     let file_lines = BufReader::new(file).lines();
        //
        //     let lines = file_lines.skip(if has_header { 1 } else { 0 });
        //
        //     for line in lines {
        //         match line {
        //             Ok(val) => {
        //                 let t = self.parse_line(&val);
        //                 self.add_task(t);
        //             }
        //             Err(e) => {
        //                 println!("{e}")
        //             }
        //         }
        //     }
        // }
        //
        // fn parse_line(&self, line: &str) -> Task {
        //     let split: Vec<&str> = line.split(',').collect();
        //
        //     let real_from = if split[6].is_empty() {
        //         None
        //     } else {
        //         Some(parse_date(split[6]))
        //     };
        //
        //     let real_duration = if split[7].is_empty() {
        //         None
        //     } else {
        //         Some(parse_timedelta(split[7]))
        //     };
        //
        //     Task {
        //         id: parse_number(split[0]),
        //         name: parse_string(split[1]),
        //         description: parse_string(split[2]),
        //         priority: parse_number(split[3]),
        //         planned_from: parse_date(split[4]),
        //         planned_duration: parse_timedelta(split[5]),
        //         real_from,
        //         real_duration,
        //     }
        // }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
