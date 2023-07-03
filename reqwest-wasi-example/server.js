var http = require('http');

//create a server object:
http.createServer(function (req, res) {
  res.write('bajja');
  res.end();
}).listen(7777);
