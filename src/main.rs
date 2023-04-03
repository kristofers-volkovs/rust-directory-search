use std::{fs, thread};

fn main() {
    let path = "/home/krysto/programming/masters";
    let extension = ".py";
    // let pool_size = 4;

    let activity = Activity::new(path, extension);
    let task = thread::spawn(move || activity.compute());

    if let Ok(res) = task.join() {
        for s in res.iter() {
            println!("{}", s);
        }
    }
}

struct Activity {
    pub path: String,
    pub extension: String,
}

impl Activity {
    fn new(path: &str, extension: &str) -> Activity {
        Activity {
            path: path.to_owned(),
            extension: extension.to_owned(),
        }
    }

    fn compute(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut join_tasks = Vec::new();

        let content = match fs::read_dir(&self.path) {
            Ok(dir) => dir,
            Err(_) => return res,
        };

        for entry in content {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();
            let path_str = path.to_str().unwrap();

            if path.is_dir() {
                let activity = Activity::new(path_str, &self.extension);
                let task = thread::spawn(move || activity.compute());
                join_tasks.push(task);
            } else if path.is_file() && self.check_file_extension(path_str) {
                res.push(path.display().to_string());
            }
        }

        for task in join_tasks {
            if let Ok(path_vec) = task.join() {
                res.extend(path_vec);
            }
        }

        res
    }

    fn check_file_extension(&self, path: &str) -> bool {
        path.ends_with(&self.extension)
    }
}
