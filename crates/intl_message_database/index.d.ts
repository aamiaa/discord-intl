/* auto-generated by NAPI-RS */
/* eslint-disable */
export declare class IntlMessagesDatabase {
  constructor()
  processDefinitionsFile(filePath: string, locale?: string | undefined | null): string
  processDefinitionsFileContent(filePath: string, content: string, locale?: string | undefined | null): string
  processAllTranslationFiles(localeMap: Record<string, string>): void
  processTranslationFile(filePath: string, locale: string): string
  processTranslationFileContent(filePath: string, locale: string, content: string): string
  getKnownLocales(): Array<string>
  getSourceFile(filePath: string): IntlSourceFile
  getAllSourceFilePaths(): Array<string>
  /**
   * Return a map of all message keys contained in the given source file, where the key of the
   * map is the hashed name and the value is the original.
   */
  getSourceFileKeyMap(filePath: string): Record<string, string>
  getMessage(key: string): IntlMessage
  generateTypes(sourceFilePath: string, outputFilePath: string, allowNullability?: boolean | undefined | null): void
  precompile(filePath: string, locale: string, outputPath: string, options?: IntlMessageBundlerOptions | undefined | null): void
  precompileToBuffer(filePath: string, locale: string, options?: IntlMessageBundlerOptions | undefined | null): Buffer
  validateMessages(): Array<IntlDiagnostic>
  exportTranslations(fileExtension?: string | undefined | null): Array<string>
  getSourceFileMessageValues(filePath: string): Record<string, IntlMessageValue | undefined>
}

export declare function hashMessageKey(key: string): string

export declare const enum IntlCompiledMessageFormat {
  Json = 0,
  KeylessJson = 1
}

export interface IntlDiagnostic {
  key: string
  file: string
  line: number
  col: number
  locale: string
  severity: string
  description: string
  help?: string
}

export interface IntlMessage {
  /** Original, plain text name of the message given in its definition. */
  key: string
  /** Hashed version of the key, used everywhere for minification and obfuscation. */
  hashedKey: string
  /** Map of all translations for this message, including the default. */
  translations: Record<string, IntlMessageValue>
  /** The source definition information for this message (locale and location). */
  sourceLocale?: string
  /** Meta information about how to handle and process this message. */
  meta: IntlMessageMeta
}

export interface IntlMessageBundlerOptions {
  format?: IntlCompiledMessageFormat
  bundleSecrets?: boolean
}

export interface IntlMessageMeta {
  secret: boolean
  translate: boolean
  translationsPath: string
}

export interface IntlMessageValue {
  raw: string
  parsed: object
  variables: object
  filePosition: object
}

export interface IntlSourceFile {
  type: string
  file: string
  messageKeys: Array<number>
  meta: IntlMessageMeta
  locale?: number
}

export declare function isMessageDefinitionsFile(key: string): boolean

export declare function isMessageTranslationsFile(key: string): boolean

