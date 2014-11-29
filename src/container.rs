use std::any::{ Any };
use std::collections::HashMap;

use super::{ GetterErr, Registry };
use super::definition::{ TypeDef };
use super::getter::{ GetterWrap };

pub struct Container {
    getters: HashMap<String, (TypeDef, Box<Any>)>,
}

impl Container {
    /// Create container from registry definitions.
    pub fn from_registry(registry: &Registry) -> Result<Container, Vec<GetterErr>> {
        let mut c = Container {
            getters: HashMap::new(),
        };

        let mut errors = Vec::<GetterErr>::new();

        for name in registry.all_names().iter() {
            match registry.any_getter_for(name.as_slice()) {
                Ok((typedef, getter)) => {
                    c.getters.insert(name.clone(), (typedef, getter));
                },
                Err(e) => { errors.push(e); },
            };
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(c)
    }
}
