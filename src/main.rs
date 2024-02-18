use std::{env, path::Path};
use calamine::{open_workbook, Reader, Xlsx, XlsxError};
use futures::future;
use image_search::{download, Arguments};




#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    let icon_success: char = char::from_u32(0x2705).unwrap();
    let icon_error: char = char::from_u32(0x274C).unwrap();

    if args.len() < 7 {
        println!("\nUSAGE:\n\n");
        println!("imxceldownload ARQUIVO.xlsx SHEET_NAME LINHA_INICIAL COLUNA_INICIAL LINHA_FINAL COLUNA_FINAL\n");
        return Err(format!("{icon_error} Digite corretamente os campos."));
    }

   

    // Open the Xlsx file
    let workbook: Result<Xlsx<_>, XlsxError> = open_workbook(format!("./{}", args[1]));
    println!("\n");
    if workbook.is_err() { return Err(format!("{icon_error} Não foi possível encontrar o arquivo especificado.")) };
    let mut workbook = workbook.unwrap();
    

    // Find the Sheet
    let range = workbook.worksheet_range(&args[2]);
    println!("\n");
    if range.is_err() { return Err(format!("{icon_error} Não foi possivel encontrar o Sheet ..")); }
    let range = range.unwrap();

    // Save the initial coordinates and final coordinates    
    let line = args[3].parse::<u32>().unwrap();
    let col = args[4].parse::<u32>().unwrap() - 1;
    let final_line = args[5].parse::<u32>().unwrap();
    let final_col = args[6].parse::<u32>().unwrap() - 1;

    let mut all_products: Vec<String> = vec![]; 
    let values = range.range((line, col), (final_line, final_col));
    
    // Add names in vec
    for i in values.used_cells() {
        let value = i.2.to_string();
        all_products.push(value);
    }

    // Download the images by name
    let tasks: Vec<_> = all_products.iter().map(|value| {
        let value = value.clone();
        tokio::spawn(async move {
            let image = Arguments::new(value.as_str(), 1).format(image_search::Format::Jpg).directory(Path::new("downloads"));
            if let Err(err) = download(image).await {
                println!("Error: {}", err)
            }
        })
    }).collect();
    future::join_all(tasks).await;

    println!("{icon_success} Operação concluída com sucesso!");

    Ok(())
}


