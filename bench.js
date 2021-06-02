const Benchmark = require("tiny-benchy");
const prettier = require('prettier');
const dprint = require('./');

let input = `
function Example() {
  let alertDismiss = (close) => {
    close();
    alert('Dialog dismissed.');
  };
  return (
    <DialogTrigger isDismissable>
      <ActionButton>Info</ActionButton>
      {(close) => (
        <Dialog onDismiss={() => alertDismiss(close)}>
          <Heading>Version Info</Heading>
          <Divider />
          <Content>
            <Text>Version 1.0.0, Copyright 2020</Text>
          </Content>
        </Dialog>
      )}
    </DialogTrigger>
  );
}
`;

let suite = new Benchmark();

suite.add('prettier', () => {
  prettier.format(input, {parser: 'babel'});
});

suite.add('dprint', () => {
  dprint.format('input.js', input);
});

suite.run();
