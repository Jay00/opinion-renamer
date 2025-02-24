use chrono::NaiveDate;
use pdfium_render::prelude::*;
use std::{fmt::format, path::Path};

use super::extract::Opinion;

pub fn watermark_case(
    mut document: PdfDocument,
    output_path: &(impl AsRef<Path> + ?Sized),
    opiniun: Opinion,
) -> Result<(), PdfiumError> {
    // let total_pages = document.pages().len();
    // let pages = document.pages();

    let font = document.fonts_mut().helvetica();
    let font_size: f32 = 10.0;
    let font_color: PdfColor = PdfColor::new(16, 170, 50, 255);

    document.pages().watermark(|group, index, width, height| {
        let s = format!("{}", &opiniun.to_date_medium());
        // Reporter
        let mut reporter_obj =
            PdfPageTextObject::new(&document, &s, font, PdfPoints::new(font_size))?;

        reporter_obj.set_fill_color(font_color)?;

        reporter_obj.rotate_clockwise_degrees(0.0)?;

        ///// BOTTOM ////////////////////////////////////////////////////////////
        let x = PdfPoints::new((width.value / 2.0) - (reporter_obj.width().unwrap().value / 2.0));
        let y = PdfPoints::new(reporter_obj.height().unwrap().value + 5.0);
        reporter_obj.translate(x, y)?;

        let reporter_block_height = reporter_obj.height().unwrap().value;

        group.push(&mut reporter_obj.into())?;

        // The Caption Watermark
        let full_string = format!("{}, {}", &opiniun.caption, &opiniun.reporter);

        // println!("Current page number: {}", (index + 1));

        let mut caption_obj =
            PdfPageTextObject::new(&document, &full_string, font, PdfPoints::new(font_size))?;

        caption_obj.set_fill_color(font_color)?;

        caption_obj.rotate_clockwise_degrees(0.0)?;

        let x = PdfPoints::new((width.value / 2.0) - (caption_obj.width().unwrap().value / 2.0));
        let y =
            PdfPoints::new(caption_obj.height().unwrap().value + 5.0 + reporter_block_height + 3.0);
        caption_obj.translate(x, y)?;

        group.push(&mut caption_obj.into())?;

        /////////////////////// TOP

        // Caption Top
        let mut top_obj =
            PdfPageTextObject::new(&document, &full_string, font, PdfPoints::new(font_size))?;

        top_obj.set_fill_color(font_color)?;

        top_obj.rotate_clockwise_degrees(0.0)?;

        let x = PdfPoints::new((width.value / 2.0) - (top_obj.width().unwrap().value / 2.0));
        let y = PdfPoints::new(height.value - top_obj.height().unwrap().value - 5.0);
        top_obj.translate(x, y)?;

        let top_caption_height = top_obj.height().unwrap().value;

        group.push(&mut top_obj.into())?;

        let mut top_date_obj = PdfPageTextObject::new(
            &document,
            &opiniun.to_date_medium(),
            font,
            PdfPoints::new(font_size),
        )?;

        top_date_obj.set_fill_color(font_color)?;

        top_date_obj.rotate_clockwise_degrees(0.0)?;

        let x = PdfPoints::new((width.value / 2.0) - (top_date_obj.width().unwrap().value / 2.0));
        let y = PdfPoints::new(
            height.value - top_date_obj.height().unwrap().value - 5.0 - top_caption_height - 3.0,
        );
        top_date_obj.translate(x, y)?;

        group.push(&mut top_date_obj.into())?;

        let mut top_date_obj =
            PdfPageTextObject::new(&document, &opiniun.court, font, PdfPoints::new(7.0))?;

        top_date_obj.set_fill_color(font_color)?;

        top_date_obj.rotate_clockwise_degrees(0.0)?;

        let x = PdfPoints::new((width.value / 2.0) - (top_date_obj.width().unwrap().value / 2.0));
        let y = PdfPoints::new(
            height.value
                - top_date_obj.height().unwrap().value
                - 5.0
                - top_caption_height
                - 4.0
                - top_caption_height,
        );
        top_date_obj.translate(x, y)?;

        group.push(&mut top_date_obj.into())?;

        // SIDE //

        let mut top_date_obj =
            PdfPageTextObject::new(&document, &opiniun.caption, font, PdfPoints::new(font_size))?;

        top_date_obj.set_fill_color(font_color)?;

        top_date_obj.rotate_clockwise_degrees(90.0)?;

        let x = PdfPoints::new(width.value - top_date_obj.width().unwrap().value - 5.0);
        let y = PdfPoints::new((height.value / 2.0) + (top_date_obj.height().unwrap().value / 2.0));
        top_date_obj.translate(x, y)?;

        group.push(&mut top_date_obj.into())?;

        // OTHER SIDE
        // let mut top_date_obj =
        //     PdfPageTextObject::new(&document, &opiniun.court, font, PdfPoints::new(font_size))?;

        // top_date_obj.set_fill_color(font_color)?;

        // top_date_obj.rotate_clockwise_degrees(270.0)?;

        // let x = PdfPoints::new(top_date_obj.width().unwrap().value);
        // let y = PdfPoints::new((height.value / 2.0) - (top_date_obj.height().unwrap().value / 2.0));
        // top_date_obj.translate(x, y)?;

        // group.push(&mut top_date_obj.into())?;

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

    #[test]
    fn watermark() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let file_path = std::path::Path::new("./Curry v. United States.pdf");
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();
        let op = crate::rename::extract::extract_data_from_pdf(&document).unwrap();

        let _ = watermark_case(document, std::path::Path::new("./out_curry.pdf"), op);

        assert!(true);
    }

    #[test]
    fn watermark_Le() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let file_path = std::path::Path::new("./Lesher v. United States.pdf");
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();
        let op = crate::rename::extract::extract_data_from_pdf(&document).unwrap();

        let _ = watermark_case(document, std::path::Path::new("./out_curry.pdf"), op);

        assert!(true);
    }

    #[test]
    fn watermark_hyde() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let file_path = std::path::Path::new("./Hyde v. Shine.pdf");
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();
        let op = crate::rename::extract::extract_data_from_pdf(&document).unwrap();

        let _ = watermark_case(document, std::path::Path::new("./out_shine.pdf"), op);

        assert!(true);
    }
}
