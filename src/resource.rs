use chashmap::{CHashMap, ReadGuard, WriteGuard};
use std::any::{Any, TypeId, type_name};

/// Stores a single instance of types, and allows for concurrent R/W protected access to them.
pub struct ResourceMap {
    map: CHashMap<TypeId, Resource>,
}

type Resource = Box<dyn Any>;

fn type_name_for_param<T>(_: T) -> &'static str {
    type_name::<T>()
}

// initialize a `ResourceMap` with the provided `Resource`s
pub fn init_resource_map(resources: Vec<Resource>) -> ResourceMap {
    let resource_map = CHashMap::<TypeId, Resource>::new();

    for resource in resources {
        if let None = resource_map.get(&resource.type_id()) {
            panic!("resource map initialized with two {} resources", type_name_for_param(&*resource));
        }
        resource_map.insert(resource.type_id(), resource);
    }

    ResourceMap { map: resource_map }
}

pub trait Map {
    /// obtain a `ReadGuard` protected immutable reference to the resource of type `T`
    fn get_resource<T: 'static>(&self) -> ReadGuard<TypeId, Resource>;

    /// obtain a `WriteGuard` protected mutable reference to the resource of type `T`
    fn get_resource_mut<T: 'static>(&self) -> WriteGuard<TypeId, Resource>;

    /// insert `resource` into the resource map
    fn set_resource<T: 'static>(&mut self, resource: T);
}

impl Map for ResourceMap {
    fn get_resource<T: 'static>(&self) -> ReadGuard<TypeId, Resource> {
        self.map.get(&TypeId::of::<T>()).unwrap()
    }

    fn get_resource_mut<T: 'static>(&self) -> WriteGuard<TypeId, Resource> {
        self.map.get_mut(&TypeId::of::<T>()).unwrap()
    }

    fn set_resource<T: 'static>(&mut self, resource: T) {
        self.map.insert(resource.type_id(), Box::new(resource));
    }
}

/// public `ResourceMap` methods
impl ResourceMap {
    /// adds the libmodal managed resource for tracking mode with mode enum T
    pub fn add_mode_resource<T: 'static>(&mut self, initial_mode: T) -> &mut ResourceMap {
        self.set_resource(initial_mode);
        self
    }
}
