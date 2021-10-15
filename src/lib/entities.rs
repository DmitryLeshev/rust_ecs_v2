use std::{any::TypeId, collections::HashMap};

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<>>
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    use crate::lib::world::World;

    #[test]
    fn register_an_entity() {
        let entities = Entities::default();
        entities.register_component::<Size>();
        let size_type_id = entities.
    }

    #[test]
    fn create_entity() {
        let location = Location(42.0, 24.0);
        let mut world = World::new();
        world.register_component::<Location>();
        world.register_component::<Size>();

        world
            .create_entity()
            .with_component(Location(42.0, 24.0))
            .with_component(Size(10.0));
    }

    struct Location(pub f32, pub f32);
    struct Size(pub f32);
}
