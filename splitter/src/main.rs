struct PDFPath {
    in_path: String,
    out_path: String,
    range_first: u32,
    range_last: u32,
    case_name: String,
}

impl PDFPath {
    fn new(
        in_path: String,
        out_path: String,
        range_first: u32,
        range_last: u32,
        case_name: String,
    ) -> Self {
        PDFPath {
            in_path,
            out_path,
            range_first,
            range_last,
            case_name,
        }
    }

    fn out_path(&self) -> String {
        format!(
            "{}_{}-{}_{}.pdf",
            self.out_path, self.range_first, self.range_last, self.case_name
        )
    }

    fn split(&self) -> Result<(), PdfSplitError> {
        split_pdf_range(
            self.in_path.as_str(),
            self.out_path().as_str(),
            self.range_first,
            self.range_last,
        )
    }
}

fn main() {
    let in_path = "../pdf/bges.pdf";
    let out_path = "../out_pdf/bges";
    let paths = vec![
        // BGE 80 II 26 („Fall Seelig")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            4,
            14,
            "Fall_Seelig".to_string(),
        ),
        // BGE 82 II 25 ff. („Lyceum Alpinum")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            15,
            21,
            "Lyceum_Alpinum".to_string(),
        ),
        // BGE 92 II 15 ff. („Der übergriffige Arzt")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            22,
            27,
            "Der_uebergriffige_Arzt".to_string(),
        ),
        // BGE 98 II 109 ff. („Institut Rosenberg")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            28,
            30,
            "Institut_Rosenberg".to_string(),
        ),
        // BGE 99 II 39 ff. („Gasthaus Gemsli")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            31,
            35,
            "Gasthaus_Gemsli".to_string(),
        ),
        // BGE 105 II 16 ff. („Ätznatron-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            36,
            40,
            "Aetznatron_Fall".to_string(),
        ),
        // BGE 105 II 23 ff. („Juwelier Nussberger")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            41,
            44,
            "Juwelier_Nussberger".to_string(),
        ),
        // BGE 105 II 75 ff. („Escophon")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            45,
            50,
            "Escophon".to_string(),
        ),
        // BGE 110 II 456 ff. („Schachtrahmen-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            51,
            57,
            "Schachtrahmen_Fall".to_string(),
        ),
        // BGE 111 II 352 ff. („Die Mini-Verdampfungsanlage")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            58,
            60,
            "Mini_Verdampfungsanlage".to_string(),
        ),
        // BGE 111 II 471 ff. („Die falsche Kreditauskunft")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            61,
            67,
            "Falsche_Kreditauskunft".to_string(),
        ),
        // BGE 113 II 49 ff. („Der Mäklervertrag")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            68,
            70,
            "Maeklervertrag".to_string(),
        ),
        // BGE 113 II 163 ff. („Globalzessions-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            71,
            74,
            "Globalzessions_Fall".to_string(),
        ),
        // BGE 114 II 131 ff. („Picasso-Entscheid")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            75,
            83,
            "Picasso_Entscheid".to_string(),
        ),
        // BGE 116 II 422 ff. („Plauschbad-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            84,
            87,
            "Plauschbad_Fall".to_string(),
        ),
        // BGE 116 II 431 ff. („Fiat Panorama")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            88,
            91,
            "Fiat_Panorama".to_string(),
        ),
        // BGE 116 II 519 ff. („Genugtuung bei Hirnschädigung")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            92,
            96,
            "Genugtuung_Hirnschaedigung".to_string(),
        ),
        // BGE 116 II 695 ff. („Paradieserhof")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            97,
            100,
            "Paradieserhof".to_string(),
        ),
        // BGE 119 II 232 ff. („Postanweisungs-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            101,
            103,
            "Postanweisungs_Fall".to_string(),
        ),
        // BGE 120 II 197 ff. („H Sport")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            104,
            110,
            "H_Sport".to_string(),
        ),
        // BGE 120 II 331 ff. („Swissair-Entscheid")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            111,
            117,
            "Swissair_Entscheid".to_string(),
        ),
        // BGE 121 III 358 ff. („Churwaldner Skipisten-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            118,
            121,
            "Churwaldner_Skipisten_Fall".to_string(),
        ),
        // ZBGR 77/1996, S. 338 ff. („Sparheft-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            122,
            124,
            "Sparheft_Fall".to_string(),
        ),
        // BGE 123 III 16 ff. („Die Praxisübernahme I")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            125,
            130,
            "Praxisuebernahme_I".to_string(),
        ),
        // BGE 125 III 353 ff. („Die angedrohte Strafanzeige")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            131,
            134,
            "Angedrohte_Strafanzeige".to_string(),
        ),
        // BGE 126 III 75 ff. („Die schwangere Opernsängerin")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            135,
            136,
            "Schwangere_Opernsaengerin".to_string(),
        ),
        // BGE 129 III 35 („Verein gegen Tierfabriken")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            137,
            145,
            "Verein_gegen_Tierfabriken".to_string(),
        ),
        // BGE 129 III 209 ff. („Das ewige Kaufrecht")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            146,
            150,
            "Ewige_Kaufrecht".to_string(),
        ),
        // BGE 129 III 331 ff. („Schädigung von Bäumen")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            151,
            153,
            "Schaedigung_von_Baeumen".to_string(),
        ),
        // BGE 129 III 604 ff. („Le téléphone rose")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            154,
            162,
            "Le_telephone_rose".to_string(),
        ),
        // BGE 130 III 345 ff. („Liegenschaftsschätzer-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            163,
            166,
            "Liegenschaftsschaetzer_Fall".to_string(),
        ),
        // BGE 133 III 356 ff. („Kreditkarten-Fall")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            167,
            171,
            "Kreditkarten_Fall".to_string(),
        ),
        // BGE 135 III 1 ff. („Die ungewöhnlichen Versicherungsbedingungen")
        PDFPath::new(
            in_path.to_string(),
            out_path.to_string(),
            172,
            180,
            "Ungewoehnliche_Versicherungsbedingungen".to_string(),
        ),
    ];

    paths.iter().for_each(|p| p.split().unwrap());
}

use lopdf::{Document, Object, ObjectId, dictionary};
use std::collections::BTreeMap;

use std::path::Path;

#[derive(Debug)]
pub enum PdfSplitError {
    IoError(std::io::Error),
    PdfError(lopdf::Error),
    InvalidRange(String),
}

impl From<std::io::Error> for PdfSplitError {
    fn from(err: std::io::Error) -> Self {
        PdfSplitError::IoError(err)
    }
}

impl From<lopdf::Error> for PdfSplitError {
    fn from(err: lopdf::Error) -> Self {
        PdfSplitError::PdfError(err)
    }
}

impl std::fmt::Display for PdfSplitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfSplitError::IoError(e) => write!(f, "IO error: {}", e),
            PdfSplitError::PdfError(e) => write!(f, "PDF error: {}", e),
            PdfSplitError::InvalidRange(e) => write!(f, "Invalid range: {}", e),
        }
    }
}

impl std::error::Error for PdfSplitError {}

/// Splits a PDF file extracting pages from `start_page` to `end_page` (inclusive, 1-indexed)
///
/// # Arguments
/// * `input_path` - Path to the input PDF file
/// * `output_path` - Path where the split PDF will be saved
/// * `start_page` - First page to include (1-indexed)
/// * `end_page` - Last page to include (1-indexed)
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(PdfSplitError)` if an error occurs
///
/// # Example
/// ```rust
/// // Extract pages 5-10 from input.pdf and save as output.pdf
/// split_pdf_range("input.pdf", "output.pdf", 5, 10)?;
/// ```
pub fn split_pdf_range<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    start_page: u32,
    end_page: u32,
) -> Result<(), PdfSplitError> {
    // Validate input parameters
    if start_page == 0 {
        return Err(PdfSplitError::InvalidRange(
            "Page numbers are 1-indexed, start_page cannot be 0".to_string(),
        ));
    }

    if end_page == 0 {
        return Err(PdfSplitError::InvalidRange(
            "Page numbers are 1-indexed, end_page cannot be 0".to_string(),
        ));
    }

    if start_page > end_page {
        return Err(PdfSplitError::InvalidRange(format!(
            "start_page ({}) cannot be greater than end_page ({})",
            start_page, end_page
        )));
    }

    // Load the input PDF document
    let input_doc = Document::load(input_path)?;

    // Get the total number of pages
    let total_pages = input_doc.get_pages().len() as u32;

    if start_page > total_pages {
        return Err(PdfSplitError::InvalidRange(format!(
            "start_page ({}) exceeds total pages ({})",
            start_page, total_pages
        )));
    }

    if end_page > total_pages {
        return Err(PdfSplitError::InvalidRange(format!(
            "end_page ({}) exceeds total pages ({})",
            end_page, total_pages
        )));
    }

    // Get all page IDs from the input document
    let page_ids: Vec<ObjectId> = input_doc
        .get_pages()
        .into_iter()
        .map(|(_, object_id)| object_id)
        .collect();

    // Create a new document for the output
    let mut output_doc = Document::with_version("1.4");

    // Copy document info and catalog from original
    if let Ok(info) = input_doc.trailer.get(b"Info") {
        if let Ok(info_id) = info.as_reference() {
            if let Ok(info_obj) = input_doc.get_object(info_id) {
                let new_info_id = output_doc.add_object(info_obj.clone());
                output_doc
                    .trailer
                    .set("Info", Object::Reference(new_info_id));
            }
        }
    }

    // Collect page references for the specified range (convert to 0-indexed)
    let start_idx = (start_page - 1) as usize;
    let end_idx = (end_page - 1) as usize;
    let selected_pages = &page_ids[start_idx..=end_idx];

    // Copy each page and its dependencies
    let mut new_page_ids = Vec::new();
    for &page_id in selected_pages {
        let new_page_id = copy_page_and_dependencies(&input_doc, &mut output_doc, page_id)?;
        new_page_ids.push(new_page_id);
    }

    // Create the pages tree for the output document
    create_pages_tree(&mut output_doc, new_page_ids)?;

    // Save the output document
    output_doc.save(output_path.as_ref())?;

    println!(
        "Successfully extracted pages {}-{} to {:?}",
        start_page,
        end_page,
        output_path.as_ref()
    );

    Ok(())
}

fn copy_page_and_dependencies(
    input_doc: &Document,
    output_doc: &mut Document,
    page_id: ObjectId,
) -> Result<ObjectId, lopdf::Error> {
    let mut copied_objects = BTreeMap::new();
    copy_object_recursive(input_doc, output_doc, page_id, &mut copied_objects)
}

fn copy_object_recursive(
    input_doc: &Document,
    output_doc: &mut Document,
    obj_id: ObjectId,
    copied_objects: &mut BTreeMap<ObjectId, ObjectId>,
) -> Result<ObjectId, lopdf::Error> {
    // If we've already copied this object, return the new ID
    if let Some(&new_id) = copied_objects.get(&obj_id) {
        return Ok(new_id);
    }

    // Reserve a spot for this object to prevent infinite recursion
    let placeholder_id = output_doc.add_object(Object::Null);
    copied_objects.insert(obj_id, placeholder_id);

    // Get the object from the input document
    let obj = input_doc.get_object(obj_id)?.clone();

    // Recursively copy any referenced objects
    let new_obj = copy_object_references(input_doc, output_doc, obj, copied_objects)?;

    // Replace the placeholder with the actual object
    if let Ok(placeholder) = output_doc.get_object_mut(placeholder_id) {
        *placeholder = new_obj;
    }

    Ok(placeholder_id)
}

fn copy_object_references(
    input_doc: &Document,
    output_doc: &mut Document,
    mut obj: Object,
    copied_objects: &mut BTreeMap<ObjectId, ObjectId>,
) -> Result<Object, lopdf::Error> {
    match &mut obj {
        Object::Reference(ref_id) => {
            // Check if we've already processed this reference
            if let Some(&new_ref_id) = copied_objects.get(ref_id) {
                Ok(Object::Reference(new_ref_id))
            } else {
                let new_ref_id =
                    copy_object_recursive(input_doc, output_doc, *ref_id, copied_objects)?;
                Ok(Object::Reference(new_ref_id))
            }
        }
        Object::Dictionary(dict) => {
            let mut new_dict = lopdf::Dictionary::new();
            for (key, value) in dict.iter() {
                let new_value =
                    copy_object_references(input_doc, output_doc, value.clone(), copied_objects)?;
                new_dict.set(key.clone(), new_value);
            }
            Ok(Object::Dictionary(new_dict))
        }
        Object::Array(array) => {
            let mut new_array = Vec::new();
            for item in array.iter() {
                let new_item =
                    copy_object_references(input_doc, output_doc, item.clone(), copied_objects)?;
                new_array.push(new_item);
            }
            Ok(Object::Array(new_array))
        }
        Object::Stream(stream) => {
            let mut new_dict = lopdf::Dictionary::new();
            for (key, value) in stream.dict.iter() {
                let new_value =
                    copy_object_references(input_doc, output_doc, value.clone(), copied_objects)?;
                new_dict.set(key.clone(), new_value);
            }
            Ok(Object::Stream(lopdf::Stream::new(
                new_dict,
                stream.content.clone(),
            )))
        }
        _ => Ok(obj),
    }
}

fn create_pages_tree(
    output_doc: &mut Document,
    page_ids: Vec<ObjectId>,
) -> Result<(), lopdf::Error> {
    // Create the pages dictionary
    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Count" => page_ids.len() as i64,
        "Kids" => page_ids.iter().map(|&id| Object::Reference(id)).collect::<Vec<_>>(),
    };

    let pages_id = output_doc.add_object(Object::Dictionary(pages_dict));

    // Update each page to reference the pages tree
    for &page_id in &page_ids {
        if let Ok(Object::Dictionary(page_dict)) = output_doc.get_object_mut(page_id) {
            page_dict.set("Parent", Object::Reference(pages_id));
        }
    }

    // Create the catalog
    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };

    let catalog_id = output_doc.add_object(Object::Dictionary(catalog));
    output_doc
        .trailer
        .set("Root", Object::Reference(catalog_id));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_ranges() {
        // Test zero page numbers
        assert!(split_pdf_range("test.pdf", "out.pdf", 0, 5).is_err());
        assert!(split_pdf_range("test.pdf", "out.pdf", 1, 0).is_err());
    }
}
