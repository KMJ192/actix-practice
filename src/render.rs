pub fn render(component: String, meta: String) -> String {
  format!(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8" />
      <meta http-equiv="X-UA-Compatible" content="IE=edge" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      {}
      <title>Document</title>
    </head>
    <body>
      {}
      <main id="App"></main>
      <script src="./src/index.js" type="module"></script>
    </body>
    </html>
  "#, meta, component)
}