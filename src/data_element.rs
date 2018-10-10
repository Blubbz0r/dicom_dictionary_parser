use std::fmt;

/// A unit of information as defined by a single entry in the DICOM data dictionary.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct DataElement {
    /// Unique identifier for a data element composed of an ordered pair of
    /// numbers (a Group Number followed by an Element Number) in the format
    /// "(gggg,eeee)".
    pub tag: String,

    /// The unique name of the data element as a human-readable string (e.g.
    /// "Specific Character Set").
    pub name: String,

    /// The unique name of the data element with zero-width spaces instead of
    /// actual spaces.as one word. This can be used to e.g. transform it more
    /// easily into something like a function name or identifier. The format
    /// is: "Length\u{200b}To\u{200b}End" where "\u{200b}" is the code point
    /// for the zero-width space.
    pub keyword: String,

    /// The Value Representation of the data element as two upper-case letters.
    /// The format is: "TM".
    pub vr: String,

    /// The Value Multiplicity of the data element as single digit or range.
    /// The format is: "2-n".
    pub vm: String,

    /// Additional comment for the data element (e.g. "RET" for retired elements).
    pub comment: Option<String>,
}

impl DataElement {
    pub fn new() -> DataElement {
        Default::default()
    }
}

impl fmt::Display for DataElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tag: {}, Name: {}, Keyword: {}, VR: {}, VM: {}, Comment: {}",
            self.tag,
            self.name,
            self.keyword,
            self.vr,
            self.vm,
            match self.comment {
                Some(ref c) => c,
                None => "",
            }
        )
    }
}
