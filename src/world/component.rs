use super::*;

#[derive(Debug, Clone)]
pub struct ComponentStorage<T> {
    name: ComponentName,
    inner: HashMap<Id, T>,
}

pub type ComponentResult<T> = Result<T, ComponentError>;

pub type ComponentName = &'static str;

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentError {
    NotFound { id: Id, component: ComponentName },
    AlreadyExists { id: Id, component: ComponentName },
}

macro_rules! comp_iter {
    ($iter:expr) => {{
        $iter.map(|(id, value)| (id.to_owned(), value))
    }};
}

impl<T> ComponentStorage<T> {
    pub fn new(name: ComponentName) -> Self {
        Self {
            name,
            inner: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Id, &T)> {
        comp_iter!(self.inner.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut T)> {
        comp_iter!(self.inner.iter_mut())
    }

    pub fn contains(&self, id: Id) -> bool {
        self.inner.contains_key(&id)
    }

    pub fn get(&self, id: Id) -> ComponentResult<&T> {
        option_comp_result(id, &self.name, self.inner.get(&id))
    }

    pub fn get_mut(&mut self, id: Id) -> ComponentResult<&mut T> {
        option_comp_result(id, &self.name, self.inner.get_mut(&id))
    }

    pub fn remove(&mut self, id: Id) -> ComponentResult<T> {
        option_comp_result(id, &self.name, self.inner.remove(&id))
    }

    /// Updates the value of the unit's component if it exists.
    /// Fails if the component for that unit does not exist.
    pub fn update(&mut self, id: Id, value: T) -> ComponentResult<()> {
        *self.get_mut(id)? = value;
        Ok(())
    }

    /// Fails if the component for that unit already exists.
    pub fn insert(&mut self, id: Id, value: T) -> ComponentResult<()> {
        match self.inner.entry(id) {
            std::collections::hash_map::Entry::Occupied(_) => Err(ComponentError::AlreadyExists {
                id,
                component: self.name,
            }),
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }

    /// Returns the old value if it exists.
    pub fn insert_or_update(&mut self, id: Id, value: T) -> Option<T> {
        self.inner.insert(id, value)
    }
}

fn option_comp_result<T>(
    id: Id,
    component: &ComponentName,
    option: Option<T>,
) -> ComponentResult<T> {
    match option {
        Some(value) => Ok(value),
        None => Err(ComponentError::NotFound {
            id,
            component: component.to_owned(),
        }),
    }
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentError::NotFound { id, component } => {
                write!(f, "component {component} not found for {id:?}")
            }
            ComponentError::AlreadyExists { id, component } => {
                write!(f, "component {component} already exists for {id:?}")
            }
        }
    }
}
