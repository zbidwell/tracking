use chrono::prelude::*;
use rusqlite::{Connection};
// use uuid::Uuid;

use std::collections::HashMap;

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub struct TaskID(Uuid);

// impl TaskID {
//     pub fn new() -> TaskID {
//         TaskID(Uuid::new_v4())
//     }
// }

#[derive(Clone, Debug)]
pub struct Task {
    // pub id: TaskID,
    pub name: String,
    pub events: Vec<TimeEvent>,
}

impl Task {
    pub fn new(name: String) ->  Task {
        Task {
            // id: TaskID::new(),
            name,
            events: Vec::new()
        }
    }

    pub fn status(&self) -> TaskStatus {
        match self.events.last() {
            Some(TimeEvent::Start(_)) => TaskStatus::Running,
            Some(TimeEvent::Stop(_)) => TaskStatus::Stopped,
            _ => TaskStatus::Unknown,
        }
    }

    pub fn start(&mut self) {
        match self.status() {
            TaskStatus::Stopped | TaskStatus::Unknown => {
                self.events.push(TimeEvent::Start(Utc::now()))
            },
            TaskStatus::Running => {
                // do nothing cause we are already running
            }
        }
    }

    pub fn stop(&mut self) {
        match self.status() {
            TaskStatus::Running | TaskStatus::Unknown => {
                self.events.push(TimeEvent::Stop(Utc::now()))
            },
            TaskStatus::Stopped => {
                // do nothing cause we are already stopped
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TaskStatus {
    Stopped,
    Running,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimeEvent {
    Start(DateTime<Utc>),
    Stop(DateTime<Utc>),
}

impl TimeEvent {
    pub fn get_time(&self) -> DateTime<Utc> {
        match self {
            TimeEvent::Start(t) => *t,
            TimeEvent::Stop(t) => *t, 
        }
    }
}

#[derive(Clone, Debug)]
pub struct TaskManager {
    pub tasks: HashMap<String, Task>,
}

impl TaskManager {
    pub fn get_task<S: AsRef<str>>(&self, name: S) -> Option<&Task> {
        self.tasks.get(name.as_ref())
    }

    pub fn all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn create_task<S: Into<String>>(&mut self, name: S) {
        let name = name.into();
        let task = Task::new(name.clone());
        self.tasks.insert(name, task);
    }

    pub fn start_task<S: AsRef<str>>(&mut self, name: S) {
        let task = self.get_task(name.as_ref());
        
    }
}