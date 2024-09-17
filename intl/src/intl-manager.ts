import { createIntl, IntlShape } from '@formatjs/intl';
import { Formats, Formatters, IntlMessageFormat } from 'intl-messageformat';

import type {
  FormatterImplementation,
  IntlMessageGetter,
  RequiredFormatValues,
  TypedIntlMessageGetter,
} from './types';
import { InternalIntlMessage } from './message';
import { bindFormatValues, FormatBuilderConstructor } from './format';

/**
 * Fallback locale used for all internationalization when an operation in the
 * requested locale is not possible.
 */
export const DEFAULT_LOCALE: string = 'en-US';

type FormatterReturnType<F extends FormatterImplementation<any, any>> = ReturnType<F['format']>;

type FormatterFunctionTypes<F extends FormatterImplementation<any, any>> =
  F extends FormatterImplementation<infer Functions, any> ? Functions : never;

export type FormatFunction<F extends FormatterImplementation<any, any>> = <
  T extends IntlMessageGetter,
>(
  this: IntlManager,
  message: T,
  values: RequiredFormatValues<T, FormatterFunctionTypes<F>>,
) => FormatterReturnType<F>;

type ThisWithFormatters<
  This,
  T extends Record<string, FormatterImplementation<any, any>>,
> = This & {
  [K in keyof T]: FormatFunction<T[K]>;
};

export class IntlManager {
  defaultLocale: string;
  currentLocale: string;
  intl: IntlShape;

  _localeSubscriptions: Set<(locale: string) => void>;

  constructor(defaultLocale: string = DEFAULT_LOCALE) {
    this.defaultLocale = defaultLocale;
    this.currentLocale = defaultLocale;
    this.intl = createIntl({
      formats: IntlMessageFormat.formats,
      defaultLocale,
      locale: defaultLocale,
    });

    this._localeSubscriptions = new Set();
  }

  /**
   * Add a set of formatter implementations to this manager, making each available as a direct
   * property
   */
  withFormatters<const T extends Record<string, FormatterImplementation<any, any>>>(
    formatters: T,
  ): ThisWithFormatters<this, T> {
    for (const [name, formatter] of Object.entries(formatters)) {
      this[name] = this.makeFormatFunction(formatter);
    }

    return this as ThisWithFormatters<this, T>;
  }

  /**
   * Return a new function bound to this manager that uses the given `FormatterImplementation` to
   * format a message after it has been resolved for the current locale and potentially processed in
   * other ways by the manager.
   */
  makeFormatFunction<F extends FormatterImplementation<any, any>>({
    format,
    builder,
  }: F): FormatFunction<FormatterReturnType<F>> {
    const formatter = format.bind(this);
    return (message, values) => {
      return formatter(message(this.currentLocale), values, builder);
    };
  }

  /**
   * Set the locale for this package to use. This is a global change, and all
   * methods on this object will use this locale for formatting.
   */
  setLocale(locale: string) {
    this.currentLocale = locale;
    this.intl = createIntl({ defaultLocale: this.defaultLocale, locale });
    this.emitLocaleChange(locale);
  }

  emitLocaleChange(locale: string) {
    for (const callback of this._localeSubscriptions) {
      callback(locale);
    }
  }

  /**
   * Subscribe to changes in the current locale for this manager.
   */
  onLocaleChange = (callback: (locale: string) => void) => {
    this._localeSubscriptions.add(callback);
    return () => this._localeSubscriptions.delete(callback);
  };

  /**
   * For static messages with no rich text and no dynamic placeholders, use this method to
   * immediately return the plain string value of the message in the current locale.
   */
  string<T extends TypedIntlMessageGetter<undefined>>(message: T): string {
    // @ts-expect-error Figure out how to make this typing exact. This
    // currently relies on the generic typing being sound enough to know that
    // the message can only contain a single static text node and no
    // placeholders or rich text.
    return message(this.currentLocale).ast[0].value;
  }

  bindFormatValues<T>(
    Builder: FormatBuilderConstructor<T>,
    message: InternalIntlMessage,
    values: Record<string, any>,
  ): T[] {
    return bindFormatValues(
      Builder,
      message.ast,
      [this.currentLocale, this.defaultLocale],
      this.intl.formatters as Formatters,
      this.intl.formats as Formats,
      values,
    );
  }
}
