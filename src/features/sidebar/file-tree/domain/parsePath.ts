export const parsePath = (value: string) => {
  return value.split(/[\\/]/).filter(Boolean);
};
