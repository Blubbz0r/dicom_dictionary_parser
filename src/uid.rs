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
    pub value: String,
    pub name: String,
    pub kind: Kind,
}

impl UID {
    pub fn new() -> Self {
        UID {
            value: String::new(),
            name: String::new(),
            kind: Kind::TransferSyntax,
        }
    }
}
