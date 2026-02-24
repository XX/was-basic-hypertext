pub use {hypertext, was_basic_hypertext_macros as macros};

pub mod appearance;
pub mod attributes;
pub mod components;
pub mod layouts;
#[cfg(test)]
pub mod tests;
pub mod variant;

pub mod hypertext_elements {
    // Re-export all standard HTML elements
    use hypertext::define_elements;
    pub use hypertext::validation::hypertext_elements::*;

    define_elements! {
        svg { xmlns viewBox }
        path { fill d }
    }
}
