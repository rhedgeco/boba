use crate::{storage::StageRunners, BobaResources, BobaStage};
use anyhow::Result;
use log::error;
use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    rc::Rc,
    sync::atomic::AtomicU64,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PearlId {
    _id: u64,
}

impl PearlId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self {
            _id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}

pub struct Pearl<T> {
    id: PearlId,
    data: Rc<RefCell<T>>,
}

impl<T> Clone for Pearl<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            data: self.data.clone(),
        }
    }
}

impl<T> Pearl<T> {
    pub fn id(&self) -> &PearlId {
        &self.id
    }

    pub fn data(&self) -> Result<Ref<T>, BorrowError> {
        self.data.as_ref().try_borrow()
    }

    pub fn data_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.data.as_ref().try_borrow_mut()
    }
}

pub trait PearlRunner<Stage>
where
    Stage: 'static + BobaStage,
{
    fn run(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}

impl<Stage, Update> PearlRunner<Stage> for Pearl<Update>
where
    Stage: 'static + BobaStage,
    Update: 'static + PearlStage<Stage>,
{
    fn run(&mut self, data: &<Stage as BobaStage>::StageData, resources: &mut BobaResources) {
        match Update::update(data, self, resources) {
            Ok(_) => {}
            Err(error) => error!(
                "There was a(n) {:?} when updating pearl: {:?}",
                error,
                self.id()
            ),
        }
    }
}

pub type PearlResult = Result<()>;

pub trait PearlStage<Stage>: PearlRegister
where
    Stage: 'static + BobaStage,
{
    fn update(
        data: &Stage::StageData,
        pearl: &mut Pearl<Self>,
        resources: &mut BobaResources,
    ) -> PearlResult;
}

pub trait PearlRegister
where
    Self: Sized,
{
    fn register(pearl: Pearl<Self>, storage: &mut StageRunners);
}

pub trait AsPearl<T>
where
    T: PearlRegister,
{
    fn as_pearl(self) -> Pearl<T>;
}

impl<T> AsPearl<T> for T
where
    T: PearlRegister,
{
    fn as_pearl(self) -> Pearl<T> {
        Pearl::<T> {
            id: PearlId::new(),
            data: Rc::new(RefCell::new(self)),
        }
    }
}