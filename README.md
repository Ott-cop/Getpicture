<h1>Imxeldownload</h1>

> <p>Automation to download images using Excel cell names.</p>

<p>Project made 100% in Rust 🦀 to automate image downloads using the name of each cell specified in Excel.</p>
<p>Special thanks to owners of <a href="https://crates.io/crates/calamine">Calamine</a> and <a href="https://crates.io/crates/image_search">Image_search</a> libs.</p><br>

<h2>Build</h2>
<p>To compile the script, simply download the repository and run the following command:</p>

    $ cargo build --release

<br>
<h2>Usage</h2>
<p>Follow the template for using the script:</p>

    $ imxceldownload FILE.xlsx SHEET_NAME INITIAL_LINE INITIAL_COLUMN FINAL_LINE FINAL_COLUMN

<p>When using this command it will create a folder called "downloads" and the downloaded images will be in the same folder.</p><br/>

<h2>Example</h2>

    $ imxceldownload products.xlsx ProductsSheet 3 4 20 4

<p>Using this exact command it will look for the products.xlsx file, the ProductsSheet sheet and then it will download all the images based on the cell names starting from line 3 of column 4 to line 20 of the same column.</p>
