/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface IntlDiagnostic {
  key: string
  diagnostics: unknown
}
export interface IntlSourceFile {
  type: string
  file: string
  messageKeys: Array<number>
  meta: IntlMessageMeta
  locale?: number
}
export interface IntlMessageMeta {
  secret: boolean
  translate: boolean
  bundleSecrets: boolean
  translationsPath: string
}
export const enum IntlCompiledMessageFormat {
  Json = 0,
  KeylessJson = 1
}
export function hashMessageKey(key: string): string
export function isMessageDefinitionsFile(key: string): boolean
export function isMessageTranslationsFile(key: string): boolean
export function resolveSymbol(symbol: number): string | null
export class IntlMessagesDatabase {
  constructor()
  processDefinitionsFile(filePath: string): number
  processDefinitionsFileContent(filePath: string, content: string): number
  processAllTranslationFiles(localeMap: Record<string, string>): void
  processTranslationFileContent(filePath: string, locale: string, content: string): number
  getKnownLocales(): Array<string>
  getSourceFile(filePath: string): IntlSourceFile
  getAllSourceFilePaths(): Array<string>
  getSourceFileHashedKeys(filePath: string): string[]
  getMessage(key: string): unknown
  generateTypes(sourceFilePath: string, outputFilePath: string): void
  precompile(locale: string, outputPath: string, format?: IntlCompiledMessageFormat | undefined | null): void
  precompileToBuffer(locale: string, format?: IntlCompiledMessageFormat | undefined | null): Buffer
  validateMessages(): Array<IntlDiagnostic>
}
