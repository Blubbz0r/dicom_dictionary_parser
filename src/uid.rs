#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    ApplicationContextName,
    ApplicationHostingModel,
    CodingScheme,
    DicomUidsAsCodingScheme,
    LdapOid,
    MappingResource,
    MetaSopClass,
    ServiceClass,
    SopClass,
    SynchronizationFrameOfReferences,
    TransferSyntax,
    WellKnownFrameOfReference,
    WellKnownPrinterSopInstance,
    WellKnownPrintQueueSopInstance,
    WellKnownSopInstance,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UID {
    /// The value of the UID (e.g. "1.2.840.10008.1.1" for "Verification SOP
    /// Class")
    pub value: String,

    /// The full name of the UID as given in the DICOM Standard (e.g. "Implicit
    /// VR Little Endian: Default Transfer Syntax for DICOM")
    pub full_name: String,

    /// A normalized form of the full name. The following content is trimmed
    /// from the full name:
    /// * everything behind a colon (e.g. full name "Implicit VR Little Endian:
    /// Default Transfer Syntax for DICOM" is trimmed down to "Implicit VR
    /// Little Endian")
    /// * the string " (Retired)" (e.g. "Explicit VR Big Endian (Retired)" is
    /// trimmed down to "Explicit VR Big Endian")
    ///
    /// Note that there can still be some "noise" in this due to the format of
    /// the original names. Examples: "JPEG Lossless, Non-Hierarchical (Process
    /// 14)" or "MPEG-4 AVC/H.264 High Profile / Level 4.2 For 2D Video".
    pub normalized_name: String,

    /// The type of this UID
    pub kind: Kind,
}

impl UID {
    pub fn new() -> Self {
        UID {
            value: String::new(),
            normalized_name: String::new(),
            full_name: String::new(),
            kind: Kind::TransferSyntax,
        }
    }
}
