// Need to optimize and think about it necessary

struct IncrementalId(usize);

impl IncrementalId {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

trait Component {}

pub struct Entity {
    id: usize,
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            components: Vec::new(),
        }
    }
}

pub struct World {
    entities: Vec<Entity>,
    entity_id_inc: IncrementalId,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            entity_id_inc: IncrementalId::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let id = self.entity_id_inc.next();
        self.entities.push(Entity::new(id));
        id
    }

    fn add_component_to_entity<ComponentType: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: ComponentType,
    ) {
        if let Some(entity) = self
            .entities
            .iter_mut()
            .find(|entity| entity.id == entity_id)
        {
            entity.components.push(Box::new(component));
        }
    }
}
