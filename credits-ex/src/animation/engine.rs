use crate::canvas::{Canvas, Vector2};
use std::{any::Any, collections::HashMap};

// Preserves Python's named data dictionaries, with a separate store per owner.
// HashMap provides lookup by name; String owns keys supplied through &str.
// Box<dyn Any> owns differently sized concrete values in a single map.
// get::<T>() and get_mut::<T>() recover the exact stored type; missing keys or
// type mismatches return None. Any excludes values containing non-'static borrows.
pub struct DataStore {
    pub values: HashMap<String, Box<dyn Any>>,
}

pub struct Manager {
    pub scenes: HashMap<String, Scene>,
    pub events: HashMap<i32, Vec<Event>>,
    pub active_scene: Vec<String>,
    pub current_beat: i32,
    pub data: DataStore,
    pub canvas: Canvas,
}

pub struct Scene {
    pub name: String,
    pub generators: Vec<Generator>,
    pub start_beat: i32,
    pub internal_beat: i32,
    pub data: DataStore,
}

pub struct Event {
    pub beat: i32,
    pub action: Action,
}

#[derive(Clone)]
pub enum Action {
    None,
    SwapScene (String, i32),
    LayerScene (String, i32),
    RemoveScene (String),
    Action(fn(&mut Manager))
}

pub struct Generator {
    pub start_beat: i32,
    pub condition: Condition,
    pub on_create: Option<fn(&mut Self, &mut Context<'_>)>,
    pub request: fn(&mut Self, i32, &mut Context<'_>),
    pub request_clear: Option<fn(&mut Self, i32, &mut Context<'_>)>,
    pub data: DataStore,
}

pub enum Condition {
    Always,
    Every(i32),
    EveryOnOff(i32, i32), // on, off
    EveryOffOn(i32, i32), // off, on
    Before(i32),
    At(i32),
    All(Vec<Condition>),
}

// Python callbacks keep parent and scene references; Rust callbacks receive
// temporary access through Context while each object retains ownership.
// Borrowing only manager data, scene data and canvas keeps those resources
// separate from the generator being mutated, avoiding overlapping mutable
// borrows of its owning Scene or Manager.
// 'a bounds these resource borrows; beat fields are copied scheduling snapshots.
pub struct Context<'a> {
    pub manager_data: &'a mut DataStore,
    pub scene_data: &'a mut DataStore,
    pub canvas: &'a mut Canvas,
    pub current_beat: i32,
    pub start_beat: i32,
    pub internal_beat: i32,
}

impl DataStore {

    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn set<T: Any>(&mut self, key: &str, value: T) {
        self.values.insert(key.to_string(), Box::<T>::new(value));
    }

    pub fn get<T: Any>(&self, key: &str) -> Option<&T> {
        self.values.get(key)?.downcast_ref::<T>()
    }

    pub fn get_mut<T: Any>(&mut self, key: &str) -> Option<&mut T> {
        self.values.get_mut(key)?.downcast_mut::<T>()
    }

    // In Rust, it is better to use get_mut() instead of oper().
    // I kept it just for consistency with the original Python.
    pub fn oper<T: Any>(&mut self, key: &str, operation: impl FnOnce(&mut T)) {
        operation(self.get_mut::<T>(key)
            .expect("data not found or type mismatch"));
    }

}

impl Manager {

    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            events: HashMap::new(),
            active_scene: Vec::new(),
            current_beat: 0,
            data: DataStore::new(),
            canvas: Canvas::new(),
        }
    }

    pub fn init(scenes: Vec<Scene>, events: Vec<Event>, dimensions: Vector2) -> Manager {
        let mut events_map: HashMap<i32, Vec<Event>> = HashMap::new();

        for event in events {
            events_map.entry(event.beat).or_default().push(event);
        }

        Self {
            scenes: scenes.into_iter()
                .map(|scene: Scene| (scene.name.clone(), scene))
                .collect::<HashMap<String, Scene>>(),
            events: events_map,
            current_beat: -1,
            canvas: Canvas::init(dimensions),
            ..Self::new()
        }
    }

    pub fn set_data<T: Any>(&mut self, key: &str, value: T) {
        self.data.set::<T>(key, value);
    }

    pub fn get_data<T: Any>(&self, key: &str) -> Option<&T> {
        self.data.get::<T>(key)
    }

    pub fn get_data_mut<T: Any>(&mut self, key: &str) -> Option<&mut T> {
        self.data.get_mut::<T>(key)
    }

    pub fn oper_data<T: Any>(&mut self, key: &str, operation: impl FnOnce(&mut T)) {
        self.data.oper::<T>(key, operation);
    }

    pub fn start_scene(&mut self, scene: &str, at: i32) {
        if self.active_scene.is_empty() {
            self.active_scene.push(scene.to_string());
        }
        else {
            self.active_scene[0] = scene.to_string();
        }

        self.scenes.get_mut(&self.active_scene[0])
            .expect("scene not found")
            .start(at, self.current_beat, &mut self.data, &mut self.canvas);
    }

    pub fn add_scene(&mut self, scene: &str, at: i32) {
        self.active_scene.push(scene.to_string());
        self.scenes.get_mut(scene)
            .expect("scene not found")
            .start(at, self.current_beat, &mut self.data, &mut self.canvas);
    }

    pub fn remove_scene(&mut self, scene: &str) {
        self.scenes.get(scene)
            .expect("scene not found");
        if let Some(index) = self.active_scene.iter().position(|name: &String| name == scene){
            self.active_scene.remove(index);
        }
    }

    pub fn request_next(&mut self, render: bool) {
        for scene in &self.active_scene {
            self.scenes.get_mut(scene)
                .expect("scene not found")
                .request_frame(render, self.current_beat, &mut self.data, &mut self.canvas);
        }
        self.next_beat();
    }

    pub fn next_beat(&mut self) {
        self.current_beat += 1;
        let Some(events) = self.events.get(&self.current_beat) else { return;};
        let actions: Vec<Action> = events.iter()
            .map(|event: &Event| event.action.clone())
            .collect::<Vec<Action>>();

        for action in actions {
            action.check(self);
        }
    }

    pub fn set_scene_data<T: Any>(&mut self, scene: &str, key: &str, value: T) {
        self.scenes.get_mut(scene)
            .expect("scene not found")
            .set_data(key, value);
    }

    pub fn set_generator_data<T: Any>(&mut self, scene: &str, generator_index: usize, key: &str, value: T) {
        self.scenes.get_mut(scene)
            .expect("scene not found")
            .generators.get_mut(generator_index)
            .expect("generator not found")
            .set_data(key, value);
    }

}

impl Scene {

    pub fn new() -> Self { 
        Self {
            name: String::new(),
            generators: Vec::new(),
            start_beat: 0,
            internal_beat: 0,
            data: DataStore::new(),
        }
    }

    pub fn init(name: &str, generators: Vec<Generator>) -> Self {
        Self {
            name: name.to_string(),
            generators,
            ..Self::new()
        }
    }

    pub fn set_data<T: Any>(&mut self, key: &str, value: T) {
        self.data.set::<T>(key, value);
    }

    pub fn get_data<T: Any>(&self, key: &str) -> Option<&T> {
        self.data.get::<T>(key)
    }

    pub fn get_data_mut<T: Any>(&mut self, key: &str) -> Option<&mut T> {
        self.data.get_mut::<T>(key)
    }

    pub fn oper_data<T: Any>(&mut self, key: &str,operation: impl FnOnce(&mut T)) {
        self.data.oper::<T>(key, operation);
    }

    pub fn request_frame(&mut self, render: bool, current_beat: i32, manager_data: &mut DataStore, canvas: &mut Canvas) {
        let beat: i32 = self.internal_beat;
        if render {
            let mut context: Context<'_> = Context {
                manager_data,
                scene_data: &mut self.data,
                canvas,
                current_beat,
                start_beat: self.start_beat,
                internal_beat: beat,
            };
            for generator in &mut self.generators {
                generator.request_frame(beat, &mut context);
            }
        }
        self.internal_beat += 1;
    }

    pub fn start(&mut self, at: i32, current_beat: i32, manager_data: &mut DataStore, canvas: &mut Canvas,) {
        let mut context: Context<'_> = Context {
            manager_data: &mut *manager_data,
            scene_data: &mut self.data,
            canvas: &mut *canvas,
            current_beat,
            start_beat: self.start_beat,
            internal_beat: self.internal_beat,
        };

        for generator in &mut self.generators {
            generator.start(&mut context);
        }

        self.start_beat = at;
        self.internal_beat = at;
        self.request_frame(true, current_beat, manager_data, canvas);
    }

    pub fn set_generator_data<T: Any>(&mut self, generator: usize, key: &str, value: T) {
        self.generators.get_mut(generator)
            .expect("generator not found")
            .set_data(key, value);
    }

}

impl Event {

    pub fn new() -> Self {
        Self {
            beat: 0,
            action: Action::None,
        }
    }

    pub fn init(beat: i32, action: Action) -> Self {
        Self {
            beat,
            action,
        }
    }

    pub fn swap_scene(scene: &str, at: i32) -> Action {
        Action::SwapScene(scene.to_string(), at)
    }

    pub fn layer_scene(scene: &str, at: i32) -> Action {
        Action::LayerScene(scene.to_string(), at)
    }

    pub fn remove_scene(scene: &str) -> Action {
        Action::RemoveScene(scene.to_string())
    }

}

impl Action {

    pub fn check(&self, manager: &mut Manager) {
        match self {
            Self::None                  => {},
            Self::SwapScene(scene, at)  => manager.start_scene(scene, *at),
            Self::LayerScene(scene, at) => manager.add_scene(scene, *at),
            Self::RemoveScene(scene)    => manager.remove_scene(scene),
            Self::Action(action)        => action(manager)
        }
    }

}

impl Generator {

    pub fn new() -> Self {
        Self {
            start_beat: 0,
            condition: Condition::Always,
            on_create: None,
            request: |_, _, _| {},
            request_clear: None,
            data: DataStore::new(),
        }
    }

    pub fn init(start_beat: i32, condition: Condition, on_create: Option<fn(&mut Self, &mut Context<'_>)>, request: fn(&mut Self, i32, &mut Context<'_>), request_clear: Option<fn(&mut Self, i32, &mut Context<'_>)>) -> Self {
        Self {
            start_beat,
            condition,
            on_create,
            request,
            request_clear,
            ..Self::new()
        }
    }

    pub fn set_data<T: Any>(&mut self, key: &str, value: T) {
        self.data.set::<T>(key, value);
    }

    pub fn get_data<T: Any>(&self, key: &str) -> Option<&T> {
        self.data.get::<T>(key)
    }

    pub fn get_data_mut<T: Any>(&mut self, key: &str) -> Option<&mut T> {
        self.data.get_mut::<T>(key)
    }

    pub fn oper_data<T: Any>(&mut self, key: &str,operation: impl FnOnce(&mut T)) {
        self.data.oper::<T>(key, operation);
    }

    pub fn start(&mut self, context: &mut Context<'_>) {
        if let Some(on_create) = self.on_create {
            on_create(self, context);
        }
    }

    pub fn request_frame(&mut self, beat: i32, context: &mut Context<'_>) {
        if beat >= self.start_beat && self.condition.check(beat) {
            if beat != context.start_beat && beat != self.start_beat {
                if let Some(request_clear) = self.request_clear {
                    request_clear(self, beat - 1, context);
                }
            }
            (self.request)(self, beat, context);
        }
    }

}

impl Condition {

    pub fn check(&self, beat: i32) -> bool {
        match self {
            Self::Always              => true,
            Self::Every(interval)     => beat.rem_euclid(*interval) == 0,
            Self::EveryOnOff(on, off) => beat.rem_euclid(*on + *off) < *on,
            Self::EveryOffOn(off, on) => beat.rem_euclid(*off + *on) >= *off,
            Self::Before(end)         => beat < *end,
            Self::At(at)              => beat == *at,
            Self::All(conditions)     => {
                for condition in conditions {
                    if !condition.check(beat) { return false }
                }
                true
            },
        }
    }

}