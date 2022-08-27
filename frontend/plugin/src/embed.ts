document.addEventListener('DOMContentLoaded', (_) => {
    initEmbed();
});

function getSrcDoc() {
    return `<html>
  <head>
      <base target="_parent" />
  </head>
  <body>
    <div id="root"></div>
    <script src="http://localhost:8081/plugin.es.js"></script>
    <link rel="stylesheet" href="http://localhost:8081/style.css">
  </body>
</html>`
}

function createIframe() {
    let iframe = document.createElement('iframe');
    iframe.srcdoc = getSrcDoc();
    iframe.width = "100%";
    iframe.frameBorder = "0";
    return iframe
}


function initEmbed() {

  const siteId = 'aawaz';
  const node = document.getElementById(siteId);

  if (!node) {
    console.error('Aawaz: Can\'t find root node.');
    return;
  }

    let iframe = createIframe();
    node.appendChild(iframe)
}
