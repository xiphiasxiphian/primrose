use wgpu::core::command::QueryError::Resolve;

pub mod assetpool;

pub struct ManagedResource<T>(ManagedResourceInner<T>);
enum ManagedResourceInner<T>
{
    Resolved(T),
    Lazy(Box<dyn FnOnce() -> T>),

    #[doc(hidden)]
    Evaluating,
}

impl<T> ManagedResourceInner<T>
{
    pub fn resolve(&mut self) -> &mut T
    {
        if let Self::Lazy(_) = self
        {
            let old = std::mem::replace(self, Self::Evaluating);
            let scene = match old
            {
                Self::Lazy(f) => f(),
                _ => unreachable!(),
            };

            *self = Self::Resolved(scene);
        }

        match self
        {
            Self::Resolved(s) => s,
            _ => unreachable!(),
        }
    }
}

impl<T> ManagedResource<T>
where
    T: 'static
{
    pub fn get(&mut self) -> &mut T { self.0.resolve() }

    pub fn eager(scene: T) -> Self { Self(ManagedResourceInner::Resolved(scene)) }

    pub fn lazy(f: Box<dyn FnOnce() -> T>) -> Self { Self(ManagedResourceInner::Lazy(f)) }

    pub fn map<F, O>(self, f: F) -> ManagedResource<O>
    where
        F: FnOnce(T) -> O + 'static,
    {
        ManagedResource(match self.0 {
                ManagedResourceInner::Resolved(val) => ManagedResourceInner::Resolved(f(val)),

                ManagedResourceInner::Lazy(lazy_fn) => {
                    ManagedResourceInner::Lazy(Box::new(move || f(lazy_fn())))
                }

                ManagedResourceInner::Evaluating => ManagedResourceInner::Evaluating,
            }
        )
    }
}
