//! # Art
//!
//! A library for modeling artistic concepts.

// To remove the internal organization from the public API, 
// we can modify the art crate code to add pub use statements to re-export the items at the top level
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}

// Note that the PrimaryColor and SecondaryColor types aren’t listed on the front page (cargo doc), 
// nor is the mix function. We have to click kinds and utils to see them.

// In order to publish one need an api key from crates.io.
// To do this, we need to create an account on crates.io and then run the following command in the terminal:
// $ cargo login
// abcdefghijklmnopqrstuvwxyz012345 (<- api key)
// This command will inform Cargo of your API token and store it locally in ~/.cargo/credentials

// To publish the crate, we need to make sure that the version number in Cargo.toml is updated.
// along with a unique name for the crate, a description, and a license type.
// We can also add a README.md file to provide more information about the crate and its usage.
// once we have the API key, we can publish the crate using the following command:
// $ cargo publish --dry-run    // (to check if everything is ok)
// $ cargo publish             // (to publish the crate)

// To republish a crate, we need to update the version number in Cargo.toml to a new version number.
// Then just run the cargo publish command again.
// See https://semver.org/ for versioning guidelines.

// -.-.-.-.-.-.-.-.-.-.-.-.-.-

// Yanking a version prevents new projects from depending on that version while allowing all existing projects 
// that depend on it to continue. Essentially, a yank means that all projects with a Cargo.lock will not break, 
// and any future Cargo.lock files generated will not use the yanked version.

// $ cargo yank --vers 1.0.1
// $ cargo yank --vers 1.0.1 --undo