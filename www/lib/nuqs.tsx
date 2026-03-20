import { createParser } from "nuqs";

export const parseU8 = createParser({
  parse(value) {
    const valueAsNumber = Number(value);
    if (Number.isNaN(valueAsNumber)) return null;
    if (valueAsNumber > 255) return null;
    if (valueAsNumber < 0) return null;
    return valueAsNumber;
  },
  serialize: (value) => value.toString(),
});
