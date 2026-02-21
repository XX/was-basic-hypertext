pub mod appearance;
pub mod attributes;
pub mod components;
#[cfg(test)]
pub mod tests;
pub mod variant;

mod hypertext_elements {
    // Re-export all standard HTML elements
    pub use hypertext::validation::hypertext_elements::*;
}
