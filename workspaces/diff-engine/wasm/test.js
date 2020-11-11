const DiffEngine = require('.');
const Path = require('path');
const Fs = require('fs-extra');
const Readline = require('readline');

DiffEngine.init();

let exampleSessionPath = Path.join(
  __dirname,
  '../tests/fixtures/ergast-example-spec.json'
);
let exampleInteractionsPath = Path.join(
  __dirname,
  '../tests/fixtures/ergast-captures/ergast-simulated-traffic.jsonl'
);

let rawSpec = Fs.readFileSync(exampleSessionPath);

let spec = DiffEngine.spec_from_events(rawSpec.toString());

async function diffInteractions(source) {
  let lines = Readline.createInterface({ input: source, crlfDelay: Infinity });

  let i = 0;
  for await (const line of lines) {
    const diffs = DiffEngine.diff_interaction(line, spec);
    console.log(`\n\ndiffs for line ${i}:`);
    console.log(diffs);
    i++;
  }
}

diffInteractions(Fs.createReadStream(exampleInteractionsPath));
