// individual card will have - datetime, user name, avtar, content

function showComments(data) {
  data.forEach(comment => {
    createCommentCard(comment);
  });
}

function createCommentCard(comment) {
  console.log('comment :>> ', comment);
  // body = document.getElementsByTagName("body")

  div = document.createElement('div');
  // document.createElement("")
  // body[0].append(div)
  var html = `<h5 id="title">${comment.created_at}</h5><span style="display:inline-block; width=100px;">${comment.content}</span>`;
  appendHtml(document.body, html); // "body" has two more children - h1 and span.
  // document.body.appendChild(div);
  console.log("added div to the body");
}

function appendHtml(el, str) {
  var div = document.createElement('div');
  div.innerHTML = str;
  while (div.children.length > 0) {
    el.appendChild(div.children[0]);
  }
}


fetch("http://localhost:8080/api/v1/comments", {
  method: "GET",
}).then(function(response) {
  return response.json();
}).then(function(data) {
  console.log(data);
  showComments(data);
}).catch(function (err) {
	// There was an error
	console.warn('Something went wrong.', err);
});
