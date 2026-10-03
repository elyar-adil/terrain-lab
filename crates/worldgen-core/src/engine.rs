//! Layers and the engine that evaluates them.
//!
//! A [`Layer`] is a versioned, pure generator: given a cell and what the layers it
//! depends on produced for it, it returns its own product. It declares those
//! dependencies, so the engine knows the order, can reject a cycle, and can cache
//! every product by (layer, version, cell). The engine evaluates on demand and
//! recursively: asking for a product pulls in exactly the products it needs and no
//! others, which is what makes the world lazy.
//!
//! A layer may only read the layers it declared. That is the dependency rule made
//! mechanical: if the tree layer never declared the street layer, it cannot reach
//! it, so the two cannot become entangled behind the declaration's back.

use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::cell::{Cell, Frame};
use crate::seed::Seed;

/// The identity of a layer. A short, stable, human-readable name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LayerId(pub &'static str);

impl fmt::Display for LayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// An input a layer reads.
#[derive(Clone, Copy, Debug)]
pub struct Dependency {
    pub layer: LayerId,
    /// An optional input may be absent from the engine: the layer then degrades
    /// instead of failing (trees without a road layer simply do not avoid roads).
    pub optional: bool,
}

impl Dependency {
    pub const fn required(layer: LayerId) -> Self {
        Self {
            layer,
            optional: false,
        }
    }
    pub const fn optional(layer: LayerId) -> Self {
        Self {
            layer,
            optional: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Two layers registered under one id.
    DuplicateLayer(LayerId),
    /// A layer needs a required input nothing provides.
    MissingDependency { layer: LayerId, needs: LayerId },
    /// The declared dependencies form a loop.
    DependencyCycle(Vec<LayerId>),
    /// A layer asked for a layer it did not declare.
    UndeclaredInput { layer: LayerId, asked: LayerId },
    /// No layer is registered under this id.
    UnknownLayer(LayerId),
    /// The same layer and cell asked for itself while being computed.
    CellCycle { layer: LayerId, cell: Cell },
    /// A product was not of the type the caller asked for.
    WrongType { layer: LayerId },
    /// A layer's own failure.
    Layer(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DuplicateLayer(l) => write!(f, "layer `{l}` is registered twice"),
            Error::MissingDependency { layer, needs } => {
                write!(
                    f,
                    "layer `{layer}` needs `{needs}`, which is not registered"
                )
            }
            Error::DependencyCycle(path) => {
                let names: Vec<String> = path.iter().map(ToString::to_string).collect();
                write!(
                    f,
                    "layers depend on each other in a loop: {}",
                    names.join(" -> ")
                )
            }
            Error::UndeclaredInput { layer, asked } => {
                write!(
                    f,
                    "layer `{layer}` read `{asked}` without declaring it as an input"
                )
            }
            Error::UnknownLayer(l) => write!(f, "no layer `{l}`"),
            Error::CellCycle { layer, cell } => {
                write!(
                    f,
                    "layer `{layer}` at {cell:?} needed itself to compute itself"
                )
            }
            Error::WrongType { layer } => {
                write!(f, "the product of `{layer}` is not the requested type")
            }
            Error::Layer(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Error {}

/// A generator of one kind of thing.
pub trait Layer: Send + Sync + 'static {
    type Output: Send + Sync + 'static;

    fn id(&self) -> LayerId;

    /// Bumped whenever the layer's behaviour changes. Part of the cache key and of
    /// the world's identity.
    fn version(&self) -> u32 {
        1
    }

    /// Everything this layer reads.
    fn inputs(&self) -> Vec<Dependency> {
        Vec::new()
    }

    /// Decide the layer's content for a cell. Must be a pure function of the cell,
    /// the context's seed, and the inputs read through the context.
    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<Self::Output, Error>;
}

type Product = Arc<dyn Any + Send + Sync>;

trait ErasedLayer: Send + Sync {
    fn id(&self) -> LayerId;
    fn version(&self) -> u32;
    fn inputs(&self) -> Vec<Dependency>;
    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<Product, Error>;
}

impl<L: Layer> ErasedLayer for L {
    fn id(&self) -> LayerId {
        Layer::id(self)
    }
    fn version(&self) -> u32 {
        Layer::version(self)
    }
    fn inputs(&self) -> Vec<Dependency> {
        Layer::inputs(self)
    }
    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<Product, Error> {
        Layer::collapse(self, ctx, cell).map(|output| Arc::new(output) as Product)
    }
}

/// Builds an [`Engine`] and checks the layer graph.
pub struct EngineBuilder {
    seed: Seed,
    frame: Frame,
    layers: Vec<Box<dyn ErasedLayer>>,
}

impl EngineBuilder {
    pub fn new(seed: Seed, frame: Frame) -> Self {
        Self {
            seed,
            frame,
            layers: Vec::new(),
        }
    }

    pub fn with<L: Layer>(mut self, layer: L) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    /// Validate and build: every required input must exist, no two layers may share
    /// an id, and the dependencies must not loop.
    pub fn build(self) -> Result<Engine, Error> {
        let mut by_id: HashMap<LayerId, Box<dyn ErasedLayer>> = HashMap::new();
        for layer in self.layers {
            let id = layer.id();
            if by_id.insert(id, layer).is_some() {
                return Err(Error::DuplicateLayer(id));
            }
        }
        for layer in by_id.values() {
            for dep in layer.inputs() {
                if !dep.optional && !by_id.contains_key(&dep.layer) {
                    return Err(Error::MissingDependency {
                        layer: layer.id(),
                        needs: dep.layer,
                    });
                }
            }
        }
        // Depth-first search for a loop, over the dependencies that exist.
        let mut state: HashMap<LayerId, u8> = HashMap::new(); // 1 = on the path, 2 = done
        fn visit(
            id: LayerId,
            by_id: &HashMap<LayerId, Box<dyn ErasedLayer>>,
            state: &mut HashMap<LayerId, u8>,
            path: &mut Vec<LayerId>,
        ) -> Result<(), Error> {
            match state.get(&id) {
                Some(2) => return Ok(()),
                Some(1) => {
                    let start = path.iter().position(|p| *p == id).unwrap_or(0);
                    let mut cycle = path[start..].to_vec();
                    cycle.push(id);
                    return Err(Error::DependencyCycle(cycle));
                }
                _ => {}
            }
            state.insert(id, 1);
            path.push(id);
            if let Some(layer) = by_id.get(&id) {
                let mut deps: Vec<LayerId> = layer.inputs().iter().map(|d| d.layer).collect();
                deps.sort();
                for dep in deps {
                    if by_id.contains_key(&dep) {
                        visit(dep, by_id, state, path)?;
                    }
                }
            }
            path.pop();
            state.insert(id, 2);
            Ok(())
        }
        let mut ids: Vec<LayerId> = by_id.keys().copied().collect();
        ids.sort();
        for id in ids {
            visit(id, &by_id, &mut state, &mut Vec::new())?;
        }
        Ok(Engine {
            seed: self.seed,
            frame: self.frame,
            layers: by_id,
            cache: Mutex::new(HashMap::new()),
            computed: AtomicUsize::new(0),
        })
    }
}

/// The world: layers, a seed, and a cache of what has been computed so far.
///
/// The cache only ever saves work. Dropping it, or evaluating in another order,
/// gives the same products.
pub struct Engine {
    seed: Seed,
    frame: Frame,
    layers: HashMap<LayerId, Box<dyn ErasedLayer>>,
    cache: Mutex<HashMap<(LayerId, u32, Cell), Product>>,
    computed: AtomicUsize,
}

impl Engine {
    pub fn seed(&self) -> Seed {
        self.seed
    }

    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    pub fn has_layer(&self, id: LayerId) -> bool {
        self.layers.contains_key(&id)
    }

    /// The world's identity: its seed and every layer with its version, in a fixed
    /// order. Two engines with the same identity produce the same world.
    pub fn identity(&self) -> (u64, Vec<(&'static str, u32)>) {
        let mut layers: Vec<(&'static str, u32)> = self
            .layers
            .values()
            .map(|l| (l.id().0, l.version()))
            .collect();
        layers.sort();
        (self.seed.0, layers)
    }

    /// How many (layer, cell) products have been computed, as opposed to served
    /// from the cache. For tests and for profiling.
    pub fn computed(&self) -> usize {
        self.computed.load(Ordering::Relaxed)
    }

    /// The product of a layer for a cell.
    pub fn get<T: Any + Send + Sync>(&self, layer: LayerId, cell: Cell) -> Result<Arc<T>, Error> {
        let product = self.evaluate(layer, cell, &[])?;
        product
            .downcast::<T>()
            .map_err(|_| Error::WrongType { layer })
    }

    fn evaluate(
        &self,
        layer: LayerId,
        cell: Cell,
        stack: &[(LayerId, Cell)],
    ) -> Result<Product, Error> {
        let l = self.layers.get(&layer).ok_or(Error::UnknownLayer(layer))?;
        let key = (layer, l.version(), cell);
        if let Some(hit) = self.cache.lock().map_err(poisoned)?.get(&key) {
            return Ok(hit.clone());
        }
        if stack.contains(&(layer, cell)) {
            return Err(Error::CellCycle { layer, cell });
        }
        let mut next: Vec<(LayerId, Cell)> = stack.to_vec();
        next.push((layer, cell));
        let ctx = Context {
            engine: self,
            layer,
            inputs: l.inputs(),
            stack: &next,
        };
        let product = l.collapse(&ctx, cell)?;
        self.computed.fetch_add(1, Ordering::Relaxed);
        // If another thread got there first, keep its product: they are equal.
        let mut cache = self.cache.lock().map_err(poisoned)?;
        Ok(cache.entry(key).or_insert(product).clone())
    }
}

fn poisoned<T>(_: std::sync::PoisonError<T>) -> Error {
    Error::Layer("the engine cache is poisoned".into())
}

/// What a layer sees while it computes: its seed, the frame, and its declared
/// inputs.
pub struct Context<'a> {
    engine: &'a Engine,
    layer: LayerId,
    inputs: Vec<Dependency>,
    stack: &'a [(LayerId, Cell)],
}

impl Context<'_> {
    pub fn frame(&self) -> &Frame {
        &self.engine.frame
    }

    /// The world seed.
    pub fn world_seed(&self) -> Seed {
        self.engine.seed
    }

    /// This layer's seed: the world seed derived with the layer's name, so two
    /// layers never share randomness by accident.
    pub fn seed(&self) -> Seed {
        self.engine.seed.derive(self.layer.0)
    }

    /// The seed for one cell of this layer.
    pub fn cell_seed(&self, cell: Cell) -> Seed {
        self.seed().derive_cell(cell)
    }

    fn declared(&self, layer: LayerId) -> Result<&Dependency, Error> {
        self.inputs
            .iter()
            .find(|d| d.layer == layer)
            .ok_or(Error::UndeclaredInput {
                layer: self.layer,
                asked: layer,
            })
    }

    /// A required input's product for a cell.
    pub fn input<T: Any + Send + Sync>(&self, layer: LayerId, cell: Cell) -> Result<Arc<T>, Error> {
        self.declared(layer)?;
        let product = self.engine.evaluate(layer, cell, self.stack)?;
        product
            .downcast::<T>()
            .map_err(|_| Error::WrongType { layer })
    }

    /// An optional input's product, or `None` if no such layer is registered.
    pub fn optional_input<T: Any + Send + Sync>(
        &self,
        layer: LayerId,
        cell: Cell,
    ) -> Result<Option<Arc<T>>, Error> {
        self.declared(layer)?;
        if !self.engine.has_layer(layer) {
            return Ok(None);
        }
        self.input(layer, cell).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: LayerId = LayerId("a");
    const B: LayerId = LayerId("b");
    const C: LayerId = LayerId("c");

    struct Source;
    impl Layer for Source {
        type Output = u64;
        fn id(&self) -> LayerId {
            A
        }
        fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<u64, Error> {
            Ok(ctx.cell_seed(cell).0)
        }
    }

    struct Doubler;
    impl Layer for Doubler {
        type Output = u64;
        fn id(&self) -> LayerId {
            B
        }
        fn inputs(&self) -> Vec<Dependency> {
            vec![Dependency::required(A)]
        }
        fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<u64, Error> {
            Ok(ctx.input::<u64>(A, cell)?.wrapping_mul(2))
        }
    }

    fn builder() -> EngineBuilder {
        EngineBuilder::new(Seed::new(7), Frame::new([0.0, 0.0], 1000.0))
    }

    #[test]
    fn a_layer_pulls_in_exactly_what_it_needs_and_the_cache_saves_the_rest() {
        let engine = builder().with(Source).with(Doubler).build().unwrap();
        let cell = Cell::new(3, 1, 2);
        let b = engine.get::<u64>(B, cell).unwrap();
        assert_eq!(engine.computed(), 2, "b and the a it needed");
        let a = engine.get::<u64>(A, cell).unwrap();
        assert_eq!(*b, a.wrapping_mul(2));
        assert_eq!(engine.computed(), 2, "a was already cached");
        engine.get::<u64>(B, cell.neighbour(1, 0)).unwrap();
        assert_eq!(engine.computed(), 4);
    }

    #[test]
    fn registration_rejects_duplicates_missing_inputs_and_loops() {
        assert_eq!(
            builder().with(Source).with(Source).build().err(),
            Some(Error::DuplicateLayer(A))
        );
        assert_eq!(
            builder().with(Doubler).build().err(),
            Some(Error::MissingDependency { layer: B, needs: A })
        );

        struct X;
        struct Y;
        impl Layer for X {
            type Output = ();
            fn id(&self) -> LayerId {
                LayerId("x")
            }
            fn inputs(&self) -> Vec<Dependency> {
                vec![Dependency::required(LayerId("y"))]
            }
            fn collapse(&self, _: &Context<'_>, _: Cell) -> Result<(), Error> {
                Ok(())
            }
        }
        impl Layer for Y {
            type Output = ();
            fn id(&self) -> LayerId {
                LayerId("y")
            }
            fn inputs(&self) -> Vec<Dependency> {
                vec![Dependency::required(LayerId("x"))]
            }
            fn collapse(&self, _: &Context<'_>, _: Cell) -> Result<(), Error> {
                Ok(())
            }
        }
        match builder().with(X).with(Y).build().err() {
            Some(Error::DependencyCycle(path)) => assert!(path.len() >= 3, "{path:?}"),
            other => panic!("expected a cycle, got {other:?}"),
        }
    }

    #[test]
    fn a_layer_cannot_read_what_it_did_not_declare() {
        struct Sneaky;
        impl Layer for Sneaky {
            type Output = u64;
            fn id(&self) -> LayerId {
                C
            }
            // Declares nothing, reads `a` anyway.
            fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<u64, Error> {
                Ok(*ctx.input::<u64>(A, cell)?)
            }
        }
        let engine = builder().with(Source).with(Sneaky).build().unwrap();
        assert_eq!(
            engine.get::<u64>(C, Cell::new(0, 0, 0)).err(),
            Some(Error::UndeclaredInput { layer: C, asked: A })
        );
    }

    #[test]
    fn an_optional_input_may_be_absent() {
        struct Tolerant;
        impl Layer for Tolerant {
            type Output = Option<u64>;
            fn id(&self) -> LayerId {
                C
            }
            fn inputs(&self) -> Vec<Dependency> {
                vec![Dependency::optional(A)]
            }
            fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<Option<u64>, Error> {
                Ok(ctx.optional_input::<u64>(A, cell)?.map(|v| *v))
            }
        }
        let without = builder().with(Tolerant).build().unwrap();
        assert_eq!(
            *without.get::<Option<u64>>(C, Cell::new(1, 0, 0)).unwrap(),
            None
        );
        let with = builder().with(Source).with(Tolerant).build().unwrap();
        assert!(
            with.get::<Option<u64>>(C, Cell::new(1, 0, 0))
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn asking_for_the_wrong_type_or_layer_is_an_error_not_a_panic() {
        let engine = builder().with(Source).build().unwrap();
        let cell = Cell::new(0, 0, 0);
        assert_eq!(
            engine.get::<String>(A, cell).err(),
            Some(Error::WrongType { layer: A })
        );
        assert_eq!(
            engine.get::<u64>(B, cell).err(),
            Some(Error::UnknownLayer(B))
        );
    }

    #[test]
    fn the_identity_names_the_seed_and_every_layer_version() {
        let engine = builder().with(Doubler).with(Source).build().unwrap();
        assert_eq!(engine.identity(), (7, vec![("a", 1), ("b", 1)]));
    }
}
