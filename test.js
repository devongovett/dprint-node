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

console.log(dprint.format('test.js', input, {lineWidth: 80}));
