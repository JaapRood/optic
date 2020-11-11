const DiffEngine = require('.');
const Path = require('path');
const Fs = require('fs-extra');

DiffEngine.init();

let exampleSessionPath = Path.join(
  __dirname,
  '../tests/fixtures/ergast-example-spec.json'
);

let rawSpec = Fs.readFileSync(exampleSessionPath);

let spec = DiffEngine.spec_from_events(rawSpec.toString());

console.log(spec);
