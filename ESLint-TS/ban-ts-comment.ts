interface Options {
  'ts-expect-error'?: boolean | 'allow-with-description';
  'ts-ignore'?: boolean | 'allow-with-description';
  'ts-nocheck'?: boolean | 'allow-with-description';
  'ts-check'?: boolean | 'allow-with-description';
  minimumDescriptionLength?: number;
}

const defaultOptions: Options = {
  'ts-expect-error': 'allow-with-description',
  'ts-ignore': true,
  'ts-nocheck': true,
  'ts-check': false,
  minimumDescriptionLength: 3,
};
