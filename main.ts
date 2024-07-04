import express = require('express');
const app = express();
const port = process.env.port || 3000;

app.use(express.static('public'))
app.set('view engine', 'ejs');

app.get('/', (req, res) => {
    res.render('index');
});
app.listen(port);