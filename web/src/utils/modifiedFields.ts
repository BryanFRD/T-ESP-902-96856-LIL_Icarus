export const getModifiedFields = <T extends Record<string, unknown>>(
  allValues: Partial<T>,
  dirtyFields: Partial<Record<keyof T, boolean>>
): Partial<T> => {
  return Object.keys(allValues).reduce((acc, key) => {
    const typedKey = key as keyof T
    if (dirtyFields[typedKey] && allValues[typedKey] !== undefined) {
      acc[typedKey] = allValues[typedKey]
    }
    return acc
  }, {} as Partial<T>)
}
