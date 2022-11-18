use crate::{BobaResources, BobaStage, Pearl, PearlId, PearlRunner, PearlStage};
use hashbrown::HashMap;

/// A storage solution for `Pearl` objects.
///
/// You may be `insert` and `remove` pearls in any order.
/// The storage system has an `update` function that will iterate over
/// every component and call its corresponding `update` function.
/// This struct will typically be used inside a `BobaStage` as the
/// owner of all the pearls to be run for that stage.
pub struct PearlStorage<Stage: 'static + ?Sized + BobaStage> {
    pearls: HashMap<PearlId, Box<dyn PearlRunner<Stage>>>,
}

/// The default implementation for `PearlStorage<BobaStage>`
impl<Stage: 'static + BobaStage> Default for PearlStorage<Stage> {
    fn default() -> Self {
        Self {
            pearls: Default::default(),
        }
    }
}

impl<Stage: 'static + BobaStage> PearlStorage<Stage> {
    /// Adds a pearl to the storage system.
    pub fn add<T>(&mut self, pearl: Pearl<T>)
    where
        T: 'static + PearlStage<Stage>,
    {
        self.pearls.insert(*pearl.id(), Box::new(pearl));
    }

    /// Removed a pearl from the storage system
    pub fn remove(&mut self, id: &PearlId) {
        self.pearls.remove(id);
    }

    // updates all pearls that are currently in storage
    pub fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources) {
        for pearl in self.pearls.values_mut() {
            pearl.run(data, resources);
        }
    }
}