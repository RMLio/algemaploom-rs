const fs = require('fs')
const translator = require('.')

var mapping = fs.readFileSync(0);
console.log(translator.translate(mapping.toString()))
