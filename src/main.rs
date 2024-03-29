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
        println!("getpicture ARQUIVO.xlsx SHEET_NAME LINHA_INICIAL COLUNA_INICIAL LINHA_FINAL COLUNA_FINAL\n");
        return Err(format!("{icon_error} Digite corretamente os campos."));
    }

    let i_row = &args[3].trim().parse::<u32>().unwrap();
    let i_column = &args[4].trim().parse::<u32>().unwrap();

    let f_row = &args[5].trim().parse::<u32>().unwrap();
    let f_column = &args[6].trim().parse::<u32>().unwrap();

    if *i_row > *f_row || *i_column > *f_column {
        println!("\n");
        return Err(format!("{icon_error} Os valores finais não podem ser menores que seus valores iniciais"))
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

    // Get the range
    let mut all_products: Vec<String> = vec![]; 
    let values = range.range((*i_row - 1, *i_column - 1), (*f_row - 1, *f_column - 1));
    

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
            let _ = download(image).await;
        })
    }).collect();
    
    future::join_all(tasks).await;

    println!("{icon_success} Operação concluída com sucesso!");

    Ok(())
}


