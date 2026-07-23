use proj::{ProjectManager, SimpleManifester};

pub struct MdProjectManager {}

pub struct MdManifest {}

impl ProjectManager for MdProjectManager {
    type Manifester = SimpleManifester<MdManifest>;
}
