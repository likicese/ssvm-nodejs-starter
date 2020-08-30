const { read_file, edit_file} = require('../pkg/ssvm_nodejs_starter_lib.js');
console.log( read_file("/hello.txt") );
console.log( edit_file(["小r", "rust真好玩"]) );
// del_file("/hello.txt");