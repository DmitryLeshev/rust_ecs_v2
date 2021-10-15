use std::any::Any;

use super::resource::Resource;

#[derive(Debug, Default)]
pub struct World {
    resources: Resource,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }
    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }
}

#[cfg(test)]
mod tests {

    use crate::lib::world::World;

    const FPS: u32 = 60;
    struct FpsResource(pub u32);

    #[test]
    fn create_and_get_resource_immutable() {
        let world = initialize_world_and_add_resource();
        let fps = world.get_resource::<FpsResource>().unwrap();

        assert_eq!(fps.0, FPS);
    }

    #[test]
    fn get_resource_mutably() {
        let mut world = initialize_world_and_add_resource();
        {
            let fps: &mut FpsResource = world.get_resource_mut::<FpsResource>().unwrap();
            fps.0 += 1;
        }
        let fps = world.get_resource::<FpsResource>().unwrap();
        assert_eq!(fps.0, 61);
    }

    #[test]
    fn delete_resource() {
        let mut world = initialize_world_and_add_resource();
        world.delete_resource::<FpsResource>();
        let delete_resource = world.get_resource::<FpsResource>();

        assert!(delete_resource.is_none());
    }

    fn initialize_world_and_add_resource() -> World {
        let mut world = World::new();

        world.add_resource(FpsResource(FPS));
        world
    }
}
