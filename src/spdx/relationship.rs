use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Relationship {
    pub spdx_element_id: String,
    pub related_spdx_element: String,
    pub relationship_type: RelationshipType,
    pub comment: Option<String>,
}

impl Relationship {
    pub fn new(
        spdx_element_id: String,
        related_spdx_element: String,
        relationship_type: RelationshipType,
        comment: Option<String>,
    ) -> Self {
        Self {
            spdx_element_id,
            related_spdx_element,
            relationship_type,
            comment,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipType {
    Describes,
    DescribedBy,
    Contains,
    ContainedBy,
    DependsOn,
    DependencyOf,
    DependencyManifestOf,
    BuildDependencyOf,
    DevDependencyOf,
    OptionalDependencyOf,
    ProvidedDependencyOf,
    TestDependencyOf,
    ExampleOf,
    Generates,
    GeneratedFrom,
    AncestorOf,
    DescendantOf,
    VariantOf,
    DistributionArtifact,
    PatchFor,
    PatchApplied,
    CopyOf,
    FileAdded,
    FileDeleted,
    FileModified,
    ExpandedFromArchive,
    DynamicLink,
    StaticLink,
    DataFileOf,
    TestCaseOf,
    BuildToolOf,
    DevToolOf,
    TestOf,
    TestToolOf,
    DocumentationOf,
    OptionalComponentOf,
    MetafileOf,
    PackageOf,
    Amends,
    PrerequisiteFor,
    HasPrerequisite,
    Other,
}
