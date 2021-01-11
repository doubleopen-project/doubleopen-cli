use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub spdx_element_id: String,
    pub related_spdx_element: String,
    pub relationship_type: RelationshipType,
    pub comment: Option<String>,
}

impl Relationship {
    pub fn new(
        spdx_element_id: &str,
        related_spdx_element: &str,
        relationship_type: RelationshipType,
        comment: Option<String>,
    ) -> Self {
        Self {
            spdx_element_id: spdx_element_id.to_string(),
            related_spdx_element: related_spdx_element.to_string(),
            relationship_type,
            comment,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
