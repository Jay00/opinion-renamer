use chrono::NaiveDate;
use pdfium_render::prelude::*;
use std::path::Path;

pub fn watermark_case(
    mut document: PdfDocument,
    output_path: &(impl AsRef<Path> + ?Sized),
    date_of_opinion: NaiveDate,
) -> Result<(), PdfiumError> {
    // let total_pages = document.pages().len();
    // let pages = document.pages();

    let font = document.fonts_mut().helvetica();
    let font_size: f32 = 10.0;
    let font_color: PdfColor = PdfColor::new(16, 170, 50, 255);

    document.pages().watermark(|group, index, width, height| {
        let mut ds = date_of_opinion.format("%Y-%m-%d").to_string();
        // The Primary Watermark
        let full_string = format!("PRIMARY");

        println!("Current page number: {}", (index + 1));

        let mut watermark_obj =
            PdfPageTextObject::new(&document, &full_string, font, PdfPoints::new(font_size))?;

        watermark_obj.set_fill_color(font_color)?;

        watermark_obj.rotate_clockwise_degrees(0.0)?;

        ///// BOTTOM ////////////////////////////////////////////////////////////
        let x = PdfPoints::new((width.value / 2.0) - (watermark_obj.width().unwrap().value / 2.0));
        let y = PdfPoints::new(watermark_obj.height().unwrap().value + 5.0);
        watermark_obj.translate(x, y)?;

        group.push(&mut watermark_obj.into())?;

        Ok(())
    })?;

    let save_res = document.save_to_file(output_path);

    match save_res {
        Ok(_) => {
            // println!("PDF saved.")
        }
        Err(err) => {
            eprintln!("Failed to save to path: {:?}", output_path.as_ref());
            panic!("Failed to save Watermarked PDF. {err}");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // use std::path::Path;
    // use std::path::Pathbuf;
    #[test]
    fn pfium_binds_to_binary() {
        // BIND to PDFIUM
        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())
                .expect("Failed to Bind to pdfium"),
        );

        assert!(true);
    }
}
