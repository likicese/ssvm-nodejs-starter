const express = require('express');
const { read_file, edit_file } = require('../pkg/ssvm_nodejs_starter_lib.js');

const app = express();
const port = 3006;
app.use(express.static(__dirname + '/public'));
app.use(express.urlencoded({ extended: false }));

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/get_message', function (req, res) {
    res.send(get_html_meg(read_file()))
})

app.post('/set_message', function (req, res) {
  var a = req.body.a;
  var b = req.body.b;
  if (a == "" | b == ""){
      res.send("不能发空云朵哦");
  } else {
    res.send(get_html_meg(edit_file([a, b])));
  }
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))

function get_html_meg(m){
    content_list = m.split("\n");
    var ci = "";
    for(i = 0; i < content_list.length - 1; i = i + 2){
        ci1 = '<p class="message_p"><font style="font-weight:bold">' + content_list[i] + ':</font><br>';
        ci2 = content_list[i + 1] + '</p>';
        ci = '<div class="message_div">' + ci1 + ci2 + '</p></div>' + ci;
    }
    return ci;
}