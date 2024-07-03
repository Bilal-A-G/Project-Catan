import http = require('http');
import fs = require('fs');
const port = process.env.port || 1337

http.createServer(function (req, res) {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    fs.readFile('index.html', (error, data) => {
        if (error) {
            res.writeHead(404);
            res.write("Error: file not found");
        }
        else {
            res.write(data);
            res.end();
        }
    })
}).listen(port, () => {
    console.log("Listening to server on port : " + port);
});