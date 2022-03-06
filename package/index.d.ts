/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function buildGifFromDir(inDir: string, outDir: string, outName: string): void
