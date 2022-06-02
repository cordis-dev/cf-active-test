await 'value';
const createValue = () => 'value';
await createValue();

await Promise.resolve('value');
const createValue2 = async () => 'value';
await createValue2();
