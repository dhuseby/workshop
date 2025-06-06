/// Errors generated from this module
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Workshop data not found
    #[error("Workshop data not found for {0}")]
    WorkshopNotFound(String),

    /// Workshop programming language not found
    #[error("Workshop programming language not found for {0}")]
    WorkshopProgrammingLanguageNotFound(String),

    /// Workshop spoken language not found
    #[error("Workshop spoken language not found for {0}")]
    WorkshopSpokenLanguageNotFound(String),

    /// Workshop no spoken languages
    #[error("Workshop has no spoken languages")]
    WorkshopNoSpokenLanguages,

    /// Workshop no setup instructions
    #[error("Workshop has no setup instructions")]
    WorkshopNoSetupInstructions,

    /// Workshop no descriptions
    #[error("Workshop has no descriptions")]
    WorkshopNoDescriptions,

    /// Workshop no metadata
    #[error("Workshop has no metadata")]
    WorkshopNoMetadata,

    /// Workshop no lessons data
    #[error("Workshop has no lessons data")]
    WorkshopNoLessonsData,

    /// Workshop data programming dir not found
    #[error("Workshop data programming dir not found for {0}")]
    WorkshopDataProgrammingDirNotFound(String),

    /// Workshop no programming languages for spoken language
    #[error("Workshop has no programming languages for spoken language {0}")]
    WorkshopNoProgrammingLanguagesForSpokenLanguage(String),

    /// workshop license not found
    #[error("Workshop license not found for {0}")]
    WorkshopLicenseNotFound(String),

    /// workshop defaults not found
    #[error("Workshop defaults not found for {0}")]
    WorkshopDefaultsNotFound(String),

    /// workshop data spoken dir not found
    #[error("Workshop data spoken dir not found for {0}")]
    WorkshopDataSpokenDirNotFound(String),

    /// Workshop data directory not found
    #[error("Workshop data directory not found")]
    WorkshopDataDirNotFound,

    /// No workshop specified
    #[error("No workshop specified")]
    NoWorkshopSpecified,

    /// No programming language specified
    #[error("No programming language specified")]
    NoProgrammingLanguageSpecified,

    /// No spoken language specified
    #[error("No spoken language specified")]
    NoSpokenLanguageSpecified,

    /// Lesson data directory not found
    #[error("Lesson data directory not found")]
    LessonDataDirNotFound,

    /// Lesson metadata not found
    #[error("Lesson metadata not found")]
    LessonMetadataFileMissing,

    /// Lesson text not found
    #[error("Lesson text not found")]
    LessonTextFileMissing,

    /// No lesson data
    #[error("No lesson data found: {0}")]
    NoLessonData(String),

    /// No lesson specified
    #[error("No lesson specified")]
    NoLessonSpecified,
}
