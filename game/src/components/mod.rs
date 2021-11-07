pub mod box_drawable;
pub mod bullet;
pub mod enemy;
pub mod health;
pub mod missile;
pub mod movable;
pub mod player;
pub mod sprite_drawable;

pub trait Remover {
    fn remove(&mut self, entity: crate::entity::Entity);
}

#[macro_export]
macro_rules! impl_system {
    ($component_ident: ident) => {
        #[allow(dead_code)]
        pub struct System {
            components: alloc::vec::Vec<$component_ident>,
            entities: alloc::vec::Vec<crate::entity::Entity>,
            map: hashbrown::HashMap<crate::entity::Entity, usize, n64_math::BuildFnvHasher>,
        }

        impl System {
            #[allow(dead_code)]
            pub fn new() -> Self {
                Self {
                    components: alloc::vec::Vec::with_capacity(256),
                    entities: alloc::vec::Vec::with_capacity(256),
                    map: hashbrown::HashMap::with_capacity_and_hasher(
                        256,
                        n64_math::BuildFnvHasher,
                    ),
                }
            }

            #[allow(dead_code)]
            pub fn add(&mut self, entity: crate::entity::Entity, component: $component_ident) {
                self.components.push(component);
                self.entities.push(entity);
                self.map.insert(entity, self.components.len() - 1);
            }

            #[allow(dead_code)]
            pub fn lookup(&self, entity: crate::entity::Entity) -> Option<&$component_ident> {
                if let Some(&index) = self.map.get(&entity) {
                    return Some(&self.components[index]);
                }

                None
            }

            #[allow(dead_code)]
            pub fn lookup_mut(
                &mut self,
                entity: crate::entity::Entity,
            ) -> Option<&mut $component_ident> {
                if let Some(&index) = self.map.get(&entity) {
                    return Some(&mut self.components[index]);
                }

                None
            }

            #[allow(dead_code)]
            pub fn components(&self) -> &[$component_ident] {
                &self.components
            }

            #[allow(dead_code)]
            pub fn components_mut(&mut self) -> &mut [$component_ident] {
                &mut self.components
            }

            #[allow(dead_code)]
            pub fn entities(&self) -> &[crate::entity::Entity] {
                &self.entities
            }

            #[allow(dead_code)]
            pub fn components_and_entities(
                &self,
            ) -> impl Iterator<Item = (&$component_ident, crate::entity::Entity)> {
                self.components.iter().zip(self.entities.iter().copied())
            }

            #[allow(dead_code)]
            pub fn components_and_entities_mut(
                &mut self,
            ) -> impl Iterator<Item = (&mut $component_ident, crate::entity::Entity)> {
                self.components
                    .iter_mut()
                    .zip(self.entities.iter().copied())
            }
        }

        impl crate::components::Remover for System {
            fn remove(&mut self, entity: crate::entity::Entity) {
                if let Some(&index) = self.map.get(&entity) {
                    let last = self.components.len() - 1;
                    let last_entity = self.entities[last];

                    self.components[index as usize] = self.components[last];
                    self.components.remove(last);

                    self.entities[index as usize] = self.entities[last];
                    self.entities.remove(last);

                    self.map.insert(last_entity, index);
                    self.map.remove(&entity);
                }
            }
        }
    };
}
