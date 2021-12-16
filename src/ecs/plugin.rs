pub use legion::systems::Resources;
pub use legion::systems::Step;


/// Group of behaviour
pub trait Plugin {
    /// resources.insert(ResourceType::default());
    /// Schedule::builder().add_system(some_system_system()).build().into_vec();
    fn startup(&mut self, resources: &mut Resources) -> Vec<Step>;

    /// Schedule::builder().add_system(some_system_system()).build().into_vec()
    fn run() -> Vec<Step>;

    /// Schedule::builder().add_system(some_system_system()).build().into_vec()
    fn shutdown() -> Vec<Step>;
}
