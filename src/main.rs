use std::fs;
use std::thread;

fn main() {
    let path = "/home/krysto/programming/masters";
    let extension = ".py";
    let pool_size = 4;

    let activity = Activity::new(path, extension);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(pool_size)
        .build()
        .unwrap();

    let result = pool.install(|| activity.compute());
    for s in result.iter() {
        println!("{}", s);
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
        let mut list = Vec::new();
        let mut tasks = Vec::new();

        let content = match fs::read_dir(&self.path) {
            Ok(dir) => dir,
            Err(_) => return list,
        };

        for entry in content {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();
            let path_str = path.to_str().unwrap();

            if path.is_dir() {
                let task = Activity::new(path_str, &self.extension);
                tasks.push(thread::spawn(move || task.compute()));
            } else if path.is_file() && self.check_file_extension(path_str) {
                list.push(path.display().to_string());
            }
        }

        for task in tasks {
            if let Ok(file_path_list) = task.join() {
                list.extend(file_path_list);
            }
        }

        list
    }

    fn check_file_extension(&self, path: &str) -> bool {
        path.ends_with(&self.extension)
    }
}
